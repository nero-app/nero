use leptos::{
    html::{div, ElementChild},
    prelude::{AnyView, ClassAttribute, IntoAny, StyleAttribute},
    tachys::html::style::IntoStyle,
    IntoView,
};
use typewind::{
    transitions_animation::{
        TransitionDelay, TransitionDuration, TransitionProperty, TransitionTimingFunction,
    },
    ToClasses,
};

use crate::IntoComponent;

/// A component that applies a transition to its children.
#[derive(ToClasses)]
pub struct Transition<T: IntoStyle + 'static> {
    property: Option<TransitionProperty>,
    duration: Option<TransitionDuration>,
    timing_function: Option<TransitionTimingFunction>,
    delay: Option<TransitionDelay>,
    #[tw(skip)]
    style: Option<T>,
    #[tw(skip)]
    children: AnyView,
}

impl<T: IntoStyle> Transition<T> {
    /// Creates a new `Transition` with the given children.
    pub fn new(children: impl IntoView + 'static) -> Self {
        Self {
            property: None,
            duration: None,
            timing_function: None,
            delay: None,
            style: None,
            children: children.into_any(),
        }
    }

    /// Sets the transition property.
    pub fn property(mut self, property: TransitionProperty) -> Self {
        self.property = Some(property);
        self
    }

    /// Sets the transition duration.
    pub fn duration(mut self, duration: TransitionDuration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Sets the transition timing function.
    pub fn timing_function(mut self, timing_function: TransitionTimingFunction) -> Self {
        self.timing_function = Some(timing_function);
        self
    }

    /// Sets the transition delay.
    pub fn delay(mut self, delay: TransitionDelay) -> Self {
        self.delay = Some(delay);
        self
    }

    /// Sets a custom style for the transition component.
    ///
    /// # Example
    /// ```
    /// use nero_components::Transition;
    ///
    /// let transition = Transition::new("Hello, world!")
    ///     .style("width: 100px; height: 100px;");
    /// ```
    pub fn style(mut self, style: T) -> Self {
        self.style = Some(style);
        self
    }
}

impl<T: IntoStyle> IntoComponent for Transition<T> {
    fn into_component(self) -> impl IntoView {
        let div = div().class(self.classes()).child(self.children);

        match self.style {
            Some(style) => div.style(style).into_any(),
            None => div.into_any(),
        }
    }
}
