//! In this example we generate a new texture atlas (sprite sheet) from a folder containing
//! individual sprites.

use bevy::prelude::*;
use bevy::log::{Level, LogPlugin};
use bevy::render::color::Color;

use rand::{thread_rng, Rng};

use noise::{utils::*, BasicMulti, Perlin};

mod plugins;
pub mod common;

use crate::common::AppState;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(LogPlugin{
                level: Level::DEBUG,
                filter: "wgpu=error,naga=error,bevy_render=error,bevy_app=info,bevy_ecs=info".to_string()
            }),
            plugins::camera::CameraPlugin
        )) // prevents blurry sprites
        .add_state::<AppState>()
        .add_systems(OnEnter(AppState::Setup), generate_world)
        .run();
}


fn generate_noise_map() -> NoiseMap {
    let mut rng = thread_rng();
    let seed: u32 = rng.gen();

    let basicmulti = BasicMulti::<Perlin>::new(seed);

    PlaneMapBuilder::<_, 2>::new(&basicmulti)
        // .set_size(100, 100)
        // .set_x_bounds(-5.0, 5.0)
        // .set_y_bounds(-5.0, 5.0)
        .build()
}

fn get_color(val: f64) -> Color {
    let color_result = match val.abs() {
        v if v < 0.1 => Color::hex("#0a7e0a"),
        v if v < 0.2 => Color::hex("#0da50d"),
        v if v < 0.3 => Color::hex("#10cb10"),
        v if v < 0.4 => Color::hex("#18ed18"),
        v if v < 0.5 => Color::hex("#3ff03f"),
        v if v < 0.6 => Color::hex("#65f365"),
        v if v < 0.7 => Color::hex("#8cf68c"),
        v if v < 0.8 => Color::hex("#b2f9b2"),
        v if v < 0.9 => Color::hex("#d9fcd9"),
        v if v <= 1.0 => Color::hex("#ffffff"),
        _ => panic!("unexpected value")
    };
    color_result.expect("Getting color from HEX error")
}

fn generate_world(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>
) {
    let map = generate_noise_map();
    let (grid_width, grid_height) = map.size();
    debug!("Map size: {}x{}", grid_width, grid_height);

    let tile_size = 64_f32;

    let start_x = -(grid_width as f32) * tile_size / 2.0;
    let start_y = -(grid_height as f32) * tile_size / 2.0;

    for col_x in 0..grid_width {
        for col_y in 0..grid_height {
            let val = map.get_value(col_x, col_y);
            // if val > 0.8_f64 {
                // debug!("Value for {}:{} = {}", col_x, col_y, val);
            // }
            let x = start_x + col_x as f32 * tile_size;
            let y = start_y + col_y as f32 * tile_size;

            commands.spawn(
                SpriteBundle {
                    sprite: Sprite {
                        color: get_color(val),
                        custom_size: Some(Vec2::new(tile_size, tile_size)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                    ..default()
                }
            );
        }
    }
     next_state.set(AppState::Finished);
}
