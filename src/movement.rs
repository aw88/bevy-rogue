use bevy::prelude::*;
use bevy_easings::*;
use leafwing_input_manager::orientation::Direction;

#[derive(Component)]
pub struct Moveable;

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
    mut query: Query<&Transform, With<Moveable>>,
    mut event_reader: EventReader<MoveEvent>,
) {
    for event in event_reader.iter() {
        if let Ok(transform) = query.get_mut(event.target) {
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
