use typewind_macros::{Display, Parse};

tailwind_types!(
    FlexBasis, FlexDirection, FlexWrap, Flex, FlexGrow, FlexShrink, Order, GridTemplateColumns, 
    GridColumnStartEnd, GridTemplateRows, GridRowStartEnd, GridAutoFlow, GridAutoColumns, 
    GridAutoRows, Gap, JustifyContent, JustifyItems, JustifySelf, AlignContent, AlignItems, 
    AlignSelf, PlaceContent, PlaceItems, PlaceSelf
);

/// Utilities for controlling the initial size of flex items.
/// 
/// <https://tailwindcss.com/docs/flex-basis>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(
    prefix = "basis",
    replace(from = "_", to = "."),
    replace(from = "div", to = "/")
)]
pub enum FlexBasis {
    /// `flex-basis: 0px;`
    _0,
    /// `flex-basis: 0.25rem; /* 4px */`
    _1,
    /// `flex-basis: 0.5rem; /* 8px */`
    _2,
    /// `flex-basis: 0.75rem; /* 12px */`
    _3,
    /// `flex-basis: 1rem; /* 16px */`
    _4,
    /// `flex-basis: 1.25rem; /* 20px */`
    _5,
    /// `flex-basis: 1.5rem; /* 24px */`
    _6,
    /// `flex-basis: 1.75rem; /* 28px */`
    _7,
    /// `flex-basis: 2rem; /* 32px */`
    _8,
    /// `flex-basis: 2.25rem; /* 36px */`
    _9,
    /// `flex-basis: 2.5rem; /* 40px */`
    _10,
    /// `flex-basis: 2.75rem; /* 44px */`
    _11,
    /// `flex-basis: 3rem; /* 48px */`
    _12,
    /// `flex-basis: 3.5rem; /* 56px */`
    _14,
    /// `flex-basis: 4rem; /* 64px */`
    _16,
    /// `flex-basis: 5rem; /* 80px */`
    _20,
    /// `flex-basis: 6rem; /* 96px */`
    _24,
    /// `flex-basis: 7rem; /* 112px */`
    _28,
    /// `flex-basis: 8rem; /* 128px */`
    _32,
    /// `flex-basis: 9rem; /* 144px */`
    _36,
    /// `flex-basis: 10rem; /* 160px */`
    _40,
    /// `flex-basis: 11rem; /* 176px */`
    _44,
    /// `flex-basis: 12rem; /* 192px */`
    _48,
    /// `flex-basis: 13rem; /* 208px */`
    _52,
    /// `flex-basis: 14rem; /* 224px */`
    _56,
    /// `flex-basis: 15rem; /* 240px */`
    _60,
    /// `flex-basis: 16rem; /* 256px */`
    _64,
    /// `flex-basis: 18rem; /* 288px */`
    _72,
    /// `flex-basis: 20rem; /* 320px */`
    _80,
    /// `flex-basis: 24rem; /* 384px */`
    _96,
    /// `flex-basis: auto;`
    Auto,
    /// `flex-basis: 1px;`
    Px,
    /// `flex-basis: 0.125rem; /* 2px */`
    _0_5,
    /// `flex-basis: 0.375rem; /* 6px */`
    _1_5,
    /// `flex-basis: 0.625rem; /* 10px */`
    _2_5,
    /// `flex-basis: 0.875rem; /* 14px */`
    _3_5,
    /// `flex-basis: 50%;`
    _1div2,
    /// `flex-basis: 33.333333%;`
    _1div3,
    /// `flex-basis: 66.666667%;`
    _2div3,
    /// `flex-basis: 25%;`
    _1div4,
    /// `flex-basis: 50%;`
    _2div4,
    /// `flex-basis: 75%;`
    _3div4,
    /// `flex-basis: 20%;`
    _1div5,
    /// `flex-basis: 40%;`
    _2div5,
    /// `flex-basis: 60%;`
    _3div5,
    /// `flex-basis: 80%;`
    _4div5,
    /// `flex-basis: 16.666667%;`
    _1div6,
    /// `flex-basis: 33.333333%;`
    _2div6,
    /// `flex-basis: 50%;`
    _3div6,
    /// `flex-basis: 66.666667%;`
    _4div6,
    /// `flex-basis: 83.333333%;`
    _5div6,
    /// `flex-basis: 8.333333%;`
    _1div12,
    /// `flex-basis: 16.666667%;`
    _2div12,
    /// `flex-basis: 25%;`
    _3div12,
    /// `flex-basis: 33.333333%;`
    _4div12,
    /// `flex-basis: 41.666667%;`
    _5div12,
    /// `flex-basis: 50%;`
    _6div12,
    /// `flex-basis: 58.333333%;`
    _7div12,
    /// `flex-basis: 66.666667%;`
    _8div12,
    /// `flex-basis: 75%;`
    _9div12,
    /// `flex-basis: 83.333333%;`
    _10div12,
    /// `flex-basis: 91.666667%;`
    _11div12,
    /// `flex-basis: 100%;`
    Full,
}

