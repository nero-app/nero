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
    /// ```css
    /// {
    ///     flex-basis: 0px;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     flex-basis: 0.25rem; /* 4px */
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     flex-basis: 0.5rem; /* 8px */
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     flex-basis: 0.75rem; /* 12px */
    /// }
    /// ```
    _3,
    /// ```css
    /// {
    ///     flex-basis: 1rem; /* 16px */
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     flex-basis: 1.25rem; /* 20px */
    /// }
    /// ```
    _5,
    /// ```css
    /// {
    ///     flex-basis: 1.5rem; /* 24px */
    /// }
    /// ```
    _6,
    /// ```css
    /// {
    ///     flex-basis: 1.75rem; /* 28px */
    /// }
    /// ```
    _7,
    /// ```css
    /// {
    ///     flex-basis: 2rem; /* 32px */
    /// }
    /// ```
    _8,
    /// ```css
    /// {
    ///     flex-basis: 2.25rem; /* 36px */
    /// }
    /// ```
    _9,
    /// ```css
    /// {
    ///     flex-basis: 2.5rem; /* 40px */
    /// }
    /// ```
    _10,
    /// ```css
    /// {
    ///     flex-basis: 2.75rem; /* 44px */
    /// }
    /// ```
    _11,
    /// ```css
    /// {
    ///     flex-basis: 3rem; /* 48px */
    /// }
    /// ```
    _12,
    /// ```css
    /// {
    ///     flex-basis: 3.5rem; /* 56px */
    /// }
    /// ```
    _14,
    /// ```css
    /// {
    ///     flex-basis: 4rem; /* 64px */
    /// }
    /// ```
    _16,
    /// ```css
    /// {
    ///     flex-basis: 5rem; /* 80px */
    /// }
    /// ```
    _20,
    /// ```css
    /// {
    ///     flex-basis: 6rem; /* 96px */
    /// }
    /// ```
    _24,
    /// ```css
    /// {
    ///     flex-basis: 7rem; /* 112px */
    /// }
    /// ```
    _28,
    /// ```css
    /// {
    ///     flex-basis: 8rem; /* 128px */
    /// }
    /// ```
    _32,
    /// ```css
    /// {
    ///     flex-basis: 9rem; /* 144px */
    /// }
    /// ```
    _36,
    /// ```css
    /// {
    ///     flex-basis: 10rem; /* 160px */
    /// }
    /// ```
    _40,
    /// ```css
    /// {
    ///     flex-basis: 11rem; /* 176px */
    /// }
    /// ```
    _44,
    /// ```css
    /// {
    ///     flex-basis: 12rem; /* 192px */
    /// }
    /// ```
    _48,
    /// ```css
    /// {
    ///     flex-basis: 13rem; /* 208px */
    /// }
    /// ```
    _52,
    /// ```css
    /// {
    ///     flex-basis: 14rem; /* 224px */
    /// }
    /// ```
    _56,
    /// ```css
    /// {
    ///     flex-basis: 15rem; /* 240px */
    /// }
    /// ```
    _60,
    /// ```css
    /// {
    ///     flex-basis: 16rem; /* 256px */
    /// }
    /// ```
    _64,
    /// ```css
    /// {
    ///     flex-basis: 18rem; /* 288px */
    /// }
    /// ```
    _72,
    /// ```css
    /// {
    ///     flex-basis: 20rem; /* 320px */
    /// }
    /// ```
    _80,
    /// ```css
    /// {
    ///     flex-basis: 24rem; /* 384px */
    /// }
    /// ```
    _96,
    /// ```css
    /// {
    ///     flex-basis: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     flex-basis: 1px;
    /// }
    /// ```
    Px,
    /// ```css
    /// {
    ///     flex-basis: 0.125rem; /* 2px */
    /// }
    /// ```
    _0_5,
    /// ```css
    /// {
    ///     flex-basis: 0.375rem; /* 6px */
    /// }
    /// ```
    _1_5,
    /// ```css
    /// {
    ///     flex-basis: 0.625rem; /* 10px */
    /// }
    /// ```
    _2_5,
    /// ```css
    /// {
    ///     flex-basis: 0.875rem; /* 14px */
    /// }
    /// ```
    _3_5,
    /// ```css
    /// {
    ///     flex-basis: 50%;
    /// }
    /// ```
    _1div2,
    /// ```css
    /// {
    ///     flex-basis: 33.333333%;
    /// }
    /// ```
    _1div3,
    /// ```css
    /// {
    ///     flex-basis: 66.666667%;
    /// }
    /// ```
    _2div3,
    /// ```css
    /// {
    ///     flex-basis: 25%;
    /// }
    /// ```
    _1div4,
    /// ```css
    /// {
    ///     flex-basis: 50%;
    /// }
    /// ```
    _2div4,
    /// ```css
    /// {
    ///     flex-basis: 75%;
    /// }
    /// ```
    _3div4,
    /// ```css
    /// {
    ///     flex-basis: 20%;
    /// }
    /// ```
    _1div5,
    /// ```css
    /// {
    ///     flex-basis: 40%;
    /// }
    /// ```
    _2div5,
    /// ```css
    /// {
    ///     flex-basis: 60%;
    /// }
    /// ```
    _3div5,
    /// ```css
    /// {
    ///     flex-basis: 80%;
    /// }
    /// ```
    _4div5,
    /// ```css
    /// {
    ///     flex-basis: 16.666667%;
    /// }
    /// ```
    _1div6,
    /// ```css
    /// {
    ///     flex-basis: 33.333333%;
    /// }
    /// ```
    _2div6,
    /// ```css
    /// {
    ///     flex-basis: 50%;
    /// }
    /// ```
    _3div6,
    /// ```css
    /// {
    ///     flex-basis: 66.666667%;
    /// }
    /// ```
    _4div6,
    /// ```css
    /// {
    ///     flex-basis: 83.333333%;
    /// }
    /// ```
    _5div6,
    /// ```css
    /// {
    ///     flex-basis: 8.333333%;
    /// }
    /// ```
    _1div12,
    /// ```css
    /// {
    ///     flex-basis: 16.666667%;
    /// }
    /// ```
    _2div12,
    /// ```css
    /// {
    ///     flex-basis: 25%;
    /// }
    /// ```
    _3div12,
    /// ```css
    /// {
    ///     flex-basis: 33.333333%;
    /// }
    /// ```
    _4div12,
    /// ```css
    /// {
    ///     flex-basis: 41.666667%;
    /// }
    /// ```
    _5div12,
    /// ```css
    /// {
    ///     flex-basis: 50%;
    /// }
    /// ```
    _6div12,
    /// ```css
    /// {
    ///     flex-basis: 58.333333%;
    /// }
    /// ```
    _7div12,
    /// ```css
    /// {
    ///     flex-basis: 66.666667%;
    /// }
    /// ```
    _8div12,
    /// ```css
    /// {
    ///     flex-basis: 75%;
    /// }
    /// ```
    _9div12,
    /// ```css
    /// {
    ///     flex-basis: 83.333333%;
    /// }
    /// ```
    _10div12,
    /// ```css
    /// {
    ///     flex-basis: 91.666667%;
    /// }
    /// ```
    _11div12,
    /// ```css
    /// {
    ///     flex-basis: 100%;
    /// }
    /// ```
    Full,
}

