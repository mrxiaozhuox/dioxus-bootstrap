#![allow(non_snake_case, dead_code)]

pub mod application;
pub mod layout;

pub mod components;
pub mod script;
pub mod style;

pub mod prelude {

    pub use crate::application::*;
    pub use crate::layout::*;
    pub use crate::style::*;

    pub use crate::script::*;

    pub use crate::components::alert::*;
    pub use crate::components::badge::*;
    pub use crate::components::button::*;
    pub use crate::components::card::*;
    pub use crate::components::modal::*;
    pub use crate::components::dropdown::*;
    pub use crate::components::navbar::*;
}