/// Utilities for controlling the direction of flex items.
/// 
/// <https://tailwindcss.com/docs/flex-direction>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "flex")]
pub enum FlexDirection {
    /// `flex-direction: row;`
    Row,
    /// `flex-direction: row-reverse;`
    RowReverse,
    /// `flex-direction: column;`
    Col,
    /// `flex-direction: column-reverse;`
    ColReverse,
}

/// Utilities for controlling how flex items wrap.
/// 
/// <https://tailwindcss.com/docs/flex-wrap>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "flex")]
pub enum FlexWrap {
    /// `flex-wrap: wrap;`
    Wrap,
    /// `flex-wrap: wrap-reverse;`
    WrapReverse,
    /// `flex-wrap: nowrap;`
    Nowrap,
}

/// Utilities for controlling how flex items both grow and shrink.
/// 
/// <https://tailwindcss.com/docs/flex>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "flex")]
pub enum Flex {
    /// `flex: 1 1 0%;`
    _1,
    /// `flex: 1 1 auto;`
    Auto,
    /// `flex: 0 1 auto;`
    Initial,
    /// `flex: none;`
    None,
}

/// Utilities for controlling how flex items grow.
/// 
/// <https://tailwindcss.com/docs/flex-grow>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum FlexGrow {
    /// `flex-grow: 1;`
    Grow,
    /// `flex-grow: 0;`
    Grow0,
}

/// Utilities for controlling how flex items shrink.
///
/// <https://tailwindcss.com/docs/flex-shrink>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum FlexShrink {
    /// `flex-shrink: 1;`
    Shrink,
    /// `flex-shrink: 0;`
    Shrink0,
}

/// Utilities for controlling the order of flex and grid items.
/// 
/// <https://tailwindcss.com/docs/order>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "order")]
pub enum Order {
    /// `order: 1;`
    _1,
    /// `order: 2;`
    _2,
    /// `order: 3;`
    _3,
    /// `order: 4;`
    _4,
    /// `order: 5;`
    _5,
    /// `order: 6;`
    _6,
    /// `order: 7;`
    _7,
    /// `order: 8;`
    _8,
    /// `order: 9;`
    _9,
    /// `order: 10;`
    _10,
    /// `order: 11;`
    _11,
    /// `order: 12;`
    _12,
    /// `order: -9999;`
    First,
    /// `order: 9999;`
    Last,
    /// `order: 0;`
    None,
}

/// Utilities for specifying the columns in a grid layout.
/// 
/// <https://tailwindcss.com/docs/grid-template-columns>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "grid-cols")]
pub enum GridTemplateColumns {
    /// `grid-template-columns: repeat(1, minmax(0, 1fr));`
    _1,
    /// `grid-template-columns: repeat(2, minmax(0, 1fr));`
    _2,
    /// `grid-template-columns: repeat(3, minmax(0, 1fr));`
    _3,
    /// `grid-template-columns: repeat(4, minmax(0, 1fr));`
    _4,
    /// `grid-template-columns: repeat(5, minmax(0, 1fr));`
    _5,
    /// `grid-template-columns: repeat(6, minmax(0, 1fr));`
    _6,
    /// `grid-template-columns: repeat(7, minmax(0, 1fr));`
    _7,
    /// `grid-template-columns: repeat(8, minmax(0, 1fr));`
    _8,
    /// `grid-template-columns: repeat(9, minmax(0, 1fr));`
    _9,
    /// `grid-template-columns: repeat(10, minmax(0, 1fr));`
    _10,
    /// `grid-template-columns: repeat(11, minmax(0, 1fr));`
    _11,
    /// `grid-template-columns: repeat(12, minmax(0, 1fr));`
    _12,
    /// `grid-template-columns: none;`
    None,
    /// `grid-template-columns: subgrid;`
    Subgrid,
}

