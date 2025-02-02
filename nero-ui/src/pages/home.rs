use rustwind::{
    backgrounds::BackgroundColor,
    borders::BorderRadius,
    flexbox_grid::{AlignItems, FlexDirection, JustifyContent},
    layout::{Display, ObjectFit, Overflow},
    sizing::{Height, Width},
    spacing::Padding,
    transforms::Rotate,
    typography::TextAlign,
};
use sycamore::{
    prelude::HtmlImgAttributes,
    web::{
        tags::{article, br, div, figure, img, p},
        GlobalProps, HtmlGlobalAttributes, View,
    },
};
use sycamore_router::navigate;

use crate::{
    components::{Button, Icon, IconType},
    tw,
    types::Series,
};

const SHOCKED_CAT_IMG_PATH: &str = "assets/images/shocked_cat.svg";

pub struct HomePage;

impl From<HomePage> for View {
    fn from(_: HomePage) -> Self {
        let sample_series = Series::default();

        div()
            .class(tw!(Display::Flex, Height::HFull))
            .children(
                figure()
                    .class(tw!(
                        Width::WFraction(2, 5),
                        Padding::BNumber("8"),
                        Overflow::Hidden
                    ))
                    // TODO: Dynamic series poster
                    .children(
                        img()
                            .class(tw!(
                                Width::WFull,
                                Height::HFull,
                                ObjectFit::Cover,
                                BorderRadius::Xl
                            ))
                            // TODO: Default image
                            .src(sample_series.poster_url.unwrap_or_default())
                            .alt(sample_series.title.clone()),
                    ),
            )
            .children(
                // TODO: Progress indicators
                div()
                    .class(tw!(
                        Width::WNumber("20"),
                        Display::Flex,
                        AlignItems::Center,
                        Padding::BNumber("8")
                    ))
                    .children(
                        p().class(tw!(Rotate::Number("90")))
                            .children("Indicators..."),
                    ),
            )
            .children(
                // TODO: Series categories if the filter search is available in the extension
                // (series card is needed to display the category with its series)
                article()
                    .class(tw!(
                        Display::Flex,
                        FlexDirection::Col,
                        Width::WFraction(3, 5),
                        Overflow::Auto,
                        JustifyContent::Center,
                        AlignItems::Center,
                        Padding::BNumber("8")
                    ))
                    .children(
                        img()
                            .class(tw!(Width::WNumber("64")))
                            .src(SHOCKED_CAT_IMG_PATH),
                    )
                    .children(
                        p().class(tw!(TextAlign::Center, Padding::BNumber("2")))
                            .children("Whoops...")
                            .children(br())
                            .children("Apparently there's nothing around here."),
                    )
                    .children(
                        Button::icon_label(
                            Icon::new(IconType::Search),
                            "Search series",
                            // This is to be able to navigate to the series screen for the moment.
                            // TODO: Remove when the search page is created.
                            |_| navigate("/series"),
                        )
                        .color(BackgroundColor::Orange200),
                    ),
            )
            .into()
    }
}
