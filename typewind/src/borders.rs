use typewind_macros::{Display, Parse};

use crate::customization::Color;

tailwind_types!(
    BorderRadius, BorderWidth, BorderStyle, DivideWidth, DivideColor, DivideStyle, 
    OutlineWidth, OutlineColor, OutlineStyle, OutlineOffset, RingWidth, RingColor, 
    RingOffsetWidth, RingOffsetColor
);

/// Utilities for controlling the border radius of an element.
/// 
/// <https://tailwindcss.com/docs/border-radius>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "rounded")]
pub enum BorderRadius {
    /// `border-radius: 0px;`
    None,
    /// `border-radius: 0.125rem; /* 2px */`
    Sm,
    /// `border-radius: 0.25rem; /* 4px */`
    #[display(no_prefix)]
    Rounded,
    /// `border-radius: 0.375rem; /* 6px */`
    Md,
    /// `border-radius: 0.5rem; /* 8px */`
    Lg,
    /// `border-radius: 0.75rem; /* 12px */`
    Xl,
    /// `border-radius: 1rem; /* 16px */`
    _2xl,
    /// `border-radius: 1.5rem; /* 24px */`
    _3xl,
    /// `border-radius: 9999px;`
    Full,
    /// `border-start-start-radius: 0px;`
    /// 
    /// `border-end-start-radius: 0px;`
    SNone,
    /// `border-start-start-radius: 0.125rem; /* 2px */`
    /// 
    /// `border-end-start-radius: 0.125rem; /* 2px */`
    SSm,
    /// `border-start-start-radius: 0.25rem; /* 4px */`
    /// 
    /// `border-end-start-radius: 0.25rem; /* 4px */`
    S,
    /// `border-start-start-radius: 0.375rem; /* 6px */`
    /// 
    /// `border-end-start-radius: 0.375rem; /* 6px */`
    SMd,
    /// `border-start-start-radius: 0.5rem; /* 8px */`
    /// 
    /// `border-end-start-radius: 0.5rem; /* 8px */`
    SLg,
    /// `border-start-start-radius: 0.75rem; /* 12px */`
    /// 
    /// `border-end-start-radius: 0.75rem; /* 12px */`
    SXl,
    /// `border-start-start-radius: 1rem; /* 16px */`
    /// 
    /// `border-end-start-radius: 1rem; /* 16px */`
    S2xl,
    /// `border-start-start-radius: 1.5rem; /* 24px */`
    /// 
    /// `border-end-start-radius: 1.5rem; /* 24px */`
    S3xl,
    /// `border-start-start-radius: 9999px;`
    /// 
    /// `border-end-start-radius: 9999px;`
    SFull,
    /// `border-start-end-radius: 0px;`
    /// 
    /// `border-end-end-radius: 0px;`
    ENone,
    /// `border-start-end-radius: 0.125rem; /* 2px */`
    /// 
    /// `border-end-end-radius: 0.125rem; /* 2px */`
    ESm,
    /// `border-start-end-radius: 0.25rem; /* 4px */`
    /// 
    /// `border-end-end-radius: 0.25rem; /* 4px */`
    E,
    /// `border-start-end-radius: 0.375rem; /* 6px */`
    /// 
    /// `border-end-end-radius: 0.375rem; /* 6px */`
    EMd,
    /// `border-start-end-radius: 0.5rem; /* 8px */`
    /// 
    /// `border-end-end-radius: 0.5rem; /* 8px */`
    ELg,
    /// `border-start-end-radius: 0.75rem; /* 12px */`
    /// 
    /// `border-end-end-radius: 0.75rem; /* 12px */`
    EXl,
    /// `border-start-end-radius: 1rem; /* 16px */`
    /// 
    /// `border-end-end-radius: 1rem; /* 16px */`
    E2xl,
    /// `border-start-end-radius: 1.5rem; /* 24px */`
    /// 
    /// `border-end-end-radius: 1.5rem; /* 24px */`
    E3xl,
    /// `border-start-end-radius: 9999px;`
    /// 
    /// `border-end-end-radius: 9999px;`
    EFull,
    /// `border-top-left-radius: 0px;`
    /// 
    /// `border-top-right-radius: 0px;`
    TNone,
    /// `border-top-left-radius: 0.125rem; /* 2px */`
    /// 
    /// `border-top-right-radius: 0.125rem; /* 2px */`
    TSm,
    /// `border-top-left-radius: 0.25rem; /* 4px */`
    /// 
    /// `border-top-right-radius: 0.25rem; /* 4px */`
    T,
    /// `border-top-left-radius: 0.375rem; /* 6px */`
    /// 
    /// `border-top-right-radius: 0.375rem; /* 6px */`
    TMd,
    /// `border-top-left-radius: 0.5rem; /* 8px */`
    /// 
    /// `border-top-right-radius: 0.5rem; /* 8px */`
    TLg,
    /// `border-top-left-radius: 0.75rem; /* 12px */`
    /// 
    /// `border-top-right-radius: 0.75rem; /* 12px */`
    TXl,
    /// `border-top-left-radius: 1rem; /* 16px */`
    /// 
    /// `border-top-right-radius: 1rem; /* 16px */`
    T2xl,
    /// `border-top-left-radius: 1.5rem; /* 24px */`
    /// 
    /// `border-top-right-radius: 1.5rem; /* 24px */`
    T3xl,
    /// `border-top-left-radius: 9999px;`
    /// 
    /// `border-top-right-radius: 9999px;`
    TFull,
    /// `border-top-right-radius: 0px;`
    /// 
    /// `border-bottom-right-radius: 0px;`
    RNone,
    /// `border-top-right-radius: 0.125rem; /* 2px */`
    /// 
    /// `border-bottom-right-radius: 0.125rem; /* 2px */`
    RSm,
    /// `border-top-right-radius: 0.25rem; /* 4px */`
    /// 
    /// `border-bottom-right-radius: 0.25rem; /* 4px */`
    R,
    /// `border-top-right-radius: 0.375rem; /* 6px */`
    /// 
    /// `border-bottom-right-radius: 0.375rem; /* 6px */`
    RMd,
    /// `border-top-right-radius: 0.5rem; /* 8px */`
    /// 
    /// `border-bottom-right-radius: 0.5rem; /* 8px */`
    RLg,
    /// `border-top-right-radius: 0.75rem; /* 12px */`
    /// 
    /// `border-bottom-right-radius: 0.75rem; /* 12px */`
    RXl,
    /// `border-top-right-radius: 1rem; /* 16px */`
    /// 
    /// `border-bottom-right-radius: 1rem; /* 16px */`
    R2xl,
    /// `border-top-right-radius: 1.5rem; /* 24px */`
    /// 
    /// `border-bottom-right-radius: 1.5rem; /* 24px */`
    R3xl,
    /// `border-top-right-radius: 9999px;`
    /// 
    /// `border-bottom-right-radius: 9999px;`
    RFull,
    /// `border-bottom-right-radius: 0px;`
    /// 
    /// `border-bottom-left-radius: 0px;`
    BNone,
    /// `border-bottom-right-radius: 0.125rem; /* 2px */`
    /// 
    /// `border-bottom-left-radius: 0.125rem; /* 2px */`
    BSm,
    /// `border-bottom-right-radius: 0.25rem; /* 4px */`
    /// 
    /// `border-bottom-left-radius: 0.25rem; /* 4px */`
    B,
    /// `border-bottom-right-radius: 0.375rem; /* 6px */`
    /// 
    /// `border-bottom-left-radius: 0.375rem; /* 6px */`
    BMd,
    /// `border-bottom-right-radius: 0.5rem; /* 8px */`
    /// 
    /// `border-bottom-left-radius: 0.5rem; /* 8px */`
    BLg,
    /// `border-bottom-right-radius: 0.75rem; /* 12px */`
    /// 
    /// `border-bottom-left-radius: 0.75rem; /* 12px */`
    BXl,
    /// `border-bottom-right-radius: 1rem; /* 16px */`
    /// 
    /// `border-bottom-left-radius: 1rem; /* 16px */`
    B2xl,
    /// `border-bottom-right-radius: 1.5rem; /* 24px */`
    /// 
    /// `border-bottom-left-radius: 1.5rem; /* 24px */`
    B3xl,
    /// `border-bottom-right-radius: 9999px;`
    /// 
    /// `border-bottom-left-radius: 9999px;`
    BFull,
    /// `border-top-left-radius: 0px;`
    /// 
    /// `border-bottom-left-radius: 0px;`
    LNone,
    /// `border-top-left-radius: 0.125rem; /* 2px */`
    /// 
    /// `border-bottom-left-radius: 0.125rem; /* 2px */`
    LSm,
    /// `border-top-left-radius: 0.25rem; /* 4px */`
    /// 
    /// `border-bottom-left-radius: 0.25rem; /* 4px */`
    L,
    /// `border-top-left-radius: 0.375rem; /* 6px */`
    /// 
    /// `border-bottom-left-radius: 0.375rem; /* 6px */`
    LMd,
    /// `border-top-left-radius: 0.5rem; /* 8px */`
    /// 
    /// `border-bottom-left-radius: 0.5rem; /* 8px */`
    LLg,
    /// `border-top-left-radius: 0.75rem; /* 12px */`
    /// 
    /// `border-bottom-left-radius: 0.75rem; /* 12px */`
    LXl,
    /// `border-top-left-radius: 1rem; /* 16px */`
    /// 
    /// `border-bottom-left-radius: 1rem; /* 16px */`
    L2xl,
    /// `border-top-left-radius: 1.5rem; /* 24px */`
    /// 
    /// `border-bottom-left-radius: 1.5rem; /* 24px */`
    L3xl,
    /// `border-top-left-radius: 9999px;`
    /// 
    /// `border-bottom-left-radius: 9999px;`
    LFull,
    /// `border-start-start-radius: 0px;`
    SsNone,
    /// `border-start-start-radius: 0.125rem; /* 2px */`
    SsSm,
    /// `border-start-start-radius: 0.25rem; /* 4px */`
    Ss,
    /// `border-start-start-radius: 0.375rem; /* 6px */`
    SsMd,
    /// `border-start-start-radius: 0.5rem; /* 8px */`
    SsLg,
    /// `border-start-start-radius: 0.75rem; /* 12px */`
    SsXl,
    /// `border-start-start-radius: 1rem; /* 16px */`
    Ss2xl,
    /// `border-start-start-radius: 1.5rem; /* 24px */`
    Ss3xl,
    /// `border-start-start-radius: 9999px;`
    SsFull,
    /// `border-start-end-radius: 0px;`
    SeNone,
    /// `border-start-end-radius: 0.125rem; /* 2px */`
    SeSm,
    /// `border-start-end-radius: 0.25rem; /* 4px */`
    Se,
    /// `border-start-end-radius: 0.375rem; /* 6px */`
    SeMd,
    /// `border-start-end-radius: 0.5rem; /* 8px */`
    SeLg,
    /// `border-start-end-radius: 0.75rem; /* 12px */`
    SeXl,
    /// `border-start-end-radius: 1rem; /* 16px */`
    Se2xl,
    /// `border-start-end-radius: 1.5rem; /* 24px */`
    Se3xl,
    /// `border-start-end-radius: 9999px;`
    SeFull,
    /// `border-end-end-radius: 0px;`
    EeNone,
    /// `border-end-end-radius: 0.125rem; /* 2px */`
    EeSm,
    /// `border-end-end-radius: 0.25rem; /* 4px */`
    Ee,
    /// `border-end-end-radius: 0.375rem; /* 6px */`
    EeMd,
    /// `border-end-end-radius: 0.5rem; /* 8px */`
    EeLg,
    /// `border-end-end-radius: 0.75rem; /* 12px */`
    EeXl,
    /// `border-end-end-radius: 1rem; /* 16px */`
    Ee2xl,
    /// `border-end-end-radius: 1.5rem; /* 24px */`
    Ee3xl,
    /// `border-end-end-radius: 9999px;`
    EeFull,
    /// `border-end-start-radius: 0px;`
    EsNone,
    /// `border-end-start-radius: 0.125rem; /* 2px */`
    EsSm,
    /// `border-end-start-radius: 0.25rem; /* 4px */`
    Es,
    /// `border-end-start-radius: 0.375rem; /* 6px */`
    EsMd,
    /// `border-end-start-radius: 0.5rem; /* 8px */`
    EsLg,
    /// `border-end-start-radius: 0.75rem; /* 12px */`
    EsXl,
    /// `border-end-start-radius: 1rem; /* 16px */`
    Es2xl,
    /// `border-end-start-radius: 1.5rem; /* 24px */`
    Es3xl,
    /// `border-end-start-radius: 9999px;`
    EsFull,
    /// `border-top-left-radius: 0px;`
    TlNone,
    /// `border-top-left-radius: 0.125rem; /* 2px */`
    TlSm,
    /// `border-top-left-radius: 0.25rem; /* 4px */`
    Tl,
    /// `border-top-left-radius: 0.375rem; /* 6px */`
    TlMd,
    /// `border-top-left-radius: 0.5rem; /* 8px */`
    TlLg,
    /// `border-top-left-radius: 0.75rem; /* 12px */`
    TlXl,
    /// `border-top-left-radius: 1rem; /* 16px */`
    Tl2xl,
    /// `border-top-left-radius: 1.5rem; /* 24px */`
    Tl3xl,
    /// `border-top-left-radius: 9999px;`
    TlFull,
    /// `border-top-right-radius: 0px;`
    TrNone,
    /// `border-top-right-radius: 0.125rem; /* 2px */`
    TrSm,
    /// `border-top-right-radius: 0.25rem; /* 4px */`
    Tr,
    /// `border-top-right-radius: 0.375rem; /* 6px */`
    TrMd,
    /// `border-top-right-radius: 0.5rem; /* 8px */`
    TrLg,
    /// `border-top-right-radius: 0.75rem; /* 12px */`
    TrXl,
    /// `border-top-right-radius: 1rem; /* 16px */`
    Tr2xl,
    /// `border-top-right-radius: 1.5rem; /* 24px */`
    Tr3xl,
    /// `border-top-right-radius: 9999px;`
    TrFull,
    /// `border-bottom-right-radius: 0px;`
    BrNone,
    /// `border-bottom-right-radius: 0.125rem; /* 2px */`
    BrSm,
    /// `border-bottom-right-radius: 0.25rem; /* 4px */`
    Br,
    /// `border-bottom-right-radius: 0.375rem; /* 6px */`
    BrMd,
    /// `border-bottom-right-radius: 0.5rem; /* 8px */`
    BrLg,
    /// `border-bottom-right-radius: 0.75rem; /* 12px */`
    BrXl,
    /// `border-bottom-right-radius: 1rem; /* 16px */`
    Br2xl,
    /// `border-bottom-right-radius: 1.5rem; /* 24px */`
    Br3xl,
    /// `border-bottom-right-radius: 9999px;`
    BrFull,
    /// `border-bottom-left-radius: 0px;`
    BlNone,
    /// `border-bottom-left-radius: 0.125rem; /* 2px */`
    BlSm,
    /// `border-bottom-left-radius: 0.25rem; /* 4px */`
    Bl,
    /// `border-bottom-left-radius: 0.375rem; /* 6px */`
    BlMd,
    /// `border-bottom-left-radius: 0.5rem; /* 8px */`
    BlLg,
    /// `border-bottom-left-radius: 0.75rem; /* 12px */`
    BlXl,
    /// `border-bottom-left-radius: 1rem; /* 16px */`
    Bl2xl,
    /// `border-bottom-left-radius: 1.5rem; /* 24px */`
    Bl3xl,
    /// `border-bottom-left-radius: 9999px;`
    BlFull,
}