/// Utilities for controlling how elements are sized and placed across grid columns.
/// 
/// <https://tailwindcss.com/docs/grid-column>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "col")]
pub enum GridColumnStartEnd {
    /// `grid-column: auto;`
    Auto,
    /// `grid-column: span 1 / span 1;`
    Span1,
    /// `grid-column: span 2 / span 2;`
    Span2,
    /// `grid-column: span 3 / span 3;`
    Span3,
    /// `grid-column: span 4 / span 4;`
    Span4,
    /// `grid-column: span 5 / span 5;`
    Span5,
    /// `grid-column: span 6 / span 6;`
    Span6,
    /// `grid-column: span 7 / span 7;`
    Span7,
    /// `grid-column: span 8 / span 8;`
    Span8,
    /// `grid-column: span 9 / span 9;`
    Span9,
    /// `grid-column: span 10 / span 10;`
    Span10,
    /// `grid-column: span 11 / span 11;`
    Span11,
    /// `grid-column: span 12 / span 12;`
    Span12,
    /// `grid-column: 1 / -1;`
    SpanFull,
    /// `grid-column-start: 1;`
    Start1,
    /// `grid-column-start: 2;`
    Start2,
    /// `grid-column-start: 3;`
    Start3,
    /// `grid-column-start: 4;`
    Start4,
    /// `grid-column-start: 5;`
    Start5,
    /// `grid-column-start: 6;`
    Start6,
    /// `grid-column-start: 7;`
    Start7,
    /// `grid-column-start: 8;`
    Start8,
    /// `grid-column-start: 9;`
    Start9,
    /// `grid-column-start: 10;`
    Start10,
    /// `grid-column-start: 11;`
    Start11,
    /// `grid-column-start: 12;`
    Start12,
    /// `grid-column-start: 13;`
    Start13,
    /// `grid-column-start: auto;`
    StartAuto,
    /// `grid-column-end: 1;`
    End1,
    /// `grid-column-end: 2;`
    End2,
    /// `grid-column-end: 3;`
    End3,
    /// `grid-column-end: 4;`
    End4,
    /// `grid-column-end: 5;`
    End5,
    /// `grid-column-end: 6;`
    End6,
    /// `grid-column-end: 7;`
    End7,
    /// `grid-column-end: 8;`
    End8,
    /// `grid-column-end: 9;`
    End9,
    /// `grid-column-end: 10;`
    End10,
    /// `grid-column-end: 11;`
    End11,
    /// `grid-column-end: 12;`
    End12,
    /// `grid-column-end: 13;`
    End13,
    /// `grid-column-end: auto;`
    EndAuto,
}

/// Utilities for specifying the rows in a grid layout.
/// 
/// <https://tailwindcss.com/docs/grid-template-rows>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "grid-rows")]
pub enum GridTemplateRows {
    /// `grid-template-rows: repeat(1, minmax(0, 1fr));`
    _1,
    /// `grid-template-rows: repeat(2, minmax(0, 1fr));`
    _2,
    /// `grid-template-rows: repeat(3, minmax(0, 1fr));`
    _3,
    /// `grid-template-rows: repeat(4, minmax(0, 1fr));`
    _4,
    /// `grid-template-rows: repeat(5, minmax(0, 1fr));`
    _5,
    /// `grid-template-rows: repeat(6, minmax(0, 1fr));`
    _6,
    /// `grid-template-rows: repeat(7, minmax(0, 1fr));`
    _7,
    /// `grid-template-rows: repeat(8, minmax(0, 1fr));`
    _8,
    /// `grid-template-rows: repeat(9, minmax(0, 1fr));`
    _9,
    /// `grid-template-rows: repeat(10, minmax(0, 1fr));`
    _10,
    /// `grid-template-rows: repeat(11, minmax(0, 1fr));`
    _11,
    /// `grid-template-rows: repeat(12, minmax(0, 1fr));`
    _12,
    /// `grid-template-rows: none;`
    None,
    /// `grid-template-rows: subgrid;`
    Subgrid,
}

