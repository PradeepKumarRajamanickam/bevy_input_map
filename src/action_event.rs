use crate::{Kurinji, Actionable};

use bevy::app::Events;
use bevy::ecs::{Res, ResMut};

/// Event that is fired when action is active.
/// This depends on what event phase is set to
/// the action by default it will be OnProgress.
pub struct OnActionActive<T: Actionable> {
    pub action: T,
    pub strength: f32,
}

/// Event that gets fired at the beginning
/// of an action
pub struct OnActionBegin<T: Actionable> {
    pub action: T,
    pub strength: f32,
}

/// Event that gets fired during
/// an action
pub struct OnActionProgress<T: Actionable> {
    pub action: T,
    pub strength: f32,
}

/// Event that gets fired at the end
/// of an action
pub struct OnActionEnd<T: Actionable> {
    pub action: T,
}

impl<T: Actionable> Kurinji<T> {
    pub(crate) fn action_event_producer(
        input_map: Res<Kurinji<T>>,
        mut on_active_event: ResMut<Events<OnActionActive<T>>>,
        mut on_begin_event: ResMut<Events<OnActionBegin<T>>>,
        mut on_progress_event: ResMut<Events<OnActionProgress<T>>>,
        mut on_end_event: ResMut<Events<OnActionEnd<T>>>,
    ) {
        // merge keys, required for action end to be fired properly.
        // Since released actions won't be part of raw strength
        let mut _actions = input_map.action_raw_strength.clone();
        for (a, s) in input_map.action_prev_strength.iter() {
            if !_actions.contains_key(a) {
                _actions.insert(a.clone(), s.clone());
            }
        }

        for (action, strength) in _actions {
            if input_map.is_action_active(action) {
                on_active_event.send(OnActionActive {
                    action: action.clone(),
                    strength: strength,
                });
            }

            if input_map.did_action_just_began(action) {
                on_begin_event.send(OnActionBegin {
                    action: action.clone(),
                    strength: strength,
                });
            }

            if input_map.is_action_in_progress(action) {
                on_progress_event.send(OnActionProgress {
                    action: action.clone(),
                    strength: strength,
                });
            }

            if input_map.did_action_just_end(action) {
                on_end_event.send(OnActionEnd {
                    action: action.clone(),
                });
            }
        }
    }
}
