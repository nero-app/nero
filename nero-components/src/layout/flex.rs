use leptos::IntoView;
use typewind::{
    flexbox_grid::{
        AlignItems, Flex, FlexBasis, FlexDirection, FlexGrow, FlexShrink, FlexWrap, JustifyContent,
    },
    layout::Display,
    ToClasses,
};

use super::Layout;

/// Flex layout container that arranges child elements
/// in a configurable flexbox layout, allowing for dynamic alignment, spacing,
/// direction, and growth or shrinkage of items within the container.
#[derive(ToClasses)]
pub struct FlexLayout {
    display: Display,
    align: Option<AlignItems>,
    justify: Option<JustifyContent>,
    wrap: Option<FlexWrap>,
    direction: Option<FlexDirection>,
    basis: Option<FlexBasis>,
    grow: Option<FlexGrow>,
    shrink: Option<FlexShrink>,
    flex: Option<Flex>,
}

impl Layout<FlexLayout> {
    /// Creates a new `FlexLayout` with the specified children.
    pub fn flex(children: impl IntoView + 'static) -> Self {
        Layout::new(
            FlexLayout {
                display: Display::Flex,
                align: None,
                justify: None,
                wrap: None,
                direction: None,
                basis: None,
                grow: None,
                shrink: None,
                flex: None,
            },
            children,
        )
    }

    /// Sets the alignment of items along the cross-axis.
    pub fn align_items(mut self, align: AlignItems) -> Self {
        self.layout.align = Some(align);
        self
    }

    /// Sets the alignment of items along the main axis.
    pub fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.layout.justify = Some(justify);
        self
    }

    /// Sets the wrapping behavior for items in the flex layout.
    pub fn wrap(mut self, wrap: FlexWrap) -> Self {
        self.layout.wrap = Some(wrap);
        self
    }

    /// Sets the direction of the main axis for the flex layout.
    pub fn direction(mut self, direction: FlexDirection) -> Self {
        self.layout.direction = Some(direction);
        self
    }

    /// Defines the initial size of a flex item in the layout.
    pub fn basis(mut self, basis: FlexBasis) -> Self {
        self.layout.basis = Some(basis);
        self
    }

    /// Defines the grow factor for an item, allowing it to grow relative to others.
    pub fn grow(mut self, grow: FlexGrow) -> Self {
        self.layout.grow = Some(grow);
        self
    }

    /// Defines the shrink factor for an item, allowing it to shrink relative to others.
    pub fn shrink(mut self, shrink: FlexShrink) -> Self {
        self.layout.shrink = Some(shrink);
        self
    }

    /// Sets the shorthand `flex` property.
    pub fn flex_type(mut self, flex: Flex) -> Self {
        self.layout.flex = Some(flex);
        self
    }
}
