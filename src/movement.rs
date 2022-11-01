use bevy::prelude::*;
use bevy_easings::*;
use bevy_ecs_tilemap::prelude::TilePos;
use leafwing_input_manager::orientation::Direction;

#[derive(Component)]
pub struct Moveable {
    pub tile_pos: TilePos,
}

pub struct MoveEvent {
    pub target: Entity,
    pub direction: Direction,
}

pub struct MovePlugin;

impl Plugin for MovePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MoveEvent>()
            .add_system(move_entities);
    }
}

fn move_entities(
    mut commands: Commands,
    mut moveable_entity_query: Query<(&Transform, &mut Moveable)>,
    mut event_reader: EventReader<MoveEvent>,
) {
    for event in event_reader.iter() {
        if let Ok((transform, mut moveable)) = moveable_entity_query.get_mut(event.target) {
            let unit_vector = event.direction.unit_vector();
            let tile_x = (moveable.tile_pos.x as f32 + unit_vector.x) as u32;
            let tile_y = (moveable.tile_pos.y as f32 + unit_vector.y) as u32;
            
            moveable.tile_pos = TilePos { x: tile_x, y: tile_y };

            commands
                .entity(event.target)
                .insert(transform.ease_to(
                    Transform::from_translation(transform.translation + (event.direction * 16.0).extend(0.0)),
                    EaseFunction::CubicInOut,
                    EasingType::Once { duration: std::time::Duration::from_millis(200) },
                ));
        }
    }
}