/// Utilities for controlling the direction of flex items.
/// 
/// <https://tailwindcss.com/docs/flex-direction>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "flex")]
pub enum FlexDirection {
    /// ```css
    /// {
    ///     flex-direction: row;
    /// }
    /// ```
    Row,
    /// ```css
    /// {
    ///     flex-direction: row-reverse;
    /// }
    /// ```
    RowReverse,
    /// ```css
    /// {
    ///     flex-direction: column;
    /// }
    /// ```
    Col,
    /// ```css
    /// {
    ///     flex-direction: column-reverse;
    /// }
    /// ```
    ColReverse,
}

/// Utilities for controlling how flex items wrap.
/// 
/// <https://tailwindcss.com/docs/flex-wrap>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "flex")]
pub enum FlexWrap {
    /// ```css
    /// {
    ///     flex-wrap: wrap;
    /// }
    /// ```
    Wrap,
    /// ```css
    /// {
    ///     flex-wrap: wrap-reverse;
    /// }
    /// ```
    WrapReverse,
    /// ```css
    /// {
    ///     flex-wrap: nowrap;
    /// }
    /// ```
    Nowrap,
}

/// Utilities for controlling how flex items both grow and shrink.
/// 
/// <https://tailwindcss.com/docs/flex>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "flex")]
pub enum Flex {
    /// ```css
    /// {
    ///     flex: 1 1 0%;
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     flex: 1 1 auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     flex: 0 1 auto;
    /// }
    /// ```
    Initial,
    /// ```css
    /// {
    ///     flex: none;
    /// }
    /// ```
    None,
}

/// Utilities for controlling how flex items grow.
/// 
/// <https://tailwindcss.com/docs/flex-grow>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum FlexGrow {
    /// ```css
    /// {
    ///     flex-grow: 1;
    /// }
    /// ```
    Grow,
    /// ```css
    /// {
    ///     flex-grow: 0;
    /// }
    /// ```
    Grow0,
}

/// Utilities for controlling how flex items shrink.
///
/// <https://tailwindcss.com/docs/flex-shrink>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum FlexShrink {
    /// ```css
    /// {
    ///     flex-shrink: 1;
    /// }
    /// ```
    Shrink,
    /// ```css
    /// {
    ///     flex-shrink: 0;
    /// }
    /// ```
    Shrink0,
}

/// Utilities for controlling the order of flex and grid items.
/// 
/// <https://tailwindcss.com/docs/order>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "order")]
pub enum Order {
    /// ```css
    /// {
    ///     order: 1;
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     order: 2;
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     order: 3;
    /// }
    /// ```
    _3,
    /// ```css
    /// {
    ///     order: 4;
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     order: 5;
    /// }
    /// ```
    _5,
    /// ```css
    /// {
    ///     order: 6;
    /// }
    /// ```
    _6,
    /// ```css
    /// {
    ///     order: 7;
    /// }
    /// ```
    _7,
    /// ```css
    /// {
    ///     order: 8;
    /// }
    /// ```
    _8,
    /// ```css
    /// {
    ///     order: 9;
    /// }
    /// ```
    _9,
    /// ```css
    /// {
    ///     order: 10;
    /// }
    /// ```
    _10,
    /// ```css
    /// {
    ///     order: 11;
    /// }
    /// ```
    _11,
    /// ```css
    /// {
    ///     order: 12;
    /// }
    /// ```
    _12,
    /// ```css
    /// {
    ///     order: -9999;
    /// }
    /// ```
    First,
    /// ```css
    /// {
    ///     order: 9999;
    /// }
    /// ```
    Last,
    /// ```css
    /// {
    ///     order: 0;
    /// }
    /// ```
    None,
}

