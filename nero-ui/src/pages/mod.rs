mod home;
mod search;
mod series;
mod watch;

pub use home::*;
pub use search::*;
pub use series::*;
pub use watch::*;

use sycamore_router::{use_search_query, HistoryIntegration, Route, Router, RouterProps};

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
    #[to("/search")]
    Search,
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
                AppRoutes::Search => {
                    let q = use_search_query("q");
                    SearchPage::new(q.get_clone().unwrap_or_default()).into()
                }
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
                Height::HScreen,
                Width::WFull,
                Gap::Number("4"),
                Padding::XNumber("12"),
                Padding::TNumber("4")
            ))
            .children(Toolbar)
            .children(
                main()
                    .class(tw!(Height::HFull, Flex::Number("1"), Overflow::Auto))
                    .children(layout.children),
            )
            .into()
    }
}
