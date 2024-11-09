use leptos::{
    ev::MouseEvent,
    html::{button, ElementChild},
    prelude::{AnyView, ClassAttribute, IntoAny, OnAttribute, Signal},
    IntoView,
};
use typewind::{
    backgrounds::BackgroundColor,
    borders::BorderRadius,
    flexbox_grid::{AlignItems, Gap},
    spacing::Padding,
    ToClasses,
};

use crate::{layout::Layout, Icon, IntoComponent, Label, LabelTag};

/// Represents a button with configurable properties for background color,
/// border radius, and on click callback.
#[derive(ToClasses)]
pub struct Button<T>
where
    T: FnMut(MouseEvent) + 'static,
{
    #[tw(skip)]
    children: AnyView,
    background_color: Option<BackgroundColor>,
    radius: Option<BorderRadius>,
    #[tw(skip)]
    on_click: T,
}

impl<T: FnMut(MouseEvent) + 'static> Button<T> {
    /// Creates a new `Button` with the specified children and on_click callback.
    pub fn new(children: impl IntoView + 'static, on_click: T) -> Self {
        Self {
            children: children.into_any(),
            background_color: None,
            radius: None,
            on_click,
        }
    }

    /// Creates a new `Button` with the specified icon.
    ///
    /// # Example
    /// ```
    /// use nero_components::{Button, Icon, IconType};
    ///
    /// let button = Button::with_icon(Icon::new(IconType::Play), |_| todo!());
    /// ```
    pub fn with_icon(icon: Icon, on_click: T) -> Self {
        Self::new(icon.into_component(), on_click)
    }

    /// Creates a new `Button` with the specified icon and label.
    ///
    /// By default the button is created with the `Gap::_2`, `Padding::Px3`, `Padding::Py1_5`,
    /// `AlignItems::Center` and `BorderRadius::Lg` properties.
    ///
    /// # Example
    /// ```
    /// use nero_components::{Button, Icon, IconType};
    ///
    /// let button = Button::with_icon_label(
    ///     Icon::new(IconType::Share),
    ///     "Share".into(),
    ///     |_| todo!()
    /// );
    /// ```
    pub fn with_icon_label(icon: Icon, label: Signal<String>, on_click: T) -> Self {
        Self::new(
            Layout::h_stack((
                icon.into_component(),
                Label::new(label).tag(LabelTag::Span).into_component(),
            ))
            .gap(Gap::_2)
            .paddings(vec![Padding::Px3, Padding::Py1_5])
            .align_items(AlignItems::Center)
            .into_component(),
            on_click,
        )
        .radius(BorderRadius::Lg)
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

impl<T: FnMut(MouseEvent) + 'static> IntoComponent for Button<T> {
    fn into_component(mut self) -> impl IntoView {
        button()
            .class(self.classes())
            .child(self.children)
            .on(leptos::ev::click, move |ev| (self.on_click)(ev))
    }
}
