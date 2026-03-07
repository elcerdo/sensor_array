use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use rand::Rng;

// --- Plugin ---

pub struct IncomingProjectilePlugin;

impl Plugin for IncomingProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_random)
            .add_systems(Update, (tick_clock, draw_segment_and_dot));
    }
}

// --- Components ---

/// Tracks elapsed time and controls the speed of the segment clock.
#[derive(Component)]
struct IncomingProjectile {
    pub aa: Vec2,
    pub bb: Vec2,
    pub speed: f32,
    pub elapsed: f32,
}

impl IncomingProjectile {
    fn current_position(&self) -> Vec2 {
        let delta = self.bb - self.aa;
        let xx = (self.speed * self.elapsed).clamp(0.0, delta.length());
        self.aa + xx * delta.normalize()
    }
}

// /// Marks the text entity that displays the clock face.
// #[derive(Component)]
// struct ClockDisplay;

// --- Setup ---

fn init_random(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    let wh = Vec2::new(400.0, 300.0);
    for _ in 0..10 {
        let aa = wh * Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        let bb = wh * Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        let speed = rng.gen_range(50.0..150.0);
        commands.spawn(IncomingProjectile {
            aa,
            bb,
            speed,
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

// --- Systems ---

fn tick_clock(time: Res<Time>, mut projectiles: Query<&mut IncomingProjectile>) {
    for mut projectile in projectiles.iter_mut() {
        projectile.elapsed += time.delta_secs();
    }
}

fn draw_segment_and_dot(mut gizmos: Gizmos, projectiles: Query<&IncomingProjectile>) {
    for projectile in projectiles.iter() {
        gizmos.line_2d(projectile.aa, projectile.bb, tailwind::GRAY_600);
        gizmos.circle_2d(
            Isometry2d::from_translation(projectile.aa),
            5.0,
            tailwind::AMBER_200,
        );
        gizmos.circle_2d(
            Isometry2d::from_translation(projectile.bb),
            5.0,
            tailwind::LIME_200,
        );
        gizmos.circle_2d(
            Isometry2d::from_translation(projectile.current_position()),
            5.0,
            Color::WHITE,
        );
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
