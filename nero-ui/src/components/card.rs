use nero_extensions::types::{Episode, Series};
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
    typography::{Color, FontSize, FontWeight, LineClamp, TextOverflow},
};
use sycamore::{
    prelude::HtmlImgAttributes,
    web::{
        events::{click, MouseEvent},
        tags::{div, h3, img, p, span},
        GlobalAttributes, GlobalProps, HtmlGlobalAttributes, View,
    },
};

use crate::{tw, utils::ViewBuilder};

// TODO: set a fixed size for posters, to handle cases where images are of different sizes

const BASE_CARD_CLASSES: &str = tw!(
    Display::Flex,
    AlignItems::Center,
    Padding::Number("1"),
    Cursor::Pointer,
    BorderRadius::Md,
    TransitionDuration::Number("300"),
    hover!(BackgroundColor::Gray100),
    active!(Scale::Number("95"))
);

pub trait IntoSmallClickableCard {
    fn into_small_clickable_card(self, on_click: impl FnMut(MouseEvent) + 'static) -> View;
}

pub trait IntoClickableCard {
    fn into_clickable_card(self, on_click: impl FnMut(MouseEvent) + 'static) -> View;
}

impl IntoClickableCard for Series {
    fn into_clickable_card(self, on_click: impl FnMut(MouseEvent) + 'static) -> View {
        div()
            .class(tw!(FlexDirection::Col, Gap::Number("1"), BASE_CARD_CLASSES))
            .children(
                img()
                    .class(tw!(BorderRadius::Lg))
                    // TODO: use a default thumbnail if none is provided
                    .src(self.poster_url.unwrap().to_string())
                    .alt(self.title.clone()),
            )
            .children(
                h3().class(tw!(
                    TextOverflow::Truncate,
                    FontSize::Sm,
                    FontWeight::Semibold
                ))
                .children(self.title),
            )
            .on(click, on_click)
            .into()
    }
}

impl IntoSmallClickableCard for Episode {
    fn into_small_clickable_card(self, on_click: impl FnMut(MouseEvent) + 'static) -> View {
        div()
            .class(tw!(Gap::Number("4"), BASE_CARD_CLASSES))
            .children(
                img()
                    .class(tw!(
                        Width::WFraction(1, 2),
                        BorderRadius::Lg,
                        AspectRatio::Video,
                        ObjectFit::Cover
                    ))
                    // TODO: use a default thumbnail if none is provided
                    .src(self.thumbnail_url.unwrap().to_string())
                    .alt(
                        self.title
                            .clone()
                            .unwrap_or(format!("Episode {}", self.number)),
                    ),
            )
            .children(
                div()
                    .class(tw!(
                        Width::WFraction(1, 2),
                        Display::Flex,
                        FlexDirection::Col,
                        Gap::Number("1")
                    ))
                    .children(
                        h3().class(tw!(TextOverflow::Truncate, FontWeight::Semibold))
                            .children(format!("Episode {}", self.number)),
                    )
                    .when_some(self.title, |this, title| {
                        this.children(
                            p().class(tw!(LineClamp::Number("2"), Color::Gray500, FontSize::Sm))
                                .children(title),
                        )
                    }),
            )
            .on(click, on_click)
            .into()
    }
}

impl IntoClickableCard for Episode {
    fn into_clickable_card(self, on_click: impl FnMut(MouseEvent) + 'static) -> View {
        let title = self.title.unwrap_or(format!("Episode {}", self.number));

        div()
            .class(tw!(Gap::Number("4"), BASE_CARD_CLASSES))
            .children(
                span()
                    .class(tw!(
                        Width::WFraction(1, 12),
                        Display::Flex,
                        JustifyContent::Center,
                        FontWeight::Semibold
                    ))
                    .children(self.number),
            )
            .children(
                img()
                    .class(tw!(
                        Width::WFraction(4, 12),
                        BorderRadius::Lg,
                        AspectRatio::Video,
                        ObjectFit::Cover
                    ))
                    // TODO: use a default thumbnail if none is provided
                    .src(self.thumbnail_url.unwrap().to_string())
                    .alt(title.clone()),
            )
            .children(
                div()
                    .class(tw!(
                        Display::Flex,
                        FlexDirection::Col,
                        Width::WFraction(7, 12)
                    ))
                    .children(h3().class(tw!(FontWeight::Semibold)).children(title))
                    .when_some(self.description, |this, description| {
                        this.children(
                            p().class(tw!(Color::Gray500, FontSize::Sm, LineClamp::Number("3")))
                                .children(description),
                        )
                    }),
            )
            .on(click, on_click)
            .into()
    }
}
