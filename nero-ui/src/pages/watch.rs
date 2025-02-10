use rustwind::{
    flexbox_grid::{FlexDirection, Gap},
    layout::{AspectRatio, Display, Overflow},
    sizing::{Height, Width},
    typography::{FontSize, FontWeight, LineClamp},
};
use sycamore::{
    prelude::HtmlVideoAttributes,
    web::{
        tags::{article, aside, div, h1, li, p, section, video},
        GlobalProps, HtmlGlobalAttributes, View,
    },
};

use crate::{
    components::{IntoSmallCard, List, ListHeader},
    tw,
    types::{sample_episode, sample_series_video, SAMPLE_VIDEO_SYNOPSIS, SAMPLE_VIDEO_TITLE},
    utils::ViewBuilder,
};

pub struct WatchPage;

impl From<WatchPage> for View {
    fn from(_: WatchPage) -> Self {
        let sample_video = sample_series_video();

        div()
            .class(tw!(
                Display::Flex,
                Height::HFull,
                Gap::Number("12"),
                Overflow::Hidden
            ))
            .children(
                article()
                    .class(tw!(
                        Display::Flex,
                        FlexDirection::Col,
                        Width::WFraction(4, 6),
                        Gap::Number("4")
                    ))
                    .children(
                        video()
                            .class(tw!(Width::WFull, AspectRatio::Video))
                            .controls(true)
                            .src(sample_video.video_url.to_string()),
                    )
                    .children(
                        section()
                            .class(tw!(Display::Flex, FlexDirection::Col, Gap::Number("2")))
                            .children(
                                h1().class(tw!(
                                    LineClamp::Number("2"),
                                    FontSize::_2Xl,
                                    FontWeight::Semibold
                                ))
                                .children(SAMPLE_VIDEO_TITLE),
                            )
                            .when_some(SAMPLE_VIDEO_SYNOPSIS, |this, synopsis| {
                                this.children(
                                    p().class(tw!(LineClamp::Number("3"))).children(synopsis),
                                )
                            }),
                    ),
            )
            .children(
                aside()
                    .class(tw!(Width::WFraction(2, 6), Overflow::YAuto))
                    .children(
                        List::new(
                            (1..13)
                                .map(|_| li().children(sample_episode().into_small_card()).into())
                                .collect::<Vec<_>>(),
                        )
                        .header(ListHeader::new("Episodes")),
                    ),
            )
            .into()
    }
}
