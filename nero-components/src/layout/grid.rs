use leptos::IntoView;
use typewind::{
    flexbox_grid::{
        GridAutoColumns, GridAutoFlow, GridAutoRows, GridColumnStartEnd, GridRowStartEnd,
        GridTemplateColumns, GridTemplateRows,
    },
    layout::Display,
    ToClasses,
};

use super::Layout;

/// Represents a grid-based layout system, allowing elements to be arranged
/// in a structured grid format with customizable row and column properties.
#[derive(ToClasses)]
pub struct GridLayout {
    display: Display,
    template_columns: Option<GridTemplateColumns>,
    auto_flow: Option<GridAutoFlow>,
    auto_rows: Option<GridAutoRows>,
    auto_columns: Option<GridAutoColumns>,
    template_rows: Option<GridTemplateRows>,
}

impl Layout<GridLayout> {
    /// Creates a new `GridLayout` with the specified children element.
    pub fn grid(children: impl IntoView + 'static) -> Self {
        Layout::new(
            GridLayout {
                display: Display::Grid,
                template_columns: None,
                auto_flow: None,
                auto_rows: None,
                auto_columns: None,
                template_rows: None,
            },
            children,
        )
    }

    /// Sets a custom column template to define explicit column sizes in the grid layout.
    pub fn template_columns(mut self, template_columns: GridTemplateColumns) -> Self {
        self.layout.template_columns = Some(template_columns);
        self
    }

    /// Sets the auto-flow behavior to control the direction in which items are placed automatically.
    pub fn auto_flow(mut self, auto_flow: GridAutoFlow) -> Self {
        self.layout.auto_flow = Some(auto_flow);
        self
    }

    /// Defines the row size for implicitly generated rows in the grid layout.
    pub fn auto_rows(mut self, auto_rows: GridAutoRows) -> Self {
        self.layout.auto_rows = Some(auto_rows);
        self
    }

    /// Defines the column size for implicitly generated columns in the grid layout.
    pub fn auto_columns(mut self, auto_columns: GridAutoColumns) -> Self {
        self.layout.auto_columns = Some(auto_columns);
        self
    }

    /// Sets a custom row template to define explicit row sizes in the grid layout.
    pub fn template_rows(mut self, template_rows: GridTemplateRows) -> Self {
        self.layout.template_rows = Some(template_rows);
        self
    }
}

/// Represents an individual item within a `GridLayout`. Each item can specify
/// its position within the grid through the `column` and `row` properties.
#[derive(ToClasses)]
pub struct GridItem {
    column: Vec<GridColumnStartEnd>,
    row: Vec<GridRowStartEnd>,
}

impl Layout<GridItem> {
    /// Creates a new `GridItem` with the specified children element.
    pub fn grid_item(children: impl IntoView + 'static) -> Self {
        Layout::new(
            GridItem {
                column: vec![],
                row: vec![],
            },
            children,
        )
    }

    /// Sets the column start or end position for the grid item.
    pub fn column(mut self, column: GridColumnStartEnd) -> Self {
        self.layout.column.push(column);
        self
    }

    /// Sets the row start or end position for the grid item.
    pub fn row(mut self, row: GridRowStartEnd) -> Self {
        self.layout.row.push(row);
        self
    }
}
