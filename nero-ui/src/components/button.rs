use rustwind::{
    active,
    backgrounds::BackgroundColor,
    borders::BorderRadius,
    effects::BoxShadow,
    flexbox_grid::{AlignItems, Gap},
    layout::Display,
    spacing::Padding,
    transforms::Scale,
    transitions_animation::TransitionDuration,
};
use sycamore::web::{
    events::{click, MouseEvent},
    tags::{button as button_tag, div, span},
    GlobalAttributes, GlobalProps, HtmlGlobalAttributes, View,
};

use crate::tw;

use super::Icon;

pub struct Button<T>
where
    T: FnMut(MouseEvent) + 'static,
{
    children: View,
    color: Option<BackgroundColor>,
    box_shadow: Option<BoxShadow>,
    on_click: T,
}

impl<T> Button<T>
where
    T: FnMut(MouseEvent),
{
    pub fn new(children: impl Into<View>, on_click: T) -> Self {
        Self {
            children: children.into(),
            color: None,
            box_shadow: None,
            on_click,
        }
    }

    pub fn color(mut self, color: BackgroundColor) -> Self {
        self.color = Some(color);
        self
    }

    pub fn box_shadow(mut self, box_shadow: BoxShadow) -> Self {
        self.box_shadow = Some(box_shadow);
        self
    }

    pub fn label(label: &'static str, on_click: T) -> Self {
        Self::new(span().children(label), on_click)
    }

    pub fn icon(icon: Icon, on_click: T) -> Self {
        Self::new(icon, on_click)
    }

    pub fn icon_label(icon: Icon, label: &'static str, on_click: T) -> Self {
        Self::new(
            div()
                .class(tw!(Display::Flex, Gap::_2, AlignItems::Center))
                .children(icon)
                .children(span().children(label)),
            on_click,
        )
        .box_shadow(BoxShadow::Lg)
    }
}

impl<T: FnMut(MouseEvent)> From<Button<T>> for View {
    fn from(button: Button<T>) -> Self {
        button_tag()
            .class(format!(
                "{} {} {}",
                tw!(
                    Padding::Px3,
                    Padding::Py1_5,
                    BorderRadius::Lg,
                    TransitionDuration::_300,
                    active!(Scale::_95)
                ),
                button
                    .color
                    .unwrap_or(BackgroundColor::Transparent)
                    .as_class(),
                button.box_shadow.unwrap_or(BoxShadow::None).as_class()
            ))
            .children(button.children)
            .on(click, button.on_click)
            .into()
    }
}
