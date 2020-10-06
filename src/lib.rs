// publics
pub use self::
{
    inputmap::InputMap,

    axis::Axis,
    axis::AnalogDirection,
    
    bindings::Bindings,
    
    eventphase::EventPhase,
    
    actionevent::OnActionBegin,
    actionevent::OnActionActive,
    actionevent::OnActionProgress,
    actionevent::OnActionEnd,
};

// crates
mod axis;
mod util;
mod stack;
mod inputmap;
mod bindings;
mod eventphase;
mod actionevent;

mod action;
mod gamepad;
mod keyboard;
mod mouse;
mod serde;

use bevy_app::prelude::*;
use bevy_ecs::IntoQuerySystem;

#[derive(Default)]
pub struct InputMapPlugin;

impl Plugin for InputMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // input map
            .init_resource::<InputMap>()
            // events
            .add_event::<OnActionActive>()
            .add_event::<OnActionBegin>()
            .add_event::<OnActionProgress>()
            .add_event::<OnActionEnd>()
            .add_system_to_stage(stage::EVENT_UPDATE, InputMap::action_event_producer.system())
            // reset
            .add_system_to_stage(stage::PRE_UPDATE, InputMap::action_reset_system.system())
            // joystick
            .add_system_to_stage(
                stage::UPDATE,
                InputMap::gamepad_connection_event_system.system(),
            )
            .add_system_to_stage(
                stage::UPDATE,
                InputMap::gamepad_button_press_input_system.system(),
            )
            .add_system_to_stage(stage::UPDATE, InputMap::gamepad_axis_system.system())
            // keyboard
            .add_system_to_stage(stage::UPDATE, InputMap::kb_key_press_input_system.system())
            // mouse
            .add_system_to_stage(
                stage::UPDATE,
                InputMap::mouse_button_press_input_system.system(),
            )
            .add_system_to_stage(stage::UPDATE, InputMap::mouse_move_event_system.system());
    }
}
