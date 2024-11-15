use leptos::{
    html::{div, ElementChild},
    prelude::{AnyView, ClassAttribute, IntoAny},
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
pub struct Transition {
    property: Option<TransitionProperty>,
    duration: Option<TransitionDuration>,
    timing_function: Option<TransitionTimingFunction>,
    delay: Option<TransitionDelay>,
    #[tw(skip)]
    children: AnyView,
}

impl Transition {
    /// Creates a new `Transition` with the given children.
    pub fn new(children: impl IntoView + 'static) -> Self {
        Self {
            property: None,
            duration: None,
            timing_function: None,
            delay: None,
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
}

impl IntoComponent for Transition {
    fn into_component(self) -> impl IntoView {
        div().class(self.classes()).child(self.children)
    }
}