/// Utilities for specifying the columns in a grid layout.
/// 
/// <https://tailwindcss.com/docs/grid-template-columns>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "grid-cols")]
pub enum GridTemplateColumns {
    /// ```css
    /// {
    ///     grid-template-columns: repeat(1, minmax(0, 1fr));
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     grid-template-columns: repeat(2, minmax(0, 1fr));
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     grid-template-columns: repeat(3, minmax(0, 1fr));
    /// }
    /// ```
    _3,
    /// ```css
    /// {
    ///     grid-template-columns: repeat(4, minmax(0, 1fr));
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     grid-template-columns: repeat(5, minmax(0, 1fr));
    /// }
    /// ```
    _5,
    /// ```css
    /// {
    ///     grid-template-columns: repeat(6, minmax(0, 1fr));
    /// }
    /// ```
    _6,
    /// ```css
    /// {
    ///     grid-template-columns: repeat(7, minmax(0, 1fr));
    /// }
    /// ```
    _7,
    /// ```css
    /// {
    ///     grid-template-columns: repeat(8, minmax(0, 1fr));
    /// }
    /// ```
    _8,
    /// ```css
    /// {
    ///     grid-template-columns: repeat(9, minmax(0, 1fr));
    /// }
    /// ```
    _9,
    /// ```css
    /// {
    ///     grid-template-columns: repeat(10, minmax(0, 1fr));
    /// }
    /// ```
    _10,
    /// ```css
    /// {
    ///     grid-template-columns: repeat(11, minmax(0, 1fr));
    /// }
    /// ```
    _11,
    /// ```css
    /// {
    ///     grid-template-columns: repeat(12, minmax(0, 1fr));
    /// }
    /// ```
    _12,
    /// ```css
    /// {
    ///     grid-template-columns: none;
    /// }
    /// ```
    None,
    /// ```css
    /// {
    ///     grid-template-columns: subgrid;
    /// }
    /// ```
    Subgrid,
}

/// Utilities for controlling how elements are sized and placed across grid columns.
/// 
/// <https://tailwindcss.com/docs/grid-column>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "col")]
pub enum GridColumnStartEnd {
    /// ```css
    /// {
    ///     grid-column: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     grid-column: span 1 / span 1;
    /// }
    /// ```
    Span1,
    /// ```css
    /// {
    ///     grid-column: span 2 / span 2;
    /// }
    /// ```
    Span2,
    /// ```css
    /// {
    ///     grid-column: span 3 / span 3;
    /// }
    /// ```
    Span3,
    /// ```css
    /// {
    ///     grid-column: span 4 / span 4;
    /// }
    /// ```
    Span4,
    /// ```css
    /// {
    ///     grid-column: span 5 / span 5;
    /// }
    /// ```
    Span5,
    /// ```css
    /// {
    ///     grid-column: span 6 / span 6;
    /// }
    /// ```
    Span6,
    /// ```css
    /// {
    ///     grid-column: span 7 / span 7;
    /// }
    /// ```
    Span7,
    /// ```css
    /// {
    ///     grid-column: span 8 / span 8;
    /// }
    /// ```
    Span8,
    /// ```css
    /// {
    ///     grid-column: span 9 / span 9;
    /// }
    /// ```
    Span9,
    /// ```css
    /// {
    ///     grid-column: span 10 / span 10;
    /// }
    /// ```
    Span10,
    /// ```css
    /// {
    ///     grid-column: span 11 / span 11;
    /// }
    /// ```
    Span11,
    /// ```css
    /// {
    ///     grid-column: span 12 / span 12;
    /// }
    /// ```
    Span12,
    /// ```css
    /// {
    ///     grid-column: 1 / -1;
    /// }
    /// ```
    SpanFull,
    /// ```css
    /// {
    ///     grid-column-start: 1;
    /// }
    /// ```
    Start1,
    /// ```css
    /// {
    ///     grid-column-start: 2;
    /// }
    /// ```
    Start2,
    /// ```css
    /// {
    ///     grid-column-start: 3;
    /// }
    /// ```
    Start3,
    /// ```css
    /// {
    ///     grid-column-start: 4;
    /// }
    /// ```
    Start4,
    /// ```css
    /// {
    ///     grid-column-start: 5;
    /// }
    /// ```
    Start5,
    /// ```css
    /// {
    ///     grid-column-start: 6;
    /// }
    /// ```
    Start6,
    /// ```css
    /// {
    ///     grid-column-start: 7;
    /// }
    /// ```
    Start7,
    /// ```css
    /// {
    ///     grid-column-start: 8;
    /// }
    /// ```
    Start8,
    /// ```css
    /// {
    ///     grid-column-start: 9;
    /// }
    /// ```
    Start9,
    /// ```css
    /// {
    ///     grid-column-start: 10;
    /// }
    /// ```
    Start10,
    /// ```css
    /// {
    ///     grid-column-start: 11;
    /// }
    /// ```
    Start11,
    /// ```css
    /// {
    ///     grid-column-start: 12;
    /// }
    /// ```
    Start12,
    /// ```css
    /// {
    ///     grid-column-start: 13;
    /// }
    /// ```
    Start13,
    /// ```css
    /// {
    ///     grid-column-start: auto;
    /// }
    /// ```
    StartAuto,
    /// ```css
    /// {
    ///     grid-column-end: 1;
    /// }
    /// ```
    End1,
    /// ```css
    /// {
    ///     grid-column-end: 2;
    /// }
    /// ```
    End2,
    /// ```css
    /// {
    ///     grid-column-end: 3;
    /// }
    /// ```
    End3,
    /// ```css
    /// {
    ///     grid-column-end: 4;
    /// }
    /// ```
    End4,
    /// ```css
    /// {
    ///     grid-column-end: 5;
    /// }
    /// ```
    End5,
    /// ```css
    /// {
    ///     grid-column-end: 6;
    /// }
    /// ```
    End6,
    /// ```css
    /// {
    ///     grid-column-end: 7;
    /// }
    /// ```
    End7,
    /// ```css
    /// {
    ///     grid-column-end: 8;
    /// }
    /// ```
    End8,
    /// ```css
    /// {
    ///     grid-column-end: 9;
    /// }
    /// ```
    End9,
    /// ```css
    /// {
    ///     grid-column-end: 10;
    /// }
    /// ```
    End10,
    /// ```css
    /// {
    ///     grid-column-end: 11;
    /// }
    /// ```
    End11,
    /// ```css
    /// {
    ///     grid-column-end: 12;
    /// }
    /// ```
    End12,
    /// ```css
    /// {
    ///     grid-column-end: 13;
    /// }
    /// ```
    End13,
    /// ```css
    /// {
    ///     grid-column-end: auto;
    /// }
    /// ```
    EndAuto,
}

