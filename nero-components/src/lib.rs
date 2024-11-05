mod button;
pub use button::*;

mod image;
pub use image::*;

mod label;
pub use label::*;

pub mod layout;

mod list;
pub use list::*;

use leptos::IntoView;

/// Trait for converting a type into a leptos component.
pub trait IntoComponent {
    fn into_component(self) -> impl IntoView;
}