/// Utilities for controlling how elements are sized and placed across grid rows.
/// 
/// <https://tailwindcss.com/docs/grid-row>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "row")]
pub enum GridRowStartEnd {
    /// `grid-row: auto;`
    Auto,
    /// `grid-row: span 1 / span 1;`
    Span1,
    /// `grid-row: span 2 / span 2;`
    Span2,
    /// `grid-row: span 3 / span 3;`
    Span3,
    /// `grid-row: span 4 / span 4;`
    Span4,
    /// `grid-row: span 5 / span 5;`
    Span5,
    /// `grid-row: span 6 / span 6;`
    Span6,
    /// `grid-row: span 7 / span 7;`
    Span7,
    /// `grid-row: span 8 / span 8;`
    Span8,
    /// `grid-row: span 9 / span 9;`
    Span9,
    /// `grid-row: span 10 / span 10;`
    Span10,
    /// `grid-row: span 11 / span 11;`
    Span11,
    /// `grid-row: span 12 / span 12;`
    Span12,
    /// `grid-row: 1 / -1;`
    SpanFull,
    /// `grid-row-start: 1;`
    Start1,
    /// `grid-row-start: 2;`
    Start2,
    /// `grid-row-start: 3;`
    Start3,
    /// `grid-row-start: 4;`
    Start4,
    /// `grid-row-start: 5;`
    Start5,
    /// `grid-row-start: 6;`
    Start6,
    /// `grid-row-start: 7;`
    Start7,
    /// `grid-row-start: 8;`
    Start8,
    /// `grid-row-start: 9;`
    Start9,
    /// `grid-row-start: 10;`
    Start10,
    /// `grid-row-start: 11;`
    Start11,
    /// `grid-row-start: 12;`
    Start12,
    /// `grid-row-start: 13;`
    Start13,
    /// `grid-row-start: auto;`
    StartAuto,
    /// `grid-row-end: 1;`
    End1,
    /// `grid-row-end: 2;`
    End2,
    /// `grid-row-end: 3;`
    End3,
    /// `grid-row-end: 4;`
    End4,
    /// `grid-row-end: 5;`
    End5,
    /// `grid-row-end: 6;`
    End6,
    /// `grid-row-end: 7;`
    End7,
    /// `grid-row-end: 8;`
    End8,
    /// `grid-row-end: 9;`
    End9,
    /// `grid-row-end: 10;`
    End10,
    /// `grid-row-end: 11;`
    End11,
    /// `grid-row-end: 12;`
    End12,
    /// `grid-row-end: 13;`
    End13,
    /// `grid-row-end: auto;`
    EndAuto,
}

/// Utilities for controlling how elements in a grid are auto-placed.
///
/// <https://tailwindcss.com/docs/grid-auto-flow>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "grid-flow")]
pub enum GridAutoFlow {
    /// `grid-auto-flow: row;`
    Row,
    /// `grid-auto-flow: column;`
    Col,
    /// `grid-auto-flow: dense;`
    Dense,
    /// `grid-auto-flow: row dense;`
    RowDense,
    /// `grid-auto-flow: column dense;`
    ColDense,
}

/// Utilities for controlling the size of implicitly-created grid columns.
/// 
/// <https://tailwindcss.com/docs/grid-auto-columns>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "auto-cols")]
pub enum GridAutoColumns {
    /// `grid-auto-columns: auto;`
    Auto,
    /// `grid-auto-columns: min-content;`
    Min,
    /// `grid-auto-columns: max-content;`
    Max,
    /// `grid-auto-columns: minmax(0, 1fr);`
    Fr,
}