/// Utilities for specifying the rows in a grid layout.
/// 
/// <https://tailwindcss.com/docs/grid-template-rows>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "grid-rows")]
pub enum GridTemplateRows {
    /// ```css
    /// {
    ///     grid-template-rows: repeat(1, minmax(0, 1fr));
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     grid-template-rows: repeat(2, minmax(0, 1fr));
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     grid-template-rows: repeat(3, minmax(0, 1fr));
    /// }
    /// ```
    _3,
    /// ```css
    /// {
    ///     grid-template-rows: repeat(4, minmax(0, 1fr));
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     grid-template-rows: repeat(5, minmax(0, 1fr));
    /// }
    /// ```
    _5,
    /// ```css
    /// {
    ///     grid-template-rows: repeat(6, minmax(0, 1fr));
    /// }
    /// ```
    _6,
    /// ```css
    /// {
    ///     grid-template-rows: repeat(7, minmax(0, 1fr));
    /// }
    /// ```
    _7,
    /// ```css
    /// {
    ///     grid-template-rows: repeat(8, minmax(0, 1fr));
    /// }
    /// ```
    _8,
    /// ```css
    /// {
    ///     grid-template-rows: repeat(9, minmax(0, 1fr));
    /// }
    /// ```
    _9,
    /// ```css
    /// {
    ///     grid-template-rows: repeat(10, minmax(0, 1fr));
    /// }
    /// ```
    _10,
    /// ```css
    /// {
    ///     grid-template-rows: repeat(11, minmax(0, 1fr));
    /// }
    /// ```
    _11,
    /// ```css
    /// {
    ///     grid-template-rows: repeat(12, minmax(0, 1fr));
    /// }
    /// ```
    _12,
    /// ```css
    /// {
    ///     grid-template-rows: none;
    /// }
    /// ```
    None,
    /// ```css
    /// {
    ///     grid-template-rows: subgrid;
    /// }
    /// ```
    Subgrid,
}

/// Utilities for controlling how elements are sized and placed across grid rows.
/// 
/// <https://tailwindcss.com/docs/grid-row>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "row")]
pub enum GridRowStartEnd {
    /// ```css
    /// {
    ///     grid-row: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     grid-row: span 1 / span 1;
    /// }
    /// ```
    Span1,
    /// ```css
    /// {
    ///     grid-row: span 2 / span 2;
    /// }
    /// ```
    Span2,
    /// ```css
    /// {
    ///     grid-row: span 3 / span 3;
    /// }
    /// ```
    Span3,
    /// ```css
    /// {
    ///     grid-row: span 4 / span 4;
    /// }
    /// ```
    Span4,
    /// ```css
    /// {
    ///     grid-row: span 5 / span 5;
    /// }
    /// ```
    Span5,
    /// ```css
    /// {
    ///     grid-row: span 6 / span 6;
    /// }
    /// ```
    Span6,
    /// ```css
    /// {
    ///     grid-row: span 7 / span 7;
    /// }
    /// ```
    Span7,
    /// ```css
    /// {
    ///     grid-row: span 8 / span 8;
    /// }
    /// ```
    Span8,
    /// ```css
    /// {
    ///     grid-row: span 9 / span 9;
    /// }
    /// ```
    Span9,
    /// ```css
    /// {
    ///     grid-row: span 10 / span 10;
    /// }
    /// ```
    Span10,
    /// ```css
    /// {
    ///     grid-row: span 11 / span 11;
    /// }
    /// ```
    Span11,
    /// ```css
    /// {
    ///     grid-row: span 12 / span 12;
    /// }
    /// ```
    Span12,
    /// ```css
    /// {
    ///     grid-row: 1 / -1;
    /// }
    /// ```
    SpanFull,
    /// ```css
    /// {
    ///     grid-row-start: 1;
    /// }
    /// ```
    Start1,
    /// ```css
    /// {
    ///     grid-row-start: 2;
    /// }
    /// ```
    Start2,
    /// ```css
    /// {
    ///     grid-row-start: 3;
    /// }
    /// ```
    Start3,
    /// ```css
    /// {
    ///     grid-row-start: 4;
    /// }
    /// ```
    Start4,
    /// ```css
    /// {
    ///     grid-row-start: 5;
    /// }
    /// ```
    Start5,
    /// ```css
    /// {
    ///     grid-row-start: 6;
    /// }
    /// ```
    Start6,
    /// ```css
    /// {
    ///     grid-row-start: 7;
    /// }
    /// ```
    Start7,
    /// ```css
    /// {
    ///     grid-row-start: 8;
    /// }
    /// ```
    Start8,
    /// ```css
    /// {
    ///     grid-row-start: 9;
    /// }
    /// ```
    Start9,
    /// ```css
    /// {
    ///     grid-row-start: 10;
    /// }
    /// ```
    Start10,
    /// ```css
    /// {
    ///     grid-row-start: 11;
    /// }
    /// ```
    Start11,
    /// ```css
    /// {
    ///     grid-row-start: 12;
    /// }
    /// ```
    Start12,
    /// ```css
    /// {
    ///     grid-row-start: 13;
    /// }
    /// ```
    Start13,
    /// ```css
    /// {
    ///     grid-row-start: auto;
    /// }
    /// ```
    StartAuto,
    /// ```css
    /// {
    ///     grid-row-end: 1;
    /// }
    /// ```
    End1,
    /// ```css
    /// {
    ///     grid-row-end: 2;
    /// }
    /// ```
    End2,
    /// ```css
    /// {
    ///     grid-row-end: 3;
    /// }
    /// ```
    End3,
    /// ```css
    /// {
    ///     grid-row-end: 4;
    /// }
    /// ```
    End4,
    /// ```css
    /// {
    ///     grid-row-end: 5;
    /// }
    /// ```
    End5,
    /// ```css
    /// {
    ///     grid-row-end: 6;
    /// }
    /// ```
    End6,
    /// ```css
    /// {
    ///     grid-row-end: 7;
    /// }
    /// ```
    End7,
    /// ```css
    /// {
    ///     grid-row-end: 8;
    /// }
    /// ```
    End8,
    /// ```css
    /// {
    ///     grid-row-end: 9;
    /// }
    /// ```
    End9,
    /// ```css
    /// {
    ///     grid-row-end: 10;
    /// }
    /// ```
    End10,
    /// ```css
    /// {
    ///     grid-row-end: 11;
    /// }
    /// ```
    End11,
    /// ```css
    /// {
    ///     grid-row-end: 12;
    /// }
    /// ```
    End12,
    /// ```css
    /// {
    ///     grid-row-end: 13;
    /// }
    /// ```
    End13,
    /// ```css
    /// {
    ///     grid-row-end: auto;
    /// }
    /// ```
    EndAuto,
}

