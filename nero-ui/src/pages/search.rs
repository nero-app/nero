use nero_extensions::types::{Filter, FilterCategory};
use rustwind::{
    flexbox_grid::{AlignItems, FlexDirection, Gap, GridTemplateColumns},
    layout::{Display, Overflow},
    sizing::{Height, Width},
    spacing::Padding,
};
use sycamore::{
    prelude::HtmlInputAttributes,
    web::{
        tags::{details, div, input, label, li, span, summary, ul},
        GlobalProps, HtmlGlobalAttributes, View,
    },
};

use crate::{
    components::{IntoCard, List, ListHeader},
    tw,
    types::{sample_filter_category, sample_series},
};

pub struct SearchPage {
    #[allow(unused)]
    query: String,
}

impl SearchPage {
    pub fn new(query: String) -> Self {
        Self { query }
    }

    fn render_filters(filters: Vec<FilterCategory>) -> View {
        List::new(
            filters
                .into_iter()
                .map(|f| li().children(Self::render_filter(f)).into())
                .collect::<Vec<_>>(),
        )
        .header(ListHeader::new("Filters"))
        .into()
    }

    fn render_filter(filter: FilterCategory) -> View {
        details()
            .children(summary().children(filter.display_name))
            .children(
                div()
                    .class(tw!(
                        Display::Flex,
                        FlexDirection::Col,
                        Gap::Number("1"),
                        Padding::YNumber("2")
                    ))
                    .children(
                        filter
                            .filters
                            .into_iter()
                            .map(Self::render_filter_option)
                            .collect::<Vec<_>>(),
                    ),
            )
            .into()
    }

    fn render_filter_option(filter: Filter) -> View {
        label()
            .class(tw!(Display::Flex, AlignItems::Center, Gap::Number("2")))
            .children(input().r#type("checkbox"))
            .children(span().children(filter.display_name))
            .into()
    }
}

impl From<SearchPage> for View {
    fn from(_: SearchPage) -> Self {
        div()
            .class(tw!(
                Height::HFull,
                Display::Flex,
                Gap::Number("12"),
                Overflow::Hidden
            ))
            .children(
                div()
                    .class(tw!(Width::WFraction(4, 6), Overflow::YAuto))
                    .children(
                        ul().class(tw!(Display::Grid, GridTemplateColumns::Number("4")))
                            .children(
                                (1..=5)
                                    .map(|_| li().children(sample_series().into_card()).into())
                                    .collect::<Vec<_>>(),
                            ),
                    ),
            )
            .children(
                div()
                    .class(tw!(Width::WFraction(2, 6), Overflow::YAuto))
                    .children(SearchPage::render_filters(
                        (1..=10).map(|_| sample_filter_category()).collect(),
                    )),
            )
            .into()
    }
}
