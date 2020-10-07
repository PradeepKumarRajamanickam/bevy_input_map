use crate::{AnalogDirection, InputMap};

use bevy::{
    prelude::Gamepad, prelude::GamepadAxis, prelude::GamepadAxisType, prelude::GamepadButton,
    prelude::GamepadButtonType, prelude::GamepadEvent,
};
use bevy_app::{EventReader, Events};
use bevy_ecs::{Local, Res, ResMut};
use bevy_input::Input;

#[derive(Default)]
pub struct GamepadState {
    reader: EventReader<GamepadEvent>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GamepadAnalog {
    pub axis: GamepadAxis,
    pub direction: AnalogDirection,
}

impl InputMap {
    // publics
    // buttons
    pub fn bind_gamepad_button_pressed(
        &mut self,
        pad_button: GamepadButtonType,
        action: &str,
    ) -> &mut InputMap {
        self.bind_gamepad_button_pressed_with_player(0, pad_button, action)
    }
    pub fn bind_gamepad_button_pressed_with_player(
        &mut self,
        player: usize,
        pad_button: GamepadButtonType,
        action: &str,
    ) -> &mut InputMap {
        self.joystick_button_binding.entry((player, pad_button)).or_insert(action.to_string());
        self
    }
    pub fn unbind_gamepad_button_pressed(
        &mut self,
        pad_button: GamepadButtonType,
    ) -> &mut InputMap {
        self.unbind_gamepad_button_pressed_with_player(0, pad_button)
    }
    pub fn unbind_gamepad_button_pressed_with_player(
        &mut self,
        player: usize,
        pad_button: GamepadButtonType,
    ) -> &mut InputMap {
        self.joystick_button_binding.remove(&(player, pad_button));
        self
    }

    // axis
    pub fn bind_gamepad_axis(
        &mut self,
        axis_type: GamepadAxisType,
        analog_direction: AnalogDirection,
        action: &str,
    ) -> &mut InputMap {
        self.bind_gamepad_axis_with_handle(0, axis_type, analog_direction, action)
    }
    pub fn bind_gamepad_axis_with_axis(
        &mut self,
        pad_axis: GamepadAxis,
        analog_direction: AnalogDirection,
        action: &str,
    ) -> &mut InputMap {
        self.joystick_axis_binding.insert(
            GamepadAnalog {
                axis: pad_axis,
                direction: analog_direction,
            },
            action.to_string(),
        );
        self
    }

    pub fn bind_gamepad_axis_with_handle(
        &mut self,
        pad_handle: usize,
        axis_type: GamepadAxisType,
        analog_direction: AnalogDirection,
        action: &str,
    ) -> &mut InputMap {
        self.bind_gamepad_axis_with_axis(
            GamepadAxis(Gamepad(pad_handle), axis_type),
            analog_direction,
            action,
        )
    }

    pub fn unbind_gamepad_axis(
        &mut self,
        axis_type: GamepadAxisType,
        analog_direction: AnalogDirection,
    ) -> &mut InputMap {
        self.unbind_gamepad_axis_with_handle(0, axis_type, analog_direction)
    }
    pub fn unbind_gamepad_axis_with_axis(
        &mut self,
        pad_axis: GamepadAxis,
        analog_direction: AnalogDirection,
    ) -> &mut InputMap {
        self.joystick_axis_binding.remove(&GamepadAnalog {
            axis: pad_axis,
            direction: analog_direction,
        });
        self
    }

    pub fn unbind_gamepad_axis_with_handle(
        &mut self,
        pad_handle: usize,
        axis_type: GamepadAxisType,
        analog_direction: AnalogDirection,
    ) -> &mut InputMap {
        self.unbind_gamepad_axis_with_axis(
            GamepadAxis(Gamepad(pad_handle), axis_type),
            analog_direction,
        );
        self
    }

    // crates
    pub(crate) fn get_available_player_handle(self) -> Option<usize>
    {
        let max_player_handles: usize = 8;
        for i in 0..(max_player_handles - 1)
        {
            if !self.player_handles_in_use.contains(&i) {
               return Some(i);
            }
        }
        None
    }
    pub(crate) fn get_player_handle_for_gamepad(self, pad: Gamepad) -> Option<usize>
    {
        return match self.joystick_to_player_map.get(&pad) {
            Some(a) => Some(*a),
            _ => None
        };
    }
    pub(crate) fn get_gamepad_from_player_handle(self, player: usize) -> Option<Gamepad>
    {
        return match self.player_to_joystick_map.get(&player) {
            Some(a) => Some(*a),
            _ => None
        };
    }
    // systems
    pub(crate) fn gamepad_button_press_input_system(
        mut input_map: ResMut<InputMap>,
        joystick_button_input: Res<Input<GamepadButton>>,
    ) {
        let button_bindings_iter = input_map.joystick_button_binding.clone();
        for (player_button_bind, action) in button_bindings_iter.iter() {
            if joystick_button_input.pressed(GamepadButton(Gamepad(player_button_bind.0), player_button_bind.1)) {
                input_map.set_raw_action_strength(action, 1.0);
            }
        }
    }
    pub(crate) fn gamepad_connection_event_system(
        mut input_map: ResMut<InputMap>,
        gamepad_event: Res<Events<GamepadEvent>>,
        mut state: Local<GamepadState>,
    ) {
        if let Some(value) = state.reader.latest(&gamepad_event) {
            let pad_handle = value.0;
            match value.1 {
                bevy::prelude::GamepadEventType::Connected => {

                    let res_player_handle = input_map.clone().get_available_player_handle();
                    match res_player_handle {

                        Some(player_handle) => 
                        {
                            println!("InputMap: Gamepad Connected {:?} to player {}", pad_handle, player_handle);
                            input_map.player_handles_in_use.insert(player_handle);
                            input_map.joystick_to_player_map.insert(pad_handle,player_handle);
                            input_map.player_to_joystick_map.insert(player_handle,pad_handle);
                        }
                        None => { println!("InputMap: Housefull. No space for more gamepads"); }
                    }
                        
                }
                bevy::prelude::GamepadEventType::Disconnected => {

                    let opt_player_handle = input_map.clone().get_player_handle_for_gamepad(pad_handle);
                    if let Some(player_handle) = opt_player_handle
                    {
                        println!("InputMap: Gamepad Disconnected {:?} for player {}", pad_handle, player_handle);
                        input_map.player_handles_in_use.remove(&player_handle);
                        input_map.joystick_to_player_map.remove(&pad_handle);
                        input_map.player_to_joystick_map.remove(&player_handle);
                    }
                }
            }
        }
    }

    pub(crate) fn gamepad_axis_system(
        mut input_map: ResMut<InputMap>,
        pad_axis: Res<bevy_input::Axis<GamepadAxis>>,
    ) {
        for (k, v) in input_map.joystick_axis_binding.clone().iter() {
            let g_axis = k.axis;
            let a_dir = k.direction;

            let signed_str = pad_axis.get(&g_axis).unwrap_or(0.);

            if signed_str > 0. && a_dir == AnalogDirection::Positve {
                input_map.set_raw_action_strength(&v.to_string(), signed_str);
            } else if signed_str < 0. && a_dir == AnalogDirection::Negative {
                input_map.set_raw_action_strength(&v.to_string(), signed_str.abs());
            }
        }
    }
}
