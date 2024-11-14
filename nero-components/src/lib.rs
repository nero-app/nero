mod button;
mod icons;
mod image;
pub mod layout;
mod list;
mod text;
mod transition;

pub use button::*;
pub use icons::*;
pub use image::*;
pub use list::*;
pub use text::*;
pub use transition::*;

use leptos::IntoView;

/// Trait for converting a type into a leptos component.
pub trait IntoComponent {
    fn into_component(self) -> impl IntoView;
}
