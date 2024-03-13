use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_player).chain())
        .insert_resource(ClearColor(Color::rgb(0.2, 0.0, 0.15)))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}


#[derive(Component)]
pub struct PlayerTag;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        PlayerTag,
        Name::new("Player"),
    ));
}