/// Utilities for controlling how elements in a grid are auto-placed.
///
/// <https://tailwindcss.com/docs/grid-auto-flow>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "grid-flow")]
pub enum GridAutoFlow {
    /// ```css
    /// {
    ///     grid-auto-flow: row;
    /// }
    /// ```
    Row,
    /// ```css
    /// {
    ///     grid-auto-flow: column;
    /// }
    /// ```
    Col,
    /// ```css
    /// {
    ///     grid-auto-flow: dense;
    /// }
    /// ```
    Dense,
    /// ```css
    /// {
    ///     grid-auto-flow: row dense;
    /// }
    /// ```
    RowDense,
    /// ```css
    /// {
    ///     grid-auto-flow: column dense;
    /// }
    /// ```
    ColDense,
}

/// Utilities for controlling the size of implicitly-created grid columns.
/// 
/// <https://tailwindcss.com/docs/grid-auto-columns>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "auto-cols")]
pub enum GridAutoColumns {
    /// ```css
    /// {
    ///     grid-auto-columns: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     grid-auto-columns: min-content;
    /// }
    /// ```
    Min,
    /// ```css
    /// {
    ///     grid-auto-columns: max-content;
    /// }
    /// ```
    Max,
    /// ```css
    /// {
    ///     grid-auto-columns: minmax(0, 1fr);
    /// }
    /// ```
    Fr,
}

/// Utilities for controlling the size of implicitly-created grid rows.
/// 
/// <https://tailwindcss.com/docs/grid-auto-rows>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "auto-rows")]
pub enum GridAutoRows {
    /// ```css
    /// {
    ///     grid-auto-rows: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     grid-auto-rows: min-content;
    /// }
    /// ```
    Min,
    /// ```css
    /// {
    ///     grid-auto-rows: max-content;
    /// }
    /// ```
    Max,
    /// ```css
    /// {
    ///     grid-auto-rows: minmax(0, 1fr);
    /// }
    /// ```
    Fr,
}

