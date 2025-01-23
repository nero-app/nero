use rustwind::{
    backgrounds::BackgroundColor,
    borders::BorderRadius,
    flexbox_grid::{FlexDirection, Gap},
    layout::{Display, ObjectFit, Overflow},
    sizing::{Height, Width},
    spacing::Padding,
    typography::{FontSize, FontWeight, LineClamp, TextOverflow},
};
use sycamore::{
    prelude::HtmlImgAttributes,
    web::{
        tags::{article, div, figure, h1, header, img, li, p},
        GlobalProps, HtmlGlobalAttributes, View,
    },
};
use sycamore_router::navigate;

use crate::{
    components::{Button, Icon, IconType, IntoCard, List, ListHeader},
    tw,
    types::{Episode, Series},
    utils::ViewBuilder,
};

pub struct SeriesPage;

impl From<SeriesPage> for View {
    fn from(_: SeriesPage) -> Self {
        let sample_series = Series::default();

        div()
            .class(tw!(Display::Flex, Height::Full, Gap::_20))
            .children(
                figure()
                    .class(tw!(Width::_2over5, Padding::Pb8, Overflow::Hidden))
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
                article()
                    .class(tw!(
                        Display::Flex,
                        FlexDirection::Col,
                        Width::_3over5,
                        Overflow::Auto,
                        Gap::_4
                    ))
                    .children(
                        header()
                            .class(tw!(Display::Flex, FlexDirection::Col, Gap::_4))
                            .children(
                                h1().class(tw!(
                                    FontSize::_3xl,
                                    FontWeight::Bold,
                                    TextOverflow::Truncate
                                ))
                                .children(sample_series.title),
                            )
                            .children(
                                div()
                                    .class(tw!(Display::Flex, Gap::_4))
                                    .children(
                                        Button::icon_label(
                                            Icon::new(IconType::Play),
                                            "Watch now",
                                            |_| navigate("/watch"),
                                        )
                                        .color(BackgroundColor::Red300),
                                    )
                                    .children(
                                        Button::icon_label(
                                            Icon::new(IconType::Share),
                                            "Share the series",
                                            |_| todo!(),
                                        )
                                        .color(BackgroundColor::Red300),
                                    ),
                            )
                            .when_some(sample_series.synopsis, |this, synopsis| {
                                this.children(p().class(tw!(LineClamp::_5)).children(synopsis))
                            }),
                    )
                    .children(
                        List::new(
                            (1..13)
                                .map(|_| li().children(Episode::default().into_card()).into())
                                .collect::<Vec<_>>(),
                        )
                        .header(
                            ListHeader::new("Episodes")
                                .end_slot(Button::icon(Icon::new(IconType::Sort), |_| todo!())),
                        ),
                    ),
            )
            .into()
    }
}
