use rustwind::{
    backgrounds::BackgroundColor,
    borders::{BorderRadius, OutlineStyle},
    flexbox_grid::{AlignItems, Gap, GridTemplateColumns},
    layout::Display,
    sizing::Width,
    spacing::Padding,
    svg::Fill,
    tw,
    typography::Color,
};
use sycamore::{
    prelude::{create_signal, HtmlAAttributes, HtmlInputAttributes},
    web::{
        bind,
        events::{self, SubmitEvent},
        tags::{a, form, input, li, nav, p, ul},
        GlobalAttributes, GlobalProps, HtmlGlobalAttributes, View,
    },
};
use sycamore_router::navigate;

use super::{Icon, IconType};

pub struct Toolbar;

impl Toolbar {
    pub const SEARCH_INPUT_ID: &str = "search-input";

    fn search_form() -> View {
        let query = create_signal(String::new());

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
                        Width::WFull,
                        BackgroundColor::Slate100,
                        Color::Slate400,
                        OutlineStyle::None
                    ))
                    .id(Self::SEARCH_INPUT_ID)
                    .r#type("search")
                    .placeholder("Search")
                    .bind(bind::value, query),
            )
            .on(events::submit, move |ev: SubmitEvent| {
                ev.prevent_default();
                let q = query.get_clone();
                if !q.is_empty() {
                    navigate(&format!("/search?q={}", q));
                }
            })
            .into()
    }
}

impl From<Toolbar> for View {
    fn from(_: Toolbar) -> Self {
        nav()
            .class(tw!(
                Display::Grid,
                Width::WFull,
                GridTemplateColumns::Number("3")
            ))
            .children(p().children("Nero app v1.0"))
            .children(
                ul().class(tw!(Display::Flex, Gap::Number("4")))
                    .children(li().children(a().href("/").children("Home")))
                    .children(li().children(a().href("#").children("Extensions")))
                    .children(li().children(a().href("#").children("Settings"))),
            )
            .children(Toolbar::search_form())
            .into()
    }
}
