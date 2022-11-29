use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::Moveable;

#[derive(Component)]
pub struct Monster;

pub fn spawn_monsters(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("tiles/tilemap_packed.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 12, 12, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let sprites = [
        (110, Transform::from_xyz(160.0, 128.0, 1.0)),
        (110, Transform::from_xyz(64.0, 96.0, 1.0)),
        (123, Transform::from_xyz(96.0, 64.0, 1.0)),
    ];

    for (sprite_index, transform) in sprites.iter() {
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: *sprite_index,
                    ..default()
                },
                transform: *transform,
                ..default()
            })
            .insert(Monster)
            .insert(Moveable { tile_pos: TilePos { x: 0, y: 0 }});
    }
}