/// Utilities for controlling gutters between grid and flexbox items.
/// 
/// <https://tailwindcss.com/docs/gap>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "gap", replace(from = "_", to = "."))]
pub enum Gap {
    /// ```css
    /// {
    ///     gap: 0px;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     column-gap: 0px;
    /// }
    /// ```
    X0,
    /// ```css
    /// {
    ///     row-gap: 0px;
    /// }
    /// ```
    Y0,
    /// ```css
    /// {
    ///     gap: 1px;
    /// }
    /// ```
    Px,
    /// ```css
    /// {
    ///     column-gap: 1px;
    /// }
    /// ```
    XPx,
    /// ```css
    /// {
    ///     row-gap: 1px;
    /// }
    /// ```
    YPx,
    /// ```css
    /// {
    ///     gap: 0.125rem; /* 2px */
    /// }
    /// ```
    _0_5,
    /// ```css
    /// {
    ///     column-gap: 0.125rem; /* 2px */
    /// }
    /// ```
    X0_5,
    /// ```css
    /// {
    ///     row-gap: 0.125rem; /* 2px */
    /// }
    /// ```
    Y0_5,
    /// ```css
    /// {
    ///     gap: 0.25rem; /* 4px */
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     column-gap: 0.25rem; /* 4px */
    /// }
    /// ```
    X1,
    /// ```css
    /// {
    ///     row-gap: 0.25rem; /* 4px */
    /// }
    /// ```
    Y1,
    /// ```css
    /// {
    ///     gap: 0.375rem; /* 6px */
    /// }
    /// ```
    _1_5,
    /// ```css
    /// {
    ///     column-gap: 0.375rem; /* 6px */
    /// }
    /// ```
    X1_5,
    /// ```css
    /// {
    ///     row-gap: 0.375rem; /* 6px */
    /// }
    /// ```
    Y1_5,
    /// ```css
    /// {
    ///     gap: 0.5rem; /* 8px */
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     column-gap: 0.5rem; /* 8px */
    /// }
    /// ```
    X2,
    /// ```css
    /// {
    ///     row-gap: 0.5rem; /* 8px */
    /// }
    /// ```
    Y2,
    /// ```css
    /// {
    ///     gap: 0.625rem; /* 10px */
    /// }
    /// ```
    _2_5,
    /// ```css
    /// {
    ///     column-gap: 0.625rem; /* 10px */
    /// }
    /// ```
    X2_5,
    /// ```css
    /// {
    ///     row-gap: 0.625rem; /* 10px */
    /// }
    /// ```
    Y2_5,
    /// ```css
    /// {
    ///     gap: 0.75rem; /* 12px */
    /// }
    /// ```
    _3,
    /// ```css
    /// {
    ///     column-gap: 0.75rem; /* 12px */
    /// }
    /// ```
    X3,
    /// ```css
    /// {
    ///     row-gap: 0.75rem; /* 12px */
    /// }
    /// ```
    Y3,
    /// ```css
    /// {
    ///     gap: 0.875rem; /* 14px */
    /// }
    /// ```
    _3_5,
    /// ```css
    /// {
    ///     column-gap: 0.875rem; /* 14px */
    /// }
    /// ```
    X3_5,
    /// ```css
    /// {
    ///     row-gap: 0.875rem; /* 14px */
    /// }
    /// ```
    Y3_5,
    /// ```css
    /// {
    ///     gap: 1rem; /* 16px */
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     column-gap: 1rem; /* 16px */
    /// }
    /// ```
    X4,
    /// ```css
    /// {
    ///     row-gap: 1rem; /* 16px */
    /// }
    /// ```
    Y4,
    /// ```css
    /// {
    ///     gap: 1.25rem; /* 20px */
    /// }
    /// ```
    _5,
    /// ```css
    /// {
    ///     column-gap: 1.25rem; /* 20px */
    /// }
    /// ```
    X5,
    /// ```css
    /// {
    ///     row-gap: 1.25rem; /* 20px */
    /// }
    /// ```
    Y5,
    /// ```css
    /// {
    ///     gap: 1.5rem; /* 24px */
    /// }
    /// ```
    _6,
    /// ```css
    /// {
    ///     column-gap: 1.5rem; /* 24px */
    /// }
    /// ```
    X6,
    /// ```css
    /// {
    ///     row-gap: 1.5rem; /* 24px */
    /// }
    /// ```
    Y6,
    /// ```css
    /// {
    ///     gap: 1.75rem; /* 28px */
    /// }
    /// ```
    _7,
    /// ```css
    /// {
    ///     column-gap: 1.75rem; /* 28px */
    /// }
    /// ```
    X7,
    /// ```css
    /// {
    ///     row-gap: 1.75rem; /* 28px */
    /// }
    /// ```
    Y7,
    /// ```css
    /// {
    ///     gap: 2rem; /* 32px */
    /// }
    /// ```
    _8,
    /// ```css
    /// {
    ///     column-gap: 2rem; /* 32px */
    /// }
    /// ```
    X8,
    /// ```css
    /// {
    ///     row-gap: 2rem; /* 32px */
    /// }
    /// ```
    Y8,
    /// ```css
    /// {
    ///     gap: 2.25rem; /* 36px */
    /// }
    /// ```
    _9,
    /// ```css
    /// {
    ///     column-gap: 2.25rem; /* 36px */
    /// }
    /// ```
    X9,
    /// ```css
    /// {
    ///     row-gap: 2.25rem; /* 36px */
    /// }
    /// ```
    Y9,
    /// ```css
    /// {
    ///     gap: 2.5rem; /* 40px */
    /// }
    /// ```
    _10,
    /// ```css
    /// {
    ///     column-gap: 2.5rem; /* 40px */
    /// }
    /// ```
    X10,
    /// ```css
    /// {
    ///     row-gap: 2.5rem; /* 40px */
    /// }
    /// ```
    Y10,
    /// ```css
    /// {
    ///     gap: 2.75rem; /* 44px */
    /// }
    /// ```
    _11,
    /// ```css
    /// {
    ///     column-gap: 2.75rem; /* 44px */
    /// }
    /// ```
    X11,
    /// ```css
    /// {
    ///     row-gap: 2.75rem; /* 44px */
    /// }
    /// ```
    Y11,
    /// ```css
    /// {
    ///     gap: 3rem; /* 48px */
    /// }
    /// ```
    _12,
    /// ```css
    /// {
    ///     column-gap: 3rem; /* 48px */
    /// }
    /// ```
    X12,
    /// ```css
    /// {
    ///     row-gap: 3rem; /* 48px */
    /// }
    /// ```
    Y12,
    /// ```css
    /// {
    ///     gap: 3.5rem; /* 56px */
    /// }
    /// ```
    _14,
    /// ```css
    /// {
    ///     column-gap: 3.5rem; /* 56px */
    /// }
    /// ```
    X14,
    /// ```css
    /// {
    ///     row-gap: 3.5rem; /* 56px */
    /// }
    /// ```
    Y14,
    /// ```css
    /// {
    ///     gap: 4rem; /* 64px */
    /// }
    /// ```
    _16,
    /// ```css
    /// {
    ///     column-gap: 4rem; /* 64px */
    /// }
    /// ```
    X16,
    /// ```css
    /// {
    ///     row-gap: 4rem; /* 64px */
    /// }
    /// ```
    Y16,
    /// ```css
    /// {
    ///     gap: 5rem; /* 80px */
    /// }
    /// ```
    _20,
    /// ```css
    /// {
    ///     column-gap: 5rem; /* 80px */
    /// }
    /// ```
    X20,
    /// ```css
    /// {
    ///     row-gap: 5rem; /* 80px */
    /// }
    /// ```
    Y20,
    /// ```css
    /// {
    ///     gap: 6rem; /* 96px */
    /// }
    /// ```
    _24,
    /// ```css
    /// {
    ///     column-gap: 6rem; /* 96px */
    /// }
    /// ```
    X24,
    /// ```css
    /// {
    ///     row-gap: 6rem; /* 96px */
    /// }
    /// ```
    Y24,
    /// ```css
    /// {
    ///     gap: 7rem; /* 112px */
    /// }
    /// ```
    _28,
    /// ```css
    /// {
    ///     column-gap: 7rem; /* 112px */
    /// }
    /// ```
    X28,
    /// ```css
    /// {
    ///     row-gap: 7rem; /* 112px */
    /// }
    /// ```
    Y28,
    /// ```css
    /// {
    ///     gap: 8rem; /* 128px */
    /// }
    /// ```
    _32,
    /// ```css
    /// {
    ///     column-gap: 8rem; /* 128px */
    /// }
    /// ```
    X32,
    /// ```css
    /// {
    ///     row-gap: 8rem; /* 128px */
    /// }
    /// ```
    Y32,
    /// ```css
    /// {
    ///     gap: 9rem; /* 144px */
    /// }
    /// ```
    _36,
    /// ```css
    /// {
    ///     column-gap: 9rem; /* 144px */
    /// }
    /// ```
    X36,
    /// ```css
    /// {
    ///     row-gap: 9rem; /* 144px */
    /// }
    /// ```
    Y36,
    /// ```css
    /// {
    ///     gap: 10rem; /* 160px */
    /// }
    /// ```
    _40,
    /// ```css
    /// {
    ///     column-gap: 10rem; /* 160px */
    /// }
    /// ```
    X40,
    /// ```css
    /// {
    ///     row-gap: 10rem; /* 160px */
    /// }
    /// ```
    Y40,
    /// ```css
    /// {
    ///     gap: 11rem; /* 176px */
    /// }
    /// ```
    _44,
    /// ```css
    /// {
    ///     column-gap: 11rem; /* 176px */
    /// }
    /// ```
    X44,
    /// ```css
    /// {
    ///     row-gap: 11rem; /* 176px */
    /// }
    /// ```
    Y44,
    /// ```css
    /// {
    ///     gap: 12rem; /* 192px */
    /// }
    /// ```
    _48,
    /// ```css
    /// {
    ///     column-gap: 12rem; /* 192px */
    /// }
    /// ```
    X48,
    /// ```css
    /// {
    ///     row-gap: 12rem; /* 192px */
    /// }
    /// ```
    Y48,
    /// ```css
    /// {
    ///     gap: 13rem; /* 208px */
    /// }
    /// ```
    _52,
    /// ```css
    /// {
    ///     column-gap: 13rem; /* 208px */
    /// }
    /// ```
    X52,
    /// ```css
    /// {
    ///     row-gap: 13rem; /* 208px */
    /// }
    /// ```
    Y52,
    /// ```css
    /// {
    ///     gap: 14rem; /* 224px */
    /// }
    /// ```
    _56,
    /// ```css
    /// {
    ///     column-gap: 14rem; /* 224px */
    /// }
    /// ```
    X56,
    /// ```css
    /// {
    ///     row-gap: 14rem; /* 224px */
    /// }
    /// ```
    Y56,
    /// ```css
    /// {
    ///     gap: 15rem; /* 240px */
    /// }
    /// ```
    _60,
    /// ```css
    /// {
    ///     column-gap: 15rem; /* 240px */
    /// }
    /// ```
    X60,
    /// ```css
    /// {
    ///     row-gap: 15rem; /* 240px */
    /// }
    /// ```
    Y60,
    /// ```css
    /// {
    ///     gap: 16rem; /* 256px */
    /// }
    /// ```
    _64,
    /// ```css
    /// {
    ///     column-gap: 16rem; /* 256px */
    /// }
    /// ```
    X64,
    /// ```css
    /// {
    ///     row-gap: 16rem; /* 256px */
    /// }
    /// ```
    Y64,
    /// ```css
    /// {
    ///     gap: 18rem; /* 288px */
    /// }
    /// ```
    _72,
    /// ```css
    /// {
    ///     column-gap: 18rem; /* 288px */
    /// }
    /// ```
    X72,
    /// ```css
    /// {
    ///     row-gap: 18rem; /* 288px */
    /// }
    /// ```
    Y72,
    /// ```css
    /// {
    ///     gap: 20rem; /* 320px */
    /// }
    /// ```
    _80,
    /// ```css
    /// {
    ///     column-gap: 20rem; /* 320px */
    /// }
    /// ```
    X80,
    /// ```css
    /// {
    ///     row-gap: 20rem; /* 320px */
    /// }
    /// ```
    Y80,
    /// ```css
    /// {
    ///     gap: 24rem; /* 384px */
    /// }
    /// ```
    _96,
    /// ```css
    /// {
    ///     column-gap: 24rem; /* 384px */
    /// }
    /// ```
    X96,
    /// ```css
    /// {
    ///     row-gap: 24rem; /* 384px */
    /// }
    /// ```
    Y96,
}

