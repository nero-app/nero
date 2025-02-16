use nero_extensions::{
    types::{EpisodesPage, Series},
    url::Url,
};
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
    components::{Button, Icon, IconType, IntoClickableCard, List, ListHeader},
    tw,
    types::{sample_episode, sample_series},
    utils::ViewBuilder,
};

pub struct SeriesPage {
    series: Series,
    episodes: EpisodesPage,
}

impl SeriesPage {
    pub fn new() -> Self {
        Self {
            series: sample_series(),
            episodes: EpisodesPage {
                items: (1..=12).map(|_| sample_episode()).collect::<Vec<_>>(),
                has_next_page: false,
            },
        }
    }
}

impl SeriesPage {
    fn render_series_poster(src: Option<Url>, alt: String) -> View {
        img()
            .class(tw!(
                Width::WFull,
                Height::HFull,
                ObjectFit::Cover,
                BorderRadius::Xl
            ))
            // TODO: Default image
            .src(src.unwrap().to_string())
            .alt(alt)
            .into()
    }

    fn render_series_details(title: String, synopsis: Option<String>) -> View {
        header()
            .class(tw!(Display::Flex, FlexDirection::Col, Gap::Number("4")))
            .children(
                h1().class(tw!(
                    FontSize::_3Xl,
                    FontWeight::Bold,
                    TextOverflow::Truncate
                ))
                .children(title),
            )
            .children(
                div()
                    .class(tw!(Display::Flex, Gap::Number("4")))
                    .children(
                        Button::icon_label(Icon::new(IconType::Play), "Watch now", |_| {
                            navigate("/watch")
                        })
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
            .when_some(synopsis, |this, synopsis| {
                this.children(p().class(tw!(LineClamp::Number("5"))).children(synopsis))
            })
            .into()
    }

    fn render_series_episodes(episodes: EpisodesPage) -> View {
        List::new(
            episodes
                .items
                .into_iter()
                .map(|e| {
                    li().children(e.into_clickable_card(|_| navigate("/watch")))
                        .into()
                })
                .collect::<Vec<_>>(),
        )
        .header(
            ListHeader::new("Episodes")
                .end_slot(Button::icon(Icon::new(IconType::Sort), |_| todo!())),
        )
        .into()
    }
}

impl From<SeriesPage> for View {
    fn from(page: SeriesPage) -> Self {
        div()
            .class(tw!(Display::Flex, Height::HFull, Gap::Number("20")))
            .children(
                figure()
                    .class(tw!(
                        Width::WFraction(2, 5),
                        Padding::BNumber("8"),
                        Overflow::Hidden
                    ))
                    .children(SeriesPage::render_series_poster(
                        page.series.poster_url,
                        page.series.title.clone(),
                    )),
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
                    .children(SeriesPage::render_series_details(
                        page.series.title,
                        page.series.synopsis,
                    ))
                    .children(SeriesPage::render_series_episodes(page.episodes)),
            )
            .into()
    }
}
