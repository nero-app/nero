use leptos::{html::img, prelude::ClassAttribute, IntoView};
use typewind::{
    borders::BorderRadius,
    layout::ObjectFit,
    sizing::{Height, Width},
    ToClasses,
};

use crate::IntoComponent;

/// Represents an image with configurable properties for dimensions, object fit,
/// and border radius.
#[derive(ToClasses)]
pub struct Image {
    #[tw(skip)]
    src: String,
    #[tw(skip)]
    alt: String,
    height: Option<Height>,
    width: Option<Width>,
    fit: Option<ObjectFit>,
    radius: Option<BorderRadius>,
}

impl Image {
    /// Creates a new `Image` with specified `src` and `alt` text.
    ///
    /// By default the image is created with a `Height::Full` and `Width::Full`.
    pub fn new(src: String, alt: String) -> Self {
        Self {
            src,
            alt,
            height: Some(Height::Full),
            width: Some(Width::Full),
            fit: None,
            radius: None,
        }
    }

    /// Sets the height of the image.
    pub fn height(mut self, height: Height) -> Self {
        self.height = Some(height);
        self
    }

    /// Sets the width of the image.
    pub fn width(mut self, width: Width) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the object fit property, which determines how the image should
    /// be resized to fit within its container.
    pub fn fit(mut self, object_fit: ObjectFit) -> Self {
        self.fit = Some(object_fit);
        self
    }

    /// Sets the border radius of the image.
    pub fn radius(mut self, border_radius: BorderRadius) -> Self {
        self.radius = Some(border_radius);
        self
    }
}

impl IntoComponent for Image {
    fn into_component(self) -> impl IntoView {
        img().class(self.classes()).src(self.src).alt(self.alt)
    }
}
