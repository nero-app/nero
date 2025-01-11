use rustwind::{
    flexbox_grid::{FlexDirection, Gap},
    layout::{AspectRatio, Display},
    sizing::Width,
    spacing::SpaceBetween,
    typography::{FontSize, FontWeight, LineClamp},
};
use sycamore::{
    prelude::HtmlVideoAttributes,
    web::{
        tags::{h1, li, p, section, video},
        GlobalProps, HtmlGlobalAttributes, View,
    },
};

use crate::{
    components::{IntoSmallCard, List},
    tw,
    types::{Episode, Video},
    utils::ViewBuilder,
};

use super::SplitLayout;

pub struct WatchPage;

impl From<WatchPage> for View {
    fn from(_: WatchPage) -> Self {
        let sample_video = Video::default();

        SplitLayout::new_watch(
            (
                video()
                    .class(tw!(Width::Full, AspectRatio::Video))
                    .controls(true)
                    .src(sample_video.url),
                section()
                    .class(tw!(Display::Flex, FlexDirection::Col, Gap::_2))
                    .children(
                        h1().class(tw!(
                            LineClamp::_2,
                            SpaceBetween::X2,
                            FontSize::_2xl,
                            FontWeight::Semibold
                        ))
                        .children(Video::VIDEO_TITLE),
                    )
                    .when_some(Video::VIDEO_SYNOPSIS, |this, synopsis| {
                        this.children(p().class(tw!(LineClamp::_3)).children(synopsis))
                    }),
            ),
            List::new(
                (1..13)
                    .map(|_| li().children(Episode::default().into_small_card()).into())
                    .collect::<Vec<_>>(),
            ),
        )
        .into()
    }
}
