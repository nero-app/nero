mod home;
mod search;
mod series;
mod watch;

use home::HomePage;
use search::SearchPage;
use series::SeriesPage;
use sycamore_router::{use_search_query, HistoryIntegration, Route, Router, RouterProps};

use rustwind::{
    flexbox_grid::{FlexDirection, Gap},
    layout::{Display, Overflow, Position},
    sizing::{Height, Width},
    spacing::Padding,
};
use sycamore::web::{
    tags::{div, main},
    GlobalProps, HtmlGlobalAttributes, View,
};
use watch::WatchPage;

use crate::{components::Toolbar, tw};

#[derive(Clone, Route)]
enum AppRoutes {
    #[to("/")]
    Home,
    #[to("/search")]
    Search,
    #[to("/series/<series_id>")]
    Series { series_id: String },
    #[to("/watch/<series_id>/<episode_id>")]
    Watch {
        series_id: String,
        episode_id: String,
    },
    #[not_found]
    NotFound,
}

pub struct App;

impl From<App> for View {
    fn from(_: App) -> Self {
        Router(RouterProps::new(HistoryIntegration::new(), |route| {
            BaseLayout::new(move || match route.get_clone() {
                AppRoutes::Home => Into::<View>::into(HomePage::default()),
                AppRoutes::Search => {
                    let q = use_search_query("q");
                    SearchPage::new(q.get_clone().unwrap_or_default()).into()
                }
                AppRoutes::Series { series_id } => SeriesPage::new(series_id).into(),
                AppRoutes::Watch {
                    series_id,
                    episode_id,
                } => WatchPage::new(series_id, episode_id).into(),
                AppRoutes::NotFound => unreachable!(),
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
                Height::HScreen,
                Width::WFull,
                FlexDirection::Col,
                Gap::Number("4"),
                Padding::XNumber("12"),
                Padding::TNumber("4")
            ))
            .children(Toolbar)
            .children(
                main()
                    .class(tw!(Height::HFull, Overflow::Auto))
                    .children(layout.children),
            )
            .into()
    }
}
