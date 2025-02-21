use nero_extensions::types::{Filter, FilterCategory, Series};
use rustwind::{
    flexbox_grid::{AlignItems, FlexDirection, Gap, GridTemplateColumns},
    layout::{Display, Overflow},
    sizing::Height,
    spacing::Padding,
};
use sycamore::{
    prelude::HtmlInputAttributes,
    web::{
        tags::{details, div, input, label, li, span, summary, ul},
        GlobalProps, HtmlGlobalAttributes, View,
    },
};
use sycamore_router::navigate;

use crate::{
    components::{IntoClickableCard, List, ListHeader},
    tw,
    types::{sample_filter_category, sample_series},
};

/// A wrapper around [`FilterCategory`] that implements [`From`] for [`View`].
struct FilterCategoryWrapper(FilterCategory);

impl From<FilterCategoryWrapper> for View {
    fn from(category: FilterCategoryWrapper) -> Self {
        details()
            .children(summary().children(category.0.display_name))
            .children(
                div()
                    .class(tw!(
                        Display::Flex,
                        FlexDirection::Col,
                        Gap::Number("1"),
                        Padding::YNumber("2")
                    ))
                    .children(
                        category
                            .0
                            .filters
                            .into_iter()
                            .map(|f| FilterWrapper(f).into())
                            .collect::<Vec<_>>(),
                    ),
            )
            .into()
    }
}

/// A wrapper around [`Filter`] that implements [`From`] for [`View`].
struct FilterWrapper(Filter);

impl From<FilterWrapper> for View {
    fn from(filter: FilterWrapper) -> Self {
        label()
            .class(tw!(Display::Flex, AlignItems::Center, Gap::Number("2")))
            .children(input().r#type("checkbox"))
            .children(span().children(filter.0.display_name))
            .into()
    }
}

pub struct SearchPage {
    results: Vec<Series>,
    filters: Vec<FilterCategory>,
}

impl SearchPage {
    #[allow(unused_variables)]
    pub fn new(query: String) -> Self {
        Self {
            results: (1..=5).map(|_| sample_series()).collect::<Vec<_>>(),
            filters: (1..=10).map(|_| sample_filter_category()).collect(),
        }
    }
}

impl SearchPage {
    fn render_search_results(series: Vec<Series>) -> View {
        ul().class(tw!(Display::Grid, GridTemplateColumns::Number("4")))
            .children(
                series
                    .into_iter()
                    .map(|s| {
                        let nav_to = format!("/series/{}", s.id);
                        li().children(s.into_clickable_card(move |_| navigate(&nav_to)))
                            .into()
                    })
                    .collect::<Vec<_>>(),
            )
            .into()
    }

    fn render_search_filters(filters: Vec<FilterCategory>) -> View {
        List::new(
            filters
                .into_iter()
                .map(|f| li().children(FilterCategoryWrapper(f)).into())
                .collect::<Vec<_>>(),
        )
        .header(ListHeader::new("Filters"))
        .into()
    }
}

impl From<SearchPage> for View {
    fn from(page: SearchPage) -> Self {
        div()
            .class(tw!(
                Display::Grid,
                Height::HFull,
                GridTemplateColumns::Value("4fr_2fr"),
                Gap::Number("12"),
                Overflow::Hidden
            ))
            .children(
                div()
                    .class(tw!(Overflow::YAuto))
                    .children(SearchPage::render_search_results(page.results)),
            )
            .children(
                div()
                    .class(tw!(Overflow::YAuto))
                    .children(SearchPage::render_search_filters(page.filters)),
            )
            .into()
    }
}
