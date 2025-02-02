use rustwind::{
    backgrounds::BackgroundColor,
    borders::{BorderRadius, OutlineStyle},
    flexbox_grid::{AlignItems, Gap, GridTemplateColumns},
    layout::Display,
    sizing::Width,
    spacing::Padding,
    svg::Fill,
    typography::Color,
};
use sycamore::{
    prelude::{HtmlAAttributes, HtmlInputAttributes},
    web::{
        tags::{a, form, input, li, nav, p, ul},
        GlobalProps, HtmlGlobalAttributes, View,
    },
};

use crate::tw;

use super::{Icon, IconType};

pub struct Toolbar;

impl From<Toolbar> for View {
    fn from(_: Toolbar) -> Self {
        nav()
            .class(tw!(
                Width::WFull,
                Display::Grid,
                GridTemplateColumns::Number("3")
            ))
            .children(p().children("Nero app v1.0"))
            .children(
                ul().class(tw!(Display::Flex, Gap::Number("4")))
                    .children(li().children(a().href("/").children("Home")))
                    .children(li().children(a().href("#").children("Extensions")))
                    .children(li().children(a().href("#").children("Settings"))),
            )
            .children(
                form()
                    .class(tw!(
                        Display::Flex,
                        AlignItems::Center,
                        Gap::Number("2"),
                        BorderRadius::Lg,
                        BackgroundColor::Slate100,
                        Padding::XNumber("3"),
                        Padding::YNumber("1")
                    ))
                    .children(Icon::new(IconType::Search).fill(Fill::Slate500))
                    .children(
                        input()
                            .class(tw!(
                                BackgroundColor::Slate100,
                                OutlineStyle::None,
                                Color::Slate400
                            ))
                            .r#type("search")
                            .placeholder("Search"),
                    ),
            )
            .into()
    }
}
