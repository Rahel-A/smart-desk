// generics:
pub mod hardware_error;
pub mod hal;

// implementation:
pub mod settings;
pub mod motor;
pub mod distance;
pub mod desk_actions;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TableMotion {
    Up,
    Down,
    Rest,
}