/// Utilities for controlling the width of an element's borders.
/// 
/// <https://tailwindcss.com/docs/border-width>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "border")]
pub enum BorderWidth {
    /// `border-width: 0px;`
    _0,
    /// `border-width: 2px;`
    _2,
    /// `border-width: 4px;`
    _4,
    /// `border-width: 8px;`
    _8,
    /// `border-width: 1px;`
    #[display(no_prefix)]
    Border,
    /// `border-left-width: 0px;`
    /// 
    /// `border-right-width: 0px;`
    X0,
    /// `border-left-width: 2px;`
    /// 
    /// `border-right-width: 2px;`
    X2,
    /// `border-left-width: 4px;`
    /// 
    /// `border-right-width: 4px;`
    X4,
    /// `border-left-width: 8px;`
    /// 
    /// `border-right-width: 8px;`
    X8,
    /// `border-left-width: 1px;`
    /// 
    /// `border-right-width: 1px;`
    X,
    /// `border-top-width: 0px;`
    /// 
    /// `border-bottom-width: 0px;`
    Y0,
    /// `border-top-width: 2px;`
    /// 
    /// `border-bottom-width: 2px;`
    Y2,
    /// `border-top-width: 4px;`
    /// 
    /// `border-bottom-width: 4px;`
    Y4,
    /// `border-top-width: 8px;`
    /// 
    /// `border-bottom-width: 8px;`
    Y8,
    /// `border-top-width: 1px;`
    /// 
    /// `border-bottom-width: 1px;`
    Y,
    /// `border-inline-start-width: 0px;`
    S0,
    /// `border-inline-start-width: 2px;`
    S2,
    /// `border-inline-start-width: 4px;`
    S4,
    /// `border-inline-start-width: 8px;`
    S8,
    /// `border-inline-start-width: 1px;`
    S,
    /// `border-inline-end-width: 0px;`
    E0,
    /// `border-inline-end-width: 2px;`
    E2,
    /// `border-inline-end-width: 4px;`
    E4,
    /// `border-inline-end-width: 8px;`
    E8,
    /// `border-inline-end-width: 1px;`
    E,
    /// `border-top-width: 0px;`
    T0,
    /// `border-top-width: 2px;`
    T2,
    /// `border-top-width: 4px;`
    T4,
    /// `border-top-width: 8px;`
    T8,
    /// `border-top-width: 1px;`
    T,
    /// `border-right-width: 0px;`
    R0,
    /// `border-right-width: 2px;`
    R2,
    /// `border-right-width: 4px;`
    R4,
    /// `border-right-width: 8px;`
    R8,
    /// `border-right-width: 1px;`
    R,
    /// `border-bottom-width: 0px;`
    B0,
    /// `border-bottom-width: 2px;`
    B2,
    /// `border-bottom-width: 4px;`
    B4,
    /// `border-bottom-width: 8px;`
    B8,
    /// `border-bottom-width: 1px;`
    B,
    /// `border-left-width: 0px;`
    L0,
    /// `border-left-width: 2px;`
    L2,
    /// `border-left-width: 4px;`
    L4,
    /// `border-left-width: 8px;`
    L8,
    /// `border-left-width: 1px;`
    L,
}

