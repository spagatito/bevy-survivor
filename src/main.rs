use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (load_assets, spawn_camera, spawn_player).chain())
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
    commands.spawn((
        PlayerTag,
        Name::new("Player"),
        SpatialBundle::from_transform(Transform::from_translation(Vec3::ZERO)),
    ))
    .with_children(|player| {
        player.spawn((
            SpriteBundle {
                texture: sprite_assets.knight.clone(),
                transform: Transform::from_translation(Vec3::Y),
                ..Default::default()
            },
        ));
    });
}

#[derive(Resource, Debug, Default)]
pub struct SpriteAssets {
    pub knight: Handle<Image>
}

fn load_assets(
    mut sprite_assets: ResMut<SpriteAssets>,
    asset_server: Res<AssetServer>
) {
    *sprite_assets = SpriteAssets {
        knight: asset_server.load("knight.png")
    }
}