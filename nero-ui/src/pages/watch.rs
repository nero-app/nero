use nero_extensions::{
    types::{Episode, EpisodesPage, Series, SeriesVideo},
    url::Url,
};
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
    types::{sample_episode, sample_series, sample_series_video},
    utils::ViewBuilder,
};

pub struct WatchPage {
    #[allow(dead_code)]
    series: Series,
    episode: Episode,
    episodes: EpisodesPage,
    video: SeriesVideo,
}

impl WatchPage {
    pub fn new() -> Self {
        Self {
            series: sample_series(),
            episode: sample_episode(),
            episodes: EpisodesPage {
                items: (1..=12).map(|_| sample_episode()).collect::<Vec<_>>(),
                has_next_page: false,
            },
            video: sample_series_video(),
        }
    }
}

impl WatchPage {
    fn render_video_player(src: Url) -> View {
        video()
            .class(tw!(Width::WFull, AspectRatio::Video))
            .controls(true)
            .src(src.to_string())
            .into()
    }

    fn render_episode_details(title: String, synopsis: Option<String>) -> View {
        section()
            .class(tw!(Display::Flex, FlexDirection::Col, Gap::Number("2")))
            .children(
                h1().class(tw!(
                    LineClamp::Number("2"),
                    FontSize::_2Xl,
                    FontWeight::Semibold
                ))
                .children(title),
            )
            .when_some(synopsis, |this, synopsis| {
                this.children(p().class(tw!(LineClamp::Number("3"))).children(synopsis))
            })
            .into()
    }

    fn render_episodes(episodes: EpisodesPage) -> View {
        List::new(
            episodes
                .items
                .into_iter()
                .map(|e| li().children(e.into_small_card()).into())
                .collect::<Vec<_>>(),
        )
        .header(ListHeader::new("Episodes"))
        .into()
    }
}

impl From<WatchPage> for View {
    fn from(page: WatchPage) -> Self {
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
                    .children(WatchPage::render_video_player(page.video.video_url))
                    .children(WatchPage::render_episode_details(
                        page.episode
                            .title
                            .unwrap_or(format!("Episode {}", page.episode.number)),
                        page.episode.description,
                    )),
            )
            .children(
                aside()
                    .class(tw!(Width::WFraction(2, 6), Overflow::YAuto))
                    .children(WatchPage::render_episodes(page.episodes)),
            )
            .into()
    }
}
