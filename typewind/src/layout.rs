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
    /// `aspect-ratio: auto;`
    Auto,
    /// `aspect-ratio: 1 / 1;`
    Square,
    /// `aspect-ratio: 16 / 9;`
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
    /// `columns: 1;`
    _1,
    /// `columns: 2;`
    _2,
    /// `columns: 3;`
    _3,
    /// `columns: 4;`
    _4,
    /// `columns: 5;`
    _5,
    /// `columns: 6;`
    _6,
    /// `columns: 7;`
    _7,
    /// `columns: 8;`
    _8,
    /// `columns: 9;`
    _9,
    /// `columns: 10;`
    _10,
    /// `columns: 11;`
    _11,
    /// `columns: 12;`
    _12,
    /// `columns: auto;`
    Auto,
    /// `columns: 16rem; /* 256px */`
    _3xs,
    /// `columns: 18rem; /* 288px */`
    _2xs,
    /// `columns: 20rem; /* 320px */`
    Xs,
    /// `columns: 24rem; /* 384px */`
    Sm,
    /// `columns: 28rem; /* 448px */`
    Md,
    /// `columns: 32rem; /* 512px */`
    Lg,
    /// `columns: 36rem; /* 576px */`
    Xl,
    /// `columns: 42rem; /* 672px */`
    _2xl,
    /// `columns: 48rem; /* 768px */`
    _3xl,
    /// `columns: 56rem; /* 896px */`
    _4xl,
    /// `columns: 64rem; /* 1024px */`
    _5xl,
    /// `columns: 72rem; /* 1152px */`
    _6xl,
    /// `columns: 80rem; /* 1280px */`
    _7xl,
}

/// Utilities for controlling how a column or page should break after an element.
/// 
/// <https://tailwindcss.com/docs/break-after>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "break-after")]
pub enum BreakAfter {
    /// `break-after: auto;`
    Auto,
    /// `break-after: avoid;`
    Avoid,
    /// `break-after: all;`
    All,
    /// `break-after: avoid-page;`
    AvoidPage,
    /// `break-after: page;`
    Page,
    /// `break-after: left;`
    Left,
    /// `break-after: right;`
    Right,
    /// `break-after: column;`
    Column,
}

/// Utilities for controlling how a column or page should break before an element.
/// 
/// <https://tailwindcss.com/docs/break-before>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "break-before")]
pub enum BreakBefore {
    /// `break-before: auto;`
    Auto,
    /// `break-before: avoid;`
    Avoid,
    /// `break-before: all;`
    All,
    /// `break-before: avoid-page;`
    AvoidPage,
    /// `break-before: page;`
    Page,
    /// `break-before: left;`
    Left,
    /// `break-before: right;`
    Right,
    /// `break-before: column;`
    Column,
}

/// Utilities for controlling how a column or page should break within an element.
/// 
/// <https://tailwindcss.com/docs/break-inside>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "break-inside")]
pub enum BreakInside {
    /// `break-inside: auto;`
    Auto,
    /// `break-inside: avoid;`
    Avoid,
    /// `break-inside: avoid-page;`
    AvoidPage,
    /// `break-inside: avoid-column;`
    AvoidColumn,
}

/// Utilities for controlling how element fragments should be rendered across multiple lines, columns, or pages.
/// 
/// <https://tailwindcss.com/docs/box-decoration-break>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "box-decoration")]
pub enum BoxDecorationBreak {
    /// `box-decoration-break: clone;`
    Clone,
    /// `box-decoration-break: slice;`
    Slice,
}

/// Utilities for controlling how the browser should calculate an element's total size.
/// 
/// <https://tailwindcss.com/docs/box-sizing>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "box")]
pub enum BoxSizing {
    /// `box-sizing: border-box;`
    Border,
    /// `box-sizing: content-box;`
    Content,
}

/// Utilities for controlling the display box type of an element.
/// 
/// <https://tailwindcss.com/docs/display>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum Display {
    /// `display: block;`
    Block,
    /// `display: inline-block;`
    InlineBlock,
    /// `display: inline;`
    Inline,
    /// `display: flex;`
    Flex,
    /// `display: inline-flex;`
    InlineFlex,
    /// `display: table;`
    Table,
    /// `display: inline-table;`
    InlineTable,
    /// `display: table-caption;`
    TableCaption,
    /// `display: table-cell;`
    TableCell,
    /// `display: table-column;`
    TableColumn,
    /// `display: table-column-group;`
    TableColumnGroup,
    /// `display: table-footer-group;`
    TableFooterGroup,
    /// `display: table-header-group;`
    TableHeaderGroup,
    /// `display: table-row-group;`
    TableRowGroup,
    /// `display: table-row;`
    TableRow,
    /// `display: flow-root;`
    FlowRoot,
    /// `display: grid;`
    Grid,
    /// `display: inline-grid;`
    InlineGrid,
    /// `display: contents;`
    Contents,
    /// `display: list-item;`
    ListItem,
    /// `display: none;`
    Hidden,
}

/// Utilities for controlling the wrapping of content around an element.
/// 
/// <https://tailwindcss.com/docs/float>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "float")]
pub enum Floats {
    /// `float: inline-start;`
    Start,
    /// `float: inline-end;`
    End,
    /// `float: right;`
    Right,
    /// `float: left;`
    Left,
    /// `float: none;`
    None,
}

