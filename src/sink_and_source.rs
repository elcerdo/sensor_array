use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use rand::Rng;

const SHAPE_SIZE: f32 = 25.0;

// --- Plugin ---

pub struct SinkAndSourcePlugin;

impl Plugin for SinkAndSourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sink_and_source);
    }
}

// --- Components ---

#[derive(Component)]
pub struct Sink;

#[derive(Component)]
pub struct Source;

// --- Setup ---

fn spawn_sink_and_source(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    let sink_mat = materials.add(Color::from(tailwind::CYAN_400));
    let source_mat = materials.add(Color::from(tailwind::ORANGE_400));

    for _ in 0..10 {
        let raw = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        let position = Vec2::splat(300.0) * raw + Vec2::X * 225.0;
        commands.spawn((
            Sink,
            Mesh2d(meshes.add(Circle::new(SHAPE_SIZE))),
            MeshMaterial2d(sink_mat.clone()),
            Transform::from_translation(position.extend(0.0)),
        ));
    }

    for _ in 0..10 {
        let raw = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        let position = Vec2::splat(200.0) * raw - Vec2::X * 325.0;
        commands.spawn((
            Source,
            Mesh2d(meshes.add(Rectangle::from_size(Vec2::splat(SHAPE_SIZE * 2.0)))),
            MeshMaterial2d(source_mat.clone()),
            Transform::from_translation(position.extend(0.0)),
        ));
    }
}

// --- Draw ---
