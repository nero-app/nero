use typewind_macros::{Display, Parse};

tailwind_types!(BorderCollapse, BorderSpacing, TableLayout, CaptionSide);

/// Utilities for controlling whether table borders should collapse or be separated.
/// 
/// <https://tailwindcss.com/docs/border-collapse>
#[derive(Debug, Clone, PartialEq, Display, Parse)] 
#[display(prefix = "border")]
pub enum BorderCollapse {
    /// `border-collapse: collapse;`
    Collapse,
    /// `border-collapse: separate;`
    Separate,
}

/// Utilities for controlling the spacing between table borders.
/// 
/// <https://tailwindcss.com/docs/border-spacing>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "border-spacing", replace(from = "_", to = "."))]
pub enum BorderSpacing {
    /// `border-spacing: 0px 0px;`
    _0,
    /// `border-spacing: 0px var(--tw-border-spacing-y);`
    X0,
    /// `border-spacing: var(--tw-border-spacing-x) 0px;`
    Y0,
    /// `border-spacing: 1px 1px;`
    Px,
    /// `border-spacing: 1px var(--tw-border-spacing-y);`
    XPx,
    /// `border-spacing: var(--tw-border-spacing-x) 1px;`
    YPx,
    /// `border-spacing: 0.125rem 0.125rem;`
    _0_5,
    /// `border-spacing: 0.125rem var(--tw-border-spacing-y);`
    X0_5,
    /// `border-spacing: var(--tw-border-spacing-x) 0.125rem;`
    Y0_5,
    /// `border-spacing: 0.25rem 0.25rem;`
    _1,
    /// `border-spacing: 0.25rem var(--tw-border-spacing-y);`
    X1,
    /// `border-spacing: var(--tw-border-spacing-x) 0.25rem;`
    Y1,
    /// `border-spacing: 0.375rem 0.375rem;`
    _1_5,
    /// `border-spacing: 0.375rem var(--tw-border-spacing-y);`
    X1_5,
    /// `border-spacing: var(--tw-border-spacing-x) 0.375rem;`
    Y1_5,
    /// `border-spacing: 0.5rem 0.5rem;`
    _2,
    /// `border-spacing: 0.5rem var(--tw-border-spacing-y);`
    X2,
    /// `border-spacing: var(--tw-border-spacing-x) 0.5rem;`
    Y2,
    /// `border-spacing: 0.625rem 0.625rem;`
    _2_5,
    /// `border-spacing: 0.625rem var(--tw-border-spacing-y);`
    X2_5,
    /// `border-spacing: var(--tw-border-spacing-x) 0.625rem;`
    Y2_5,
    /// `border-spacing: 0.75rem 0.75rem;`
    _3,
    /// `border-spacing: 0.75rem var(--tw-border-spacing-y);`
    X3,
    /// `border-spacing: var(--tw-border-spacing-x) 0.75rem;`
    Y3,
    /// `border-spacing: 0.875rem 0.875rem;`
    _3_5,
    /// `border-spacing: 0.875rem var(--tw-border-spacing-y);`
    X3_5,
    /// `border-spacing: var(--tw-border-spacing-x) 0.875rem;`
    Y3_5,
    /// `border-spacing: 1rem 1rem;`
    _4,
    /// `border-spacing: 1rem var(--tw-border-spacing-y);`
    X4,
    /// `border-spacing: var(--tw-border-spacing-x) 1rem;`
    Y4,
    /// `border-spacing: 1.25rem 1.25rem;`
    _5,
    /// `border-spacing: 1.25rem var(--tw-border-spacing-y);`
    X5,
    /// `border-spacing: var(--tw-border-spacing-x) 1.25rem;`
    Y5,
    /// `border-spacing: 1.5rem 1.5rem;`
    _6,
    /// `border-spacing: 1.5rem var(--tw-border-spacing-y);`
    X6,
    /// `border-spacing: var(--tw-border-spacing-x) 1.5rem;`
    Y6,
    /// `border-spacing: 1.75rem 1.75rem;`
    _7,
    /// `border-spacing: 1.75rem var(--tw-border-spacing-y);`
    X7,
    /// `border-spacing: var(--tw-border-spacing-x) 1.75rem;`
    Y7,
    /// `border-spacing: 2rem 2rem;`
    _8,
    /// `border-spacing: 2rem var(--tw-border-spacing-y);`
    X8,
    /// `border-spacing: var(--tw-border-spacing-x) 2rem;`
    Y8,
    /// `border-spacing: 2.25rem 2.25rem;`
    _9,
    /// `border-spacing: 2.25rem var(--tw-border-spacing-y);`
    X9,
    /// `border-spacing: var(--tw-border-spacing-x) 2.25rem;`
    Y9,
    /// `border-spacing: 2.5rem 2.5rem;`
    _10,
    /// `border-spacing: 2.5rem var(--tw-border-spacing-y);`
    X10,
    /// `border-spacing: var(--tw-border-spacing-x) 2.5rem;`
    Y10,
    /// `border-spacing: 2.75rem 2.75rem;`
    _11,
    /// `border-spacing: 2.75rem var(--tw-border-spacing-y);`
    X11,
    /// `border-spacing: var(--tw-border-spacing-x) 2.75rem;`
    Y11,
    /// `border-spacing: 3rem 3rem;`
    _12,
    /// `border-spacing: 3rem var(--tw-border-spacing-y);`
    X12,
    /// `border-spacing: var(--tw-border-spacing-x) 3rem;`
    Y12,
    /// `border-spacing: 3.5rem 3.5rem;`
    _14,
    /// `border-spacing: 3.5rem var(--tw-border-spacing-y);`
    X14,
    /// `border-spacing: var(--tw-border-spacing-x) 3.5rem;`
    Y14,
    /// `border-spacing: 4rem 4rem;`
    _16,
    /// `border-spacing: 4rem var(--tw-border-spacing-y);`
    X16,
    /// `border-spacing: var(--tw-border-spacing-x) 4rem;`
    Y16,
    /// `border-spacing: 5rem 5rem;`
    _20,
    /// `border-spacing: 5rem var(--tw-border-spacing-y);`
    X20,
    /// `border-spacing: var(--tw-border-spacing-x) 5rem;`
    Y20,
    /// `border-spacing: 6rem 6rem;`
    _24,
    /// `border-spacing: 6rem var(--tw-border-spacing-y);`
    X24,
    /// `border-spacing: var(--tw-border-spacing-x) 6rem;`
    Y24,
    /// `border-spacing: 7rem 7rem;`
    _28,
    /// `border-spacing: 7rem var(--tw-border-spacing-y);`
    X28,
    /// `border-spacing: var(--tw-border-spacing-x) 7rem;`
    Y28,
    /// `border-spacing: 8rem 8rem;`
    _32,
    /// `border-spacing: 8rem var(--tw-border-spacing-y);`
    X32,
    /// `border-spacing: var(--tw-border-spacing-x) 8rem;`
    Y32,
    /// `border-spacing: 9rem 9rem;`
    _36,
    /// `border-spacing: 9rem var(--tw-border-spacing-y);`
    X36,
    /// `border-spacing: var(--tw-border-spacing-x) 9rem;`
    Y36,
    /// `border-spacing: 10rem 10rem;`
    _40,
    /// `border-spacing: 10rem var(--tw-border-spacing-y);`
    X40,
    /// `border-spacing: var(--tw-border-spacing-x) 10rem;`
    Y40,
    /// `border-spacing: 11rem 11rem;`
    _44,
    /// `border-spacing: 11rem var(--tw-border-spacing-y);`
    X44,
    /// `border-spacing: var(--tw-border-spacing-x) 11rem;`
    Y44,
    /// `border-spacing: 12rem 12rem;`
    _48,
    /// `border-spacing: 12rem var(--tw-border-spacing-y);`
    X48,
    /// `border-spacing: var(--tw-border-spacing-x) 12rem;`
    Y48,
    /// `border-spacing: 13rem 13rem;`
    _52,
    /// `border-spacing: 13rem var(--tw-border-spacing-y);`
    X52,
    /// `border-spacing: var(--tw-border-spacing-x) 13rem;`
    Y52,
    /// `border-spacing: 14rem 14rem;`
    _56,
    /// `border-spacing: 14rem var(--tw-border-spacing-y);`
    X56,
    /// `border-spacing: var(--tw-border-spacing-x) 14rem;`
    Y56,
    /// `border-spacing: 15rem 15rem;`
    _60,
    /// `border-spacing: 15rem var(--tw-border-spacing-y);`
    X60,
    /// `border-spacing: var(--tw-border-spacing-x) 15rem;`
    Y60,
    /// `border-spacing: 16rem 16rem;`
    _64,
    /// `border-spacing: 16rem var(--tw-border-spacing-y);`
    X64,
    /// `border-spacing: var(--tw-border-spacing-x) 16rem;`
    Y64,
    /// `border-spacing: 18rem 18rem;`
    _72,
    /// `border-spacing: 18rem var(--tw-border-spacing-y);`
    X72,
    /// `border-spacing: var(--tw-border-spacing-x) 18rem;`
    Y72,
    /// `border-spacing: 20rem 20rem;`
    _80,
    /// `border-spacing: 20rem var(--tw-border-spacing-y);`
    X80,
    /// `border-spacing: var(--tw-border-spacing-x) 20rem;`
    Y80,
    /// `border-spacing: 24rem 24rem;`
    _96,
    /// `border-spacing: 24rem var(--tw-border-spacing-y);`
    X96,
    /// `border-spacing: var(--tw-border-spacing-x) 24rem;`
    Y96,
}

/// Utilities for controlling the table layout algorithm.
/// 
/// <https://tailwindcss.com/docs/table-layout>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "table")]
pub enum TableLayout {
    /// `table-layout: auto;`
    Auto,
    /// `table-layout: fixed;`
    Fixed,
}

/// Utilities for controlling the alignment of a caption element inside of a table.
/// 
/// <https://tailwindcss.com/docs/caption-side>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "caption")]
pub enum CaptionSide {
    /// `caption-side: top;`
    Top,
    /// `caption-side: bottom;`
    Bottom,
}
