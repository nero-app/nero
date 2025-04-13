use rustwind::{
    active,
    backgrounds::BackgroundColor,
    borders::BorderRadius,
    effects::BoxShadow,
    flexbox_grid::{AlignItems, Gap},
    interactivity::Cursor,
    layout::Display,
    spacing::Padding,
    transforms::Scale,
    transitions_animation::TransitionDuration,
    tw,
};
use sycamore::{
    prelude::{HtmlButtonAttributes, MaybeDyn},
    web::{
        events::{click, MouseEvent},
        tags::{button as button_tag, div, span},
        BoolAttribute, GlobalAttributes, GlobalProps, HtmlGlobalAttributes, View,
    },
};

use crate::utils::ViewBuilder;

use super::Icon;

const BASE_BUTTON_CLASSES: &str = tw!(
    BorderRadius::Lg,
    Padding::XNumber("3"),
    Padding::YNumber("1.5")
);
const ACTIVE_BUTTON_CLASSES: &str = tw!(
    Cursor::Pointer,
    TransitionDuration::Number("300"),
    active!(Scale::Number("95"))
);

pub struct Button<T: FnMut(MouseEvent) + 'static> {
    children: View,
    color: Option<BackgroundColor>,
    box_shadow: Option<BoxShadow>,
    disabled: BoolAttribute,
    on_click: T,
}

impl<T: FnMut(MouseEvent)> Button<T> {
    pub fn new(children: impl Into<View>, on_click: T) -> Self {
        Self {
            children: children.into(),
            color: None,
            box_shadow: None,
            disabled: MaybeDyn::Static(false),
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

    pub fn disabled(mut self, disabled: impl Into<BoolAttribute>) -> Self {
        self.disabled = disabled.into();
        self
    }
}

impl<T: FnMut(MouseEvent)> Button<T> {
    pub fn new_with_label(label: &'static str, on_click: T) -> Self {
        Self::new(span().children(label), on_click)
    }

    pub fn new_with_icon(icon: Icon, on_click: T) -> Self {
        Self::new(icon, on_click)
    }

    pub fn new_with_icon_label(icon: Icon, label: &'static str, on_click: T) -> Self {
        Self::new(
            div()
                .class(tw!(Display::Flex, AlignItems::Center, Gap::Number("2")))
                .children(icon)
                .children(span().children(label)),
            on_click,
        )
        .box_shadow(BoxShadow::ShadowLg)
    }
}

impl<T: FnMut(MouseEvent)> From<Button<T>> for View {
    fn from(button: Button<T>) -> Self {
        let classes = format!(
            "{} {} {}",
            BASE_BUTTON_CLASSES,
            button.color.unwrap_or(BackgroundColor::Transparent),
            button.box_shadow.unwrap_or(BoxShadow::ShadowNone)
        );
        button_tag()
            .disabled(button.disabled.get())
            .map(|this| match button.disabled.as_static() {
                Some(value) => this.class(match value {
                    true => format!("{} {}", classes, tw!(Cursor::NotAllowed)),
                    false => format!("{} {}", classes, ACTIVE_BUTTON_CLASSES),
                }),
                None => this.class(move || match button.disabled.get() {
                    true => format!("{} {}", classes, tw!(Cursor::NotAllowed)),
                    false => format!("{} {}", classes, ACTIVE_BUTTON_CLASSES),
                }),
            })
            .children(button.children)
            .on(click, button.on_click)
            .into()
    }
}