// TODO: Border Color

/// Utilities for controlling the style of an element's borders.
/// 
/// <https://tailwindcss.com/docs/border-style>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "border")]
pub enum BorderStyle {
    /// `border-style: solid;`
    Solid,
    /// `border-style: dashed;`
    Dashed,
    /// `border-style: dotted;`
    Dotted,
    /// `border-style: double;`
    Double,
    /// `border-style: hidden;`
    Hidden,
    /// `border-style: none;`
    None,
}

/// Utilities for controlling the border width between elements.
/// 
/// <https://tailwindcss.com/docs/divide-width>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "divide")]
pub enum DivideWidth {
    /// `border-right-width: 0px;`
    /// 
    /// `border-left-width: 0px;`
    X0,
    /// `border-right-width: 0px;`
    /// 
    /// `border-left-width: 2px;`
    X2,
    /// `border-right-width: 0px;`
    /// 
    /// `border-left-width: 4px;`
    X4,
    /// `border-right-width: 0px;`
    /// 
    /// `border-left-width: 8px;`
    X8,
    /// `border-right-width: 0px;`
    /// 
    /// `border-left-width: 1px;`
    X,
    /// `border-top-width: 0px;`
    /// 
    /// `border-bottom-width: 0px;`
    Y0,
    /// `border-top-width: 2px;`
    /// 
    /// `border-bottom-width: 0px;`
    Y2,
    /// `border-top-width: 4px;`
    /// 
    /// `border-bottom-width: 0px;`
    Y4,
    /// `border-top-width: 8px;`
    /// 
    /// `border-bottom-width: 0px;`
    Y8,
    /// `border-top-width: 1px;`
    /// 
    /// `border-bottom-width: 0px;`
    Y,
    /// `--tw-divide-y-reverse: 1;`
    YReverse,
    /// `--tw-divide-x-reverse: 1;`
    XReverse,
}

