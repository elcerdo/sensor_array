use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use rand::Rng;

// --- Plugin ---

pub struct IncomingProjectilePlugin;

impl Plugin for IncomingProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_random);
        app.add_systems(
            Update,
            (
                tick_elapsed,
                despawn_elapsed,
                spawn_random,
                draw_segment_and_dot,
            ),
        );
    }
}

// --- Components ---

const BLAST_SPEED: f32 = 400.0;
const BLAST_LINGER: f32 = 0.2;

#[derive(Component)]
struct IncomingProjectile {
    pub aa: Vec2,
    pub bb: Vec2,
    pub speed: f32,
    pub radius: f32,
    pub elapsed: f32,
}

impl IncomingProjectile {
    fn current_position(&self) -> Vec2 {
        let delta = self.bb - self.aa;
        assert!(delta.length() > 0.0);
        assert!(self.speed > 0.0);
        let xx = (self.speed * self.elapsed).clamp(0.0, delta.length());
        self.aa + xx * delta.normalize()
    }

    fn time_to_target(&self) -> f32 {
        let delta = self.bb - self.aa;
        assert!(delta.length() > 0.0);
        assert!(self.speed > 0.0);
        self.elapsed - delta.length() / self.speed
    }

    fn direction_angle(&self) -> f32 {
        let delta = self.bb - self.aa;
        assert!(delta.length() > 0.0);
        delta.to_angle()
    }
}

// /// Marks the text entity that displays the clock face.
// #[derive(Component)]
// struct ClockDisplay;

// --- Setup ---

fn spawn_random(mut commands: Commands, projectiles: Query<&IncomingProjectile>) {
    let num_current_projectiles = projectiles.iter().len();
    let num_target_projectiles = 256;

    if num_current_projectiles >= num_target_projectiles {
        return;
    }

    let mut rng = rand::thread_rng();
    for _ in num_current_projectiles..num_target_projectiles {
        let aa = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        let bb = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        let aa = Vec2::splat(200.0) * aa - Vec2::X * 325.0;
        let bb = Vec2::splat(300.0) * bb + Vec2::X * 225.0;
        let speed = rng.gen_range(50.0..150.0);
        let radius = rng.gen_range(20.0..40.0);
        commands.spawn(IncomingProjectile {
            aa,
            bb,
            speed,
            radius,
            elapsed: 0.0,
        });
    }

    // // Spawn the on-screen HH:MM:SS display
    // commands.spawn((
    //     ClockDisplay,
    //     Text::new("00:00:00"),
    //     TextFont {
    //         font_size: 72.0,
    //         ..default()
    //     },
    //     TextColor(Color::srgb(0.2, 0.9, 0.4)),
    //     Node {
    //         position_type: PositionType::Absolute,
    //         top: Val::Px(40.0),
    //         left: Val::Px(0.0),
    //         right: Val::Px(0.0),
    //         ..default()
    //     },
    // ));

    // // Speed hint label
    // commands.spawn((
    //     Text::new("Speed: 1x  |  [ / ] to adjust"),
    //     TextFont {
    //         font_size: 24.0,
    //         ..default()
    //     },
    //     TextColor(Color::srgb(0.7, 0.7, 0.7)),
    //     Node {
    //         position_type: PositionType::Absolute,
    //         top: Val::Px(130.0),
    //         left: Val::Px(0.0),
    //         right: Val::Px(0.0),
    //         ..default()
    //     },
    // ));
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

fn draw_segment_and_dot(mut gizmos: Gizmos, projectiles: Query<&IncomingProjectile>) {
    gizmos.line_2d(
        -500.0 * Vec2::Y - 100.0 * Vec2::X,
        500.0 * Vec2::Y - 100.0 * Vec2::X,
        Color::WHITE,
    );

    for projectile in projectiles.iter() {
        gizmos.line_2d(projectile.aa, projectile.bb, tailwind::GRAY_600);
    }

    for projectile in projectiles.iter() {
        gizmos.cross_2d(
            Isometry2d::from_translation(projectile.aa),
            5.0,
            tailwind::AMBER_200,
        );
        alt_cross_2d(&mut gizmos, projectile.bb, 5.0, tailwind::LIME_200);
    }

    for projectile in projectiles.iter() {
        let tt = projectile.time_to_target();
        let ii = Isometry2d::new(
            projectile.current_position(),
            projectile.direction_angle().into(),
        );
        if tt < 0.0 {
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
            let radius = (tt * BLAST_SPEED).min(projectile.radius);
            gizmos.circle_2d(ii, radius, tailwind::RED_200);
        }
    }
}

fn despawn_elapsed(mut commands: Commands, projectiles: Query<(Entity, &IncomingProjectile)>) {
    for (entity, projectile) in projectiles.iter() {
        if projectile.time_to_target() > projectile.radius / BLAST_SPEED + BLAST_LINGER {
            commands.entity(entity).despawn();
        }
    }
}

// /// Writes the formatted HH:MM:SS string to the ClockDisplay text entity.
// fn update_clock_display(
//     clock_query: Query<&IncomingProjectile>,
//     mut display_query: Query<&mut Text, With<ClockDisplay>>,
// ) {
//     let Ok(clock) = clock_query.single() else {
//         return;
//     };
//     let Ok(mut text) = display_query.single_mut() else {
//         return;
//     };

//     let total = clock.elapsed as u32;
//     let h = total / 3600;
//     let m = (total % 3600) / 60;
//     let s = total % 60;

//     **text = format!("{:02}:{:02}:{:02}", h, m, s);
// }
