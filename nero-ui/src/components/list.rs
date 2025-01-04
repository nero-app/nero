use rustwind::{
    backgrounds::BackgroundColor,
    flexbox_grid::{AlignItems, FlexDirection, JustifyContent},
    layout::{Display, Position, TopRightBottomLeft},
    sizing::Width,
    spacing::Padding,
    typography::{FontSize, FontWeight},
};
use sycamore::web::{
    tags::{div, h2, header, hr, section, ul},
    GlobalProps, HtmlGlobalAttributes, View,
};

use crate::{tw, utils::ViewBuilder};

pub struct ListHeader {
    label: &'static str,
    end_slot: Option<View>,
    sticky: bool,
}

impl ListHeader {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            end_slot: None,
            sticky: true,
        }
    }

    pub fn end_slot(mut self, end_slot: impl Into<View>) -> Self {
        self.end_slot = Some(end_slot.into());
        self
    }

    pub fn sticky(mut self, sticky: bool) -> Self {
        self.sticky = sticky;
        self
    }
}

impl From<ListHeader> for View {
    fn from(list_header: ListHeader) -> Self {
        header()
            .when(list_header.sticky, |this| {
                this.class(tw!(
                    Position::Sticky,
                    TopRightBottomLeft::Top0,
                    BackgroundColor::White
                ))
            })
            .children(
                div()
                    .class(tw!(
                        Width::Full,
                        Display::Flex,
                        JustifyContent::Between,
                        AlignItems::Center,
                        Padding::P0_5
                    ))
                    .children(
                        h2().class(tw!(FontSize::_2xl, FontWeight::Semibold))
                            .children(list_header.label),
                    )
                    .when_some(list_header.end_slot, |this, slot| this.children(slot)),
            )
            .children(hr())
            .into()
    }
}

pub struct List {
    empty_message: &'static str,
    header: Option<ListHeader>,
    children: View,
}

impl List {
    pub fn new(children: impl Into<View>) -> Self {
        Self {
            empty_message: "no items",
            header: None,
            children: children.into(),
        }
    }

    pub fn header(mut self, header: ListHeader) -> Self {
        self.header = Some(header);
        self
    }
}

impl From<List> for View {
    fn from(list: List) -> Self {
        section()
            .class(tw!(Display::Flex, FlexDirection::Col))
            .when_some(list.header, |this, header| this.children(header))
            .map(|this| match list.children.as_web_sys().is_empty() {
                true => this.children(list.empty_message),
                false => this.children(ul().children(list.children)),
            })
            .into()
    }
}
