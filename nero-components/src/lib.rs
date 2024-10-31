mod image;
pub mod layout;
pub use image::*;

use leptos::IntoView;

/// Trait for converting a type into a leptos component.
pub trait IntoComponent {
    fn into_component(self) -> impl IntoView;
}
