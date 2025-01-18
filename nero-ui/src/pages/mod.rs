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
    tags::{div, main},
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