/// Utilities for controlling the size of implicitly-created grid rows.
/// 
/// <https://tailwindcss.com/docs/grid-auto-rows>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "auto-rows")]
pub enum GridAutoRows {
    /// `grid-auto-rows: auto;`
    Auto,
    /// `grid-auto-rows: min-content;`
    Min,
    /// `grid-auto-rows: max-content;`
    Max,
    /// `grid-auto-rows: minmax(0, 1fr);`
    Fr,
}

/// Utilities for controlling gutters between grid and flexbox items.
/// 
/// <https://tailwindcss.com/docs/gap>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "gap", replace(from = "_", to = "."))]
pub enum Gap {
    /// `gap: 0px;`
    _0,
    /// `column-gap: 0px;`
    X0,
    /// `row-gap: 0px;`
    Y0,
    /// `gap: 1px;`
    Px,
    /// `column-gap: 1px;`
    XPx,
    /// `row-gap: 1px;`
    YPx,
    /// `gap: 0.125rem; /* 2px */`
    _0_5,
    /// `column-gap: 0.125rem; /* 2px */`
    X0_5,
    /// `row-gap: 0.125rem; /* 2px */`
    Y0_5,
    /// `gap: 0.25rem; /* 4px */`
    _1,
    /// `column-gap: 0.25rem; /* 4px */`
    X1,
    /// `row-gap: 0.25rem; /* 4px */`
    Y1,
    /// `gap: 0.375rem; /* 6px */`
    _1_5,
    /// `column-gap: 0.375rem; /* 6px */`
    X1_5,
    /// `row-gap: 0.375rem; /* 6px */`
    Y1_5,
    /// `gap: 0.5rem; /* 8px */`
    _2,
    /// `column-gap: 0.5rem; /* 8px */`
    X2,
    /// `row-gap: 0.5rem; /* 8px */`
    Y2,
    /// `gap: 0.625rem; /* 10px */`
    _2_5,
    /// `column-gap: 0.625rem; /* 10px */`
    X2_5,
    /// `row-gap: 0.625rem; /* 10px */`
    Y2_5,
    /// `gap: 0.75rem; /* 12px */`
    _3,
    /// `column-gap: 0.75rem; /* 12px */`
    X3,
    /// `row-gap: 0.75rem; /* 12px */`
    Y3,
    /// `gap: 0.875rem; /* 14px */`
    _3_5,
    /// `column-gap: 0.875rem; /* 14px */`
    X3_5,
    /// `row-gap: 0.875rem; /* 14px */`
    Y3_5,
    /// `gap: 1rem; /* 16px */`
    _4,
    /// `column-gap: 1rem; /* 16px */`
    X4,
    /// `row-gap: 1rem; /* 16px */`
    Y4,
    /// `gap: 1.25rem; /* 20px */`
    _5,
    /// `column-gap: 1.25rem; /* 20px */`
    X5,
    /// `row-gap: 1.25rem; /* 20px */`
    Y5,
    /// `gap: 1.5rem; /* 24px */`
    _6,
    /// `column-gap: 1.5rem; /* 24px */`
    X6,
    /// `row-gap: 1.5rem; /* 24px */`
    Y6,
    /// `gap: 1.75rem; /* 28px */`
    _7,
    /// `column-gap: 1.75rem; /* 28px */`
    X7,
    /// `row-gap: 1.75rem; /* 28px */`
    Y7,
    /// `gap: 2rem; /* 32px */`
    _8,
    /// `column-gap: 2rem; /* 32px */`
    X8,
    /// `row-gap: 2rem; /* 32px */`
    Y8,
    /// `gap: 2.25rem; /* 36px */`
    _9,
    /// `column-gap: 2.25rem; /* 36px */`
    X9,
    /// `row-gap: 2.25rem; /* 36px */`
    Y9,
    /// `gap: 2.5rem; /* 40px */`
    _10,
    /// `column-gap: 2.5rem; /* 40px */`
    X10,
    /// `row-gap: 2.5rem; /* 40px */`
    Y10,
    /// `gap: 2.75rem; /* 44px */`
    _11,
    /// `column-gap: 2.75rem; /* 44px */`
    X11,
    /// `row-gap: 2.75rem; /* 44px */`
    Y11,
    /// `gap: 3rem; /* 48px */`
    _12,
    /// `column-gap: 3rem; /* 48px */`
    X12,
    /// `row-gap: 3rem; /* 48px */`
    Y12,
    /// `gap: 3.5rem; /* 56px */`
    _14,
    /// `column-gap: 3.5rem; /* 56px */`
    X14,
    /// `row-gap: 3.5rem; /* 56px */`
    Y14,
    /// `gap: 4rem; /* 64px */`
    _16,
    /// `column-gap: 4rem; /* 64px */`
    X16,
    /// `row-gap: 4rem; /* 64px */`
    Y16,
    /// `gap: 5rem; /* 80px */`
    _20,
    /// `column-gap: 5rem; /* 80px */`
    X20,
    /// `row-gap: 5rem; /* 80px */`
    Y20,
    /// `gap: 6rem; /* 96px */`
    _24,
    /// `column-gap: 6rem; /* 96px */`
    X24,
    /// `row-gap: 6rem; /* 96px */`
    Y24,
    /// `gap: 7rem; /* 112px */`
    _28,
    /// `column-gap: 7rem; /* 112px */`
    X28,
    /// `row-gap: 7rem; /* 112px */`
    Y28,
    /// `gap: 8rem; /* 128px */`
    _32,
    /// `column-gap: 8rem; /* 128px */`
    X32,
    /// `row-gap: 8rem; /* 128px */`
    Y32,
    /// `gap: 9rem; /* 144px */`
    _36,
    /// `column-gap: 9rem; /* 144px */`
    X36,
    /// `row-gap: 9rem; /* 144px */`
    Y36,
    /// `gap: 10rem; /* 160px */`
    _40,
    /// `column-gap: 10rem; /* 160px */`
    X40,
    /// `row-gap: 10rem; /* 160px */`
    Y40,
    /// `gap: 11rem; /* 176px */`
    _44,
    /// `column-gap: 11rem; /* 176px */`
    X44,
    /// `row-gap: 11rem; /* 176px */`
    Y44,
    /// `gap: 12rem; /* 192px */`
    _48,
    /// `column-gap: 12rem; /* 192px */`
    X48,
    /// `row-gap: 12rem; /* 192px */`
    Y48,
    /// `gap: 13rem; /* 208px */`
    _52,
    /// `column-gap: 13rem; /* 208px */`
    X52,
    /// `row-gap: 13rem; /* 208px */`
    Y52,
    /// `gap: 14rem; /* 224px */`
    _56,
    /// `column-gap: 14rem; /* 224px */`
    X56,
    /// `row-gap: 14rem; /* 224px */`
    Y56,
    /// `gap: 15rem; /* 240px */`
    _60,
    /// `column-gap: 15rem; /* 240px */`
    X60,
    /// `row-gap: 15rem; /* 240px */`
    Y60,
    /// `gap: 16rem; /* 256px */`
    _64,
    /// `column-gap: 16rem; /* 256px */`
    X64,
    /// `row-gap: 16rem; /* 256px */`
    Y64,
    /// `gap: 18rem; /* 288px */`
    _72,
    /// `column-gap: 18rem; /* 288px */`
    X72,
    /// `row-gap: 18rem; /* 288px */`
    Y72,
    /// `gap: 20rem; /* 320px */`
    _80,
    /// `column-gap: 20rem; /* 320px */`
    X80,
    /// `row-gap: 20rem; /* 320px */`
    Y80,
    /// `gap: 24rem; /* 384px */`
    _96,
    /// `column-gap: 24rem; /* 384px */`
    X96,
    /// `row-gap: 24rem; /* 384px */`
    Y96,
}

