use nero_extensions::{types::Series, Extension as ExtensionTrait};
use rustwind::{
    backgrounds::BackgroundColor,
    borders::BorderRadius,
    flexbox_grid::{AlignItems, FlexDirection, GridTemplateColumns, JustifyContent},
    layout::{Display, ObjectFit, Overflow},
    sizing::{Height, Width},
    spacing::Padding,
    transforms::Rotate,
    tw,
    typography::TextAlign,
};
use sycamore::{
    prelude::HtmlImgAttributes,
    web::{
        create_client_resource, document,
        tags::{article, br, div, figure, img, p},
        GlobalProps, HtmlGlobalAttributes, Resource, View,
    },
};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;

use crate::{
    components::{Button, Icon, IconType, Toolbar},
    extensions::Extension,
};

pub struct HomePage {
    series: Resource<Vec<Series>>,
}

impl Default for HomePage {
    fn default() -> Self {
        Self {
            series: create_client_resource(|| async {
                Extension
                    .search("TODO", None, vec![])
                    .await
                    .unwrap_throw()
                    .items
            }),
        }
    }
}

impl HomePage {
    // TODO: Dynamic series poster
    fn render_dynamic_series(series: Vec<Series>) -> View {
        let series = series.into_iter().next().unwrap();

        img()
            .class(tw!(Width::SizeFull, BorderRadius::Xl, ObjectFit::Cover))
            // TODO: Default image
            .src(series.poster_url.unwrap().to_string())
            .alt(series.title.clone())
            .referrerpolicy("no-referrer")
            .into()
    }

    // TODO: Progress indicators
    // should this be in a separate function?
    fn render_dynamic_indicators() -> View {
        p().class(tw!(Rotate::Number("90")))
            .children("Indicators...")
            .into()
    }

    fn render_empty_feedback() -> View {
        article()
            .class(tw!(
                Display::Flex,
                FlexDirection::Col,
                AlignItems::Center,
                JustifyContent::Center,
                Overflow::Auto,
                Padding::BNumber("8")
            ))
            .children(
                img()
                    .class(tw!(Width::WNumber("64")))
                    .src("assets/images/shocked_cat.svg"),
            )
            .children(
                p().class(tw!(Padding::BNumber("2"), TextAlign::Center))
                    .children("Whoops...")
                    .children(br())
                    .children("Apparently there's nothing around here."),
            )
            .children(
                Button::icon_label(Icon::new(IconType::Search), "Search series", |_| {
                    if let Some(el) = document().get_element_by_id(Toolbar::SEARCH_INPUT_ID) {
                        el.unchecked_into::<HtmlInputElement>().focus().unwrap();
                    }
                })
                .color(BackgroundColor::Orange200),
            )
            .into()
    }
}

impl From<HomePage> for View {
    fn from(page: HomePage) -> Self {
        div()
            .class(tw!(
                Display::Grid,
                Height::HFull,
                GridTemplateColumns::Value("2fr_auto_3fr")
            ))
            .children(
                figure()
                    .class(tw!(Overflow::Hidden, Padding::BNumber("8")))
                    .children(move || match page.series.get_clone() {
                        Some(series) => HomePage::render_dynamic_series(series),
                        None => "Loading series...".into(),
                    }),
            )
            .children(
                div()
                    .class(tw!(
                        Display::Flex,
                        Width::WNumber("20"),
                        AlignItems::Center,
                        Padding::BNumber("8")
                    ))
                    .children(HomePage::render_dynamic_indicators()),
            )
            .children(
                // TODO: Series categories if the filter search is available in the extension
                HomePage::render_empty_feedback(),
            )
            .into()
    }
}