/// Utilities for controlling how flex and grid items are positioned along a container's main axis.
/// 
/// <https://tailwindcss.com/docs/justify-content>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "justify")]
pub enum JustifyContent {
    /// ```css
    /// {
    ///     justify-content: normal;
    /// }
    /// ```
    Normal,
    /// ```css
    /// {
    ///     justify-content: flex-start;
    /// }
    /// ```
    Start,
    /// ```css
    /// {
    ///     justify-content: flex-end;
    /// }
    /// ```
    End,
    /// ```css
    /// {
    ///     justify-content: center;
    /// }
    /// ```
    Center,
    /// ```css
    /// {
    ///     justify-content: space-between;
    /// }
    /// ```
    Between,
    /// ```css
    /// {
    ///     justify-content: space-around;
    /// }
    /// ```
    Around,
    /// ```css
    /// {
    ///     justify-content: space-evenly;
    /// }
    /// ```
    Evenly,
    /// ```css
    /// {
    ///     justify-content: stretch;
    /// }
    /// ```
    Stretch,
}

/// Utilities for controlling how grid items are aligned along their inline axis.
/// 
/// <https://tailwindcss.com/docs/justify-items>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "justify-items")]
pub enum JustifyItems {
    /// ```css
    /// {
    ///     justify-items: start;
    /// }
    /// ```
    Start,
    /// ```css
    /// {
    ///     justify-items: end;
    /// }
    /// ```
    End,
    /// ```css
    /// {
    ///     justify-items: center;
    /// }
    /// ```
    Center,
    /// ```css
    /// {
    ///     justify-items: stretch;
    /// }
    /// ```
    Stretch,
}

