use bevy::prelude::*;
use bevy_easings::{EasingComponent, EasingsPlugin};
use bevy_ecs_tilemap::prelude::*;
use leafwing_input_manager::{
    errors::NearlySingularConversion, orientation::Direction, prelude::*,
};

mod monster;
mod movement;

use crate::monster::*;
use crate::movement::*;

pub const LAUNCHER_TITLE: &str = "bevy-rogue";

pub fn app() -> App {
    let mut app = App::new();

    app.init_resource::<RogueMap>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: LAUNCHER_TITLE.to_string(),
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .add_plugin(TilemapPlugin)
        .add_plugin(MovePlugin)
        .add_plugin(EasingsPlugin)
        .add_startup_system(setup)
        .add_startup_system(setup_tiles)
        .add_startup_system(spawn_monsters)
        .add_startup_system(spawn_player)
        .add_system(player_input);

    app
}

#[derive(Resource)]
struct RogueMap {
    tiles: Vec<Vec<u32>>,
    tiles_collision: Vec<Vec<u32>>,
    map_size: UVec2,
    tile_size: Vec2,
}

impl FromWorld for RogueMap {
    fn from_world(_world: &mut World) -> Self {
        Self {
            tiles: vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 0],
                vec![0, 13, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 15, 0],
                vec![0, 13, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 52, 15, 0],
                vec![0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
                vec![0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
                vec![0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
                vec![0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
                vec![0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
                vec![0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
                vec![0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
                vec![0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
                vec![0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
                vec![0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
                vec![0, 25, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 27, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            tiles_collision: vec![
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
                vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
                vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
                vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
                vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
                vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
                vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
                vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
                vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
                vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
                vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            ],
            map_size: UVec2 { x: 16, y: 16 },
            tile_size: Vec2 { x: 16., y: 16. },
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
}

impl PlayerAction {
    const DIRECTIONS: [Self; 4] = [
        PlayerAction::Up,
        PlayerAction::Down,
        PlayerAction::Left,
        PlayerAction::Right,
    ];

    fn direction(self) -> Option<Direction> {
        match self {
            PlayerAction::Up => Some(Direction::NORTH),
            PlayerAction::Down => Some(Direction::SOUTH),
            PlayerAction::Left => Some(Direction::WEST),
            PlayerAction::Right => Some(Direction::EAST),
        }
    }
}

fn setup(mut commands: Commands, rogue_map: Res<RogueMap>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.0 / 2.0,
            ..default()
        },
        transform: Transform::from_xyz(
            (rogue_map.map_size.x as f32 * rogue_map.tile_size.x) * 0.5,
            (rogue_map.map_size.y as f32 * rogue_map.tile_size.y) * 0.5,
            1.,
        ),
        ..default()
    });
}

fn setup_tiles(mut commands: Commands, rogue_map: Res<RogueMap>, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("tiles/tilemap_packed.png");

    let tilemap_size = TilemapSize {
        x: rogue_map.map_size.x,
        y: rogue_map.map_size.y,
    };

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(tilemap_size);

    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_id = rogue_map.tiles[15 - y as usize][x as usize];
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(tile_id),
                    flip: TileFlip {
                        x: x == 13 && tile_id == 50,
                        d: x == 13 && tile_id == 50,
                        ..default()
                    },
                    ..default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize {
        x: rogue_map.tile_size.x,
        y: rogue_map.tile_size.y,
    };
    let grid_size = tile_size.into();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: tilemap_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        ..default()
    });
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    walkable: Moveable,
    #[bundle]
    input_manager: InputManagerBundle<PlayerAction>,
    transform: Transform,
}

impl PlayerBundle {
    fn default_input_map() -> InputMap<PlayerAction> {
        use PlayerAction::*;
        let mut input_map = InputMap::default();

        input_map.insert(KeyCode::Up, Up);
        input_map.insert(KeyCode::W, Up);
        input_map.insert(GamepadButtonType::DPadUp, Up);

        input_map.insert(KeyCode::Down, Down);
        input_map.insert(KeyCode::S, Down);
        input_map.insert(GamepadButtonType::DPadDown, Down);

        input_map.insert(KeyCode::Left, Left);
        input_map.insert(KeyCode::A, Left);
        input_map.insert(GamepadButtonType::DPadLeft, Left);

        input_map.insert(KeyCode::Right, Right);
        input_map.insert(KeyCode::D, Right);
        input_map.insert(GamepadButtonType::DPadRight, Right);

        input_map
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            walkable: Moveable {
                tile_pos: TilePos { x: 2, y: 2 },
            },
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::default_input_map(),
                ..default()
            },
            transform: Transform::from_xyz(32.0, 32.0, 1.0),
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("tiles/tilemap_packed.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 12, 12, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: 97,
                ..default()
            },
            ..default()
        })
        .remove::<Transform>()
        .insert(PlayerBundle::default());
}

fn player_input(
    query: Query<
        (Entity, &Moveable, &ActionState<PlayerAction>),
        (With<Player>, Without<EasingComponent<Transform>>),
    >,
    rogue_map: Res<RogueMap>,
    mut event_writer: EventWriter<MoveEvent>,
) {
    if let Ok((e, moveable, action_state)) = query.get_single() {
        let mut direction_vector = Vec2::ZERO;

        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.just_pressed(input_direction) {
                if let Some(direction) = input_direction.direction() {
                    direction_vector += Vec2::from(direction);
                }
            }
        }

        let net_direction: Result<Direction, NearlySingularConversion> =
            direction_vector.try_into();

        if let Ok(direction) = net_direction {
            if player_can_move(moveable.tile_pos.clone(), direction, rogue_map) {
                event_writer.send(MoveEvent {
                    target: e,
                    direction,
                });
            }
        }
    }
}

fn player_can_move(tile_pos: TilePos, direction: Direction, rogue_map: Res<RogueMap>) -> bool {
    let unit_vector = direction.unit_vector();
    let tile_x = (tile_pos.x as f32 + unit_vector.x) as u32;
    let tile_y = (tile_pos.y as f32 + unit_vector.y) as u32;

    let collision_tile = rogue_map.tiles_collision[tile_y as usize][tile_x as usize];

    collision_tile == 0
}
