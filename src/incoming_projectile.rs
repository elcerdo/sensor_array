use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use rand::Rng;
use rand_distr::Distribution;
use rstar::{RTree, RTreeObject, AABB};

use crate::sink_and_source::{Sink, Source};

// --- Resources ---

#[derive(Resource)]
pub struct IncomingConfig {
    pub show_trajectories: bool,
    pub show_projectiles: bool,
    pub num_target_projectiles: usize,
    pub num_spawned_projectiles: usize,
}

impl Default for IncomingConfig {
    fn default() -> Self {
        Self {
            show_trajectories: false,
            show_projectiles: true,
            num_target_projectiles: 256,
            num_spawned_projectiles: 0,
        }
    }
}

// --- Plugin ---

pub struct IncomingProjectilePlugin;

impl Plugin for IncomingProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IncomingConfig>();
        app.add_systems(
            Update,
            (
                tick_elapsed,
                despawn_elapsed,
                spawn_random,
                draw_segment_and_dot,
                score_hits,
            ),
        );
    }
}

// --- Components ---

const BLAST_SPEED: f32 = 400.0;
const BLAST_LINGER: f32 = 0.2;

#[derive(Component)]
pub(crate) struct IncomingProjectile {
    emit_pos: Vec2,
    target_pos: Vec2,
    speed: f32,
    radius: f32,
    elapsed: f32,
}

impl IncomingProjectile {
    fn current_position(&self) -> Vec2 {
        let delta = self.target_pos - self.emit_pos;
        assert!(delta.length() > 0.0);
        assert!(self.speed > 0.0);
        let xx = (self.speed * self.elapsed).clamp(0.0, delta.length());
        self.emit_pos + xx * delta.normalize()
    }

    fn time_to_target(&self) -> f32 {
        let delta = self.target_pos - self.emit_pos;
        assert!(delta.length() > 0.0);
        assert!(self.speed > 0.0);
        self.elapsed - delta.length() / self.speed
    }

    fn direction_angle(&self) -> f32 {
        let delta = self.target_pos - self.emit_pos;
        assert!(delta.length() > 0.0);
        delta.to_angle()
    }

    fn should_despawn(&self) -> bool {
        self.time_to_target() > self.radius / BLAST_SPEED + BLAST_LINGER
    }

    fn blast_radius(&self) -> f32 {
        let tt = self.time_to_target();
        assert!(tt >= 0.0);
        (tt * BLAST_SPEED).min(self.radius)
    }
}

// --- Setup ---

fn spawn_random(
    mut commands: Commands,
    projectiles: Query<&IncomingProjectile>,
    mut state: ResMut<IncomingConfig>,
    sources: Query<(&Transform, &Source)>,
    sinks: Query<&Transform, With<Sink>>,
) {
    state.num_spawned_projectiles = 0;

    let num_current_projectiles = projectiles.iter().len();
    if num_current_projectiles >= state.num_target_projectiles {
        return;
    }

    let sources_data: Vec<(Vec2, &Source)> = sources
        .iter()
        .map(|(t, s)| (t.translation.truncate(), s))
        .collect();
    let target_positions: Vec<Vec2> = sinks.iter().map(|t| t.translation.truncate()).collect();
    if sources_data.is_empty() || target_positions.is_empty() {
        return;
    }

    let num_spawned_projectiles = (state.num_target_projectiles - num_current_projectiles).min(32);
    state.num_spawned_projectiles = num_spawned_projectiles;

    let mut rng = rand::thread_rng();
    for _ in 0..num_spawned_projectiles {
        let (emit_pos, source) = sources_data[rng.gen_range(0..sources_data.len())];
        let target_pos = target_positions[rng.gen_range(0..target_positions.len())];
        let target_pos =
            target_pos + Vec2::new(source.dist.sample(&mut rng), source.dist.sample(&mut rng));
        let speed = rng.gen_range(50.0..150.0);
        let radius = rng.gen_range(20.0..40.0);
        commands.spawn(IncomingProjectile {
            emit_pos,
            target_pos,
            speed,
            radius,
            elapsed: 0.0,
        });
    }
}