/// Utilities for controlling how flex and grid items are positioned along a container's main axis.
/// 
/// <https://tailwindcss.com/docs/justify-content>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "justify")]
pub enum JustifyContent {
    /// `justify-content: normal;`
    Normal,
    /// `justify-content: flex-start;`
    Start,
    /// `justify-content: flex-end;`
    End,
    /// `justify-content: center;`
    Center,
    /// `justify-content: space-between;`
    Between,
    /// `justify-content: space-around;`
    Around,
    /// `justify-content: space-evenly;`
    Evenly,
    /// `justify-content: stretch;`
    Stretch,
}

/// Utilities for controlling how grid items are aligned along their inline axis.
/// 
/// <https://tailwindcss.com/docs/justify-items>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "justify-items")]
pub enum JustifyItems {
    /// `justify-items: start;`
    Start,
    /// `justify-items: end;`
    End,
    /// `justify-items: center;`
    Center,
    /// `justify-items: stretch;`
    Stretch,
}

/// Utilities for controlling how an individual grid item is aligned along its inline axis.
/// 
/// <https://tailwindcss.com/docs/justify-self>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "justify-self")]
pub enum JustifySelf {
    /// `justify-self: auto;`
    Auto,
    /// `justify-self: start;`
    Start,
    /// `justify-self: end;`
    End,
    /// `justify-self: center;`
    Center,
    /// `justify-self: stretch;`
    Stretch,
}

