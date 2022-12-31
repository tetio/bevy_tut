use bevy::prelude::*;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 10.0;


fn main() {
    let height = 900.0;

    App::new()
    .insert_resource(ClearColor(CLEAR))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "bevy_tut".to_string(),
            present_mode: bevy::window::PresentMode::AutoVsync,
            resizable: false,
            ..Default::default()
        },
        ..default()
    }))   
    .add_startup_system(setup_system) 
    .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,
) {
    commands.spawn(Camera2dBundle::default());
}