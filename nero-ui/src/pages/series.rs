use rustwind::{
    backgrounds::BackgroundColor,
    borders::BorderRadius,
    flexbox_grid::{FlexDirection, Gap},
    layout::{Display, ObjectFit},
    sizing::{Height, Width},
    typography::{FontSize, FontWeight, LineClamp, TextOverflow},
};
use sycamore::{
    prelude::HtmlImgAttributes,
    web::{
        tags::{div, h1, header, img, li, p},
        GlobalProps, HtmlGlobalAttributes, View,
    },
};

use crate::{
    components::{Button, Icon, IconType, IntoCard, List, ListHeader},
    tw,
    types::{Episode, Series},
    utils::ViewBuilder,
};

use super::SplitLayout;

pub struct SeriesPage;

impl From<SeriesPage> for View {
    fn from(_: SeriesPage) -> Self {
        let series = Series::default();

        SplitLayout::new_default(
            img()
                .class(tw!(
                    Width::Full,
                    Height::Full,
                    BorderRadius::Xl,
                    ObjectFit::Cover
                ))
                .src(series.poster_url)
                .alt(series.title.clone()),
            (
                header()
                    .class(tw!(Display::Flex, FlexDirection::Col, Gap::_4))
                    .children(
                        h1().class(tw!(
                            FontSize::_3xl,
                            FontWeight::Bold,
                            TextOverflow::Truncate
                        ))
                        .children(series.title),
                    )
                    .children(
                        div()
                            .class(tw!(Display::Flex, Gap::_4))
                            .children(
                                Button::icon_label(
                                    Icon::new(IconType::Play),
                                    "Watch now",
                                    |_| todo!(),
                                )
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
                    .when_some(series.synopsis, |this, synopsis| {
                        this.children(p().class(tw!(LineClamp::_5)).children(synopsis))
                    }),
                List::new(
                    (1..13)
                        .map(|_| li().children(Episode::default().into_card()).into())
                        .collect::<Vec<_>>(),
                )
                .header(
                    ListHeader::new("Episodes")
                        .end_slot(Button::icon(Icon::new(IconType::Sort), |_| todo!())),
                ),
            ),
        )
        .into()
    }
}
