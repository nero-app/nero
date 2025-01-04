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
    components::{Button, Icon, IconType, List, ListHeader},
    tw,
};

use super::SplitLayout;

// Sample data
const SERIES_TITLE: &str = "SPY x FAMILY";
const SERIES_POSTER_URL: &str = "https://m.media-amazon.com/images/M/MV5BMDlmZGJkYTUtNDcwNi00YWMzLTkyNmMtOWQ3MzVhOTU5YWY0XkEyXkFqcGc@._V1_.jpg";
const SERIES_SYNOPSIS: &str =  "World peace is at stake and secret agent Twilight must undergo his most difficult mission yet—pretend to be a family man. Posing as a loving husband and father, he’ll infiltrate an elite school to get close to a high-profile politician. He has the perfect cover, except his wife’s a deadly assassin and neither knows each other’s identity. But someone does, his adopted daughter who’s a telepath!";

pub struct SeriesPage;

impl From<SeriesPage> for View {
    fn from(_: SeriesPage) -> Self {
        SplitLayout::new_default(
            img()
                .class(tw!(
                    Width::Full,
                    Height::Full,
                    BorderRadius::Xl,
                    ObjectFit::Cover
                ))
                .src(SERIES_POSTER_URL)
                .alt(SERIES_TITLE),
            (
                header()
                    .class(tw!(Display::Flex, FlexDirection::Col, Gap::_4))
                    .children(
                        h1().class(tw!(
                            FontSize::_3xl,
                            FontWeight::Bold,
                            TextOverflow::Truncate
                        ))
                        .children(SERIES_TITLE),
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
                    .children(p().class(tw!(LineClamp::_5)).children(SERIES_SYNOPSIS)),
                List::new(
                    (0..100)
                        .map(|i| li().children(i).into())
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
