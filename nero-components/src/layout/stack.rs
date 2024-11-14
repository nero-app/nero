#![allow(clippy::new_ret_no_self)]

use leptos::IntoView;
use typewind::flexbox_grid::FlexDirection;

use super::{Flex, Layout};

/// Flex layout container that arranges child elements
/// in a vertical (column) stack.
pub struct VStack;

impl VStack {
    /// Creates a vertical stack layout with child elements aligned in a column.
    pub fn new(children: impl IntoView + 'static) -> Layout<Flex> {
        Flex::new(children).direction(FlexDirection::Col)
    }
}

/// Flex layout container that arranges child elements
/// in a horizontal (row) stack.
pub struct HStack;

impl HStack {
    /// Creates a horizontal stack layout with child elements aligned in a row.
    pub fn new(children: impl IntoView + 'static) -> Layout<Flex> {
        Flex::new(children).direction(FlexDirection::Row)
    }
}
