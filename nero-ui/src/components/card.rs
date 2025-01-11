use rustwind::{
    active,
    backgrounds::BackgroundColor,
    borders::BorderRadius,
    flexbox_grid::{AlignItems, FlexDirection, Gap, JustifyContent},
    hover,
    interactivity::Cursor,
    layout::{AspectRatio, Display, ObjectFit},
    sizing::Width,
    spacing::Padding,
    transforms::Scale,
    transitions_animation::TransitionDuration,
    typography::{FontSize, FontWeight, LineClamp, TextColor, TextOverflow},
};
use sycamore::{
    prelude::HtmlImgAttributes,
    web::{
        tags::{div, h3, img, p, span, HtmlDiv},
        GlobalProps, HtmlGlobalAttributes, View,
    },
};

use crate::{tw, types::Episode, utils::ViewBuilder};

pub trait IntoSmallCard<T: Into<View>> {
    fn into_small_card(self) -> T;
}

pub trait IntoCard<T: Into<View>> {
    fn into_card(self) -> T;
}

const BASE_EPISODE_CARD_CLASSES: &str = tw!(
    Display::Flex,
    AlignItems::Center,
    Gap::_4,
    Padding::P1,
    Cursor::Pointer,
    BorderRadius::Md,
    TransitionDuration::_300,
    hover!(BackgroundColor::Gray100),
    active!(Scale::_95)
);

impl IntoSmallCard<HtmlDiv> for Episode {
    fn into_small_card(self) -> HtmlDiv {
        div()
            .class(BASE_EPISODE_CARD_CLASSES)
            .children(
                img()
                    .class(tw!(
                        Width::_1over2,
                        BorderRadius::Lg,
                        AspectRatio::Video,
                        ObjectFit::Cover
                    ))
                    // TODO: use a default thumbnail if none is provided
                    .src(self.thumbnail_url.unwrap_or_default())
                    .alt(
                        self.title
                            .clone()
                            .unwrap_or(format!("Episode {}", self.number)),
                    ),
            )
            .children(
                div()
                    .class(tw!(
                        Width::_1over2,
                        Display::Flex,
                        FlexDirection::Col,
                        Gap::_1
                    ))
                    .children(
                        h3().class(tw!(TextOverflow::Truncate, FontWeight::Semibold))
                            .children(format!("Episode {}", self.number)),
                    )
                    .when_some(self.title, |this, title| {
                        this.children(
                            p().class(tw!(LineClamp::_2, TextColor::Gray500, FontSize::Sm))
                                .children(title),
                        )
                    }),
            )
    }
}

impl IntoCard<HtmlDiv> for Episode {
    fn into_card(self) -> HtmlDiv {
        let title = self.title.unwrap_or(format!("Episode {}", self.number));

        div()
            .class(BASE_EPISODE_CARD_CLASSES)
            .children(
                span()
                    .class(tw!(
                        Width::_1over12,
                        Display::Flex,
                        JustifyContent::Center,
                        FontWeight::Semibold
                    ))
                    .children(self.number),
            )
            .children(
                img()
                    .class(tw!(
                        Width::_4over12,
                        BorderRadius::Lg,
                        AspectRatio::Video,
                        ObjectFit::Cover
                    ))
                    // TODO: use a default thumbnail if none is provided
                    .src(self.thumbnail_url.unwrap_or_default())
                    .alt(title.clone()),
            )
            .children(
                div()
                    .class(tw!(Display::Flex, FlexDirection::Col, Width::_7over12))
                    .children(h3().class(tw!(FontWeight::Semibold)).children(title))
                    .when_some(self.description, |this, description| {
                        this.children(
                            p().class(tw!(TextColor::Gray500, FontSize::Sm, LineClamp::_3))
                                .children(description),
                        )
                    }),
            )
    }
}