/// Utilities for controlling the wrapping of content around an element.
/// 
/// <https://tailwindcss.com/docs/clear>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "clear")]
pub enum Clear {
    /// `clear: inline-start;`
    Start,
    /// `clear: inline-end;`
    End,
    /// `clear: left;`
    Left,
    /// `clear: right;`
    Right,
    /// `clear: both;`
    Both,
    /// `clear: none;`
    None,
}

/// Utilities for controlling whether an element should explicitly create a new stacking context.
/// 
/// <https://tailwindcss.com/docs/isolation>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum Isolation {
    /// `isolation: isolate;`
    Isolate,
    /// `isolation: auto;`
    IsolationAuto,
}

/// Utilities for controlling how a replaced element's content should be resized.
/// 
/// <https://tailwindcss.com/docs/object-fit>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "object")]
pub enum ObjectFit {
    /// `object-fit: contain;`
    Contain,
    /// `object-fit: cover;`
    Cover,
    /// `object-fit: fill;`
    Fill,
    /// `object-fit: none;`
    None,
    /// `object-fit: scale-down;`
    ScaleDown,
}

/// Utilities for controlling how a replaced element's content should be positioned within its container.
/// 
/// <https://tailwindcss.com/docs/object-position>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "object")]
pub enum ObjectPosition {
    /// `object-position: bottom;`
    Bottom,
    /// `object-position: center;`
    Center,
    /// `object-position: left;`
    Left,
    /// `object-position: left bottom;`
    LeftBottom,
    /// `object-position: left top;`
    LeftTop,
    /// `object-position: right;`
    Right,
    /// `object-position: right bottom;`
    RightBottom,
    /// `object-position: right top;`
    RightTop,
    /// `object-position: top;`
    Top,
}

/// Utilities for controlling how an element handles content that is too large for the container.
/// 
/// <https://tailwindcss.com/docs/overflow>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "overflow")]
pub enum Overflow {
    /// `overflow: auto;`
    Auto,
    /// `overflow: hidden;`
    Hidden,
    /// `overflow: clip;`
    Clip,
    /// `overflow: visible;`
    Visible,
    /// `overflow: scroll;`
    Scroll,
    /// `overflow-x: auto;`
    XAuto,
    /// `overflow-y: auto;`
    YAuto,
    /// `overflow-x: hidden;`
    XHidden,
    /// `overflow-y: hidden;`
    YHidden,
    /// `overflow-x: clip;`
    XClip,
    /// `overflow-y: clip;`
    YClip,
    /// `overflow-x: visible;`
    XVisible,
    /// `overflow-y: visible;`
    YVisible,
    /// `overflow-x: scroll;`
    XScroll,
    /// `overflow-y: scroll;`
    YScroll,
}

/// Utilities for controlling how the browser behaves when reaching the boundary of a scrolling area.
/// 
/// <https://tailwindcss.com/docs/overscroll-behavior>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "overscroll")]
pub enum OverscrollBehavior {
    /// `overscroll-behavior: auto;`
    Auto,
    /// `overscroll-behavior: contain;`
    Contain,
    /// `overscroll-behavior: none;`
    None,
    /// `overscroll-behavior-y: auto;`
    YAuto,
    /// `overscroll-behavior-y: contain;`
    YContain,
    /// `overscroll-behavior-y: none;`
    YNone,
    /// `overscroll-behavior-x: auto;`
    XAuto,
    /// `overscroll-behavior-x: contain;`
    XContain,
    /// `overscroll-behavior-x: none;`
    XNone,
}

