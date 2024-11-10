use typewind_macros::{Display, Parse};

tailwind_types!(
    AspectRatio, Container, Columns, BreakAfter, BreakBefore, BreakInside, BoxDecorationBreak, 
    BoxSizing, Display, Floats, Clear, Isolation, ObjectFit, ObjectPosition, Overflow, 
    OverscrollBehavior, Position, TopRightBottomLeft, Visibility, ZIndex
);

/// Utilities for controlling the aspect ratio of an element.
/// 
/// <https://tailwindcss.com/docs/aspect-ratio>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "aspect")]
pub enum AspectRatio {
    /// ```css
    /// {
    ///     aspect-ratio: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     aspect-ratio: 1 / 1;
    /// }
    /// ```
    Square,
    /// ```css
    /// {
    ///     aspect-ratio: 16 / 9;
    /// }
    /// ```
    Video,
}

/// A component for fixing an element's width to the current breakpoint.
/// 
/// <https://tailwindcss.com/docs/container>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum Container {
    /// | Breakpoint     | Properties             |
    /// |----------------|------------------------|
    /// | `None`         | `width: 100%;`         |
    /// | `sm (640px)`   | `max-width: 640px;`    |
    /// | `md (768px)`   | `max-width: 768px;`    |
    /// | `lg (1024px)`  | `max-width: 1024px;`   |
    /// | `xl (1280px)`  | `max-width: 1280px;`   |
    /// | `2xl (1536px)` | `max-width: 1536px;`   |
    Container,
}

/// Utilities for controlling the number of columns within an element.
///
/// <https://tailwindcss.com/docs/columns>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "columns")]
pub enum Columns {
    /// ```css
    /// {
    ///     columns: 1;
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     columns: 2;
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     columns: 3;
    /// }
    /// ```
    _3,
    /// ```css
    /// {
    ///     columns: 4;
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     columns: 5;
    /// }
    /// ```
    _5,
    /// ```css
    /// {
    ///     columns: 6;
    /// }
    /// ```
    _6,
    /// ```css
    /// {
    ///     columns: 7;
    /// }
    /// ```
    _7,
    /// ```css
    /// {
    ///     columns: 8;
    /// }
    /// ```
    _8,
    /// ```css
    /// {
    ///     columns: 9;
    /// }
    /// ```
    _9,
    /// ```css
    /// {
    ///     columns: 10;
    /// }
    /// ```
    _10,
    /// ```css
    /// {
    ///     columns: 11;
    /// }
    /// ```
    _11,
    /// ```css
    /// {
    ///     columns: 12;
    /// }
    /// ```
    _12,
    /// ```css
    /// {
    ///     columns: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     columns: 16rem; /* 256px */
    /// }
    /// ```
    _3xs,
    /// ```css
    /// {
    ///     columns: 18rem; /* 288px */
    /// }
    /// ```
    _2xs,
    /// ```css
    /// {
    ///     columns: 20rem; /* 320px */
    /// }
    /// ```
    Xs,
    /// ```css
    /// {
    ///     columns: 24rem; /* 384px */
    /// }
    /// ```
    Sm,
    /// ```css
    /// {
    ///     columns: 28rem; /* 448px */
    /// }
    /// ```
    Md,
    /// ```css
    /// {
    ///     columns: 32rem; /* 512px */
    /// }
    /// ```
    Lg,
    /// ```css
    /// {
    ///     columns: 36rem; /* 576px */
    /// }
    /// ```
    Xl,
    /// ```css
    /// {
    ///     columns: 42rem; /* 672px */
    /// }
    /// ```
    _2xl,
    /// ```css
    /// {
    ///     columns: 48rem; /* 768px */
    /// }
    /// ```
    _3xl,
    /// ```css
    /// {
    ///     columns: 56rem; /* 896px */
    /// }
    /// ```
    _4xl,
    /// ```css
    /// {
    ///     columns: 64rem; /* 1024px */
    /// }
    /// ```
    _5xl,
    /// ```css
    /// {
    ///     columns: 72rem; /* 1152px */
    /// }
    /// ```
    _6xl,
    /// ```css
    /// {
    ///     columns: 80rem; /* 1280px */
    /// }
    /// ```
    _7xl,
}

/// Utilities for controlling how a column or page should break after an element.
/// 
/// <https://tailwindcss.com/docs/break-after>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "break-after")]
pub enum BreakAfter {
    /// ```css
    /// {
    ///     break-after: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     break-after: avoid;
    /// }
    /// ```
    Avoid,
    /// ```css
    /// {
    ///     break-after: all;
    /// }
    /// ```
    All,
    /// ```css
    /// {
    ///     break-after: avoid-page;
    /// }
    /// ```
    AvoidPage,
    /// ```css
    /// {
    ///     break-after: page;
    /// }
    /// ```
    Page,
    /// ```css
    /// {
    ///     break-after: left;
    /// }
    /// ```
    Left,
    /// ```css
    /// {
    ///     break-after: right;
    /// }
    /// ```
    Right,
    /// ```css
    /// {
    ///     break-after: column;
    /// }
    /// ```
    Column,
}

/// Utilities for controlling how a column or page should break before an element.
/// 
/// <https://tailwindcss.com/docs/break-before>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "break-before")]
pub enum BreakBefore {
    /// ```css
    /// {
    ///     break-before: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     break-before: avoid;
    /// }
    /// ```
    Avoid,
    /// ```css
    /// {
    ///     break-before: all;
    /// }
    /// ```
    All,
    /// ```css
    /// {
    ///     break-before: avoid-page;
    /// }
    /// ```
    AvoidPage,
    /// ```css
    /// {
    ///     break-before: page;
    /// }
    /// ```
    Page,
    /// ```css
    /// {
    ///     break-before: left;
    /// }
    /// ```
    Left,
    /// ```css
    /// {
    ///     break-before: right;
    /// }
    /// ```
    Right,
    /// ```css
    /// {
    ///     break-before: column;
    /// }
    /// ```
    Column,
}

/// Utilities for controlling how a column or page should break within an element.
/// 
/// <https://tailwindcss.com/docs/break-inside>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "break-inside")]
pub enum BreakInside {
    /// ```css
    /// {
    ///     break-inside: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     break-inside: avoid;
    /// }
    /// ```
    Avoid,
    /// ```css
    /// {
    ///     break-inside: avoid-page;
    /// }
    /// ```
    AvoidPage,
    /// ```css
    /// {
    ///     break-inside: avoid-column;
    /// }
    /// ```
    AvoidColumn,
}

/// Utilities for controlling how element fragments should be rendered across multiple lines, columns, or pages.
/// 
/// <https://tailwindcss.com/docs/box-decoration-break>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "box-decoration")]
pub enum BoxDecorationBreak {
    /// ```css
    /// {
    ///     box-decoration-break: clone;
    /// }
    /// ```
    Clone,
    /// ```css
    /// {
    ///     box-decoration-break: slice;
    /// }
    /// ```
    Slice,
}

/// Utilities for controlling how the browser should calculate an element's total size.
/// 
/// <https://tailwindcss.com/docs/box-sizing>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "box")]
pub enum BoxSizing {
    /// ```css
    /// {
    ///     box-sizing: border-box;
    /// }
    /// ```
    Border,
    /// ```css
    /// {
    ///     box-sizing: content-box;
    /// }
    /// ```
    Content,
}

/// Utilities for controlling the display box type of an element.
/// 
/// <https://tailwindcss.com/docs/display>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum Display {
    /// ```css
    /// {
    ///     display: block;
    /// }
    /// ```
    Block,
    /// ```css
    /// {
    ///     display: inline-block;
    /// }
    /// ```
    InlineBlock,
    /// ```css
    /// {
    ///     display: inline;
    /// }
    /// ```
    Inline,
    /// ```css
    /// {
    ///     display: flex;
    /// }
    /// ```
    Flex,
    /// ```css
    /// {
    ///     display: inline-flex;
    /// }
    /// ```
    InlineFlex,
    /// ```css
    /// {
    ///     display: table;
    /// }
    /// ```
    Table,
    /// ```css
    /// {
    ///     display: inline-table;
    /// }
    /// ```
    InlineTable,
    /// ```css
    /// {
    ///     display: table-caption;
    /// }
    /// ```
    TableCaption,
    /// ```css
    /// {
    ///     display: table-cell;
    /// }
    /// ```
    TableCell,
    /// ```css
    /// {
    ///     display: table-column;
    /// }
    /// ```
    TableColumn,
    /// ```css
    /// {
    ///     display: table-column-group;
    /// }
    /// ```
    TableColumnGroup,
    /// ```css
    /// {
    ///     display: table-footer-group;
    /// }
    /// ```
    TableFooterGroup,
    /// ```css
    /// {
    ///     display: table-header-group;
    /// }
    /// ```
    TableHeaderGroup,
    /// ```css
    /// {
    ///     display: table-row-group;
    /// }
    /// ```
    TableRowGroup,
    /// ```css
    /// {
    ///     display: table-row;
    /// }
    /// ```
    TableRow,
    /// ```css
    /// {
    ///     display: flow-root;
    /// }
    /// ```
    FlowRoot,
    /// ```css
    /// {
    ///     display: grid;
    /// }
    /// ```
    Grid,
    /// ```css
    /// {
    ///     display: inline-grid;
    /// }
    /// ```
    InlineGrid,
    /// ```css
    /// {
    ///     display: contents;
    /// }
    /// ```
    Contents,
    /// ```css
    /// {
    ///     display: list-item;
    /// }
    /// ```
    ListItem,
    /// ```css
    /// {
    ///     display: none;
    /// }
    /// ```
    Hidden,
}