/// Utilities for controlling how an individual grid item is aligned along its inline axis.
/// 
/// <https://tailwindcss.com/docs/justify-self>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "justify-self")]
pub enum JustifySelf {
    /// ```css
    /// {
    ///     justify-self: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     justify-self: start;
    /// }
    /// ```
    Start,
    /// ```css
    /// {
    ///     justify-self: end;
    /// }
    /// ```
    End,
    /// ```css
    /// {
    ///     justify-self: center;
    /// }
    /// ```
    Center,
    /// ```css
    /// {
    ///     justify-self: stretch;
    /// }
    /// ```
    Stretch,
}

/// Utilities for controlling how rows are positioned in multi-row flex and grid containers.
/// 
/// <https://tailwindcss.com/docs/align-content>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "content")]
pub enum AlignContent {
    /// ```css
    /// {
    ///     align-content: normal;
    /// }
    /// ```
    Normal,
    /// ```css
    /// {
    ///     align-content: center;
    /// }
    /// ```
    Center,
    /// ```css
    /// {
    ///     align-content: flex-start;
    /// }
    /// ```
    Start,
    /// ```css
    /// {
    ///     align-content: flex-end;
    /// }
    /// ```
    End,
    /// ```css
    /// {
    ///     align-content: space-between;
    /// }
    /// ```
    Between,
    /// ```css
    /// {
    ///     align-content: space-around;
    /// }
    /// ```
    Around,
    /// ```css
    /// {
    ///     align-content: space-evenly;
    /// }
    /// ```
    Evenly,
    /// ```css
    /// {
    ///     align-content: baseline;
    /// }
    /// ```
    Baseline,
    /// ```css
    /// {
    ///     align-content: stretch;
    /// }
    /// ```
    Stretch,
}

/// Utilities for controlling how flex and grid items are positioned along a container's cross axis.
/// 
/// <https://tailwindcss.com/docs/align-items>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "items")]
pub enum AlignItems {
    /// ```css
    /// {
    ///     align-items: flex-start;
    /// }
    /// ```
    Start,
    /// ```css
    /// {
    ///     align-items: flex-end;
    /// }
    /// ```
    End,
    /// ```css
    /// {
    ///     align-items: center;
    /// }
    /// ```
    Center,
    /// ```css
    /// {
    ///     align-items: baseline;
    /// }
    /// ```
    Baseline,
    /// ```css
    /// {
    ///     align-items: stretch;
    /// }
    /// ```
    Stretch,
}

/// Utilities for controlling how an individual flex or grid item is positioned along its container's cross axis.
/// 
/// <https://tailwindcss.com/docs/align-self>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "self")]
pub enum AlignSelf {
    /// ```css
    /// {
    ///     align-self: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     align-self: flex-start;
    /// }
    /// ```
    Start,
    /// ```css
    /// {
    ///     align-self: flex-end;
    /// }
    /// ```
    End,
    /// ```css
    /// {
    ///     align-self: center;
    /// }
    /// ```
    Center,
    /// ```css
    /// {
    ///     align-self: stretch;
    /// }
    /// ```
    Stretch,
    /// ```css
    /// {
    ///     align-self: baseline;
    /// }
    /// ```
    Baseline,
}

/// Utilities for controlling how content is justified and aligned at the same time.
/// 
/// <https://tailwindcss.com/docs/place-content>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "place-content")]
pub enum PlaceContent {
    /// ```css
    /// {
    ///     place-content: center;
    /// }
    /// ```
    Center,
    /// ```css
    /// {
    ///     place-content: start;
    /// }
    /// ```
    Start,
    /// ```css
    /// {
    ///     place-content: end;
    /// }
    /// ```
    End,
    /// ```css
    /// {
    ///     place-content: space-between;
    /// }
    /// ```
    Between,
    /// ```css
    /// {
    ///     place-content: space-around;
    /// }
    /// ```
    Around,
    /// ```css
    /// {
    ///     place-content: space-evenly;
    /// }
    /// ```
    Evenly,
    /// ```css
    /// {
    ///     place-content: baseline;
    /// }
    /// ```
    Baseline,
    /// ```css
    /// {
    ///     place-content: stretch;
    /// }
    /// ```
    Stretch,
}

/// Utilities for controlling how items are justified and aligned at the same time.
/// 
/// <https://tailwindcss.com/docs/place-items>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "place-items")]
pub enum PlaceItems {
    /// ```css
    /// {
    ///     place-items: start;
    /// }
    /// ```
    Start,
    /// ```css
    /// {
    ///     place-items: end;
    /// }
    /// ```
    End,
    /// ```css
    /// {
    ///     place-items: center;
    /// }
    /// ```
    Center,
    /// ```css
    /// {
    ///     place-items: baseline;
    /// }
    /// ```
    Baseline,
    /// ```css
    /// {
    ///     place-items: stretch;
    /// }
    /// ```
    Stretch,
}

/// Utilities for controlling how an individual item is justified and aligned at the same time.
/// 
/// <https://tailwindcss.com/docs/place-self>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "place-self")]
pub enum PlaceSelf {
    /// ```css
    /// {
    ///     place-self: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     place-self: start;
    /// }
    /// ```
    Start,
    /// ```css
    /// {
    ///     place-self: end;
    /// }
    /// ```
    End,
    /// ```css
    /// {
    ///     place-self: center;
    /// }
    /// ```
    Center,
    /// ```css
    /// {
    ///     place-self: stretch;
    /// }
    /// ```
    Stretch,
}
