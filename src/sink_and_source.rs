use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use rand::Rng;
use rand_distr::Normal;

const SHAPE_SIZE: f32 = 25.0;
const Z_LAYER: f32 = 0.0;

fn from_translation_2d(pos: Vec2) -> Transform {
    Transform::from_xyz(pos.x, pos.y, Z_LAYER)
}

// --- Plugin ---

pub struct SinkAndSourcePlugin;

impl Plugin for SinkAndSourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sink_and_source);
        app.add_systems(Update, (randomize_on_r, update_sink_labels));
    }
}

// --- Components ---

#[derive(Component, Default)]
pub struct Sink {
    pub hit_count: u32,
}

#[derive(Component)]
struct SinkLabel;

#[derive(Component)]
pub struct Source {
    pub dist: Normal<f32>,
}

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
        commands
            .spawn((
                Sink::default(),
                Mesh2d(meshes.add(Circle::new(SHAPE_SIZE))),
                MeshMaterial2d(sink_mat.clone()),
                from_translation_2d(position),
            ))
            .with_child((
                SinkLabel,
                Text2d::new("0"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::BLACK),
                Transform::from_xyz(0.0, 0.0, 1.0),
            ));
    }

    for _ in 0..10 {
        let raw = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        let position = Vec2::splat(200.0) * raw - Vec2::X * 325.0;
        let dist = match rng.gen_range(0.0..1.0) {
            x if x < 0.20 => 30.0,
            _ => 10.0,
        };
        let dist = Normal::new(0.0, dist).unwrap();
        commands.spawn((
            Source { dist },
            Mesh2d(meshes.add(Rectangle::from_size(Vec2::splat(SHAPE_SIZE * 2.0)))),
            MeshMaterial2d(source_mat.clone()),
            from_translation_2d(position),
        ));
    }
}

// --- Update ---

fn update_sink_labels(
    sinks: Query<(&Sink, &Children)>,
    mut labels: Query<&mut Text2d, With<SinkLabel>>,
) {
    for (sink, children) in sinks.iter() {
        for child in children.iter() {
            if let Ok(mut text) = labels.get_mut(*child) {
                **text = sink.hit_count.to_string();
            }
        }
    }
}

fn randomize_on_r(
    keys: Res<ButtonInput<KeyCode>>,
    mut sinks: Query<&mut Transform, (With<Sink>, Without<Source>)>,
    mut sources: Query<&mut Transform, (With<Source>, Without<Sink>)>,
) {
    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }
    let mut rng = rand::thread_rng();
    for mut transform in sinks.iter_mut() {
        let raw = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        let position = Vec2::splat(300.0) * raw + Vec2::X * 225.0;
        *transform = from_translation_2d(position);
    }
    for mut transform in sources.iter_mut() {
        let raw = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        let position = Vec2::splat(200.0) * raw - Vec2::X * 325.0;
        *transform = from_translation_2d(position);
    }
}
