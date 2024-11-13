mod button;
mod icons;
mod image;
mod label;
pub mod layout;
mod list;
mod transition;

pub use button::*;
pub use icons::*;
pub use image::*;
pub use label::*;
pub use list::*;
pub use transition::*;

use leptos::IntoView;

/// Trait for converting a type into a leptos component.
pub trait IntoComponent {
    fn into_component(self) -> impl IntoView;
}
