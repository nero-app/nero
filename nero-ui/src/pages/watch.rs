use rustwind::{
    flexbox_grid::{FlexDirection, Gap},
    layout::AspectRatio,
    layout::Display,
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

use crate::{components::List, tw, utils::ViewBuilder};

use super::SplitLayout;

// Sample data
const VIDEO_URL: &str =
    "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4";
const VIDEO_TITLE: &str = "Big Buck Bunny";
const VIDEO_SYNOPSIS: Option<&'static str> = Some("Big Buck Bunny tells the story of a giant rabbit with a heart bigger than himself. When one sunny day three rodents rudely harass him, something snaps... and the rabbit ain't no bunny anymore! In the typical cartoon tradition he prepares the nasty rodents a comical revenge.\n\nLicensed under the Creative Commons Attribution license\nhttp://www.bigbuckbunny.org");

pub struct WatchPage;

impl From<WatchPage> for View {
    fn from(_: WatchPage) -> Self {
        SplitLayout::new_watch(
            (
                video()
                    .class(tw!(Width::Full, AspectRatio::Video))
                    .controls(true)
                    .src(VIDEO_URL),
                section()
                    .class(tw!(Display::Flex, FlexDirection::Col, Gap::_2))
                    .children(
                        h1().class(tw!(
                            LineClamp::_2,
                            SpaceBetween::X2,
                            FontSize::_2xl,
                            FontWeight::Semibold
                        ))
                        .children(VIDEO_TITLE),
                    )
                    .when_some(VIDEO_SYNOPSIS, |this, synopsis| {
                        this.children(p().class(tw!(LineClamp::_3)).children(synopsis))
                    }),
            ),
            List::new(
                (0..100)
                    .map(|i| li().children(i).into())
                    .collect::<Vec<_>>(),
            ),
        )
        .into()
    }
}