/// Utilities for controlling the border color between elements.
/// 
/// <https://tailwindcss.com/docs/divide-color>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "divide")]
pub struct DivideColor(Color);

/// Utilities for controlling the border style between elements.
/// 
/// <https://tailwindcss.com/docs/divide-style>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "divide")]
pub enum DivideStyle {
    /// `border-style: solid;`
    Solid,
    /// `border-style: dashed;`
    Dashed,
    /// `border-style: dotted;`
    Dotted,
    /// `border-style: double;`
    Double,
    /// `border-style: none;`
    None,
}

/// Utilities for controlling the width of an element's outline.
/// 
/// <https://tailwindcss.com/docs/outline-width>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "outline")]
pub enum OutlineWidth {
    /// `outline-width: 0px;`
    _0,
    /// `outline-width: 1px;`
    _1,
    /// `outline-width: 2px;`
    _2,
    /// `outline-width: 4px;`
    _4,
    /// `outline-width: 8px;`
    _8,
}

/// Utilities for controlling the color of an element's outline.
/// 
/// <https://tailwindcss.com/docs/outline-color>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "outline")]
pub struct OutlineColor(Color);

/// Utilities for controlling the style of an element's outline.
/// 
/// <https://tailwindcss.com/docs/outline-style>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "outline")]
pub enum OutlineStyle {
    /// `outline: 2px solid transparent;`
    /// 
    /// `outline-offset: 2px;`
    None,
    /// `outline-style: solid;`
    #[display(no_prefix)]
    Outline,
    /// `outline-style: dashed;`
    Dashed,
    /// `outline-style: dotted;`
    Dotted,
    /// `outline-style: double;`
    Double,
}

