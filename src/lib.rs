use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_ecs_tilemap::prelude::*;
use leafwing_input_manager::{
    errors::NearlySingularConversion, orientation::Direction, prelude::*,
};

mod monster;
use crate::monster::*;

pub const LAUNCHER_TITLE: &str = "bevy-rogue";

pub fn app() -> App {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
            title: LAUNCHER_TITLE.to_string(),
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::input::InputPlugin)
        .add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .add_plugin(TilemapPlugin)
        .add_startup_system(setup)
        .add_startup_system(setup_tiles)
        .add_startup_system(spawn_monsters)
        .add_startup_system(spawn_player)
        .add_event::<WalkEvent>()
        .add_system(player_input)
        .add_system(move_entities);

    app
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

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.0 / 2.0,
            ..default()
        },
        ..default()
    });
}

fn setup_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("tiles/tilemap_packed.png");

    let tilemap_size = TilemapSize { x: 16, y: 16 };

    let tilemap_entity = commands.spawn().id();
    let mut tile_storage = TileStorage::empty(tilemap_size);

    let tiles = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 0],
        [0, 13, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 15, 0],
        [0, 13, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 52, 15, 0],
        [0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
        [0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
        [0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
        [0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
        [0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
        [0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
        [0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
        [0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
        [0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
        [0, 13, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 15, 0],
        [0, 25, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 27, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_id = tiles[15 - y as usize][x as usize];
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture: TileTexture(tile_id),
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

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size,
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: get_tilemap_center_transform(&tilemap_size, &grid_size, 0.0),
            ..default()
        });
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Walkable;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    walkable: Walkable,
    #[bundle]
    input_manager: InputManagerBundle<PlayerAction>,
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
            walkable: Walkable,
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::default_input_map(),
                ..default()
            }
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("tiles/tilemap_packed.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 12, 12);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(PlayerBundle::default())
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: 97,
                ..default()
            },
            transform: Transform::from_xyz(-16.0, -16.0, 1.0),
            ..default()
        });
}

#[derive(Debug)]
pub struct WalkEvent {
    pub target: Entity,
    pub direction: Direction,
}

fn player_input(
    query: Query<(Entity, &ActionState<PlayerAction>), With<Player>>,
    monsters: Query<Entity, (With<Monster>, With<Walkable>)>,
    mut event_writer: EventWriter<WalkEvent>,
) {
    let (e, action_state) = query.single();

    let mut direction_vector = Vec2::ZERO;

    for input_direction in PlayerAction::DIRECTIONS {
        if action_state.just_pressed(input_direction) {
            if let Some(direction) = input_direction.direction() {
                direction_vector += Vec2::from(direction);
            }
        }
    }

    let net_direction: Result<Direction, NearlySingularConversion> = direction_vector.try_into();

    if let Ok(direction) = net_direction {
        event_writer.send(WalkEvent { target: e, direction });

        for monster in monsters.iter() {
            event_writer.send(WalkEvent { target: monster, direction });
        }
    }
}

fn move_entities(
    mut query: Query<&mut Transform, With<Walkable>>,
    mut event_reader: EventReader<WalkEvent>,
) {
    for event in event_reader.iter() {
        if let Ok(mut transform) = query.get_mut(event.target) {
            transform.translation += (event.direction * 16.0).extend(0.0);
        }
    }
}