mod home;
mod series;
mod watch;

pub use home::*;
// Marked as unused until router is created
#[allow(unused_imports)]
pub use series::*;
use sycamore_router::{HistoryIntegration, Route, Router, RouterProps};
#[allow(unused_imports)]
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

#[derive(Clone, Route)]
enum AppRoutes {
    #[to("/")]
    Home,
    #[to("/series")]
    Series,
    #[to("/watch")]
    Watch,
    #[not_found]
    NotFound,
}

pub struct App;

impl From<App> for View {
    fn from(_: App) -> Self {
        Router(RouterProps::new(HistoryIntegration::new(), |route| {
            BaseLayout::new(move || match route.get_clone() {
                AppRoutes::Home => Into::<View>::into(HomePage),
                AppRoutes::Series => SeriesPage.into(),
                AppRoutes::Watch => WatchPage.into(),
                AppRoutes::NotFound => todo!(),
            })
            .into()
        }))
    }
}

struct BaseLayout {
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
