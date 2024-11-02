mod image;
pub use image::*;

mod label;
pub use label::*;

pub mod layout;

use leptos::IntoView;

/// Trait for converting a type into a leptos component.
pub trait IntoComponent {
    fn into_component(self) -> impl IntoView;
}
