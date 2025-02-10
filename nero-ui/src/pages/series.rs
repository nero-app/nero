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
    types::{sample_episode, sample_series},
    utils::ViewBuilder,
};

pub struct SeriesPage;

impl From<SeriesPage> for View {
    fn from(_: SeriesPage) -> Self {
        let sample_series = sample_series();

        div()
            .class(tw!(Display::Flex, Height::HFull, Gap::Number("20")))
            .children(
                figure()
                    .class(tw!(
                        Width::WFraction(2, 5),
                        Padding::BNumber("8"),
                        Overflow::Hidden
                    ))
                    .children(
                        img()
                            .class(tw!(
                                Width::WFull,
                                Height::HFull,
                                ObjectFit::Cover,
                                BorderRadius::Xl
                            ))
                            // TODO: Default image
                            .src(sample_series.poster_url.unwrap().to_string())
                            .alt(sample_series.title.clone()),
                    ),
            )
            .children(
                article()
                    .class(tw!(
                        Display::Flex,
                        FlexDirection::Col,
                        Width::WFraction(3, 5),
                        Overflow::Auto,
                        Gap::Number("4")
                    ))
                    .children(
                        header()
                            .class(tw!(Display::Flex, FlexDirection::Col, Gap::Number("4")))
                            .children(
                                h1().class(tw!(
                                    FontSize::_3Xl,
                                    FontWeight::Bold,
                                    TextOverflow::Truncate
                                ))
                                .children(sample_series.title),
                            )
                            .children(
                                div()
                                    .class(tw!(Display::Flex, Gap::Number("4")))
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
                                this.children(
                                    p().class(tw!(LineClamp::Number("5"))).children(synopsis),
                                )
                            }),
                    )
                    .children(
                        List::new(
                            (1..13)
                                .map(|_| li().children(sample_episode().into_card()).into())
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
