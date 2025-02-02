use rustwind::{
    backgrounds::BackgroundColor,
    borders::BorderColor,
    flexbox_grid::{AlignItems, FlexDirection, JustifyContent},
    layout::{Display, Position, TopRightBottomLeft},
    sizing::Width,
    spacing::Padding,
    typography::{FontSize, FontWeight},
};
use sycamore::web::{
    tags::{div, h2, header, hr, p, section, ul},
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
                    TopRightBottomLeft::TopNumber("0"),
                    BackgroundColor::White
                ))
            })
            .children(
                div()
                    .class(tw!(
                        Width::WFull,
                        Display::Flex,
                        JustifyContent::Between,
                        AlignItems::Center,
                        Padding::Number("0.5")
                    ))
                    .children(
                        h2().class(tw!(FontSize::_2Xl, FontWeight::Semibold))
                            .children(list_header.label),
                    )
                    .when_some(list_header.end_slot, |this, slot| this.children(slot)),
            )
            .children(hr().class(tw!(BorderColor::BorderGray300)))
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
        let content: View = match list.children.as_web_sys().is_empty() {
            true => p().children(list.empty_message).into(),
            false => ul().children(list.children).into(),
        };

        match list.header {
            Some(list_header) => section()
                .class(tw!(Display::Flex, FlexDirection::Col))
                .children(list_header)
                .children(content)
                .into(),
            None => content,
        }
    }
}