/// Utilities for controlling the wrapping of content around an element.
/// 
/// <https://tailwindcss.com/docs/float>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "float")]
pub enum Floats {
    /// ```css
    /// {
    ///     float: inline-start;
    /// }
    /// ```
    Start,
    /// ```css
    /// {
    ///     float: inline-end;
    /// }
    /// ```
    End,
    /// ```css
    /// {
    ///     float: right;
    /// }
    /// ```
    Right,
    /// ```css
    /// {
    ///     float: left;
    /// }
    /// ```
    Left,
    /// ```css
    /// {
    ///     float: none;
    /// }
    /// ```
    None,
}

/// Utilities for controlling the wrapping of content around an element.
/// 
/// <https://tailwindcss.com/docs/clear>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "clear")]
pub enum Clear {
    /// ```css
    /// {
    ///     clear: inline-start;
    /// }
    /// ```
    Start,
    /// ```css
    /// {
    ///     clear: inline-end;
    /// }
    /// ```
    End,
    /// ```css
    /// {
    ///     clear: left;
    /// }
    /// ```
    Left,
    /// ```css
    /// {
    ///     clear: right;
    /// }
    /// ```
    Right,
    /// ```css
    /// {
    ///     clear: both;
    /// }
    /// ```
    Both,
    /// ```css
    /// {
    ///     clear: none;
    /// }
    /// ```
    None,
}

/// Utilities for controlling whether an element should explicitly create a new stacking context.
/// 
/// <https://tailwindcss.com/docs/isolation>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum Isolation {
    /// ```css
    /// {
    ///     isolation: isolate;
    /// }
    /// ```
    Isolate,
    /// ```css
    /// {
    ///     isolation: auto;
    /// }
    /// ```
    IsolationAuto,
}

/// Utilities for controlling how a replaced element's content should be resized.
/// 
/// <https://tailwindcss.com/docs/object-fit>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "object")]
pub enum ObjectFit {
    /// ```css
    /// {
    ///     object-fit: contain;
    /// }
    /// ```
    Contain,
    /// ```css
    /// {
    ///     object-fit: cover;
    /// }
    /// ```
    Cover,
    /// ```css
    /// {
    ///     object-fit: fill;
    /// }
    /// ```
    Fill,
    /// ```css
    /// {
    ///     object-fit: none;
    /// }
    /// ```
    None,
    /// ```css
    /// {
    ///     object-fit: scale-down;
    /// }
    /// ```
    ScaleDown,
}

/// Utilities for controlling how a replaced element's content should be positioned within its container.
/// 
/// <https://tailwindcss.com/docs/object-position>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "object")]
pub enum ObjectPosition {
    /// ```css
    /// {
    ///     object-position: bottom;
    /// }
    /// ```
    Bottom,
    /// ```css
    /// {
    ///     object-position: center;
    /// }
    /// ```
    Center,
    /// ```css
    /// {
    ///     object-position: left;
    /// }
    /// ```
    Left,
    /// ```css
    /// {
    ///     object-position: left bottom;
    /// }
    /// ```
    LeftBottom,
    /// ```css
    /// {
    ///     object-position: left top;
    /// }
    /// ```
    LeftTop,
    /// ```css
    /// {
    ///     object-position: right;
    /// }
    /// ```
    Right,
    /// ```css
    /// {
    ///     object-position: right bottom;
    /// }
    /// ```
    RightBottom,
    /// ```css
    /// {
    ///     object-position: right top;
    /// }
    /// ```
    RightTop,
    /// ```css
    /// {
    ///     object-position: top;
    /// }
    /// ```
    Top,
}

/// Utilities for controlling how an element handles content that is too large for the container.
/// 
/// <https://tailwindcss.com/docs/overflow>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "overflow")]
pub enum Overflow {
    /// ```css
    /// {
    ///     overflow: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     overflow: hidden;
    /// }
    /// ```
    Hidden,
    /// ```css
    /// {
    ///     overflow: clip;
    /// }
    /// ```
    Clip,
    /// ```css
    /// {
    ///     overflow: visible;
    /// }
    /// ```
    Visible,
    /// ```css
    /// {
    ///     overflow: scroll;
    /// }
    /// ```
    Scroll,
    /// ```css
    /// {
    ///     overflow-x: auto;
    /// }
    /// ```
    XAuto,
    /// ```css
    /// {
    ///     overflow-y: auto;
    /// }
    /// ```
    YAuto,
    /// ```css
    /// {
    ///     overflow-x: hidden;
    /// }
    /// ```
    XHidden,
    /// ```css
    /// {
    ///     overflow-y: hidden;
    /// }
    /// ```
    YHidden,
    /// ```css
    /// {
    ///     overflow-x: clip;
    /// }
    /// ```
    XClip,
    /// ```css
    /// {
    ///     overflow-y: clip;
    /// }
    /// ```
    YClip,
    /// ```css
    /// {
    ///     overflow-x: visible;
    /// }
    /// ```
    XVisible,
    /// ```css
    /// {
    ///     overflow-y: visible;
    /// }
    /// ```
    YVisible,
    /// ```css
    /// {
    ///     overflow-x: scroll;
    /// }
    /// ```
    XScroll,
    /// ```css
    /// {
    ///     overflow-y: scroll;
    /// }
    /// ```
    YScroll,
}

/// Utilities for controlling how the browser behaves when reaching the boundary of a scrolling area.
/// 
/// <https://tailwindcss.com/docs/overscroll-behavior>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "overscroll")]
pub enum OverscrollBehavior {
    /// ```css
    /// {
    ///     overscroll-behavior: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     overscroll-behavior: contain;
    /// }
    /// ```
    Contain,
    /// ```css
    /// {
    ///     overscroll-behavior: none;
    /// }
    /// ```
    None,
    /// ```css
    /// {
    ///     overscroll-behavior-y: auto;
    /// }
    /// ```
    YAuto,
    /// ```css
    /// {
    ///     overscroll-behavior-y: contain;
    /// }
    /// ```
    YContain,
    /// ```css
    /// {
    ///     overscroll-behavior-y: none;
    /// }
    /// ```
    YNone,
    /// ```css
    /// {
    ///     overscroll-behavior-x: auto;
    /// }
    /// ```
    XAuto,
    /// ```css
    /// {
    ///     overscroll-behavior-x: contain;
    /// }
    /// ```
    XContain,
    /// ```css
    /// {
    ///     overscroll-behavior-x: none;
    /// }
    /// ```
    XNone,
}

/// Utilities for controlling how an element is positioned in the DOM.
/// 
/// <https://tailwindcss.com/docs/position>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum Position {
    /// ```css
    /// {
    ///     position: static;
    /// }
    /// ```
    Static,
    /// ```css
    /// {
    ///     position: fixed;
    /// }
    /// ```
    Fixed,
    /// ```css
    /// {
    ///     position: absolute;
    /// }
    /// ```
    Absolute,
    /// ```css
    /// {
    ///     position: relative;
    /// }
    /// ```
    Relative,
    /// ```css
    /// {
    ///     position: sticky;
    /// }
    /// ```
    Sticky,
}