// --- Update ---

fn tick_elapsed(time: Res<Time>, mut projectiles: Query<&mut IncomingProjectile>) {
    for mut projectile in projectiles.iter_mut() {
        projectile.elapsed += time.delta_secs();
    }
}

fn alt_cross_2d(gizmos: &mut Gizmos, center: Vec2, radius: f32, color: impl Into<Color>) {
    let aa = radius * (Vec2::X + Vec2::Y);
    let bb = radius * (Vec2::X - Vec2::Y);
    let color: Color = color.into();
    gizmos.line_2d(center - aa, center + aa, color);
    gizmos.line_2d(center - bb, center + bb, color);
}

fn draw_segment_and_dot(
    mut gizmos: Gizmos,
    projectiles: Query<&IncomingProjectile>,
    state: Res<IncomingConfig>,
) {
    gizmos.line_2d(
        -500.0 * Vec2::Y - 100.0 * Vec2::X,
        500.0 * Vec2::Y - 100.0 * Vec2::X,
        Color::WHITE,
    );

    if state.show_trajectories {
        for projectile in projectiles.iter() {
            gizmos.line_2d(
                projectile.emit_pos,
                projectile.target_pos,
                tailwind::GRAY_600,
            );
        }

        for projectile in projectiles.iter() {
            gizmos.cross_2d(
                Isometry2d::from_translation(projectile.emit_pos),
                5.0,
                tailwind::AMBER_200,
            );
            alt_cross_2d(&mut gizmos, projectile.target_pos, 5.0, tailwind::LIME_200);
        }
    }

    if state.show_projectiles {
        for projectile in projectiles.iter() {
            let ii = Isometry2d::new(
                projectile.current_position(),
                projectile.direction_angle().into(),
            );
            if projectile.time_to_target() < 0.0 {
                gizmos.primitive_2d(
                    &Triangle2d::new(
                        Vec2::new(5.0, 0.0),
                        Vec2::new(-5.0, 3.0),
                        Vec2::new(-5.0, -3.0),
                    ),
                    ii,
                    tailwind::RED_200,
                );
            } else {
                let radius = projectile.blast_radius();
                gizmos.circle_2d(ii, radius, tailwind::RED_200);
            }
        }
    }
}

fn despawn_elapsed(mut commands: Commands, projectiles: Query<(Entity, &IncomingProjectile)>) {
    for (entity, projectile) in projectiles.iter() {
        if projectile.should_despawn() {
            commands.entity(entity).despawn();
        }
    }
}

struct ProjectileEntry {
    target_pos: Vec2,
    blast_radius: f32,
}

impl RTreeObject for ProjectileEntry {
    type Envelope = AABB<[f32; 2]>;
    fn envelope(&self) -> Self::Envelope {
        let blast_radius = self.blast_radius;
        AABB::from_corners(
            [
                self.target_pos.x - blast_radius,
                self.target_pos.y - blast_radius,
            ],
            [
                self.target_pos.x + blast_radius,
                self.target_pos.y + blast_radius,
            ],
        )
    }
}

fn score_hits(mut sinks: Query<(&Transform, &mut Sink)>, projectiles: Query<&IncomingProjectile>) {
    let entries: Vec<ProjectileEntry> = projectiles
        .iter()
        .filter_map(|projectile| {
            if projectile.should_despawn() {
                return None;
            }
            let time_to_target = projectile.time_to_target();
            if time_to_target < 0.0 {
                return None;
            }
            Some(ProjectileEntry {
                target_pos: projectile.target_pos,
                blast_radius: projectile.blast_radius(),
            })
        })
        .collect();

    let tree = RTree::bulk_load(entries);

    for (sink_transform, mut sink) in sinks.iter_mut() {
        let sink_pos = sink_transform.translation.truncate();
        let query = AABB::from_point([sink_pos.x, sink_pos.y]);
        for entry in tree.locate_in_envelope_intersecting(&query) {
            if entry.target_pos.distance(sink_pos) <= entry.blast_radius {
                sink.hit_count += 1;
            }
        }
    }
}