/// Utilities for controlling how rows are positioned in multi-row flex and grid containers.
/// 
/// <https://tailwindcss.com/docs/align-content>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "content")]
pub enum AlignContent {
    /// `align-content: normal;`
    Normal,
    /// `align-content: center;`
    Center,
    /// `align-content: flex-start;`
    Start,
    /// `align-content: flex-end;`
    End,
    /// `align-content: space-between;`
    Between,
    /// `align-content: space-around;`
    Around,
    /// `align-content: space-evenly;`
    Evenly,
    /// `align-content: baseline;`
    Baseline,
    /// `align-content: stretch;`
    Stretch,
}

/// Utilities for controlling how flex and grid items are positioned along a container's cross axis.
/// 
/// <https://tailwindcss.com/docs/align-items>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "items")]
pub enum AlignItems {
    /// `align-items: flex-start;`
    Start,
    /// `align-items: flex-end;`
    End,
    /// `align-items: center;`
    Center,
    /// `align-items: baseline;`
    Baseline,
    /// `align-items: stretch;`
    Stretch,
}

/// Utilities for controlling how an individual flex or grid item is positioned along its container's cross axis.
/// 
/// <https://tailwindcss.com/docs/align-self>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "self")]
pub enum AlignSelf {
    /// `align-self: auto;`
    Auto,
    /// `align-self: flex-start;`
    Start,
    /// `align-self: flex-end;`
    End,
    /// `align-self: center;`
    Center,
    /// `align-self: stretch;`
    Stretch,
    /// `align-self: baseline;`
    Baseline,
}

/// Utilities for controlling how content is justified and aligned at the same time.
/// 
/// <https://tailwindcss.com/docs/place-content>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "place-content")]
pub enum PlaceContent {
    /// `place-content: center;`
    Center,
    /// `place-content: start;`
    Start,
    /// `place-content: end;`
    End,
    /// `place-content: space-between;`
    Between,
    /// `place-content: space-around;`
    Around,
    /// `place-content: space-evenly;`
    Evenly,
    /// `place-content: baseline;`
    Baseline,
    /// `place-content: stretch;`
    Stretch,
}

/// Utilities for controlling how items are justified and aligned at the same time.
/// 
/// <https://tailwindcss.com/docs/place-items>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "place-items")]
pub enum PlaceItems {
    /// `place-items: start;`
    Start,
    /// `place-items: end;`
    End,
    /// `place-items: center;`
    Center,
    /// `place-items: baseline;`
    Baseline,
    /// `place-items: stretch;`
    Stretch,
}

/// Utilities for controlling how an individual item is justified and aligned at the same time.
/// 
/// <https://tailwindcss.com/docs/place-self>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "place-self")]
pub enum PlaceSelf {
    /// `place-self: auto;`
    Auto,
    /// `place-self: start;`
    Start,
    /// `place-self: end;`
    End,
    /// `place-self: center;`
    Center,
    /// `place-self: stretch;`
    Stretch,
}
