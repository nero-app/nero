use leptos::{
    html::{article, aside, div, main, section, ElementChild},
    prelude::{AnyView, ClassAttribute, IntoAny},
    IntoView,
};
use typewind::{
    flexbox_grid::Gap,
    layout::{Overflow, Position},
    sizing::{Height, Width},
    spacing::Padding,
    ToClasses,
};

mod flex;
pub use flex::*;
mod grid;
pub use grid::*;

mod stack;
pub use stack::*;

use crate::IntoComponent;

/// Possible HTML tags that can be used for layout containers.
pub enum LayoutTag {
    /// `<div>`
    Div,
    /// `<main>`
    Main,
    /// `<aside>`
    Aside,
    /// `<section>`
    Section,
    /// `<article>`
    Article,
}

/// A generic layout container for creating flexible layouts with various configurations.
/// The `Layout` struct allows defining attributes like `position`, `height`, `width`,
/// `gap`, `padding`, and `overflow`, with the ability to customize the HTML tag used
/// to wrap the layout content.
#[derive(ToClasses)]
pub struct Layout<L> {
    #[tw(skip)]
    layout: L,
    #[tw(skip)]
    tag: LayoutTag,
    position: Option<Position>,
    height: Option<Height>,
    width: Option<Width>,
    gap: Option<Gap>,
    padding: Vec<Padding>,
    overflow: Option<Overflow>,
    #[tw(skip)]
    children: AnyView,
}

impl<L> Layout<L> {
    fn new(layout: L, children: impl IntoView + 'static) -> Self {
        Self {
            layout,
            tag: LayoutTag::Div,
            position: None,
            height: None,
            width: None,
            gap: None,
            padding: vec![],
            overflow: None,
            children: children.into_any(),
        }
    }

    /// Sets the HTML tag to be used for the layout container (default is `Div`).
    pub fn tag(mut self, tag: LayoutTag) -> Self {
        self.tag = tag;
        self
    }

    /// Sets the `position` property for the layout container.
    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }

    /// Sets the `height` property for the layout container.
    pub fn height(mut self, height: Height) -> Self {
        self.height = Some(height);
        self
    }

    /// Sets the `width` property for the layout container.
    pub fn width(mut self, width: Width) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the `gap` property, specifying spacing between child elements within the layout.
    pub fn gap(mut self, gap: Gap) -> Self {
        self.gap = Some(gap);
        self
    }

    /// Adds padding to the layout container.
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding.push(padding);
        self
    }

    /// Adds multiple padding values to the layout container.
    pub fn paddings(mut self, paddings: impl IntoIterator<Item = Padding>) -> Self {
        self.padding.extend(paddings);
        self
    }

    /// Sets the `overflow` property for the layout container.
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }
}

impl<L: ToClasses> IntoComponent for Layout<L> {
    fn into_component(self) -> impl IntoView {
        let classes = format!("{} {}", self.classes(), self.layout.classes());

        match self.tag {
            LayoutTag::Div => div().class(classes).child(self.children).into_any(),
            LayoutTag::Main => main().class(classes).child(self.children).into_any(),
            LayoutTag::Aside => aside().class(classes).child(self.children).into_any(),
            LayoutTag::Section => section().class(classes).child(self.children).into_any(),
            LayoutTag::Article => article().class(classes).child(self.children).into_any(),
        }
    }
}
