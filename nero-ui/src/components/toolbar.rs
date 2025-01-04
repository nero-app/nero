use rustwind::{flexbox_grid::JustifyContent, layout::Display, sizing::Width};
use sycamore::web::{
    tags::{nav, p},
    GlobalProps, HtmlGlobalAttributes, View,
};

use crate::tw;

pub struct Toolbar;

impl From<Toolbar> for View {
    fn from(_: Toolbar) -> Self {
        nav()
            .class(tw!(Display::Flex, Width::Full, JustifyContent::Between))
            .children(p().children("Toolbar goes here!"))
            .children(p().children("Options goes here..."))
            .children(p().children("And more options here..."))
            .into()
    }
}
