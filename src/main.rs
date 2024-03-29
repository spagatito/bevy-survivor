use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (load_assets, spawn_camera, spawn_player).chain())
        .add_systems(Update, (player_movment, velocity_move).chain()) // Add velocity move
        .insert_resource(ClearColor(Color::rgb(0.2, 0.0, 0.15)))
        .init_resource::<SpriteAssets>()
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
pub struct PlayerTag;

pub fn spawn_player(mut commands: Commands, sprite_assets: Res<SpriteAssets>) {
    commands
        .spawn((
            PlayerTag,
            Velocity::default(),
            Name::new("Player"),
            SpatialBundle::from_transform(Transform::from_translation(Vec3::ZERO)),
        ))
        .with_children(|player| {
            player.spawn((SpriteBundle {
                texture: sprite_assets.knight.clone(),
                transform: Transform::from_translation(Vec3::Y),
                ..Default::default()
            },));
        });
}

#[derive(Resource, Debug, Default)]
pub struct SpriteAssets {
    pub knight: Handle<Image>,
}

fn load_assets(mut sprite_assets: ResMut<SpriteAssets>, asset_server: Res<AssetServer>) {
    *sprite_assets = SpriteAssets {
        knight: asset_server.load("knight.png"),
    }
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub linvel: Vec2,
}

impl Default for Velocity {
    fn default() -> Self {
        Self { linvel: Vec2::ZERO }
    }
}

pub fn player_movment(
    mut query: Query<&mut Velocity, With<PlayerTag>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::ZERO;
    for key in input.get_pressed() {
        match key {
            KeyCode::KeyW | KeyCode::ArrowUp => direction += Vec2::Y,
            KeyCode::KeyS | KeyCode::ArrowDown => direction -= Vec2::Y,
            KeyCode::KeyD | KeyCode::ArrowRight => direction += Vec2::X,
            KeyCode::KeyA | KeyCode::ArrowLeft => direction -= Vec2::X,
            _ => {}
        }
    }

    for mut velocity in &mut query {
        *velocity = Velocity {
            linvel: direction.normalize_or_zero(),
            ..default()
        };
    }
}

pub fn velocity_move(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in &mut query {
        transform.translation += velocity.linvel.extend(0.0) * time.delta_seconds() * 50.;
    }
}
