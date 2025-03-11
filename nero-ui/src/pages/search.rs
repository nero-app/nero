use std::rc::Rc;

use nero_extensions::types::{FilterCategory, Series};
use rustwind::{
    flexbox_grid::{Gap, GridTemplateColumns},
    layout::{Display, Overflow},
    sizing::Height,
};
use sycamore::{
    prelude::ReadSignal,
    web::{
        tags::{div, h1, li, ul},
        GlobalProps, HtmlGlobalAttributes, Resource, View,
    },
};
use sycamore_router::navigate;

use crate::{
    components::{IntoClickableCard, List, ListHeader, OnReachBottom},
    hooks::{use_filters, use_infinite_search, InfinitePage},
    tw,
};

// TODO: Selected filters
pub struct SearchPage {
    results: Rc<InfinitePage<Series>>,
    filters: Resource<Vec<FilterCategory>>,
}

impl SearchPage {
    pub fn new(query: String) -> Self {
        Self {
            results: Rc::new(use_infinite_search(query)),
            filters: use_filters(),
        }
    }
}

impl SearchPage {
    fn render_results(series: ReadSignal<Vec<Series>>) -> View {
        ul().class(tw!(Display::Grid, GridTemplateColumns::Number("4")))
            .children(move || {
                series
                    .get_clone()
                    .into_iter()
                    .map(|s| {
                        let nav_to = format!("/series/{}", s.id);
                        li().children(s.into_clickable_card(move |_| navigate(&nav_to)))
                            .into()
                    })
                    .collect::<Vec<_>>()
            })
            .into()
    }

    fn render_filters(filters: Vec<FilterCategory>) -> View {
        List::new(
            filters
                .into_iter()
                .map(|_| h1().children("TODO").into())
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
                    .on_reach_bottom({
                        let results = page.results.clone();
                        move || results.load_more()
                    })
                    .class(tw!(Overflow::YAuto))
                    .children(SearchPage::render_results(page.results.items())),
            )
            .children(div().class(tw!(Overflow::YAuto)).children(move || {
                match page.filters.get_clone() {
                    Some(filters) => SearchPage::render_filters(filters),
                    None => "Loading...".into(),
                }
            }))
            .into()
    }
}
