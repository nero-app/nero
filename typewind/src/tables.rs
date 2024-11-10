use typewind_macros::{Display, Parse};

tailwind_types!(BorderCollapse, BorderSpacing, TableLayout, CaptionSide);

/// Utilities for controlling whether table borders should collapse or be separated.
/// 
/// <https://tailwindcss.com/docs/border-collapse>
#[derive(Debug, Clone, PartialEq, Display, Parse)] 
#[display(prefix = "border")]
pub enum BorderCollapse {
    /// ```css
    /// {
    ///     border-collapse: collapse;
    /// }
    /// ```
    Collapse,
    /// ```css
    /// {
    ///     border-collapse: separate;
    /// }
    /// ```
    Separate,
}

/// Utilities for controlling the spacing between table borders.
/// 
/// <https://tailwindcss.com/docs/border-spacing>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "border-spacing", replace(from = "_", to = "."))]
pub enum BorderSpacing {
    /// ```css
    /// {
    ///     border-spacing: 0px 0px;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     border-spacing: 0px var(--tw-border-spacing-y);
    /// }
    /// ```
    X0,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 0px;
    /// }
    /// ```
    Y0,
    /// ```css
    /// {
    ///     border-spacing: 1px 1px;
    /// }
    /// ```
    Px,
    /// ```css
    /// {
    ///     border-spacing: 1px var(--tw-border-spacing-y);
    /// }
    /// ```
    XPx,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 1px;
    /// }
    /// ```
    YPx,
    /// ```css
    /// {
    ///     border-spacing: 0.125rem 0.125rem;
    /// }
    /// ```
    _0_5,
    /// ```css
    /// {
    ///     border-spacing: 0.125rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X0_5,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 0.125rem;
    /// }
    /// ```
    Y0_5,
    /// ```css
    /// {
    ///     border-spacing: 0.25rem 0.25rem;
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     border-spacing: 0.25rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X1,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 0.25rem;
    /// }
    /// ```
    Y1,
    /// ```css
    /// {
    ///     border-spacing: 0.375rem 0.375rem;
    /// }
    /// ```
    _1_5,
    /// ```css
    /// {
    ///     border-spacing: 0.375rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X1_5,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 0.375rem;
    /// }
    /// ```
    Y1_5,
    /// ```css
    /// {
    ///     border-spacing: 0.5rem 0.5rem;
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     border-spacing: 0.5rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X2,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 0.5rem;
    /// }
    /// ```
    Y2,
    /// ```css
    /// {
    ///     border-spacing: 0.625rem 0.625rem;
    /// }
    /// ```
    _2_5,
    /// ```css
    /// {
    ///     border-spacing: 0.625rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X2_5,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 0.625rem;
    /// }
    /// ```
    Y2_5,
    /// ```css
    /// {
    ///     border-spacing: 0.75rem 0.75rem;
    /// }
    /// ```
    _3,
    /// ```css
    /// {
    ///     border-spacing: 0.75rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X3,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 0.75rem;
    /// }
    /// ```
    Y3,
    /// ```css
    /// {
    ///     border-spacing: 0.875rem 0.875rem;
    /// }
    /// ```
    _3_5,
    /// ```css
    /// {
    ///     border-spacing: 0.875rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X3_5,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 0.875rem;
    /// }
    /// ```
    Y3_5,
    /// ```css
    /// {
    ///     border-spacing: 1rem 1rem;
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     border-spacing: 1rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X4,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 1rem;
    /// }
    /// ```
    Y4,
    /// ```css
    /// {
    ///     border-spacing: 1.25rem 1.25rem;
    /// }
    /// ```
    _5,
    /// ```css
    /// {
    ///     border-spacing: 1.25rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X5,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 1.25rem;
    /// }
    /// ```
    Y5,
    /// ```css
    /// {
    ///     border-spacing: 1.5rem 1.5rem;
    /// }
    /// ```
    _6,
    /// ```css
    /// {
    ///     border-spacing: 1.5rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X6,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 1.5rem;
    /// }
    /// ```
    Y6,
    /// ```css
    /// {
    ///     border-spacing: 1.75rem 1.75rem;
    /// }
    /// ```
    _7,
    /// ```css
    /// {
    ///     border-spacing: 1.75rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X7,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 1.75rem;
    /// }
    /// ```
    Y7,
    /// ```css
    /// {
    ///     border-spacing: 2rem 2rem;
    /// }
    /// ```
    _8,
    /// ```css
    /// {
    ///     border-spacing: 2rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X8,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 2rem;
    /// }
    /// ```
    Y8,
    /// ```css
    /// {
    ///     border-spacing: 2.25rem 2.25rem;
    /// }
    /// ```
    _9,
    /// ```css
    /// {
    ///     border-spacing: 2.25rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X9,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 2.25rem;
    /// }
    /// ```
    Y9,
    /// ```css
    /// {
    ///     border-spacing: 2.5rem 2.5rem;
    /// }
    /// ```
    _10,
    /// ```css
    /// {
    ///     border-spacing: 2.5rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X10,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 2.5rem;
    /// }
    /// ```
    Y10,
    /// ```css
    /// {
    ///     border-spacing: 2.75rem 2.75rem;
    /// }
    /// ```
    _11,
    /// ```css
    /// {
    ///     border-spacing: 2.75rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X11,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 2.75rem;
    /// }
    /// ```
    Y11,
    /// ```css
    /// {
    ///     border-spacing: 3rem 3rem;
    /// }
    /// ```
    _12,
    /// ```css
    /// {
    ///     border-spacing: 3rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X12,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 3rem;
    /// }
    /// ```
    Y12,
    /// ```css
    /// {
    ///     border-spacing: 3.5rem 3.5rem;
    /// }
    /// ```
    _14,
    /// ```css
    /// {
    ///     border-spacing: 3.5rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X14,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 3.5rem;
    /// }
    /// ```
    Y14,
    /// ```css
    /// {
    ///     border-spacing: 4rem 4rem;
    /// }
    /// ```
    _16,
    /// ```css
    /// {
    ///     border-spacing: 4rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X16,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 4rem;
    /// }
    /// ```
    Y16,
    /// ```css
    /// {
    ///     border-spacing: 5rem 5rem;
    /// }
    /// ```
    _20,
    /// ```css
    /// {
    ///     border-spacing: 5rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X20,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 5rem;
    /// }
    /// ```
    Y20,
    /// ```css
    /// {
    ///     border-spacing: 6rem 6rem;
    /// }
    /// ```
    _24,
    /// ```css
    /// {
    ///     border-spacing: 6rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X24,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 6rem;
    /// }
    /// ```
    Y24,
    /// ```css
    /// {
    ///     border-spacing: 7rem 7rem;
    /// }
    /// ```
    _28,
    /// ```css
    /// {
    ///     border-spacing: 7rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X28,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 7rem;
    /// }
    /// ```
    Y28,
    /// ```css
    /// {
    ///     border-spacing: 8rem 8rem;
    /// }
    /// ```
    _32,
    /// ```css
    /// {
    ///     border-spacing: 8rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X32,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 8rem;
    /// }
    /// ```
    Y32,
    /// ```css
    /// {
    ///     border-spacing: 9rem 9rem;
    /// }
    /// ```
    _36,
    /// ```css
    /// {
    ///     border-spacing: 9rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X36,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 9rem;
    /// }
    /// ```
    Y36,
    /// ```css
    /// {
    ///     border-spacing: 10rem 10rem;
    /// }
    /// ```
    _40,
    /// ```css
    /// {
    ///     border-spacing: 10rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X40,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 10rem;
    /// }
    /// ```
    Y40,
    /// ```css
    /// {
    ///     border-spacing: 11rem 11rem;
    /// }
    /// ```
    _44,
    /// ```css
    /// {
    ///     border-spacing: 11rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X44,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 11rem;
    /// }
    /// ```
    Y44,
    /// ```css
    /// {
    ///     border-spacing: 12rem 12rem;
    /// }
    /// ```
    _48,
    /// ```css
    /// {
    ///     border-spacing: 12rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X48,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 12rem;
    /// }
    /// ```
    Y48,
    /// ```css
    /// {
    ///     border-spacing: 13rem 13rem;
    /// }
    /// ```
    _52,
    /// ```css
    /// {
    ///     border-spacing: 13rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X52,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 13rem;
    /// }
    /// ```
    Y52,
    /// ```css
    /// {
    ///     border-spacing: 14rem 14rem;
    /// }
    /// ```
    _56,
    /// ```css
    /// {
    ///     border-spacing: 14rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X56,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 14rem;
    /// }
    /// ```
    Y56,
    /// ```css
    /// {
    ///     border-spacing: 15rem 15rem;
    /// }
    /// ```
    _60,
    /// ```css
    /// {
    ///     border-spacing: 15rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X60,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 15rem;
    /// }
    /// ```
    Y60,
    /// ```css
    /// {
    ///     border-spacing: 16rem 16rem;
    /// }
    /// ```
    _64,
    /// ```css
    /// {
    ///     border-spacing: 16rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X64,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 16rem;
    /// }
    /// ```
    Y64,
    /// ```css
    /// {
    ///     border-spacing: 18rem 18rem;
    /// }
    /// ```
    _72,
    /// ```css
    /// {
    ///     border-spacing: 18rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X72,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 18rem;
    /// }
    /// ```
    Y72,
    /// ```css
    /// {
    ///     border-spacing: 20rem 20rem;
    /// }
    /// ```
    _80,
    /// ```css
    /// {
    ///     border-spacing: 20rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X80,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 20rem;
    /// }
    /// ```
    Y80,
    /// ```css
    /// {
    ///     border-spacing: 24rem 24rem;
    /// }
    /// ```
    _96,
    /// ```css
    /// {
    ///     border-spacing: 24rem var(--tw-border-spacing-y);
    /// }
    /// ```
    X96,
    /// ```css
    /// {
    ///     border-spacing: var(--tw-border-spacing-x) 24rem;
    /// }
    /// ```
    Y96,
}

/// Utilities for controlling the table layout algorithm.
/// 
/// <https://tailwindcss.com/docs/table-layout>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "table")]
pub enum TableLayout {
    /// ```css
    /// {
    ///     table-layout: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     table-layout: fixed;
    /// }
    /// ```
    Fixed,
}

/// Utilities for controlling the alignment of a caption element inside of a table.
/// 
/// <https://tailwindcss.com/docs/caption-side>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "caption")]
pub enum CaptionSide {
    /// ```css
    /// {
    ///     caption-side: top;
    /// }
    /// ```
    Top,
    /// ```css
    /// {
    ///     caption-side: bottom;
    /// }
    /// ```
    Bottom,
}
