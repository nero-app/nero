use nero_extensions::types::{Episode, Series};
use rustwind::{
    active,
    backgrounds::BackgroundColor,
    borders::BorderRadius,
    flexbox_grid::{AlignItems, FlexDirection, Gap, GridTemplateColumns},
    hover,
    interactivity::Cursor,
    layout::{AspectRatio, Display, ObjectFit},
    sizing::{MinWidth, Width},
    spacing::Padding,
    transforms::Scale,
    transitions_animation::TransitionDuration,
    tw,
    typography::{Color, FontSize, FontWeight, LineClamp, TextAlign, TextOverflow},
};
use sycamore::{
    prelude::HtmlImgAttributes,
    web::{
        events::{click, MouseEvent},
        tags::{div, h3, img, p, span},
        GlobalAttributes, GlobalProps, HtmlGlobalAttributes, View,
    },
};

use crate::utils::ViewBuilder;

const BASE_CARD_CLASSES: &str = tw!(
    Cursor::Pointer,
    BorderRadius::Md,
    Padding::Number("1"),
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
            .class(tw!(
                Display::Flex,
                FlexDirection::Col,
                Gap::Number("1"),
                BASE_CARD_CLASSES
            ))
            .children(
                img()
                    .class(tw!(
                        AspectRatio::Value("2/3"),
                        BorderRadius::Lg,
                        ObjectFit::Cover
                    ))
                    // TODO: use a default thumbnail if none is provided
                    .src(self.poster_url.unwrap().to_string())
                    .alt(self.title.clone())
                    .referrerpolicy("no-referrer"),
            )
            .children(
                h3().class(tw!(
                    TextOverflow::Truncate,
                    TextAlign::Center,
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
            .class(tw!(
                Display::Grid,
                GridTemplateColumns::Number("2"),
                AlignItems::Center,
                Gap::Number("4"),
                BASE_CARD_CLASSES
            ))
            .children(
                div()
                    .class(tw!(AspectRatio::Video, MinWidth::Value("125px")))
                    .children(
                        img()
                            .class(tw!(Width::SizeFull, BorderRadius::Lg, ObjectFit::Cover))
                            // TODO: use a default thumbnail if none is provided
                            .src(self.thumbnail_url.unwrap().to_string())
                            .alt(
                                self.title
                                    .clone()
                                    .unwrap_or(format!("Episode {}", self.number)),
                            )
                            .referrerpolicy("no-referrer"),
                    ),
            )
            .children(
                div()
                    .class(tw!(MinWidth::Number("0")))
                    .children(
                        h3().class(tw!(TextOverflow::Truncate, FontWeight::Semibold))
                            .children(format!("Episode {}", self.number)),
                    )
                    .when_some(self.title, |this, title| {
                        this.children(
                            p().class(tw!(LineClamp::Number("2"), FontSize::Sm, Color::Gray500))
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
            .class(tw!(
                Display::Grid,
                GridTemplateColumns::Value("1fr_4fr_7fr"),
                AlignItems::Center,
                Gap::Number("4"),
                BASE_CARD_CLASSES
            ))
            .children(
                span()
                    .class(tw!(
                        TextOverflow::Truncate,
                        TextAlign::Center,
                        FontWeight::Semibold
                    ))
                    .children(self.number),
            )
            .children(
                div()
                    .class(tw!(AspectRatio::Video, MinWidth::Value("150px")))
                    .children(
                        img()
                            .class(tw!(Width::SizeFull, BorderRadius::Lg, ObjectFit::Cover))
                            // TODO: use a default thumbnail if none is provided
                            .src(self.thumbnail_url.unwrap().to_string())
                            .alt(title.clone())
                            .referrerpolicy("no-referrer"),
                    ),
            )
            .children(
                div()
                    .class(tw!(MinWidth::Number("0")))
                    .children(
                        h3().class(tw!(TextOverflow::Truncate, FontWeight::Semibold))
                            .children(title),
                    )
                    .when_some(self.description, |this, description| {
                        this.children(
                            p().class(tw!(LineClamp::Number("3"), FontSize::Sm, Color::Gray500))
                                .children(description),
                        )
                    }),
            )
            .on(click, on_click)
            .into()
    }
}