/// Utilities for controlling the offset of an element's outline.
/// 
/// <https://tailwindcss.com/docs/outline-offset>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "outline-offset")]
pub enum OutlineOffset {
    /// `outline-offset: 0px;`
    _0,
    /// `outline-offset: 1px;`
    _1,
    /// `outline-offset: 2px;`
    _2,
    /// `outline-offset: 4px;`
    _4,
    /// `outline-offset: 8px;`
    _8,
}

/// Utilities for creating outline rings with box-shadows.
/// 
/// <https://tailwindcss.com/docs/ring-width>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "ring")]
pub enum RingWidth {
    /// `box-shadow: var(--tw-ring-inset) 0 0 0 calc(0px + var(--tw-ring-offset-width)) var(--tw-ring-color);`
    _0,
    /// `box-shadow: var(--tw-ring-inset) 0 0 0 calc(1px + var(--tw-ring-offset-width)) var(--tw-ring-color);`
    _1,
    /// `box-shadow: var(--tw-ring-inset) 0 0 0 calc(2px + var(--tw-ring-offset-width)) var(--tw-ring-color);`
    _2,
    /// `box-shadow: var(--tw-ring-inset) 0 0 0 calc(3px + var(--tw-ring-offset-width)) var(--tw-ring-color);`
    #[display(no_prefix)]
    Ring,
    /// `box-shadow: var(--tw-ring-inset) 0 0 0 calc(4px + var(--tw-ring-offset-width)) var(--tw-ring-color);`
    _4,
    /// `box-shadow: var(--tw-ring-inset) 0 0 0 calc(8px + var(--tw-ring-offset-width)) var(--tw-ring-color);`
    _8,
    /// `--tw-ring-inset: inset;`
    Inset,
}

/// Utilities for setting the color of outline rings.
/// 
/// <https://tailwindcss.com/docs/ring-color>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "ring")]
pub struct RingColor(Color);

/// Utilities for simulating an offset when adding outline rings.
/// 
/// <https://tailwindcss.com/docs/ring-offset-width>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "ring-offset")]
pub enum RingOffsetWidth {
    /// `--tw-ring-offset-width: 0px;`
    /// 
    /// `box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);`
    _0,
    /// `--tw-ring-offset-width: 1px;`
    /// 
    /// `box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);`
    _1,
    /// `--tw-ring-offset-width: 2px;`
    /// 
    /// `box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);`
    _2,
    /// `--tw-ring-offset-width: 4px;`
    /// 
    /// `box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);`
    _4,
    /// `--tw-ring-offset-width: 8px;`
    /// 
    /// `box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);`
    _8,
}

/// Utilities for setting the color of outline ring offsets.
/// 
/// <https://tailwindcss.com/docs/ring-offset-color>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "ring-offset")]
pub struct RingOffsetColor(Color);
