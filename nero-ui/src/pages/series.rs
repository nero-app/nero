use std::rc::Rc;

use nero_extensions::{
    types::{Episode, Series},
    url::Url,
};
use rustwind::{
    backgrounds::BackgroundColor,
    borders::BorderRadius,
    flexbox_grid::{FlexDirection, Gap, GridTemplateColumns},
    layout::{Display, ObjectFit, Overflow},
    sizing::{Height, Width},
    spacing::Padding,
    tw,
    typography::{FontSize, FontWeight, LineClamp, TextOverflow},
};
use serde_wasm_bindgen::to_value;
use sycamore::{
    prelude::{create_memo, HtmlImgAttributes, ReadSignal},
    web::{
        tags::{article, div, figure, h1, header, img, li, p},
        GlobalProps, HtmlGlobalAttributes, Resource, View,
    },
};
use wasm_bindgen::UnwrapThrowExt;

use crate::{
    components::{Button, Icon, IconType, IntoClickableCard, List, ListHeader, OnReachBottom},
    hooks::{use_infinite_episodes, use_series_details, InfinitePage},
    utils::{navigate_with_state, ViewBuilder},
};

pub struct SeriesPage {
    series: Resource<Series>,
    episodes_page: Rc<InfinitePage<Episode>>,
}

impl SeriesPage {
    pub fn new(series_id: String) -> Self {
        Self {
            series: use_series_details(series_id.clone()),
            episodes_page: Rc::new(use_infinite_episodes(series_id)),
        }
    }
}

impl SeriesPage {
    fn render_poster(src: Option<Url>, alt: String) -> View {
        img()
            .class(tw!(Width::SizeFull, BorderRadius::Xl, ObjectFit::Cover))
            // TODO: Default image
            .src(src.unwrap().to_string())
            .alt(alt)
            .referrerpolicy("no-referrer")
            .into()
    }

    fn render_title(title: String) -> View {
        h1().class(tw!(
            TextOverflow::Truncate,
            FontSize::_3Xl,
            FontWeight::Bold
        ))
        .children(title)
        .into()
    }

    fn render_quick_actions(series_id: String, first_episode: ReadSignal<Option<Episode>>) -> View {
        div()
            .class(tw!(Display::Flex, Gap::Number("4")))
            .children(
                Button::new_with_icon_label(Icon::new(IconType::Play), "Watch now", move |_| {
                    match first_episode.get_clone() {
                        Some(episode) => {
                            let state = to_value(&episode).unwrap_throw();
                            let nav_to = format!("/watch/{}/{}", series_id, episode.id);
                            navigate_with_state(&nav_to, &state);
                        }
                        None => unreachable!("Button should be disabled"),
                    }
                })
                .color(BackgroundColor::Red300)
                .disabled(move || first_episode.with(|e| e.is_none())),
            )
            .children(
                Button::new_with_icon_label(
                    Icon::new(IconType::Share),
                    "Share the series",
                    |_| todo!(),
                )
                .color(BackgroundColor::Red300),
            )
            .into()
    }

    fn render_details(synopsis: String) -> View {
        div()
            .class(tw!(Display::Flex, Gap::Number("4")))
            .children(p().class(tw!(LineClamp::Number("5"))).children(synopsis))
            .into()
    }

    fn render_episodes(series_id: String, episodes: ReadSignal<Vec<Episode>>) -> View {
        List::new(move || {
            episodes
                .get_clone()
                .into_iter()
                .map(|e| {
                    let nav_to = format!("/watch/{}/{}", series_id, e.id);
                    let state = to_value(&e).unwrap_throw();
                    li().children(
                        e.into_clickable_card(move |_| navigate_with_state(&nav_to, &state)),
                    )
                    .into()
                })
                .collect::<Vec<_>>()
        })
        .header(ListHeader::new("Episodes").end_slot(Button::new_with_icon(
            Icon::new(IconType::Sort),
            |_| todo!(),
        )))
        .into()
    }
}

impl From<SeriesPage> for View {
    fn from(page: SeriesPage) -> Self {
        View::from_dynamic(move || match page.series.get_clone() {
            Some(series) => div()
                .class(tw!(
                    Display::Grid,
                    Height::HFull,
                    GridTemplateColumns::Value("2fr_3fr"),
                    Gap::Number("20")
                ))
                .children(
                    figure()
                        .class(tw!(Overflow::Hidden, Padding::BNumber("8")))
                        .children(SeriesPage::render_poster(
                            series.poster_url,
                            series.title.clone(),
                        )),
                )
                .children(
                    article()
                        // TODO: Think some way to show a loading spinner while the new episodes are loading...
                        .on_reach_bottom({
                            let episodes_page = page.episodes_page.clone();
                            move || episodes_page.load_more()
                        })
                        .class(tw!(
                            Display::Flex,
                            FlexDirection::Col,
                            Gap::Number("4"),
                            Overflow::Auto
                        ))
                        .children(
                            header()
                                .class(tw!(Display::Flex, FlexDirection::Col, Gap::Number("4")))
                                .children(SeriesPage::render_title(series.title))
                                .children(SeriesPage::render_quick_actions(series.id.clone(), {
                                    let episodes_page = page.episodes_page.clone();
                                    create_memo(move || {
                                        episodes_page.items().get_clone().first().cloned()
                                    })
                                }))
                                .when_some(series.synopsis, |this, synopsis| {
                                    this.children(SeriesPage::render_details(synopsis))
                                }),
                        )
                        .children(SeriesPage::render_episodes(
                            series.id,
                            page.episodes_page.items(),
                        )),
                ),
            _ => div().children("Loading..."),
        })
    }
}
