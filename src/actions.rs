use crate::actions::select_action::SelectAction;

pub mod select_action;

pub struct Actions {
    pub select_action: SelectAction,
}

impl Actions {
    pub fn new() -> Self {
        Self {
            select_action: SelectAction::new(),
        }
    }
}
