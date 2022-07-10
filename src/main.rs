use bevy::prelude::*;
use rand::seq::SliceRandom;

const SPRITE_SCALE: f32 = 0.2;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Darkstone".to_string(),
            width: 900.0,
            height: 1200.0,
            ..Default::default()
        })
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Res<WindowDescriptor>,
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let cards: Vec<HandleUntyped> = asset_server.load_folder("cards").unwrap();
    let tile_size = tile_size(window, cards.len());

    let board_size = (5.0, 5.0);

    let board_position = Vec3::new(-(board_size.0 / 2.), -(board_size.1 / 2.), 0.);

    commands
        .spawn()
        .insert(Transform::from_translation(board_position))
        .insert(GlobalTransform::default())
        .with_children(|parent| {
            for x in 1..(board_size.0 as u16) {
                for y in 1..(board_size.1 as u16) {
                    let random_card = cards
                        .choose(&mut rand::thread_rng())
                        .unwrap()
                        .clone()
                        .typed();
                    // add card
                    parent.spawn_bundle(SpriteBundle {
                        texture: random_card,
                        transform: Transform {
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                            translation: Vec3::new(
                                (x as f32 * tile_size) - (tile_size / 2.),
                                (y as f32 * tile_size) - (tile_size / 2.),
                                1.,
                            ),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                }
            }
        });
}

fn tile_size(window: Res<WindowDescriptor>, card_count: usize) -> f32 {
    window.width / card_count as f32
}