/// Utilities for controlling the placement of positioned elements.
/// 
/// <https://tailwindcss.com/docs/top-right-bottom-left>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(
    replace(from = "_", to = "."), 
    replace(from = "div", to = "/")
)]
pub enum TopRightBottomLeft {
    /// ```css
    /// {
    ///     inset: 0px;
    /// }
    /// ```
    Inset0,
    /// ```css
    /// {
    ///     left: 0px;
    ///     right: 0px;
    /// }
    /// ```
    InsetX0,
    /// ```css
    /// {
    ///     top: 0px;
    ///     bottom: 0px;
    /// }
    /// ```
    InsetY0,
    /// ```css
    /// {
    ///     inset-inline-start: 0px;
    /// }
    /// ```
    Start0,
    /// ```css
    /// {
    ///     inset-inline-end: 0px;
    /// }
    /// ```
    End0,
    /// ```css
    /// {
    ///     top: 0px;
    /// }
    /// ```
    Top0,
    /// ```css
    /// {
    ///     right: 0px;
    /// }
    /// ```
    Right0,
    /// ```css
    /// {
    ///     bottom: 0px;
    /// }
    /// ```
    Bottom0,
    /// ```css
    /// {
    ///     left: 0px;
    /// }
    /// ```
    Left0,
    /// ```css
    /// {
    ///     inset: 1px;
    /// }
    /// ```
    InsetPx,
    /// ```css
    /// {
    ///     left: 1px;
    ///     right: 1px;
    /// }
    /// ```
    InsetXPx,
    /// ```css
    /// {
    ///     top: 1px;
    ///     bottom: 1px;
    /// }
    /// ```
    InsetYPx,
    /// ```css
    /// {
    ///     inset-inline-start: 1px;
    /// }
    /// ```
    StartPx,
    /// ```css
    /// {
    ///     inset-inline-end: 1px;
    /// }
    /// ```
    EndPx,
    /// ```css
    /// {
    ///     top: 1px;
    /// }
    /// ```
    TopPx,
    /// ```css
    /// {
    ///     right: 1px;
    /// }
    /// ```
    RightPx,
    /// ```css
    /// {
    ///     bottom: 1px;
    /// }
    /// ```
    BottomPx,
    /// ```css
    /// {
    ///     left: 1px;
    /// }
    /// ```
    LeftPx,
    /// ```css
    /// {
    ///     inset: 0.125rem; /* 2px */
    /// }
    /// ```
    Inset0_5,
    /// ```css
    /// {
    ///     left: 0.125rem; /* 2px */
    ///     right: 0.125rem; /* 2px */
    /// }
    /// ```
    InsetX0_5,
    /// ```css
    /// {
    ///     top: 0.125rem; /* 2px */
    ///     bottom: 0.125rem; /* 2px */
    /// }
    /// ```
    InsetY0_5,
    /// ```css
    /// {
    ///     inset-inline-start: 0.125rem; /* 2px */
    /// }
    /// ```
    Start0_5,
    /// ```css
    /// {
    ///     inset-inline-end: 0.125rem; /* 2px */
    /// }
    /// ```
    End0_5,
    /// ```css
    /// {
    ///     top: 0.125rem; /* 2px */
    /// }
    /// ```
    Top0_5,
    /// ```css
    /// {
    ///     right: 0.125rem; /* 2px */
    /// }
    /// ```
    Right0_5,
    /// ```css
    /// {
    ///     bottom: 0.125rem; /* 2px */
    /// }
    /// ```
    Bottom0_5,
    /// ```css
    /// {
    ///     left: 0.125rem; /* 2px */
    /// }
    /// ```
    Left0_5,
    /// ```css
    /// {
    ///     inset: 0.25rem; /* 4px */
    /// }
    /// ```
    Inset1,
    /// ```css
    /// {
    ///     left: 0.25rem; /* 4px */
    ///     right: 0.25rem; /* 4px */
    /// }
    /// ```
    InsetX1,
    /// ```css
    /// {
    ///     top: 0.25rem; /* 4px */
    ///     bottom: 0.25rem; /* 4px */
    /// }
    /// ```
    InsetY1,
    /// ```css
    /// {
    ///     inset-inline-start: 0.25rem; /* 4px */
    /// }
    /// ```
    Start1,
    /// ```css
    /// {
    ///     inset-inline-end: 0.25rem; /* 4px */
    /// }
    /// ```
    End1,
    /// ```css
    /// {
    ///     top: 0.25rem; /* 4px */
    /// }
    /// ```
    Top1,
    /// ```css
    /// {
    ///     right: 0.25rem; /* 4px */
    /// }
    /// ```
    Right1,
    /// ```css
    /// {
    ///     bottom: 0.25rem; /* 4px */
    /// }
    /// ```
    Bottom1,
    /// ```css
    /// {
    ///     left: 0.25rem; /* 4px */
    /// }
    /// ```
    Left1,
    /// ```css
    /// {
    ///     inset: 0.375rem; /* 6px */
    /// }
    /// ```
    Inset1_5,
    /// ```css
    /// {
    ///     left: 0.375rem; /* 6px */
    ///     right: 0.375rem; /* 6px */
    /// }
    /// ```
    InsetX1_5,
    /// ```css
    /// {
    ///     top: 0.375rem; /* 6px */
    ///     bottom: 0.375rem; /* 6px */
    /// }
    /// ```
    InsetY1_5,
    /// ```css
    /// {
    ///     inset-inline-start: 0.375rem; /* 6px */
    /// }
    /// ```
    Start1_5,
    /// ```css
    /// {
    ///     inset-inline-end: 0.375rem; /* 6px */
    /// }
    /// ```
    End1_5,
    /// ```css
    /// {
    ///     top: 0.375rem; /* 6px */
    /// }
    /// ```
    Top1_5,
    /// ```css
    /// {
    ///     right: 0.375rem; /* 6px */
    /// }
    /// ```
    Right1_5,
    /// ```css
    /// {
    ///     bottom: 0.375rem; /* 6px */
    /// }
    /// ```
    Bottom1_5,
    /// ```css
    /// {
    ///     left: 0.375rem; /* 6px */
    /// }
    /// ```
    Left1_5,
    /// ```css
    /// {
    ///     inset: 0.5rem; /* 8px */
    /// }
    /// ```
    Inset2,
    /// ```css
    /// {
    ///     left: 0.5rem; /* 8px */
    ///     right: 0.5rem; /* 8px */
    /// }
    /// ```
    InsetX2,
    /// ```css
    /// {
    ///     top: 0.5rem; /* 8px */
    ///     bottom: 0.5rem; /* 8px */
    /// }
    /// ```
    InsetY2,
    /// ```css
    /// {
    ///     inset-inline-start: 0.5rem; /* 8px */
    /// }
    /// ```
    Start2,
    /// ```css
    /// {
    ///     inset-inline-end: 0.5rem; /* 8px */
    /// }
    /// ```
    End2,
    /// ```css
    /// {
    ///     top: 0.5rem; /* 8px */
    /// }
    /// ```
    Top2,
    /// ```css
    /// {
    ///     right: 0.5rem; /* 8px */
    /// }
    /// ```
    Right2,
    /// ```css
    /// {
    ///     bottom: 0.5rem; /* 8px */
    /// }
    /// ```
    Bottom2,
    /// ```css
    /// {
    ///     left: 0.5rem; /* 8px */
    /// }
    /// ```
    Left2,
    /// ```css
    /// {
    ///     inset: 0.625rem; /* 10px */
    /// }
    /// ```
    Inset2_5,
    /// ```css
    /// {
    ///     left: 0.625rem; /* 10px */
    ///     right: 0.625rem; /* 10px */
    /// }
    /// ```
    InsetX2_5,
    /// ```css
    /// {
    ///     top: 0.625rem; /* 10px */
    ///     bottom: 0.625rem; /* 10px */
    /// }
    /// ```
    InsetY2_5,
    /// ```css
    /// {
    ///     inset-inline-start: 0.625rem; /* 10px */
    /// }
    /// ```
    Start2_5,
    /// ```css
    /// {
    ///     inset-inline-end: 0.625rem; /* 10px */
    /// }
    /// ```
    End2_5,
    /// ```css
    /// {
    ///     top: 0.625rem; /* 10px */
    /// }
    /// ```
    Top2_5,
    /// ```css
    /// {
    ///     right: 0.625rem; /* 10px */
    /// }
    /// ```
    Right2_5,
    /// ```css
    /// {
    ///     bottom: 0.625rem; /* 10px */
    /// }
    /// ```
    Bottom2_5,
    /// ```css
    /// {
    ///     left: 0.625rem; /* 10px */
    /// }
    /// ```
    Left2_5,
    /// ```css
    /// {
    ///     inset: 0.75rem; /* 12px */
    /// }
    /// ```
    Inset3,
    /// ```css
    /// {
    ///     left: 0.75rem; /* 12px */
    ///     right: 0.75rem; /* 12px */
    /// }
    /// ```
    InsetX3,
    /// ```css
    /// {
    ///     top: 0.75rem; /* 12px */
    ///     bottom: 0.75rem; /* 12px */
    /// }
    /// ```
    InsetY3,
    /// ```css
    /// {
    ///     inset-inline-start: 0.75rem; /* 12px */
    /// }
    /// ```
    Start3,
    /// ```css
    /// {
    ///     inset-inline-end: 0.75rem; /* 12px */
    /// }
    /// ```
    End3,
    /// ```css
    /// {
    ///     top: 0.75rem; /* 12px */
    /// }
    /// ```
    Top3,
    /// ```css
    /// {
    ///     right: 0.75rem; /* 12px */
    /// }
    /// ```
    Right3,
    /// ```css
    /// {
    ///     bottom: 0.75rem; /* 12px */
    /// }
    /// ```
    Bottom3,
    /// ```css
    /// {
    ///     left: 0.75rem; /* 12px */
    /// }
    /// ```
    Left3,
    /// ```css
    /// {
    ///     inset: 0.875rem; /* 14px */
    /// }
    /// ```
    Inset3_5,
    /// ```css
    /// {
    ///     left: 0.875rem; /* 14px */
    ///     right: 0.875rem; /* 14px */
    /// }
    /// ```
    InsetX3_5,
    /// ```css
    /// {
    ///     top: 0.875rem; /* 14px */
    ///     bottom: 0.875rem; /* 14px */
    /// }
    /// ```
    InsetY3_5,
    /// ```css
    /// {
    ///     inset-inline-start: 0.875rem; /* 14px */
    /// }
    /// ```
    Start3_5,
    /// ```css
    /// {
    ///     inset-inline-end: 0.875rem; /* 14px */
    /// }
    /// ```
    End3_5,
    /// ```css
    /// {
    ///     top: 0.875rem; /* 14px */
    /// }
    /// ```
    Top3_5,
    /// ```css
    /// {
    ///     right: 0.875rem; /* 14px */
    /// }
    /// ```
    Right3_5,
    /// ```css
    /// {
    ///     bottom: 0.875rem; /* 14px */
    /// }
    /// ```
    Bottom3_5,
    /// ```css
    /// {
    ///     left: 0.875rem; /* 14px */
    /// }
    /// ```
    Left3_5,
    /// ```css
    /// {
    ///     inset: 1rem; /* 16px */
    /// }
    /// ```
    Inset4,
    /// ```css
    /// {
    ///     left: 1rem; /* 16px */
    ///     right: 1rem; /* 16px */
    /// }
    /// ```
    InsetX4,
    /// ```css
    /// {
    ///     top: 1rem; /* 16px */
    ///     bottom: 1rem; /* 16px */
    /// }
    /// ```
    InsetY4,
    /// ```css
    /// {
    ///     inset-inline-start: 1rem; /* 16px */
    /// }
    /// ```
    Start4,
    /// ```css
    /// {
    ///     inset-inline-end: 1rem; /* 16px */
    /// }
    /// ```
    End4,
    /// ```css
    /// {
    ///     top: 1rem; /* 16px */
    /// }
    /// ```
    Top4,
    /// ```css
    /// {
    ///     right: 1rem; /* 16px */
    /// }
    /// ```
    Right4,
    /// ```css
    /// {
    ///     bottom: 1rem; /* 16px */
    /// }
    /// ```
    Bottom4,
    /// ```css
    /// {
    ///     left: 1rem; /* 16px */
    /// }
    /// ```
    Left4,
    /// ```css
    /// {
    ///     inset: 1.25rem; /* 20px */
    /// }
    /// ```
    Inset5,
    /// ```css
    /// {
    ///     left: 1.25rem; /* 20px */
    ///     right: 1.25rem; /* 20px */
    /// }
    /// ```
    InsetX5,
    /// ```css
    /// {
    ///     top: 1.25rem; /* 20px */
    ///     bottom: 1.25rem; /* 20px */
    /// }
    /// ```
    InsetY5,
    /// ```css
    /// {
    ///     inset-inline-start: 1.25rem; /* 20px */
    /// }
    /// ```
    Start5,
    /// ```css
    /// {
    ///     inset-inline-end: 1.25rem; /* 20px */
    /// }
    /// ```
    End5,
    /// ```css
    /// {
    ///     top: 1.25rem; /* 20px */
    /// }
    /// ```
    Top5,
    /// ```css
    /// {
    ///     right: 1.25rem; /* 20px */
    /// }
    /// ```
    Right5,
    /// ```css
    /// {
    ///     bottom: 1.25rem; /* 20px */
    /// }
    /// ```
    Bottom5,
    /// ```css
    /// {
    ///     left: 1.25rem; /* 20px */
    /// }
    /// ```
    Left5,
    /// ```css
    /// {
    ///     inset: 1.5rem; /* 24px */
    /// }
    /// ```
    Inset6,
    /// ```css
    /// {
    ///     left: 1.5rem; /* 24px */
    ///     right: 1.5rem; /* 24px */
    /// }
    /// ```
    InsetX6,
    /// ```css
    /// {
    ///     top: 1.5rem; /* 24px */
    ///     bottom: 1.5rem; /* 24px */
    /// }
    /// ```
    InsetY6,
    /// ```css
    /// {
    ///     inset-inline-start: 1.5rem; /* 24px */
    /// }
    /// ```
    Start6,
    /// ```css
    /// {
    ///     inset-inline-end: 1.5rem; /* 24px */
    /// }
    /// ```
    End6,
    /// ```css
    /// {
    ///     top: 1.5rem; /* 24px */
    /// }
    /// ```
    Top6,
    /// ```css
    /// {
    ///     right: 1.5rem; /* 24px */
    /// }
    /// ```
    Right6,
    /// ```css
    /// {
    ///     bottom: 1.5rem; /* 24px */
    /// }
    /// ```
    Bottom6,
    /// ```css
    /// {
    ///     left: 1.5rem; /* 24px */
    /// }
    /// ```
    Left6,
    /// ```css
    /// {
    ///     inset: 1.75rem; /* 28px */
    /// }
    /// ```
    Inset7,
    /// ```css
    /// {
    ///     left: 1.75rem; /* 28px */
    ///     right: 1.75rem; /* 28px */
    /// }
    /// ```
    InsetX7,
    /// ```css
    /// {
    ///     top: 1.75rem; /* 28px */
    ///     bottom: 1.75rem; /* 28px */
    /// }
    /// ```
    InsetY7,
    /// ```css
    /// {
    ///     inset-inline-start: 1.75rem; /* 28px */
    /// }
    /// ```
    Start7,
    /// ```css
    /// {
    ///     inset-inline-end: 1.75rem; /* 28px */
    /// }
    /// ```
    End7,
    /// ```css
    /// {
    ///     top: 1.75rem; /* 28px */
    /// }
    /// ```
    Top7,
    /// ```css
    /// {
    ///     right: 1.75rem; /* 28px */
    /// }
    /// ```
    Right7,
    /// ```css
    /// {
    ///     bottom: 1.75rem; /* 28px */
    /// }
    /// ```
    Bottom7,
    /// ```css
    /// {
    ///     left: 1.75rem; /* 28px */
    /// }
    /// ```
    Left7,
    /// ```css
    /// {
    ///     inset: 2rem; /* 32px */
    /// }
    /// ```
    Inset8,
    /// ```css
    /// {
    ///     left: 2rem; /* 32px */
    ///     right: 2rem; /* 32px */
    /// }
    /// ```
    InsetX8,
    /// ```css
    /// {
    ///     top: 2rem; /* 32px */
    ///     bottom: 2rem; /* 32px */
    /// }
    /// ```
    InsetY8,
    /// ```css
    /// {
    ///     inset-inline-start: 2rem; /* 32px */
    /// }
    /// ```
    Start8,
    /// ```css
    /// {
    ///     inset-inline-end: 2rem; /* 32px */
    /// }
    /// ```
    End8,
    /// ```css
    /// {
    ///     top: 2rem; /* 32px */
    /// }
    /// ```
    Top8,
    /// ```css
    /// {
    ///     right: 2rem; /* 32px */
    /// }
    /// ```
    Right8,
    /// ```css
    /// {
    ///     bottom: 2rem; /* 32px */
    /// }
    /// ```
    Bottom8,
    /// ```css
    /// {
    ///     left: 2rem; /* 32px */
    /// }
    /// ```
    Left8,
    /// ```css
    /// {
    ///     inset: 2.25rem; /* 36px */
    /// }
    /// ```
    Inset9,
    /// ```css
    /// {
    ///     left: 2.25rem; /* 36px */
    ///     right: 2.25rem; /* 36px */
    /// }
    /// ```
    InsetX9,
    /// ```css
    /// {
    ///     top: 2.25rem; /* 36px */
    ///     bottom: 2.25rem; /* 36px */
    /// }
    /// ```
    InsetY9,
    /// ```css
    /// {
    ///     inset-inline-start: 2.25rem; /* 36px */
    /// }
    /// ```
    Start9,
    /// ```css
    /// {
    ///     inset-inline-end: 2.25rem; /* 36px */
    /// }
    /// ```
    End9,
    /// ```css
    /// {
    ///     top: 2.25rem; /* 36px */
    /// }
    /// ```
    Top9,
    /// ```css
    /// {
    ///     right: 2.25rem; /* 36px */
    /// }
    /// ```
    Right9,
    /// ```css
    /// {
    ///     bottom: 2.25rem; /* 36px */
    /// }
    /// ```
    Bottom9,
    /// ```css
    /// {
    ///     left: 2.25rem; /* 36px */
    /// }
    /// ```
    Left9,
    /// ```css
    /// {
    ///     inset: 2.5rem; /* 40px */
    /// }
    /// ```
    Inset10,
    /// ```css
    /// {
    ///     left: 2.5rem; /* 40px */
    ///     right: 2.5rem; /* 40px */
    /// }
    /// ```
    InsetX10,
    /// ```css
    /// {
    ///     top: 2.5rem; /* 40px */
    ///     bottom: 2.5rem; /* 40px */
    /// }
    /// ```
    InsetY10,
    /// ```css
    /// {
    ///     inset-inline-start: 2.5rem; /* 40px */
    /// }
    /// ```
    Start10,
    /// ```css
    /// {
    ///     inset-inline-end: 2.5rem; /* 40px */
    /// }
    /// ```
    End10,
    /// ```css
    /// {
    ///     top: 2.5rem; /* 40px */
    /// }
    /// ```
    Top10,
    /// ```css
    /// {
    ///     right: 2.5rem; /* 40px */
    /// }
    /// ```
    Right10,
    /// ```css
    /// {
    ///     bottom: 2.5rem; /* 40px */
    /// }
    /// ```
    Bottom10,
    /// ```css
    /// {
    ///     left: 2.5rem; /* 40px */
    /// }
    /// ```
    Left10,
    /// ```css
    /// {
    ///     inset: 2.75rem; /* 44px */
    /// }
    /// ```
    Inset11,
    /// ```css
    /// {
    ///     left: 2.75rem; /* 44px */
    ///     right: 2.75rem; /* 44px */
    /// }
    /// ```
    InsetX11,
    /// ```css
    /// {
    ///     top: 2.75rem; /* 44px */
    ///     bottom: 2.75rem; /* 44px */
    /// }
    /// ```
    InsetY11,
    /// ```css
    /// {
    ///     inset-inline-start: 2.75rem; /* 44px */
    /// }
    /// ```
    Start11,
    /// ```css
    /// {
    ///     inset-inline-end: 2.75rem; /* 44px */
    /// }
    /// ```
    End11,
    /// ```css
    /// {
    ///     top: 2.75rem; /* 44px */
    /// }
    /// ```
    Top11,
    /// ```css
    /// {
    ///     right: 2.75rem; /* 44px */
    /// }
    /// ```
    Right11,
    /// ```css
    /// {
    ///     bottom: 2.75rem; /* 44px */
    /// }
    /// ```
    Bottom11,
    /// ```css
    /// {
    ///     left: 2.75rem; /* 44px */
    /// }
    /// ```
    Left11,
    /// ```css
    /// {
    ///     inset: 3rem; /* 48px */
    /// }
    /// ```
    Inset12,
    /// ```css
    /// {
    ///     left: 3rem; /* 48px */
    ///     right: 3rem; /* 48px */
    /// }
    /// ```
    InsetX12,
    /// ```css
    /// {
    ///     top: 3rem; /* 48px */
    ///     bottom: 3rem; /* 48px */
    /// }
    /// ```
    InsetY12,
    /// ```css
    /// {
    ///     inset-inline-start: 3rem; /* 48px */
    /// }
    /// ```
    Start12,
    /// ```css
    /// {
    ///     inset-inline-end: 3rem; /* 48px */
    /// }
    /// ```
    End12,
    /// ```css
    /// {
    ///     top: 3rem; /* 48px */
    /// }
    /// ```
    Top12,
    /// ```css
    /// {
    ///     right: 3rem; /* 48px */
    /// }
    /// ```
    Right12,
    /// ```css
    /// {
    ///     bottom: 3rem; /* 48px */
    /// }
    /// ```
    Bottom12,
    /// ```css
    /// {
    ///     left: 3rem; /* 48px */
    /// }
    /// ```
    Left12,
    /// ```css
    /// {
    ///     inset: 3.5rem; /* 56px */
    /// }
    /// ```
    Inset14,
    /// ```css
    /// {
    ///     left: 3.5rem; /* 56px */
    ///     right: 3.5rem; /* 56px */
    /// }
    /// ```
    InsetX14,
    /// ```css
    /// {
    ///     top: 3.5rem; /* 56px */
    ///     bottom: 3.5rem; /* 56px */
    /// }
    /// ```
    InsetY14,
    /// ```css
    /// {
    ///     inset-inline-start: 3.5rem; /* 56px */
    /// }
    /// ```
    Start14,
    /// ```css
    /// {
    ///     inset-inline-end: 3.5rem; /* 56px */
    /// }
    /// ```
    End14,
    /// ```css
    /// {
    ///     top: 3.5rem; /* 56px */
    /// }
    /// ```
    Top14,
    /// ```css
    /// {
    ///     right: 3.5rem; /* 56px */
    /// }
    /// ```
    Right14,
    /// ```css
    /// {
    ///     bottom: 3.5rem; /* 56px */
    /// }
    /// ```
    Bottom14,
    /// ```css
    /// {
    ///     left: 3.5rem; /* 56px */
    /// }
    /// ```
    Left14,
    /// ```css
    /// {
    ///     inset: 4rem; /* 64px */
    /// }
    /// ```
    Inset16,
    /// ```css
    /// {
    ///     left: 4rem; /* 64px */
    ///     right: 4rem; /* 64px */
    /// }
    /// ```
    InsetX16,
    /// ```css
    /// {
    ///     top: 4rem; /* 64px */
    ///     bottom: 4rem; /* 64px */
    /// }
    /// ```
    InsetY16,
    /// ```css
    /// {
    ///     inset-inline-start: 4rem; /* 64px */
    /// }
    /// ```
    Start16,
    /// ```css
    /// {
    ///     inset-inline-end: 4rem; /* 64px */
    /// }
    /// ```
    End16,
    /// ```css
    /// {
    ///     top: 4rem; /* 64px */
    /// }
    /// ```
    Top16,
    /// ```css
    /// {
    ///     right: 4rem; /* 64px */
    /// }
    /// ```
    Right16,
    /// ```css
    /// {
    ///     bottom: 4rem; /* 64px */
    /// }
    /// ```
    Bottom16,
    /// ```css
    /// {
    ///     left: 4rem; /* 64px */
    /// }
    /// ```
    Left16,
    /// ```css
    /// {
    ///     inset: 5rem; /* 80px */
    /// }
    /// ```
    Inset20,
    /// ```css
    /// {
    ///     left: 5rem; /* 80px */
    ///     right: 5rem; /* 80px */
    /// }
    /// ```
    InsetX20,
    /// ```css
    /// {
    ///     top: 5rem; /* 80px */
    ///     bottom: 5rem; /* 80px */
    /// }
    /// ```
    InsetY20,
    /// ```css
    /// {
    ///     inset-inline-start: 5rem; /* 80px */
    /// }
    /// ```
    Start20,
    /// ```css
    /// {
    ///     inset-inline-end: 5rem; /* 80px */
    /// }
    /// ```
    End20,
    /// ```css
    /// {
    ///     top: 5rem; /* 80px */
    /// }
    /// ```
    Top20,
    /// ```css
    /// {
    ///     right: 5rem; /* 80px */
    /// }
    /// ```
    Right20,
    /// ```css
    /// {
    ///     bottom: 5rem; /* 80px */
    /// }
    /// ```
    Bottom20,
    /// ```css
    /// {
    ///     left: 5rem; /* 80px */
    /// }
    /// ```
    Left20,
    /// ```css
    /// {
    ///     inset: 6rem; /* 96px */
    /// }
    /// ```
    Inset24,
    /// ```css
    /// {
    ///     left: 6rem; /* 96px */
    ///     right: 6rem; /* 96px */
    /// }
    /// ```
    InsetX24,
    /// ```css
    /// {
    ///     top: 6rem; /* 96px */
    ///     bottom: 6rem; /* 96px */
    /// }
    /// ```
    InsetY24,
    /// ```css
    /// {
    ///     inset-inline-start: 6rem; /* 96px */
    /// }
    /// ```
    Start24,
    /// ```css
    /// {
    ///     inset-inline-end: 6rem; /* 96px */
    /// }
    /// ```
    End24,
    /// ```css
    /// {
    ///     top: 6rem; /* 96px */
    /// }
    /// ```
    Top24,
    /// ```css
    /// {
    ///     right: 6rem; /* 96px */
    /// }
    /// ```
    Right24,
    /// ```css
    /// {
    ///     bottom: 6rem; /* 96px */
    /// }
    /// ```
    Bottom24,
    /// ```css
    /// {
    ///     left: 6rem; /* 96px */
    /// }
    /// ```
    Left24,
    /// ```css
    /// {
    ///     inset: 7rem; /* 112px */
    /// }
    /// ```
    Inset28,
    /// ```css
    /// {
    ///     left: 7rem; /* 112px */
    ///     right: 7rem; /* 112px */
    /// }
    /// ```
    InsetX28,
    /// ```css
    /// {
    ///     top: 7rem; /* 112px */
    ///     bottom: 7rem; /* 112px */
    /// }
    /// ```
    InsetY28,
    /// ```css
    /// {
    ///     inset-inline-start: 7rem; /* 112px */
    /// }
    /// ```
    Start28,
    /// ```css
    /// {
    ///     inset-inline-end: 7rem; /* 112px */
    /// }
    /// ```
    End28,
    /// ```css
    /// {
    ///     top: 7rem; /* 112px */
    /// }
    /// ```
    Top28,
    /// ```css
    /// {
    ///     right: 7rem; /* 112px */
    /// }
    /// ```
    Right28,
    /// ```css
    /// {
    ///     bottom: 7rem; /* 112px */
    /// }
    /// ```
    Bottom28,
    /// ```css
    /// {
    ///     left: 7rem; /* 112px */
    /// }
    /// ```
    Left28,
    /// ```css
    /// {
    ///     inset: 8rem; /* 128px */
    /// }
    /// ```
    Inset32,
    /// ```css
    /// {
    ///     left: 8rem; /* 128px */
    ///     right: 8rem; /* 128px */
    /// }
    /// ```
    InsetX32,
    /// ```css
    /// {
    ///     top: 8rem; /* 128px */
    ///     bottom: 8rem; /* 128px */
    /// }
    /// ```
    InsetY32,
    /// ```css
    /// {
    ///     inset-inline-start: 8rem; /* 128px */
    /// }
    /// ```
    Start32,
    /// ```css
    /// {
    ///     inset-inline-end: 8rem; /* 128px */
    /// }
    /// ```
    End32,
    /// ```css
    /// {
    ///     top: 8rem; /* 128px */
    /// }
    /// ```
    Top32,
    /// ```css
    /// {
    ///     right: 8rem; /* 128px */
    /// }
    /// ```
    Right32,
    /// ```css
    /// {
    ///     bottom: 8rem; /* 128px */
    /// }
    /// ```
    Bottom32,
    /// ```css
    /// {
    ///     left: 8rem; /* 128px */
    /// }
    /// ```
    Left32,
    /// ```css
    /// {
    ///     inset: 9rem; /* 144px */
    /// }
    /// ```
    Inset36,
    /// ```css
    /// {
    ///     left: 9rem; /* 144px */
    ///     right: 9rem; /* 144px */
    /// }
    /// ```
    InsetX36,
    /// ```css
    /// {
    ///     top: 9rem; /* 144px */
    ///     bottom: 9rem; /* 144px */
    /// }
    /// ```
    InsetY36,
    /// ```css
    /// {
    ///     inset-inline-start: 9rem; /* 144px */
    /// }
    /// ```
    Start36,
    /// ```css
    /// {
    ///     inset-inline-end: 9rem; /* 144px */
    /// }
    /// ```
    End36,
    /// ```css
    /// {
    ///     top: 9rem; /* 144px */
    /// }
    /// ```
    Top36,
    /// ```css
    /// {
    ///     right: 9rem; /* 144px */
    /// }
    /// ```
    Right36,
    /// ```css
    /// {
    ///     bottom: 9rem; /* 144px */
    /// }
    /// ```
    Bottom36,
    /// ```css
    /// {
    ///     left: 9rem; /* 144px */
    /// }
    /// ```
    Left36,
    /// ```css
    /// {
    ///     inset: 10rem; /* 160px */
    /// }
    /// ```
    Inset40,
    /// ```css
    /// {
    ///     left: 10rem; /* 160px */
    ///     right: 10rem; /* 160px */
    /// }
    /// ```
    InsetX40,
    /// ```css
    /// {
    ///     top: 10rem; /* 160px */
    ///     bottom: 10rem; /* 160px */
    /// }
    /// ```
    InsetY40,
    /// ```css
    /// {
    ///     inset-inline-start: 10rem; /* 160px */
    /// }
    /// ```
    Start40,
    /// ```css
    /// {
    ///     inset-inline-end: 10rem; /* 160px */
    /// }
    /// ```
    End40,
    /// ```css
    /// {
    ///     top: 10rem; /* 160px */
    /// }
    /// ```
    Top40,
    /// ```css
    /// {
    ///     right: 10rem; /* 160px */
    /// }
    /// ```
    Right40,
    /// ```css
    /// {
    ///     bottom: 10rem; /* 160px */
    /// }
    /// ```
    Bottom40,
    /// ```css
    /// {
    ///     left: 10rem; /* 160px */
    /// }
    /// ```
    Left40,
    /// ```css
    /// {
    ///     inset: 11rem; /* 176px */
    /// }
    /// ```
    Inset44,
    /// ```css
    /// {
    ///     left: 11rem; /* 176px */
    ///     right: 11rem; /* 176px */
    /// }
    /// ```
    InsetX44,
    /// ```css
    /// {
    ///     top: 11rem; /* 176px */
    ///     bottom: 11rem; /* 176px */
    /// }
    /// ```
    InsetY44,
    /// ```css
    /// {
    ///     inset-inline-start: 11rem; /* 176px */
    /// }
    /// ```
    Start44,
    /// ```css
    /// {
    ///     inset-inline-end: 11rem; /* 176px */
    /// }
    /// ```
    End44,
    /// ```css
    /// {
    ///     top: 11rem; /* 176px */
    /// }
    /// ```
    Top44,
    /// ```css
    /// {
    ///     right: 11rem; /* 176px */
    /// }
    /// ```
    Right44,
    /// ```css
    /// {
    ///     bottom: 11rem; /* 176px */
    /// }
    /// ```
    Bottom44,
    /// ```css
    /// {
    ///     left: 11rem; /* 176px */
    /// }
    /// ```
    Left44,
    /// ```css
    /// {
    ///     inset: 12rem; /* 192px */
    /// }
    /// ```
    Inset48,
    /// ```css
    /// {
    ///     left: 12rem; /* 192px */
    ///     right: 12rem; /* 192px */
    /// }
    /// ```
    InsetX48,
    /// ```css
    /// {
    ///     top: 12rem; /* 192px */
    ///     bottom: 12rem; /* 192px */
    /// }
    /// ```
    InsetY48,
    /// ```css
    /// {
    ///     inset-inline-start: 12rem; /* 192px */
    /// }
    /// ```
    Start48,
    /// ```css
    /// {
    ///     inset-inline-end: 12rem; /* 192px */
    /// }
    /// ```
    End48,
    /// ```css
    /// {
    ///     top: 12rem; /* 192px */
    /// }
    /// ```
    Top48,
    /// ```css
    /// {
    ///     right: 12rem; /* 192px */
    /// }
    /// ```
    Right48,
    /// ```css
    /// {
    ///     bottom: 12rem; /* 192px */
    /// }
    /// ```
    Bottom48,
    /// ```css
    /// {
    ///     left: 12rem; /* 192px */
    /// }
    /// ```
    Left48,
    /// ```css
    /// {
    ///     inset: 13rem; /* 208px */
    /// }
    /// ```
    Inset52,
    /// ```css
    /// {
    ///     left: 13rem; /* 208px */
    ///     right: 13rem; /* 208px */
    /// }
    /// ```
    InsetX52,
    /// ```css
    /// {
    ///     top: 13rem; /* 208px */
    ///     bottom: 13rem; /* 208px */
    /// }
    /// ```
    InsetY52,
    /// ```css
    /// {
    ///     inset-inline-start: 13rem; /* 208px */
    /// }
    /// ```
    Start52,
    /// ```css
    /// {
    ///     inset-inline-end: 13rem; /* 208px */
    /// }
    /// ```
    End52,
    /// ```css
    /// {
    ///     top: 13rem; /* 208px */
    /// }
    /// ```
    Top52,
    /// ```css
    /// {
    ///     right: 13rem; /* 208px */
    /// }
    /// ```
    Right52,
    /// ```css
    /// {
    ///     bottom: 13rem; /* 208px */
    /// }
    /// ```
    Bottom52,
    /// ```css
    /// {
    ///     left: 13rem; /* 208px */
    /// }
    /// ```
    Left52,
    /// ```css
    /// {
    ///     inset: 14rem; /* 224px */
    /// }
    /// ```
    Inset56,
    /// ```css
    /// {
    ///     left: 14rem; /* 224px */
    ///     right: 14rem; /* 224px */
    /// }
    /// ```
    InsetX56,
    /// ```css
    /// {
    ///     top: 14rem; /* 224px */
    ///     bottom: 14rem; /* 224px */
    /// }
    /// ```
    InsetY56,
    /// ```css
    /// {
    ///     inset-inline-start: 14rem; /* 224px */
    /// }
    /// ```
    Start56,
    /// ```css
    /// {
    ///     inset-inline-end: 14rem; /* 224px */
    /// }
    /// ```
    End56,
    /// ```css
    /// {
    ///     top: 14rem; /* 224px */
    /// }
    /// ```
    Top56,
    /// ```css
    /// {
    ///     right: 14rem; /* 224px */
    /// }
    /// ```
    Right56,
    /// ```css
    /// {
    ///     bottom: 14rem; /* 224px */
    /// }
    /// ```
    Bottom56,
    /// ```css
    /// {
    ///     left: 14rem; /* 224px */
    /// }
    /// ```
    Left56,
    /// ```css
    /// {
    ///     inset: 15rem; /* 240px */
    /// }
    /// ```
    Inset60,
    /// ```css
    /// {
    ///     left: 15rem; /* 240px */
    ///     right: 15rem; /* 240px */
    /// }
    /// ```
    InsetX60,
    /// ```css
    /// {
    ///     top: 15rem; /* 240px */
    ///     bottom: 15rem; /* 240px */
    /// }
    /// ```
    InsetY60,
    /// ```css
    /// {
    ///     inset-inline-start: 15rem; /* 240px */
    /// }
    /// ```
    Start60,
    /// ```css
    /// {
    ///     inset-inline-end: 15rem; /* 240px */
    /// }
    /// ```
    End60,
    /// ```css
    /// {
    ///     top: 15rem; /* 240px */
    /// }
    /// ```
    Top60,
    /// ```css
    /// {
    ///     right: 15rem; /* 240px */
    /// }
    /// ```
    Right60,
    /// ```css
    /// {
    ///     bottom: 15rem; /* 240px */
    /// }
    /// ```
    Bottom60,
    /// ```css
    /// {
    ///     left: 15rem; /* 240px */
    /// }
    /// ```
    Left60,
    /// ```css
    /// {
    ///     inset: 16rem; /* 256px */
    /// }
    /// ```
    Inset64,
    /// ```css
    /// {
    ///     left: 16rem; /* 256px */
    ///     right: 16rem; /* 256px */
    /// }
    /// ```
    InsetX64,
    /// ```css
    /// {
    ///     top: 16rem; /* 256px */
    ///     bottom: 16rem; /* 256px */
    /// }
    /// ```
    InsetY64,
    /// ```css
    /// {
    ///     inset-inline-start: 16rem; /* 256px */
    /// }
    /// ```
    Start64,
    /// ```css
    /// {
    ///     inset-inline-end: 16rem; /* 256px */
    /// }
    /// ```
    End64,
    /// ```css
    /// {
    ///     top: 16rem; /* 256px */
    /// }
    /// ```
    Top64,
    /// ```css
    /// {
    ///     right: 16rem; /* 256px */
    /// }
    /// ```
    Right64,
    /// ```css
    /// {
    ///     bottom: 16rem; /* 256px */
    /// }
    /// ```
    Bottom64,
    /// ```css
    /// {
    ///     left: 16rem; /* 256px */
    /// }
    /// ```
    Left64,
    /// ```css
    /// {
    ///     inset: 18rem; /* 288px */
    /// }
    /// ```
    Inset72,
    /// ```css
    /// {
    ///     left: 18rem; /* 288px */
    ///     right: 18rem; /* 288px */
    /// }
    /// ```
    InsetX72,
    /// ```css
    /// {
    ///     top: 18rem; /* 288px */
    ///     bottom: 18rem; /* 288px */
    /// }
    /// ```
    InsetY72,
    /// ```css
    /// {
    ///     inset-inline-start: 18rem; /* 288px */
    /// }
    /// ```
    Start72,
    /// ```css
    /// {
    ///     inset-inline-end: 18rem; /* 288px */
    /// }
    /// ```
    End72,
    /// ```css
    /// {
    ///     top: 18rem; /* 288px */
    /// }
    /// ```
    Top72,
    /// ```css
    /// {
    ///     right: 18rem; /* 288px */
    /// }
    /// ```
    Right72,
    /// ```css
    /// {
    ///     bottom: 18rem; /* 288px */
    /// }
    /// ```
    Bottom72,
    /// ```css
    /// {
    ///     left: 18rem; /* 288px */
    /// }
    /// ```
    Left72,
    /// ```css
    /// {
    ///     inset: 20rem; /* 320px */
    /// }
    /// ```
    Inset80,
    /// ```css
    /// {
    ///     left: 20rem; /* 320px */
    ///     right: 20rem; /* 320px */
    /// }
    /// ```
    InsetX80,
    /// ```css
    /// {
    ///     top: 20rem; /* 320px */
    ///     bottom: 20rem; /* 320px */
    /// }
    /// ```
    InsetY80,
    /// ```css
    /// {
    ///     inset-inline-start: 20rem; /* 320px */
    /// }
    /// ```
    Start80,
    /// ```css
    /// {
    ///     inset-inline-end: 20rem; /* 320px */
    /// }
    /// ```
    End80,
    /// ```css
    /// {
    ///     top: 20rem; /* 320px */
    /// }
    /// ```
    Top80,
    /// ```css
    /// {
    ///     right: 20rem; /* 320px */
    /// }
    /// ```
    Right80,
    /// ```css
    /// {
    ///     bottom: 20rem; /* 320px */
    /// }
    /// ```
    Bottom80,
    /// ```css
    /// {
    ///     left: 20rem; /* 320px */
    /// }
    /// ```
    Left80,
    /// ```css
    /// {
    ///     inset: 24rem; /* 384px */
    /// }
    /// ```
    Inset96,
    /// ```css
    /// {
    ///     left: 24rem; /* 384px */
    ///     right: 24rem; /* 384px */
    /// }
    /// ```
    InsetX96,
    /// ```css
    /// {
    ///     top: 24rem; /* 384px */
    ///     bottom: 24rem; /* 384px */
    /// }
    /// ```
    InsetY96,
    /// ```css
    /// {
    ///     inset-inline-start: 24rem; /* 384px */
    /// }
    /// ```
    Start96,
    /// ```css
    /// {
    ///     inset-inline-end: 24rem; /* 384px */
    /// }
    /// ```
    End96,
    /// ```css
    /// {
    ///     top: 24rem; /* 384px */
    /// }
    /// ```
    Top96,
    /// ```css
    /// {
    ///     right: 24rem; /* 384px */
    /// }
    /// ```
    Right96,
    /// ```css
    /// {
    ///     bottom: 24rem; /* 384px */
    /// }
    /// ```
    Bottom96,
    /// ```css
    /// {
    ///     left: 24rem; /* 384px */
    /// }
    /// ```
    Left96,
    /// ```css
    /// {
    ///     inset: auto;
    /// }
    /// ```
    InsetAuto,
    /// ```css
    /// {
    ///     inset: 50%;
    /// }
    /// ```
    Inset1div2,
    /// ```css
    /// {
    ///     inset: 33.333333%;
    /// }
    /// ```
    Inset1div3,
    /// ```css
    /// {
    ///     inset: 66.666667%;
    /// }
    /// ```
    Inset2div3,
    /// ```css
    /// {
    ///     inset: 25%;
    /// }
    /// ```
    Inset1div4,
    /// ```css
    /// {
    ///     inset: 50%;
    /// }
    /// ```
    Inset2div4,
    /// ```css
    /// {
    ///     inset: 75%;
    /// }
    /// ```
    Inset3div4,
    /// ```css
    /// {
    ///     inset: 100%;
    /// }
    /// ```
    InsetFull,
    /// ```css
    /// {
    ///     left: auto;
    ///     right: auto;
    /// }
    /// ```
    InsetXAuto,
    /// ```css
    /// {
    ///     left: 50%;
    ///     right: 50%;
    /// }
    /// ```
    InsetX1div2,
    /// ```css
    /// {
    ///     left: 33.333333%;
    ///     right: 33.333333%;
    /// }
    /// ```
    InsetX1div3,
    /// ```css
    /// {
    ///     left: 66.666667%;
    ///     right: 66.666667%;
    /// }
    /// ```
    InsetX2div3,
    /// ```css
    /// {
    ///     left: 25%;
    ///     right: 25%;
    /// }
    /// ```
    InsetX1div4,
    /// ```css
    /// {
    ///     left: 50%;
    ///     right: 50%;
    /// }
    /// ```
    InsetX2div4,
    /// ```css
    /// {
    ///     left: 75%;
    ///     right: 75%;
    /// }
    /// ```
    InsetX3div4,
    /// ```css
    /// {
    ///     left: 100%;
    ///     right: 100%;
    /// }
    /// ```
    InsetXFull,
    /// ```css
    /// {
    ///     top: auto;
    ///     bottom: auto;
    /// }
    /// ```
    InsetYAuto,
    /// ```css
    /// {
    ///     top: 50%;
    ///     bottom: 50%;
    /// }
    /// ```
    InsetY1div2,
    /// ```css
    /// {
    ///     top: 33.333333%;
    ///     bottom: 33.333333%;
    /// }
    /// ```
    InsetY1div3,
    /// ```css
    /// {
    ///     top: 66.666667%;
    ///     bottom: 66.666667%;
    /// }
    /// ```
    InsetY2div3,
    /// ```css
    /// {
    ///     top: 25%;
    ///     bottom: 25%;
    /// }
    /// ```
    InsetY1div4,
    /// ```css
    /// {
    ///     top: 50%;
    ///     bottom: 50%;
    /// }
    /// ```
    InsetY2div4,
    /// ```css
    /// {
    ///     top: 75%;
    ///     bottom: 75%;
    /// }
    /// ```
    InsetY3div4,
    /// ```css
    /// {
    ///     top: 100%;
    ///     bottom: 100%;
    /// }
    /// ```
    InsetYFull,
    /// ```css
    /// {
    ///     inset-inline-start: auto;
    /// }
    /// ```
    StartAuto,
    /// ```css
    /// {
    ///     inset-inline-start: 50%;
    /// }
    /// ```
    Start1div2,
    /// ```css
    /// {
    ///     inset-inline-start: 33.333333%;
    /// }
    /// ```
    Start1div3,
    /// ```css
    /// {
    ///     inset-inline-start: 66.666667%;
    /// }
    /// ```
    Start2div3,
    /// ```css
    /// {
    ///     inset-inline-start: 25%;
    /// }
    /// ```
    Start1div4,
    /// ```css
    /// {
    ///     inset-inline-start: 50%;
    /// }
    /// ```
    Start2div4,
    /// ```css
    /// {
    ///     inset-inline-start: 75%;
    /// }
    /// ```
    Start3div4,
    /// ```css
    /// {
    ///     inset-inline-start: 100%;
    /// }
    /// ```
    StartFull,
    /// ```css
    /// {
    ///     inset-inline-end: auto;
    /// }
    /// ```
    EndAuto,
    /// ```css
    /// {
    ///     inset-inline-end: 50%;
    /// }
    /// ```
    End1div2,
    /// ```css
    /// {
    ///     inset-inline-end: 33.333333%;
    /// }
    /// ```
    End1div3,
    /// ```css
    /// {
    ///     inset-inline-end: 66.666667%;
    /// }
    /// ```
    End2div3,
    /// ```css
    /// {
    ///     inset-inline-end: 25%;
    /// }
    /// ```
    End1div4,
    /// ```css
    /// {
    ///     inset-inline-end: 50%;
    /// }
    /// ```
    End2div4,
    /// ```css
    /// {
    ///     inset-inline-end: 75%;
    /// }
    /// ```
    End3div4,
    /// ```css
    /// {
    ///     inset-inline-end: 100%;
    /// }
    /// ```
    EndFull,
    /// ```css
    /// {
    ///     top: auto;
    /// }
    /// ```
    TopAuto,
    /// ```css
    /// {
    ///     top: 50%;
    /// }
    /// ```
    Top1div2,
    /// ```css
    /// {
    ///     top: 33.333333%;
    /// }
    /// ```
    Top1div3,
    /// ```css
    /// {
    ///     top: 66.666667%;
    /// }
    /// ```
    Top2div3,
    /// ```css
    /// {
    ///     top: 25%;
    /// }
    /// ```
    Top1div4,
    /// ```css
    /// {
    ///     top: 50%;
    /// }
    /// ```
    Top2div4,
    /// ```css
    /// {
    ///     top: 75%;
    /// }
    /// ```
    Top3div4,
    /// ```css
    /// {
    ///     top: 100%;
    /// }
    /// ```
    TopFull,
    /// ```css
    /// {
    ///     right: auto;
    /// }
    /// ```
    RightAuto,
    /// ```css
    /// {
    ///     right: 50%;
    /// }
    /// ```
    Right1div2,
    /// ```css
    /// {
    ///     right: 33.333333%;
    /// }
    /// ```
    Right1div3,
    /// ```css
    /// {
    ///     right: 66.666667%;
    /// }
    /// ```
    Right2div3,
    /// ```css
    /// {
    ///     right: 25%;
    /// }
    /// ```
    Right1div4,
    /// ```css
    /// {
    ///     right: 50%;
    /// }
    /// ```
    Right2div4,
    /// ```css
    /// {
    ///     right: 75%;
    /// }
    /// ```
    Right3div4,
    /// ```css
    /// {
    ///     right: 100%;
    /// }
    /// ```
    RightFull,
    /// ```css
    /// {
    ///     bottom: auto;
    /// }
    /// ```
    BottomAuto,
    /// ```css
    /// {
    ///     bottom: 50%;
    /// }
    /// ```
    Bottom1div2,
    /// ```css
    /// {
    ///     bottom: 33.333333%;
    /// }
    /// ```
    Bottom1div3,
    /// ```css
    /// {
    ///     bottom: 66.666667%;
    /// }
    /// ```
    Bottom2div3,
    /// ```css
    /// {
    ///     bottom: 25%;
    /// }
    /// ```
    Bottom1div4,
    /// ```css
    /// {
    ///     bottom: 50%;
    /// }
    /// ```
    Bottom2div4,
    /// ```css
    /// {
    ///     bottom: 75%;
    /// }
    /// ```
    Bottom3div4,
    /// ```css
    /// {
    ///     bottom: 100%;
    /// }
    /// ```
    BottomFull,
    /// ```css
    /// {
    ///     left: auto;
    /// }
    /// ```
    LeftAuto,
    /// ```css
    /// {
    ///     left: 50%;
    /// }
    /// ```
    Left1div2,
    /// ```css
    /// {
    ///     left: 33.333333%;
    /// }
    /// ```
    Left1div3,
    /// ```css
    /// {
    ///     left: 66.666667%;
    /// }
    /// ```
    Left2div3,
    /// ```css
    /// {
    ///     left: 25%;
    /// }
    /// ```
    Left1div4,
    /// ```css
    /// {
    ///     left: 50%;
    /// }
    /// ```
    Left2div4,
    /// ```css
    /// {
    ///     left: 75%;
    /// }
    /// ```
    Left3div4,
    /// ```css
    /// {
    ///     left: 100%;
    /// }
    /// ```
    LeftFull,
}

/// Utilities for controlling the visibility of an element.
/// 
/// <https://tailwindcss.com/docs/visibility>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum Visibility {
    /// ```css
    /// {
    ///     visibility: visible;
    /// }
    /// ```
    Visible,
    /// ```css
    /// {
    ///     visibility: hidden;
    /// }
    /// ```
    Invisible,
    /// ```css
    /// {
    ///     visibility: collapse;
    /// }
    /// ```
    Collapse,
}

/// Utilities for controlling the stack order of an element.
///
/// <https://tailwindcss.com/docs/z-index>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "z")]
pub enum ZIndex {
    /// ```css
    /// {
    ///     z-index: 0;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     z-index: 10;
    /// }
    /// ```
    _10,
    /// ```css
    /// {
    ///     z-index: 20;
    /// }
    /// ```
    _20,
    /// ```css
    /// {
    ///     z-index: 30;
    /// }
    /// ```
    _30,
    /// ```css
    /// {
    ///     z-index: 40;
    /// }
    /// ```
    _40,
    /// ```css
    /// {
    ///     z-index: 50;
    /// }
    /// ```
    _50,
    /// ```css
    /// {
    ///     z-index: auto;
    /// }
    /// ```
    Auto,
}