/// Utilities for controlling how an element is positioned in the DOM.
/// 
/// <https://tailwindcss.com/docs/position>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum Position {
    /// `position: static;`
    Static,
    /// `position: fixed;`
    Fixed,
    /// `position: absolute;`
    Absolute,
    /// `position: relative;`
    Relative,
    /// `position: sticky;`
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
    /// `inset: 0px;`
    Inset0,
    /// `left: 0px;`
    /// 
    /// `right: 0px;`
    InsetX0,
    /// `top: 0px;`
    /// 
    /// `bottom: 0px;`
    InsetY0,
    /// `inset-inline-start: 0px;`
    Start0,
    /// `inset-inline-end: 0px;`
    End0,
    /// `top: 0px;`
    Top0,
    /// `right: 0px;`
    Right0,
    /// `bottom: 0px;`
    Bottom0,
    /// `left: 0px;`
    Left0,
    /// `inset: 1px;`
    InsetPx,
    /// `left: 1px;`
    /// 
    /// `right: 1px;`
    InsetXPx,
    /// `top: 1px;`
    /// 
    /// `bottom: 1px;`
    InsetYPx,
    /// `inset-inline-start: 1px;`
    StartPx,
    /// `inset-inline-end: 1px;`
    EndPx,
    /// `top: 1px;`
    TopPx,
    /// `right: 1px;`
    RightPx,
    /// `bottom: 1px;`
    BottomPx,
    /// `left: 1px;`
    LeftPx,
    /// `inset: 0.125rem; /* 2px */`
    Inset0_5,
    /// `left: 0.125rem; /* 2px */`
    /// 
    /// `right: 0.125rem; /* 2px */`
    InsetX0_5,
    /// `top: 0.125rem; /* 2px */`
    /// 
    /// `bottom: 0.125rem; /* 2px */`
    InsetY0_5,
    /// `inset-inline-start: 0.125rem; /* 2px */`
    Start0_5,
    /// `inset-inline-end: 0.125rem; /* 2px */`
    End0_5,
    /// `top: 0.125rem; /* 2px */`
    Top0_5,
    /// `right: 0.125rem; /* 2px */`
    Right0_5,
    /// `bottom: 0.125rem; /* 2px */`
    Bottom0_5,
    /// `left: 0.125rem; /* 2px */`
    Left0_5,
    /// `inset: 0.25rem; /* 4px */`
    Inset1,
    /// `left: 0.25rem; /* 4px */`
    /// 
    /// `right: 0.25rem; /* 4px */`
    InsetX1,
    /// `top: 0.25rem; /* 4px */`
    /// 
    /// `bottom: 0.25rem; /* 4px */`
    InsetY1,
    /// `inset-inline-start: 0.25rem; /* 4px */`
    Start1,
    /// `inset-inline-end: 0.25rem; /* 4px */`
    End1,
    /// `top: 0.25rem; /* 4px */`
    Top1,
    /// `right: 0.25rem; /* 4px */`
    Right1,
    /// `bottom: 0.25rem; /* 4px */`
    Bottom1,
    /// `left: 0.25rem; /* 4px */`
    Left1,
    /// `inset: 0.375rem; /* 6px */`
    Inset1_5,
    /// `left: 0.375rem; /* 6px */`
    /// 
    /// `right: 0.375rem; /* 6px */`
    InsetX1_5,
    /// `top: 0.375rem; /* 6px */`
    /// 
    /// `bottom: 0.375rem; /* 6px */`
    InsetY1_5,
    /// `inset-inline-start: 0.375rem; /* 6px */`
    Start1_5,
    /// `inset-inline-end: 0.375rem; /* 6px */`
    End1_5,
    /// `top: 0.375rem; /* 6px */`
    Top1_5,
    /// `right: 0.375rem; /* 6px */`
    Right1_5,
    /// `bottom: 0.375rem; /* 6px */`
    Bottom1_5,
    /// `left: 0.375rem; /* 6px */`
    Left1_5,
    /// `inset: 0.5rem; /* 8px */`
    Inset2,
    /// `left: 0.5rem; /* 8px */`
    /// 
    /// `right: 0.5rem; /* 8px */`
    InsetX2,
    /// `top: 0.5rem; /* 8px */`
    /// 
    /// `bottom: 0.5rem; /* 8px */`
    InsetY2,
    /// `inset-inline-start: 0.5rem; /* 8px */`
    Start2,
    /// `inset-inline-end: 0.5rem; /* 8px */`
    End2,
    /// `top: 0.5rem; /* 8px */`
    Top2,
    /// `right: 0.5rem; /* 8px */`
    Right2,
    /// `bottom: 0.5rem; /* 8px */`
    Bottom2,
    /// `left: 0.5rem; /* 8px */`
    Left2,
    /// `inset: 0.625rem; /* 10px */`
    Inset2_5,
    /// `left: 0.625rem; /* 10px */`
    /// 
    /// `right: 0.625rem; /* 10px */`
    InsetX2_5,
    /// `top: 0.625rem; /* 10px */`
    /// 
    /// `bottom: 0.625rem; /* 10px */`
    InsetY2_5,
    /// `inset-inline-start: 0.625rem; /* 10px */`
    Start2_5,
    /// `inset-inline-end: 0.625rem; /* 10px */`
    End2_5,
    /// `top: 0.625rem; /* 10px */`
    Top2_5,
    /// `right: 0.625rem; /* 10px */`
    Right2_5,
    /// `bottom: 0.625rem; /* 10px */`
    Bottom2_5,
    /// `left: 0.625rem; /* 10px */`
    Left2_5,
    /// `inset: 0.75rem; /* 12px */`
    Inset3,
    /// `left: 0.75rem; /* 12px */`
    /// 
    /// `right: 0.75rem; /* 12px */`
    InsetX3,
    /// `top: 0.75rem; /* 12px */`
    /// 
    /// `bottom: 0.75rem; /* 12px */`
    InsetY3,
    /// `inset-inline-start: 0.75rem; /* 12px */`
    Start3,
    /// `inset-inline-end: 0.75rem; /* 12px */`
    End3,
    /// `top: 0.75rem; /* 12px */`
    Top3,
    /// `right: 0.75rem; /* 12px */`
    Right3,
    /// `bottom: 0.75rem; /* 12px */`
    Bottom3,
    /// `left: 0.75rem; /* 12px */`
    Left3,
    /// `inset: 0.875rem; /* 14px */`
    Inset3_5,
    /// `left: 0.875rem; /* 14px */`
    /// 
    /// `right: 0.875rem; /* 14px */`
    InsetX3_5,
    /// `top: 0.875rem; /* 14px */`
    /// 
    /// `bottom: 0.875rem; /* 14px */`
    InsetY3_5,
    /// `inset-inline-start: 0.875rem; /* 14px */`
    Start3_5,
    /// `inset-inline-end: 0.875rem; /* 14px */`
    End3_5,
    /// `top: 0.875rem; /* 14px */`
    Top3_5,
    /// `right: 0.875rem; /* 14px */`
    Right3_5,
    /// `bottom: 0.875rem; /* 14px */`
    Bottom3_5,
    /// `left: 0.875rem; /* 14px */`
    Left3_5,
    /// `inset: 1rem; /* 16px */`
    Inset4,
    /// `left: 1rem; /* 16px */`
    /// 
    /// `right: 1rem; /* 16px */`
    InsetX4,
    /// `top: 1rem; /* 16px */`
    /// 
    /// `bottom: 1rem; /* 16px */`
    InsetY4,
    /// `inset-inline-start: 1rem; /* 16px */`
    Start4,
    /// `inset-inline-end: 1rem; /* 16px */`
    End4,
    /// `top: 1rem; /* 16px */`
    Top4,
    /// `right: 1rem; /* 16px */`
    Right4,
    /// `bottom: 1rem; /* 16px */`
    Bottom4,
    /// `left: 1rem; /* 16px */`
    Left4,
    /// `inset: 1.25rem; /* 20px */`
    Inset5,
    /// `left: 1.25rem; /* 20px */`
    /// 
    /// `right: 1.25rem; /* 20px */`
    InsetX5,
    /// `top: 1.25rem; /* 20px */`
    /// 
    /// `bottom: 1.25rem; /* 20px */`
    InsetY5,
    /// `inset-inline-start: 1.25rem; /* 20px */`
    Start5,
    /// `inset-inline-end: 1.25rem; /* 20px */`
    End5,
    /// `top: 1.25rem; /* 20px */`
    Top5,
    /// `right: 1.25rem; /* 20px */`
    Right5,
    /// `bottom: 1.25rem; /* 20px */`
    Bottom5,
    /// `left: 1.25rem; /* 20px */`
    Left5,
    /// `inset: 1.5rem; /* 24px */`
    Inset6,
    /// `left: 1.5rem; /* 24px */`
    /// 
    /// `right: 1.5rem; /* 24px */`
    InsetX6,
    /// `top: 1.5rem; /* 24px */`
    /// 
    /// `bottom: 1.5rem; /* 24px */`
    InsetY6,
    /// `inset-inline-start: 1.5rem; /* 24px */`
    Start6,
    /// `inset-inline-end: 1.5rem; /* 24px */`
    End6,
    /// `top: 1.5rem; /* 24px */`
    Top6,
    /// `right: 1.5rem; /* 24px */`
    Right6,
    /// `bottom: 1.5rem; /* 24px */`
    Bottom6,
    /// `left: 1.5rem; /* 24px */`
    Left6,
    /// `inset: 1.75rem; /* 28px */`
    Inset7,
    /// `left: 1.75rem; /* 28px */`
    /// 
    /// `right: 1.75rem; /* 28px */`
    InsetX7,
    /// `top: 1.75rem; /* 28px */`
    /// 
    /// `bottom: 1.75rem; /* 28px */`
    InsetY7,
    /// `inset-inline-start: 1.75rem; /* 28px */`
    Start7,
    /// `inset-inline-end: 1.75rem; /* 28px */`
    End7,
    /// `top: 1.75rem; /* 28px */`
    Top7,
    /// `right: 1.75rem; /* 28px */`
    Right7,
    /// `bottom: 1.75rem; /* 28px */`
    Bottom7,
    /// `left: 1.75rem; /* 28px */`
    Left7,
    /// `inset: 2rem; /* 32px */`
    Inset8,
    /// `left: 2rem; /* 32px */`
    /// 
    /// `right: 2rem; /* 32px */`
    InsetX8,
    /// `top: 2rem; /* 32px */`
    /// 
    /// `bottom: 2rem; /* 32px */`
    InsetY8,
    /// `inset-inline-start: 2rem; /* 32px */`
    Start8,
    /// `inset-inline-end: 2rem; /* 32px */`
    End8,
    /// `top: 2rem; /* 32px */`
    Top8,
    /// `right: 2rem; /* 32px */`
    Right8,
    /// `bottom: 2rem; /* 32px */`
    Bottom8,
    /// `left: 2rem; /* 32px */`
    Left8,
    /// `inset: 2.25rem; /* 36px */`
    Inset9,
    /// `left: 2.25rem; /* 36px */`
    /// 
    /// `right: 2.25rem; /* 36px */`
    InsetX9,
    /// `top: 2.25rem; /* 36px */`
    /// 
    /// `bottom: 2.25rem; /* 36px */`
    InsetY9,
    /// `inset-inline-start: 2.25rem; /* 36px */`
    Start9,
    /// `inset-inline-end: 2.25rem; /* 36px */`
    End9,
    /// `top: 2.25rem; /* 36px */`
    Top9,
    /// `right: 2.25rem; /* 36px */`
    Right9,
    /// `bottom: 2.25rem; /* 36px */`
    Bottom9,
    /// `left: 2.25rem; /* 36px */`
    Left9,
    /// `inset: 2.5rem; /* 40px */`
    Inset10,
    /// `left: 2.5rem; /* 40px */`
    /// 
    /// `right: 2.5rem; /* 40px */`
    InsetX10,
    /// `top: 2.5rem; /* 40px */`
    /// 
    /// `bottom: 2.5rem; /* 40px */`
    InsetY10,
    /// `inset-inline-start: 2.5rem; /* 40px */`
    Start10,
    /// `inset-inline-end: 2.5rem; /* 40px */`
    End10,
    /// `top: 2.5rem; /* 40px */`
    Top10,
    /// `right: 2.5rem; /* 40px */`
    Right10,
    /// `bottom: 2.5rem; /* 40px */`
    Bottom10,
    /// `left: 2.5rem; /* 40px */`
    Left10,
    /// `inset: 2.75rem; /* 44px */`
    Inset11,
    /// `left: 2.75rem; /* 44px */`
    /// 
    /// `right: 2.75rem; /* 44px */`
    InsetX11,
    /// `top: 2.75rem; /* 44px */`
    /// 
    /// `bottom: 2.75rem; /* 44px */`
    InsetY11,
    /// `inset-inline-start: 2.75rem; /* 44px */`
    Start11,
    /// `inset-inline-end: 2.75rem; /* 44px */`
    End11,
    /// `top: 2.75rem; /* 44px */`
    Top11,
    /// `right: 2.75rem; /* 44px */`
    Right11,
    /// `bottom: 2.75rem; /* 44px */`
    Bottom11,
    /// `left: 2.75rem; /* 44px */`
    Left11,
    /// `inset: 3rem; /* 48px */`
    Inset12,
    /// `left: 3rem; /* 48px */`
    /// 
    /// `right: 3rem; /* 48px */`
    InsetX12,
    /// `top: 3rem; /* 48px */`
    /// 
    /// `bottom: 3rem; /* 48px */`
    InsetY12,
    /// `inset-inline-start: 3rem; /* 48px */`
    Start12,
    /// `inset-inline-end: 3rem; /* 48px */`
    End12,
    /// `top: 3rem; /* 48px */`
    Top12,
    /// `right: 3rem; /* 48px */`
    Right12,
    /// `bottom: 3rem; /* 48px */`
    Bottom12,
    /// `left: 3rem; /* 48px */`
    Left12,
    /// `inset: 3.5rem; /* 56px */`
    Inset14,
    /// `left: 3.5rem; /* 56px */`
    /// 
    /// `right: 3.5rem; /* 56px */`
    InsetX14,
    /// `top: 3.5rem; /* 56px */`
    /// 
    /// `bottom: 3.5rem; /* 56px */`
    InsetY14,
    /// `inset-inline-start: 3.5rem; /* 56px */`
    Start14,
    /// `inset-inline-end: 3.5rem; /* 56px */`
    End14,
    /// `top: 3.5rem; /* 56px */`
    Top14,
    /// `right: 3.5rem; /* 56px */`
    Right14,
    /// `bottom: 3.5rem; /* 56px */`
    Bottom14,
    /// `left: 3.5rem; /* 56px */`
    Left14,
    /// `inset: 4rem; /* 64px */`
    Inset16,
    /// `left: 4rem; /* 64px */`
    /// 
    /// `right: 4rem; /* 64px */`
    InsetX16,
    /// `top: 4rem; /* 64px */`
    /// 
    /// `bottom: 4rem; /* 64px */`
    InsetY16,
    /// `inset-inline-start: 4rem; /* 64px */`
    Start16,
    /// `inset-inline-end: 4rem; /* 64px */`
    End16,
    /// `top: 4rem; /* 64px */`
    Top16,
    /// `right: 4rem; /* 64px */`
    Right16,
    /// `bottom: 4rem; /* 64px */`
    Bottom16,
    /// `left: 4rem; /* 64px */`
    Left16,
    /// `inset: 5rem; /* 80px */`
    Inset20,
    /// `left: 5rem; /* 80px */`
    /// 
    /// `right: 5rem; /* 80px */`
    InsetX20,
    /// `top: 5rem; /* 80px */`
    /// 
    /// `bottom: 5rem; /* 80px */`
    InsetY20,
    /// `inset-inline-start: 5rem; /* 80px */`
    Start20,
    /// `inset-inline-end: 5rem; /* 80px */`
    End20,
    /// `top: 5rem; /* 80px */`
    Top20,
    /// `right: 5rem; /* 80px */`
    Right20,
    /// `bottom: 5rem; /* 80px */`
    Bottom20,
    /// `left: 5rem; /* 80px */`
    Left20,
    /// `inset: 6rem; /* 96px */`
    Inset24,
    /// `left: 6rem; /* 96px */`
    /// 
    /// `right: 6rem; /* 96px */`
    InsetX24,
    /// `top: 6rem; /* 96px */`
    /// 
    /// `bottom: 6rem; /* 96px */`
    InsetY24,
    /// `inset-inline-start: 6rem; /* 96px */`
    Start24,
    /// `inset-inline-end: 6rem; /* 96px */`
    End24,
    /// `top: 6rem; /* 96px */`
    Top24,
    /// `right: 6rem; /* 96px */`
    Right24,
    /// `bottom: 6rem; /* 96px */`
    Bottom24,
    /// `left: 6rem; /* 96px */`
    Left24,
    /// `inset: 7rem; /* 112px */`
    Inset28,
    /// `left: 7rem; /* 112px */`
    /// 
    /// `right: 7rem; /* 112px */`
    InsetX28,
    /// `top: 7rem; /* 112px */`
    /// 
    /// `bottom: 7rem; /* 112px */`
    InsetY28,
    /// `inset-inline-start: 7rem; /* 112px */`
    Start28,
    /// `inset-inline-end: 7rem; /* 112px */`
    End28,
    /// `top: 7rem; /* 112px */`
    Top28,
    /// `right: 7rem; /* 112px */`
    Right28,
    /// `bottom: 7rem; /* 112px */`
    Bottom28,
    /// `left: 7rem; /* 112px */`
    Left28,
    /// `inset: 8rem; /* 128px */`
    Inset32,
    /// `left: 8rem; /* 128px */`
    /// 
    /// `right: 8rem; /* 128px */`
    InsetX32,
    /// `top: 8rem; /* 128px */`
    /// 
    /// `bottom: 8rem; /* 128px */`
    InsetY32,
    /// `inset-inline-start: 8rem; /* 128px */`
    Start32,
    /// `inset-inline-end: 8rem; /* 128px */`
    End32,
    /// `top: 8rem; /* 128px */`
    Top32,
    /// `right: 8rem; /* 128px */`
    Right32,
    /// `bottom: 8rem; /* 128px */`
    Bottom32,
    /// `left: 8rem; /* 128px */`
    Left32,
    /// `inset: 9rem; /* 144px */`
    Inset36,
    /// `left: 9rem; /* 144px */`
    /// 
    /// `right: 9rem; /* 144px */`
    InsetX36,
    /// `top: 9rem; /* 144px */`
    /// 
    /// `bottom: 9rem; /* 144px */`
    InsetY36,
    /// `inset-inline-start: 9rem; /* 144px */`
    Start36,
    /// `inset-inline-end: 9rem; /* 144px */`
    End36,
    /// `top: 9rem; /* 144px */`
    Top36,
    /// `right: 9rem; /* 144px */`
    Right36,
    /// `bottom: 9rem; /* 144px */`
    Bottom36,
    /// `left: 9rem; /* 144px */`
    Left36,
    /// `inset: 10rem; /* 160px */`
    Inset40,
    /// `left: 10rem; /* 160px */`
    /// 
    /// `right: 10rem; /* 160px */`
    InsetX40,
    /// `top: 10rem; /* 160px */`
    /// 
    /// `bottom: 10rem; /* 160px */`
    InsetY40,
    /// `inset-inline-start: 10rem; /* 160px */`
    Start40,
    /// `inset-inline-end: 10rem; /* 160px */`
    End40,
    /// `top: 10rem; /* 160px */`
    Top40,
    /// `right: 10rem; /* 160px */`
    Right40,
    /// `bottom: 10rem; /* 160px */`
    Bottom40,
    /// `left: 10rem; /* 160px */`
    Left40,
    /// `inset: 11rem; /* 176px */`
    Inset44,
    /// `left: 11rem; /* 176px */`
    /// 
    /// `right: 11rem; /* 176px */`
    InsetX44,
    /// `top: 11rem; /* 176px */`
    /// 
    /// `bottom: 11rem; /* 176px */`
    InsetY44,
    /// `inset-inline-start: 11rem; /* 176px */`
    Start44,
    /// `inset-inline-end: 11rem; /* 176px */`
    End44,
    /// `top: 11rem; /* 176px */`
    Top44,
    /// `right: 11rem; /* 176px */`
    Right44,
    /// `bottom: 11rem; /* 176px */`
    Bottom44,
    /// `left: 11rem; /* 176px */`
    Left44,
    /// `inset: 12rem; /* 192px */`
    Inset48,
    /// `left: 12rem; /* 192px */`
    /// 
    /// `right: 12rem; /* 192px */`
    InsetX48,
    /// `top: 12rem; /* 192px */`
    /// 
    /// `bottom: 12rem; /* 192px */`
    InsetY48,
    /// `inset-inline-start: 12rem; /* 192px */`
    Start48,
    /// `inset-inline-end: 12rem; /* 192px */`
    End48,
    /// `top: 12rem; /* 192px */`
    Top48,
    /// `right: 12rem; /* 192px */`
    Right48,
    /// `bottom: 12rem; /* 192px */`
    Bottom48,
    /// `left: 12rem; /* 192px */`
    Left48,
    /// `inset: 13rem; /* 208px */`
    Inset52,
    /// `left: 13rem; /* 208px */`
    /// 
    /// `right: 13rem; /* 208px */`
    InsetX52,
    /// `top: 13rem; /* 208px */`
    /// 
    /// `bottom: 13rem; /* 208px */`
    InsetY52,
    /// `inset-inline-start: 13rem; /* 208px */`
    Start52,
    /// `inset-inline-end: 13rem; /* 208px */`
    End52,
    /// `top: 13rem; /* 208px */`
    Top52,
    /// `right: 13rem; /* 208px */`
    Right52,
    /// `bottom: 13rem; /* 208px */`
    Bottom52,
    /// `left: 13rem; /* 208px */`
    Left52,
    /// `inset: 14rem; /* 224px */`
    Inset56,
    /// `left: 14rem; /* 224px */`
    /// 
    /// `right: 14rem; /* 224px */`
    InsetX56,
    /// `top: 14rem; /* 224px */`
    /// 
    /// `bottom: 14rem; /* 224px */`
    InsetY56,
    /// `inset-inline-start: 14rem; /* 224px */`
    Start56,
    /// `inset-inline-end: 14rem; /* 224px */`
    End56,
    /// `top: 14rem; /* 224px */`
    Top56,
    /// `right: 14rem; /* 224px */`
    Right56,
    /// `bottom: 14rem; /* 224px */`
    Bottom56,
    /// `left: 14rem; /* 224px */`
    Left56,
    /// `inset: 15rem; /* 240px */`
    Inset60,
    /// `left: 15rem; /* 240px */`
    /// 
    /// `right: 15rem; /* 240px */`
    InsetX60,
    /// `top: 15rem; /* 240px */`
    /// 
    /// `bottom: 15rem; /* 240px */`
    InsetY60,
    /// `inset-inline-start: 15rem; /* 240px */`
    Start60,
    /// `inset-inline-end: 15rem; /* 240px */`
    End60,
    /// `top: 15rem; /* 240px */`
    Top60,
    /// `right: 15rem; /* 240px */`
    Right60,
    /// `bottom: 15rem; /* 240px */`
    Bottom60,
    /// `left: 15rem; /* 240px */`
    Left60,
    /// `inset: 16rem; /* 256px */`
    Inset64,
    /// `left: 16rem; /* 256px */`
    /// 
    /// `right: 16rem; /* 256px */`
    InsetX64,
    /// `top: 16rem; /* 256px */`
    /// 
    /// `bottom: 16rem; /* 256px */`
    InsetY64,
    /// `inset-inline-start: 16rem; /* 256px */`
    Start64,
    /// `inset-inline-end: 16rem; /* 256px */`
    End64,
    /// `top: 16rem; /* 256px */`
    Top64,
    /// `right: 16rem; /* 256px */`
    Right64,
    /// `bottom: 16rem; /* 256px */`
    Bottom64,
    /// `left: 16rem; /* 256px */`
    Left64,
    /// `inset: 18rem; /* 288px */`
    Inset72,
    /// `left: 18rem; /* 288px */`
    /// 
    /// `right: 18rem; /* 288px */`
    InsetX72,
    /// `top: 18rem; /* 288px */`
    /// 
    /// `bottom: 18rem; /* 288px */`
    InsetY72,
    /// `inset-inline-start: 18rem; /* 288px */`
    Start72,
    /// `inset-inline-end: 18rem; /* 288px */`
    End72,
    /// `top: 18rem; /* 288px */`
    Top72,
    /// `right: 18rem; /* 288px */`
    Right72,
    /// `bottom: 18rem; /* 288px */`
    Bottom72,
    /// `left: 18rem; /* 288px */`
    Left72,
    /// `inset: 20rem; /* 320px */`
    Inset80,
    /// `left: 20rem; /* 320px */`
    /// 
    /// `right: 20rem; /* 320px */`
    InsetX80,
    /// `top: 20rem; /* 320px */`
    /// 
    /// `bottom: 20rem; /* 320px */`
    InsetY80,
    /// `inset-inline-start: 20rem; /* 320px */`
    Start80,
    /// `inset-inline-end: 20rem; /* 320px */`
    End80,
    /// `top: 20rem; /* 320px */`
    Top80,
    /// `right: 20rem; /* 320px */`
    Right80,
    /// `bottom: 20rem; /* 320px */`
    Bottom80,
    /// `left: 20rem; /* 320px */`
    Left80,
    /// `inset: 24rem; /* 384px */`
    Inset96,
    /// `left: 24rem; /* 384px */`
    /// 
    /// `right: 24rem; /* 384px */`
    InsetX96,
    /// `top: 24rem; /* 384px */`
    /// 
    /// `bottom: 24rem; /* 384px */`
    InsetY96,
    /// `inset-inline-start: 24rem; /* 384px */`
    Start96,
    /// `inset-inline-end: 24rem; /* 384px */`
    End96,
    /// `top: 24rem; /* 384px */`
    Top96,
    /// `right: 24rem; /* 384px */`
    Right96,
    /// `bottom: 24rem; /* 384px */`
    Bottom96,
    /// `left: 24rem; /* 384px */`
    Left96,
    /// `inset: auto;`
    InsetAuto,
    /// `inset: 50%;`
    Inset1div2,
    /// `inset: 33.333333%;`
    Inset1div3,
    /// `inset: 66.666667%;`
    Inset2div3,
    /// `inset: 25%;`
    Inset1div4,
    /// `inset: 50%;`
    Inset2div4,
    /// `inset: 75%;`
    Inset3div4,
    /// `inset: 100%;`
    InsetFull,
    /// `left: auto;`
    /// 
    /// `right: auto;`
    InsetXAuto,
    /// `left: 50%;`
    /// 
    /// `right: 50%;`
    InsetX1div2,
    /// `left: 33.333333%;`
    /// 
    /// `right: 33.333333%;`
    InsetX1div3,
    /// `left: 66.666667%;`
    /// 
    /// `right: 66.666667%;`
    InsetX2div3,
    /// `left: 25%;`
    /// 
    /// `right: 25%;`
    InsetX1div4,
    /// `left: 50%;`
    /// 
    /// `right: 50%;`
    InsetX2div4,
    /// `left: 75%;`
    /// 
    /// `right: 75%;`
    InsetX3div4,
    /// `left: 100%;`
    /// 
    /// `right: 100%;`
    InsetXFull,
    /// `top: auto;`
    /// 
    /// `bottom: auto;`
    InsetYAuto,
    /// `top: 50%;`
    /// 
    /// `bottom: 50%;`
    InsetY1div2,
    /// `top: 33.333333%;`
    /// 
    /// `bottom: 33.333333%;`
    InsetY1div3,
    /// `top: 66.666667%;`
    /// 
    /// `bottom: 66.666667%;`
    InsetY2div3,
    /// `top: 25%;`
    /// 
    /// `bottom: 25%;`
    InsetY1div4,
    /// `top: 50%;`
    /// 
    /// `bottom: 50%;`
    InsetY2div4,
    /// `top: 75%;`
    /// 
    /// `bottom: 75%;`
    InsetY3div4,
    /// `top: 100%;`
    /// 
    /// `bottom: 100%;`
    InsetYFull,
    /// `inset-inline-start: auto;`
    StartAuto,
    /// `inset-inline-start: 50%;`
    Start1div2,
    /// `inset-inline-start: 33.333333%;`
    Start1div3,
    /// `inset-inline-start: 66.666667%;`
    Start2div3,
    /// `inset-inline-start: 25%;`
    Start1div4,
    /// `inset-inline-start: 50%;`
    Start2div4,
    /// `inset-inline-start: 75%;`
    Start3div4,
    /// `inset-inline-start: 100%;`
    StartFull,
    /// `inset-inline-end: auto;`
    EndAuto,
    /// `inset-inline-end: 50%;`
    End1div2,
    /// `inset-inline-end: 33.333333%;`
    End1div3,
    /// `inset-inline-end: 66.666667%;`
    End2div3,
    /// `inset-inline-end: 25%;`
    End1div4,
    /// `inset-inline-end: 50%;`
    End2div4,
    /// `inset-inline-end: 75%;`
    End3div4,
    /// `inset-inline-end: 100%;`
    EndFull,
    /// `top: auto;`
    TopAuto,
    /// `top: 50%;`
    Top1div2,
    /// `top: 33.333333%;`
    Top1div3,
    /// `top: 66.666667%;`
    Top2div3,
    /// `top: 25%;`
    Top1div4,
    /// `top: 50%;`
    Top2div4,
    /// `top: 75%;`
    Top3div4,
    /// `top: 100%;`
    TopFull,
    /// `right: auto;`
    RightAuto,
    /// `right: 50%;`
    Right1div2,
    /// `right: 33.333333%;`
    Right1div3,
    /// `right: 66.666667%;`
    Right2div3,
    /// `right: 25%;`
    Right1div4,
    /// `right: 50%;`
    Right2div4,
    /// `right: 75%;`
    Right3div4,
    /// `right: 100%;`
    RightFull,
    /// `bottom: auto;`
    BottomAuto,
    /// `bottom: 50%;`
    Bottom1div2,
    /// `bottom: 33.333333%;`
    Bottom1div3,
    /// `bottom: 66.666667%;`
    Bottom2div3,
    /// `bottom: 25%;`
    Bottom1div4,
    /// `bottom: 50%;`
    Bottom2div4,
    /// `bottom: 75%;`
    Bottom3div4,
    /// `bottom: 100%;`
    BottomFull,
    /// `left: auto;`
    LeftAuto,
    /// `left: 50%;`
    Left1div2,
    /// `left: 33.333333%;`
    Left1div3,
    /// `left: 66.666667%;`
    Left2div3,
    /// `left: 25%;`
    Left1div4,
    /// `left: 50%;`
    Left2div4,
    /// `left: 75%;`
    Left3div4,
    /// `left: 100%;`
    LeftFull,
}

/// Utilities for controlling the visibility of an element.
/// 
/// <https://tailwindcss.com/docs/visibility>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum Visibility {
    /// `visibility: visible;`
    Visible,
    /// `visibility: hidden;`
    Invisible,
    /// `visibility: collapse;`
    Collapse,
}

/// Utilities for controlling the stack order of an element.
///
/// <https://tailwindcss.com/docs/z-index>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "z")]
pub enum ZIndex {
    /// `z-index: 0;`
    _0,
    /// `z-index: 10;`
    _10,
    /// `z-index: 20;`
    _20,
    /// `z-index: 30;`
    _30,
    /// `z-index: 40;`
    _40,
    /// `z-index: 50;`
    _50,
    /// `z-index: auto;`
    Auto,
}
