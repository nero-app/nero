use leptos::{
    html::{button, ElementChild},
    prelude::{AnyView, ClassAttribute, IntoAny, OnAttribute},
    IntoView,
};
use typewind::{backgrounds::BackgroundColor, borders::BorderRadius, ToClasses};

use crate::IntoComponent;

/// Represents a button with configurable properties for background color,
/// border radius, and on click callback.
#[derive(ToClasses)]
pub struct Button<T>
where
    T: FnMut() + 'static,
{
    #[tw(skip)]
    children: AnyView,
    background_color: Option<BackgroundColor>,
    radius: Option<BorderRadius>,
    #[tw(skip)]
    on_click: T,
}

impl<T: FnMut() + 'static> Button<T> {
    /// Creates a new `Button` with the specified children and on_click callback.
    pub fn new(children: impl IntoView + 'static, on_click: T) -> Self {
        Self {
            children: children.into_any(),
            background_color: None,
            radius: None,
            on_click,
        }
    }

    /// Sets the background color of the button.
    pub fn background_color(mut self, background_color: BackgroundColor) -> Self {
        self.background_color = Some(background_color);
        self
    }

    /// Sets the border radius of the button.
    pub fn radius(mut self, border_radius: BorderRadius) -> Self {
        self.radius = Some(border_radius);
        self
    }
}

impl<T: FnMut() + 'static> IntoComponent for Button<T> {
    fn into_component(mut self) -> impl IntoView {
        button()
            .class(self.classes())
            .child(self.children)
            .on(leptos::ev::click, move |_| (self.on_click)())
    }
}
