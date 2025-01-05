mod series;
mod watch;

// Marked as unused until router is created
#[allow(unused_imports)]
pub use series::*;
pub use watch::*;

use rustwind::{
    flexbox_grid::{Flex, FlexDirection, Gap},
    layout::{Display, Overflow, Position},
    sizing::{Height, Width},
    spacing::Padding,
};
use sycamore::web::{
    tags::{article, aside, div, figure, main},
    GlobalProps, HtmlGlobalAttributes, View,
};

use crate::{components::Toolbar, tw};

pub struct BaseLayout {
    children: View,
}

impl BaseLayout {
    pub fn new(children: impl Into<View>) -> Self {
        Self {
            children: children.into(),
        }
    }
}

impl From<BaseLayout> for View {
    fn from(layout: BaseLayout) -> Self {
        div()
            .class(tw!(
                Position::Fixed,
                Display::Flex,
                FlexDirection::Col,
                Height::Screen,
                Width::Full,
                Gap::_4,
                Padding::Px12,
                Padding::Pt4
            ))
            .children(Toolbar)
            .children(
                main()
                    .class(tw!(Height::Full, Flex::_1, Overflow::Auto))
                    .children(layout.children),
            )
            .into()
    }
}

pub enum SplitLayout {
    Default { left: View, right: View },
    Watch { left: View, right: View },
}

impl SplitLayout {
    pub fn new_default(figure: impl Into<View>, article: impl Into<View>) -> Self {
        Self::Default {
            left: figure.into(),
            right: article.into(),
        }
    }

    pub fn new_watch(article: impl Into<View>, aside: impl Into<View>) -> Self {
        Self::Watch {
            left: article.into(),
            right: aside.into(),
        }
    }
}

impl From<SplitLayout> for View {
    fn from(layout: SplitLayout) -> Self {
        match layout {
            SplitLayout::Default { left, right } => div()
                .class(tw!(Display::Flex, Height::Full, Gap::_20))
                .children(
                    figure()
                        .class(tw!(Width::_2over5, Padding::Pb8, Overflow::Hidden))
                        .children(left),
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
                        .children(right),
                ),
            SplitLayout::Watch { left, right } => div()
                .class(tw!(Display::Flex, Height::Full, Gap::_12, Overflow::Hidden))
                .children(
                    article()
                        .class(tw!(
                            Display::Flex,
                            FlexDirection::Col,
                            Width::_4over6,
                            Gap::_4
                        ))
                        .children(left),
                )
                .children(
                    aside()
                        .class(tw!(Width::_2over6, Overflow::YAuto))
                        .children(right),
                ),
        }
        .into()
    }
}
