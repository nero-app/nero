use sycamore::web::{
    events::MouseEvent, tags::button, GlobalAttributes, GlobalProps, HtmlGlobalAttributes, View,
};
use typewind::{
    backgrounds::BackgroundColor,
    borders::BorderRadius,
    flexbox_grid::{AlignItems, Gap},
    spacing::Padding,
    ToClasses,
};

use crate::{layout::HStack, Icon, Text, TextTag};

/// Represents a button with configurable properties for background color,
/// border radius, and on click callback.
#[derive(ToClasses)]
pub struct Button<T>
where
    T: FnMut(MouseEvent) + 'static,
{
    #[tw(skip)]
    children: View,
    background_color: Option<BackgroundColor>,
    radius: Option<BorderRadius>,
    #[tw(skip)]
    on_click: T,
}

impl<T: FnMut(MouseEvent)> Button<T> {
    /// Creates a new `Button` with the specified children and on_click callback.
    pub fn new(children: impl Into<View>, on_click: T) -> Self {
        Self {
            children: children.into(),
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
        Self::new(icon, on_click)
    }

    /// Creates a new `Button` with the specified icon and text.
    ///
    /// By default the button is created with the `Gap::_2`, `Padding::Px3`, `Padding::Py1_5`,
    /// `AlignItems::Center` and `BorderRadius::Lg` properties.
    ///
    /// # Example
    /// ```
    /// use nero_components::{Button, Icon, IconType};
    ///
    /// let button = Button::with_icon_text(
    ///     Icon::new(IconType::Share),
    ///     "Share".to_owned(),
    ///     |_| todo!()
    /// );
    /// ```
    pub fn with_icon_text(icon: Icon, text: String, on_click: T) -> Self {
        Self::new(
            HStack::new((icon, Text::new(text).tag(TextTag::Span)))
                .gap(Gap::_2)
                .paddings(vec![Padding::Px3, Padding::Py1_5])
                .align_items(AlignItems::Center),
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

impl<T: FnMut(MouseEvent)> From<Button<T>> for View {
    fn from(value: Button<T>) -> Self {
        button()
            .class(value.classes())
            .children(value.children)
            .on(sycamore::web::events::click, value.on_click)
            .into()
    }
}
