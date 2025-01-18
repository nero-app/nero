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
            .class(tw!(Display::Flex, Height::Full))
            .children(
                figure()
                    .class(tw!(Width::_2over5, Padding::Pb8, Overflow::Hidden))
                    // TODO: Dynamic series poster
                    .children(
                        img()
                            .class(tw!(
                                Width::Full,
                                Height::Full,
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
                        Width::_20,
                        Display::Flex,
                        AlignItems::Center,
                        Padding::Pb8
                    ))
                    .children(p().class(tw!(Rotate::_90)).children("Indicators...")),
            )
            .children(
                // TODO: Series categories if the filter search is available in the extension
                // (series card is needed to display the category with its series)
                article()
                    .class(tw!(
                        Display::Flex,
                        FlexDirection::Col,
                        Width::_3over5,
                        Overflow::Auto,
                        JustifyContent::Center,
                        AlignItems::Center,
                        Padding::Pb8
                    ))
                    .children(img().class(tw!(Width::_64)).src(SHOCKED_CAT_IMG_PATH))
                    .children(
                        p().class(tw!(TextAlign::Center, Padding::Pb2))
                            .children("Whoops...")
                            .children(br)
                            .children("Apparently there's nothing around here."),
                    )
                    .children(
                        Button::icon_label(
                            Icon::new(IconType::Search),
                            "Search series",
                            |_| todo!(),
                        )
                        .color(BackgroundColor::Orange200),
                    ),
            )
            .into()
    }
}
