use bevy::prelude::*;

const CARD: &str = "four.png";
const SPRITE_SCALE: f32 = 0.5;

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

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // add card
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(CARD),
        transform: Transform {
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}
