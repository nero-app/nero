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
    /// ```css
    /// {
    ///     border-radius: 0px;
    /// }
    /// ```
    None,
    /// ```css
    /// {
    ///     border-radius: 0.125rem; /* 2px */
    /// }
    /// ```
    Sm,
    /// ```css
    /// {
    ///     border-radius: 0.25rem; /* 4px */
    /// }
    /// ```
    #[display(no_prefix)]
    Rounded,
    /// ```css
    /// {
    ///     border-radius: 0.375rem; /* 6px */
    /// }
    /// ```
    Md,
    /// ```css
    /// {
    ///     border-radius: 0.5rem; /* 8px */
    /// }
    /// ```
    Lg,
    /// ```css
    /// {
    ///     border-radius: 0.75rem; /* 12px */
    /// }
    /// ```
    Xl,
    /// ```css
    /// {
    ///     border-radius: 1rem; /* 16px */
    /// }
    /// ```
    _2xl,
    /// ```css
    /// {
    ///     border-radius: 1.5rem; /* 24px */
    /// }
    /// ```
    _3xl,
    /// ```css
    /// {
    ///     border-radius: 9999px;
    /// }
    /// ```
    Full,
    /// ```css
    /// {
    ///     border-start-start-radius: 0px;
    ///     border-end-start-radius: 0px;
    /// }
    /// ```
    SNone,
    /// ```css
    /// {
    ///     border-start-start-radius: 0.125rem; /* 2px */
    ///     border-end-start-radius: 0.125rem; /* 2px */
    /// }
    /// ```
    SSm,
    /// ```css
    /// {
    ///     border-start-start-radius: 0.25rem; /* 4px */
    ///     border-end-start-radius: 0.25rem; /* 4px */
    /// }
    /// ```
    S,
    /// ```css
    /// {
    ///     border-start-start-radius: 0.375rem; /* 6px */
    ///     border-end-start-radius: 0.375rem; /* 6px */
    /// }
    /// ```
    SMd,
    /// ```css
    /// {
    ///     border-start-start-radius: 0.5rem; /* 8px */
    ///     border-end-start-radius: 0.5rem; /* 8px */
    /// }
    /// ```
    SLg,
    /// ```css
    /// {
    ///     border-start-start-radius: 0.75rem; /* 12px */
    ///     border-end-start-radius: 0.75rem; /* 12px */
    /// }
    /// ```
    SXl,
    /// ```css
    /// {
    ///     border-start-start-radius: 1rem; /* 16px */
    ///     border-end-start-radius: 1rem; /* 16px */
    /// }
    /// ```
    S2xl,
    /// ```css
    /// {
    ///     border-start-start-radius: 1.5rem; /* 24px */
    ///     border-end-start-radius: 1.5rem; /* 24px */
    /// }
    /// ```
    S3xl,
    /// ```css
    /// {
    ///     border-start-start-radius: 9999px;
    ///     border-end-start-radius: 9999px;
    /// }
    /// ```
    SFull,
    /// ```css
    /// {
    ///     border-start-end-radius: 0px;
    ///     border-end-end-radius: 0px;
    /// }
    /// ```
    ENone,
    /// ```css
    /// {
    ///     border-start-end-radius: 0.125rem; /* 2px */
    ///     border-end-end-radius: 0.125rem; /* 2px */
    /// }
    /// ```
    ESm,
    /// ```css
    /// {
    ///     border-start-end-radius: 0.25rem; /* 4px */
    ///     border-end-end-radius: 0.25rem; /* 4px */
    /// }
    /// ```
    E,
    /// ```css
    /// {
    ///     border-start-end-radius: 0.375rem; /* 6px */
    ///     border-end-end-radius: 0.375rem; /* 6px */
    /// }
    /// ```
    EMd,
    /// ```css
    /// {
    ///     border-start-end-radius: 0.5rem; /* 8px */
    ///     border-end-end-radius: 0.5rem; /* 8px */
    /// }
    /// ```
    ELg,
    /// ```css
    /// {
    ///     border-start-end-radius: 0.75rem; /* 12px */
    ///     border-end-end-radius: 0.75rem; /* 12px */
    /// }
    /// ```
    EXl,
    /// ```css
    /// {
    ///     border-start-end-radius: 1rem; /* 16px */
    ///     border-end-end-radius: 1rem; /* 16px */
    /// }
    /// ```
    E2xl,
    /// ```css
    /// {
    ///     border-start-end-radius: 1.5rem; /* 24px */
    ///     border-end-end-radius: 1.5rem; /* 24px */
    /// }
    /// ```
    E3xl,
    /// ```css
    /// {
    ///     border-start-end-radius: 9999px;
    ///     border-end-end-radius: 9999px;
    /// }
    /// ```
    EFull,
    /// ```css
    /// {
    ///     border-top-left-radius: 0px;
    ///     border-top-right-radius: 0px;
    /// }
    /// ```
    TNone,
    /// ```css
    /// {
    ///     border-top-left-radius: 0.125rem; /* 2px */
    ///     border-top-right-radius: 0.125rem; /* 2px */
    /// }
    /// ```
    TSm,
    /// ```css
    /// {
    ///     border-top-left-radius: 0.25rem; /* 4px */
    ///     border-top-right-radius: 0.25rem; /* 4px */
    /// }
    /// ```
    T,
    /// ```css
    /// {
    ///     border-top-left-radius: 0.375rem; /* 6px */
    ///     border-top-right-radius: 0.375rem; /* 6px */
    /// }
    /// ```
    TMd,
    /// ```css
    /// {
    ///     border-top-left-radius: 0.5rem; /* 8px */
    ///     border-top-right-radius: 0.5rem; /* 8px */
    /// }
    /// ```
    TLg,
    /// ```css
    /// {
    ///     border-top-left-radius: 0.75rem; /* 12px */
    ///     border-top-right-radius: 0.75rem; /* 12px */
    /// }
    /// ```
    TXl,
    /// ```css
    /// {
    ///     border-top-left-radius: 1rem; /* 16px */
    ///     border-top-right-radius: 1rem; /* 16px */
    /// }
    /// ```
    T2xl,
    /// ```css
    /// {
    ///     border-top-left-radius: 1.5rem; /* 24px */
    ///     border-top-right-radius: 1.5rem; /* 24px */
    /// }
    /// ```
    T3xl,
    /// ```css
    /// {
    ///     border-top-left-radius: 9999px;
    ///     border-top-right-radius: 9999px;
    /// }
    /// ```
    TFull,
    /// ```css
    /// {
    ///     border-top-right-radius: 0px;
    ///     border-bottom-right-radius: 0px;
    /// }
    /// ```
    RNone,
    /// ```css
    /// {
    ///     border-top-right-radius: 0.125rem; /* 2px */
    ///     border-bottom-right-radius: 0.125rem; /* 2px */
    /// }
    /// ```
    RSm,
    /// ```css
    /// {
    ///     border-top-right-radius: 0.25rem; /* 4px */
    ///     border-bottom-right-radius: 0.25rem; /* 4px */
    /// }
    /// ```
    R,
    /// ```css
    /// {
    ///     border-top-right-radius: 0.375rem; /* 6px */
    ///     border-bottom-right-radius: 0.375rem; /* 6px */
    /// }
    /// ```
    RMd,
    /// ```css
    /// {
    ///     border-top-right-radius: 0.5rem; /* 8px */
    ///     border-bottom-right-radius: 0.5rem; /* 8px */
    /// }
    /// ```
    RLg,
    /// ```css
    /// {
    ///     border-top-right-radius: 0.75rem; /* 12px */
    ///     border-bottom-right-radius: 0.75rem; /* 12px */
    /// }
    /// ```
    RXl,
    /// ```css
    /// {
    ///     border-top-right-radius: 1rem; /* 16px */
    ///     border-bottom-right-radius: 1rem; /* 16px */
    /// }
    /// ```
    R2xl,
    /// ```css
    /// {
    ///     border-top-right-radius: 1.5rem; /* 24px */
    ///     border-bottom-right-radius: 1.5rem; /* 24px */
    /// }
    /// ```
    R3xl,
    /// ```css
    /// {
    ///     border-top-right-radius: 9999px;
    ///     border-bottom-right-radius: 9999px;
    /// }
    /// ```
    RFull,
    /// ```css
    /// {
    ///     border-bottom-right-radius: 0px;
    ///     border-bottom-left-radius: 0px;
    /// }
    /// ```
    BNone,
    /// ```css
    /// {
    ///     border-bottom-right-radius: 0.125rem; /* 2px */
    ///     border-bottom-left-radius: 0.125rem; /* 2px */
    /// }
    /// ```
    BSm,
    /// ```css
    /// {
    ///     border-bottom-right-radius: 0.25rem; /* 4px */
    ///     border-bottom-left-radius: 0.25rem; /* 4px */
    /// }
    /// ```
    B,
    /// ```css
    /// {
    ///     border-bottom-right-radius: 0.375rem; /* 6px */
    ///     border-bottom-left-radius: 0.375rem; /* 6px */
    /// }
    /// ```
    BMd,
    /// ```css
    /// {
    ///     border-bottom-right-radius: 0.5rem; /* 8px */
    ///     border-bottom-left-radius: 0.5rem; /* 8px */
    /// }
    /// ```
    BLg,
    /// ```css
    /// {
    ///     border-bottom-right-radius: 0.75rem; /* 12px */
    ///     border-bottom-left-radius: 0.75rem; /* 12px */
    /// }
    /// ```
    BXl,
    /// ```css
    /// {
    ///     border-bottom-right-radius: 1rem; /* 16px */
    ///     border-bottom-left-radius: 1rem; /* 16px */
    /// }
    /// ```
    B2xl,
    /// ```css
    /// {
    ///     border-bottom-right-radius: 1.5rem; /* 24px */
    ///     border-bottom-left-radius: 1.5rem; /* 24px */
    /// }
    /// ```
    B3xl,
    /// ```css
    /// {
    ///     border-bottom-right-radius: 9999px;
    ///     border-bottom-left-radius: 9999px;
    /// }
    /// ```
    BFull,
    /// ```css
    /// {
    ///     border-top-left-radius: 0px;
    ///     border-bottom-left-radius: 0px;
    /// }
    /// ```
    LNone,
    /// ```css
    /// {
    ///     border-top-left-radius: 0.125rem; /* 2px */
    ///     border-bottom-left-radius: 0.125rem; /* 2px */
    /// }
    /// ```
    LSm,
    /// ```css
    /// {
    ///     border-top-left-radius: 0.25rem; /* 4px */
    ///     border-bottom-left-radius: 0.25rem; /* 4px */
    /// }
    /// ```
    L,
    /// ```css
    /// {
    ///     border-top-left-radius: 0.375rem; /* 6px */
    ///     border-bottom-left-radius: 0.375rem; /* 6px */
    /// }
    /// ```
    LMd,
    /// ```css
    /// {
    ///     border-top-left-radius: 0.5rem; /* 8px */
    ///     border-bottom-left-radius: 0.5rem; /* 8px */
    /// }
    /// ```
    LLg,
    /// ```css
    /// {
    ///     border-top-left-radius: 0.75rem; /* 12px */
    ///     border-bottom-left-radius: 0.75rem; /* 12px */
    /// }
    /// ```
    LXl,
    /// ```css
    /// {
    ///     border-top-left-radius: 1rem; /* 16px */
    ///     border-bottom-left-radius: 1rem; /* 16px */
    /// }
    /// ```
    L2xl,
    /// ```css
    /// {
    ///     border-top-left-radius: 1.5rem; /* 24px */
    ///     border-bottom-left-radius: 1.5rem; /* 24px */
    /// }
    /// ```
    L3xl,
    /// ```css
    /// {
    ///     border-top-left-radius: 9999px;
    ///     border-bottom-left-radius: 9999px;
    /// }
    /// ```
    LFull,
    /// ```css
    /// {
    ///     border-start-start-radius: 0px;
    /// }
    /// ```
    SsNone,
    /// ```css
    /// {
    ///     border-start-start-radius: 0.125rem; /* 2px */
    /// }
    /// ```
    SsSm,
    /// ```css
    /// {
    ///     border-start-start-radius: 0.25rem; /* 4px */
    /// }
    /// ```
    Ss,
    /// ```css
    /// {
    ///     border-start-start-radius: 0.375rem; /* 6px */
    /// }
    /// ```
    SsMd,
    /// ```css
    /// {
    ///     border-start-start-radius: 0.5rem; /* 8px */
    /// }
    /// ```
    SsLg,
    /// ```css
    /// {
    ///     border-start-start-radius: 0.75rem; /* 12px */
    /// }
    /// ```
    SsXl,
    /// ```css
    /// {
    ///     border-start-start-radius: 1rem; /* 16px */
    /// }
    /// ```
    Ss2xl,
    /// ```css
    /// {
    ///     border-start-start-radius: 1.5rem; /* 24px */
    /// }
    /// ```
    Ss3xl,
    /// ```css
    /// {
    ///     border-start-start-radius: 9999px;
    /// }
    /// ```
    SsFull,
    /// ```css
    /// {
    ///     border-start-end-radius: 0px;
    /// }
    /// ```
    SeNone,
    /// ```css
    /// {
    ///     border-start-end-radius: 0.125rem; /* 2px */
    /// }
    /// ```
    SeSm,
    /// ```css
    /// {
    ///     border-start-end-radius: 0.25rem; /* 4px */
    /// }
    /// ```
    Se,
    /// ```css
    /// {
    ///     border-start-end-radius: 0.375rem; /* 6px */
    /// }
    /// ```
    SeMd,
    /// ```css
    /// {
    ///     border-start-end-radius: 0.5rem; /* 8px */
    /// }
    /// ```
    SeLg,
    /// ```css
    /// {
    ///     border-start-end-radius: 0.75rem; /* 12px */
    /// }
    /// ```
    SeXl,
    /// ```css
    /// {
    ///     border-start-end-radius: 1rem; /* 16px */
    /// }
    /// ```
    Se2xl,
    /// ```css
    /// {
    ///     border-start-end-radius: 1.5rem; /* 24px */
    /// }
    /// ```
    Se3xl,
    /// ```css
    /// {
    ///     border-start-end-radius: 9999px;
    /// }
    /// ```
    SeFull,
    /// ```css
    /// {
    ///     border-end-end-radius: 0px;
    /// }
    /// ```
    EeNone,
    /// ```css
    /// {
    ///     border-end-end-radius: 0.125rem; /* 2px */
    /// }
    /// ```
    EeSm,
    /// ```css
    /// {
    ///     border-end-end-radius: 0.25rem; /* 4px */
    /// }
    /// ```
    Ee,
    /// ```css
    /// {
    ///     border-end-end-radius: 0.375rem; /* 6px */
    /// }
    /// ```
    EeMd,
    /// ```css
    /// {
    ///     border-end-end-radius: 0.5rem; /* 8px */
    /// }
    /// ```
    EeLg,
    /// ```css
    /// {
    ///     border-end-end-radius: 0.75rem; /* 12px */
    /// }
    /// ```
    EeXl,
    /// ```css
    /// {
    ///     border-end-end-radius: 1rem; /* 16px */
    /// }
    /// ```
    Ee2xl,
    /// ```css
    /// {
    ///     border-end-end-radius: 1.5rem; /* 24px */
    /// }
    /// ```
    Ee3xl,
    /// ```css
    /// {
    ///     border-end-end-radius: 9999px;
    /// }
    /// ```
    EeFull,
    /// ```css
    /// {
    ///     border-end-start-radius: 0px;
    /// }
    /// ```
    EsNone,
    /// ```css
    /// {
    ///     border-end-start-radius: 0.125rem; /* 2px */
    /// }
    /// ```
    EsSm,
    /// ```css
    /// {
    ///     border-end-start-radius: 0.25rem; /* 4px */
    /// }
    /// ```
    Es,
    /// ```css
    /// {
    ///     border-end-start-radius: 0.375rem; /* 6px */
    /// }
    /// ```
    EsMd,
    /// ```css
    /// {
    ///     border-end-start-radius: 0.5rem; /* 8px */
    /// }
    /// ```
    EsLg,
    /// ```css
    /// {
    ///     border-end-start-radius: 0.75rem; /* 12px */
    /// }
    /// ```
    EsXl,
    /// ```css
    /// {
    ///     border-end-start-radius: 1rem; /* 16px */
    /// }
    /// ```
    Es2xl,
    /// ```css
    /// {
    ///     border-end-start-radius: 1.5rem; /* 24px */
    /// }
    /// ```
    Es3xl,
    /// ```css
    /// {
    ///     border-end-start-radius: 9999px;
    /// }
    /// ```
    EsFull,
    /// ```css
    /// {
    ///     border-top-left-radius: 0px;
    /// }
    /// ```
    TlNone,
    /// ```css
    /// {
    ///     border-top-left-radius: 0.125rem; /* 2px */
    /// }
    /// ```
    TlSm,
    /// ```css
    /// {
    ///     border-top-left-radius: 0.25rem; /* 4px */
    /// }
    /// ```
    Tl,
    /// ```css
    /// {
    ///     border-top-left-radius: 0.375rem; /* 6px */
    /// }
    /// ```
    TlMd,
    /// ```css
    /// {
    ///     border-top-left-radius: 0.5rem; /* 8px */
    /// }
    /// ```
    TlLg,
    /// ```css
    /// {
    ///     border-top-left-radius: 0.75rem; /* 12px */
    /// }
    /// ```
    TlXl,
    /// ```css
    /// {
    ///     border-top-left-radius: 1rem; /* 16px */
    /// }
    /// ```
    Tl2xl,
    /// ```css
    /// {
    ///     border-top-left-radius: 1.5rem; /* 24px */
    /// }
    /// ```
    Tl3xl,
    /// ```css
    /// {
    ///     border-top-left-radius: 9999px;
    /// }
    /// ```
    TlFull,
    /// ```css
    /// {
    ///     border-top-right-radius: 0px;
    /// }
    /// ```
    TrNone,
    /// ```css
    /// {
    ///     border-top-right-radius: 0.125rem; /* 2px */
    /// }
    /// ```
    TrSm,
    /// ```css
    /// {
    ///     border-top-right-radius: 0.25rem; /* 4px */
    /// }
    /// ```
    Tr,
    /// ```css
    /// {
    ///     border-top-right-radius: 0.375rem; /* 6px */
    /// }
    /// ```
    TrMd,
    /// ```css
    /// {
    ///     border-top-right-radius: 0.5rem; /* 8px */
    /// }
    /// ```
    TrLg,
    /// ```css
    /// {
    ///     border-top-right-radius: 0.75rem; /* 12px */
    /// }
    /// ```
    TrXl,
    /// ```css
    /// {
    ///     border-top-right-radius: 1rem; /* 16px */
    /// }
    /// ```
    Tr2xl,
    /// ```css
    /// {
    ///     border-top-right-radius: 1.5rem; /* 24px */
    /// }
    /// ```
    Tr3xl,
    /// ```css
    /// {
    ///     border-top-right-radius: 9999px;
    /// }
    /// ```
    TrFull,
    /// ```css
    /// {
    ///     border-bottom-right-radius: 0px;
    /// }
    /// ```
    BrNone,
    /// ```css
    /// {
    ///     border-bottom-right-radius: 0.125rem; /* 2px */
    /// }
    /// ```
    BrSm,
    /// ```css
    /// {
    ///     border-bottom-right-radius: 0.25rem; /* 4px */
    /// }
    /// ```
    Br,
    /// ```css
    /// {
    ///     border-bottom-right-radius: 0.375rem; /* 6px */
    /// }
    /// ```
    BrMd,
    /// ```css
    /// {
    ///     border-bottom-right-radius: 0.5rem; /* 8px */
    /// }
    /// ```
    BrLg,
    /// ```css
    /// {
    ///     border-bottom-right-radius: 0.75rem; /* 12px */
    /// }
    /// ```
    BrXl,
    /// ```css
    /// {
    ///     border-bottom-right-radius: 1rem; /* 16px */
    /// }
    /// ```
    Br2xl,
    /// ```css
    /// {
    ///     border-bottom-right-radius: 1.5rem; /* 24px */
    /// }
    /// ```
    Br3xl,
    /// ```css
    /// {
    ///     border-bottom-right-radius: 9999px;
    /// }
    /// ```
    BrFull,
    /// ```css
    /// {
    ///     border-bottom-left-radius: 0px;
    /// }
    /// ```
    BlNone,
    /// ```css
    /// {
    ///     border-bottom-left-radius: 0.125rem; /* 2px */
    /// }
    /// ```
    BlSm,
    /// ```css
    /// {
    ///     border-bottom-left-radius: 0.25rem; /* 4px */
    /// }
    /// ```
    Bl,
    /// ```css
    /// {
    ///     border-bottom-left-radius: 0.375rem; /* 6px */
    /// }
    /// ```
    BlMd,
    /// ```css
    /// {
    ///     border-bottom-left-radius: 0.5rem; /* 8px */
    /// }
    /// ```
    BlLg,
    /// ```css
    /// {
    ///     border-bottom-left-radius: 0.75rem; /* 12px */
    /// }
    /// ```
    BlXl,
    /// ```css
    /// {
    ///     border-bottom-left-radius: 1rem; /* 16px */
    /// }
    /// ```
    Bl2xl,
    /// ```css
    /// {
    ///     border-bottom-left-radius: 1.5rem; /* 24px */
    /// }
    /// ```
    Bl3xl,
    /// ```css
    /// {
    ///     border-bottom-left-radius: 9999px;
    /// }
    /// ```
    BlFull,
}

/// Utilities for controlling the width of an element's borders.
/// 
/// <https://tailwindcss.com/docs/border-width>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "border")]
pub enum BorderWidth {
    /// ```css
    /// {
    ///     border-width: 0px;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     border-width: 2px;
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     border-width: 4px;
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     border-width: 8px;
    /// }
    /// ```
    _8,
    /// ```css
    /// {
    ///     border-width: 1px;
    /// }
    /// ```
    #[display(no_prefix)]
    Border,
    /// ```css
    /// {
    ///     border-left-width: 0px;
    ///     border-right-width: 0px;
    /// }
    /// ```
    X0,
    /// ```css
    /// {
    ///     border-left-width: 2px;
    ///     border-right-width: 2px;
    /// }
    /// ```
    X2,
    /// ```css
    /// {
    ///     border-left-width: 4px;
    ///     border-right-width: 4px;
    /// }
    /// ```
    X4,
    /// ```css
    /// {
    ///     border-left-width: 8px;
    ///     border-right-width: 8px;
    /// }
    /// ```
    X8,
    /// ```css
    /// {
    ///     border-left-width: 1px;
    ///     border-right-width: 1px;
    /// }
    /// ```
    X,
    /// ```css
    /// {
    ///     border-top-width: 0px;
    ///     border-bottom-width: 0px;
    /// }
    /// ```
    Y0,
    /// ```css
    /// {
    ///     border-top-width: 2px;
    ///     border-bottom-width: 2px;
    /// }
    /// ```
    Y2,
    /// ```css
    /// {
    ///     border-top-width: 4px;
    ///     border-bottom-width: 4px;
    /// }
    /// ```
    Y4,
    /// ```css
    /// {
    ///     border-top-width: 8px;
    ///     border-bottom-width: 8px;
    /// }
    /// ```
    Y8,
    /// ```css
    /// {
    ///     border-top-width: 1px;
    ///     border-bottom-width: 1px;
    /// }
    /// ```
    Y,
    /// ```css
    /// {
    ///     border-inline-start-width: 0px;
    /// }
    /// ```
    S0,
    /// ```css
    /// {
    ///     border-inline-start-width: 2px;
    /// }
    /// ```
    S2,
    /// ```css
    /// {
    ///     border-inline-start-width: 4px;
    /// }
    /// ```
    S4,
    /// ```css
    /// {
    ///     border-inline-start-width: 8px;
    /// }
    /// ```
    S8,
    /// ```css
    /// {
    ///     border-inline-start-width: 1px;
    /// }
    /// ```
    S,
    /// ```css
    /// {
    ///     border-inline-end-width: 0px;
    /// }
    /// ```
    E0,
    /// ```css
    /// {
    ///     border-inline-end-width: 2px;
    /// }
    /// ```
    E2,
    /// ```css
    /// {
    ///     border-inline-end-width: 4px;
    /// }
    /// ```
    E4,
    /// ```css
    /// {
    ///     border-inline-end-width: 8px;
    /// }
    /// ```
    E8,
    /// ```css
    /// {
    ///     border-inline-end-width: 1px;
    /// }
    /// ```
    E,
    /// ```css
    /// {
    ///     border-top-width: 0px;
    /// }
    /// ```
    T0,
    /// ```css
    /// {
    ///     border-top-width: 2px;
    /// }
    /// ```
    T2,
    /// ```css
    /// {
    ///     border-top-width: 4px;
    /// }
    /// ```
    T4,
    /// ```css
    /// {
    ///     border-top-width: 8px;
    /// }
    /// ```
    T8,
    /// ```css
    /// {
    ///     border-top-width: 1px;
    /// }
    /// ```
    T,
    /// ```css
    /// {
    ///     border-right-width: 0px;
    /// }
    /// ```
    R0,
    /// ```css
    /// {
    ///     border-right-width: 2px;
    /// }
    /// ```
    R2,
    /// ```css
    /// {
    ///     border-right-width: 4px;
    /// }
    /// ```
    R4,
    /// ```css
    /// {
    ///     border-right-width: 8px;
    /// }
    /// ```
    R8,
    /// ```css
    /// {
    ///     border-right-width: 1px;
    /// }
    /// ```
    R,
    /// ```css
    /// {
    ///     border-bottom-width: 0px;
    /// }
    /// ```
    B0,
    /// ```css
    /// {
    ///     border-bottom-width: 2px;
    /// }
    /// ```
    B2,
    /// ```css
    /// {
    ///     border-bottom-width: 4px;
    /// }
    /// ```
    B4,
    /// ```css
    /// {
    ///     border-bottom-width: 8px;
    /// }
    /// ```
    B8,
    /// ```css
    /// {
    ///     border-bottom-width: 1px;
    /// }
    /// ```
    B,
    /// ```css
    /// {
    ///     border-left-width: 0px;
    /// }
    /// ```
    L0,
    /// ```css
    /// {
    ///     border-left-width: 2px;
    /// }
    /// ```
    L2,
    /// ```css
    /// {
    ///     border-left-width: 4px;
    /// }
    /// ```
    L4,
    /// ```css
    /// {
    ///     border-left-width: 8px;
    /// }
    /// ```
    L8,
    /// ```css
    /// {
    ///     border-left-width: 1px;
    /// }
    /// ```
    L,
}

// TODO: Border Color

/// Utilities for controlling the style of an element's borders.
/// 
/// <https://tailwindcss.com/docs/border-style>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "border")]
pub enum BorderStyle {
    /// ```css
    /// {
    ///     border-style: solid;
    /// }
    /// ```
    Solid,
    /// ```css
    /// {
    ///     border-style: dashed;
    /// }
    /// ```
    Dashed,
    /// ```css
    /// {
    ///     border-style: dotted;
    /// }
    /// ```
    Dotted,
    /// ```css
    /// {
    ///     border-style: double;
    /// }
    /// ```
    Double,
    /// ```css
    /// {
    ///     border-style: hidden;
    /// }
    /// ```
    Hidden,
    /// ```css
    /// {
    ///     border-style: none;
    /// }
    /// ```
    None,
}

/// Utilities for controlling the border width between elements.
/// 
/// <https://tailwindcss.com/docs/divide-width>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "divide")]
pub enum DivideWidth {
    /// ```css
    /// {
    ///     border-right-width: 0px;
    ///     border-left-width: 0px;
    /// }
    /// ```
    X0,
    /// ```css
    /// {
    ///     border-right-width: 0px;
    ///     border-left-width: 2px;
    /// }
    /// ```
    X2,
    /// ```css
    /// {
    ///     border-right-width: 0px;
    ///     border-left-width: 4px;
    /// }
    /// ```
    X4,
    /// ```css
    /// {
    ///     border-right-width: 0px;
    ///     border-left-width: 8px;
    /// }
    /// ```
    X8,
    /// ```css
    /// {
    ///     border-right-width: 0px;
    ///     border-left-width: 1px;
    /// }
    /// ```
    X,
    /// ```css
    /// {
    ///     border-top-width: 0px;
    ///     border-bottom-width: 0px;
    /// }
    /// ```
    Y0,
    /// ```css
    /// {
    ///     border-top-width: 2px;
    ///     border-bottom-width: 0px;
    /// }
    /// ```
    Y2,
    /// ```css
    /// {
    ///     border-top-width: 4px;
    ///     border-bottom-width: 0px;
    /// }
    /// ```
    Y4,
    /// ```css
    /// {
    ///     border-top-width: 8px;
    ///     border-bottom-width: 0px;
    /// }
    /// ```
    Y8,
    /// ```css
    /// {
    ///     border-top-width: 1px;
    ///     border-bottom-width: 0px;
    /// }
    /// ```
    Y,
    /// ```css
    /// {
    ///     --tw-divide-y-reverse: 1;
    /// }
    /// ```
    YReverse,
    /// ```css
    /// {
    ///     --tw-divide-x-reverse: 1;
    /// }
    /// ```
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
    /// ```css
    /// {
    ///     border-style: solid;
    /// }
    /// ```
    Solid,
    /// ```css
    /// {
    ///     border-style: dashed;
    /// }
    /// ```
    Dashed,
    /// ```css
    /// {
    ///     border-style: dotted;
    /// }
    /// ```
    Dotted,
    /// ```css
    /// {
    ///     border-style: double;
    /// }
    /// ```
    Double,
    /// ```css
    /// {
    ///     border-style: none;
    /// }
    /// ```
    None,
}

/// Utilities for controlling the width of an element's outline.
/// 
/// <https://tailwindcss.com/docs/outline-width>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "outline")]
pub enum OutlineWidth {
    /// ```css
    /// {
    ///     outline-width: 0px;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     outline-width: 1px;
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     outline-width: 2px;
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     outline-width: 4px;
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     outline-width: 8px;
    /// }
    /// ```
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
    /// ```css
    /// {
    ///     outline: 2px solid transparent;
    ///     outline-offset: 2px;
    /// }
    /// ```
    None,
    /// ```css
    /// {
    ///     outline-style: solid;
    /// }
    /// ```
    #[display(no_prefix)]
    Outline,
    /// ```css
    /// {
    ///     outline-style: dashed;
    /// }
    /// ```
    Dashed,
    /// ```css
    /// {
    ///     outline-style: dotted;
    /// }
    /// ```
    Dotted,
    /// ```css
    /// {
    ///     outline-style: double;
    /// }
    /// ```
    Double,
}

/// Utilities for controlling the offset of an element's outline.
/// 
/// <https://tailwindcss.com/docs/outline-offset>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "outline-offset")]
pub enum OutlineOffset {
    /// ```css
    /// {
    ///     outline-offset: 0px;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     outline-offset: 1px;
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     outline-offset: 2px;
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     outline-offset: 4px;
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     outline-offset: 8px;
    /// }
    /// ```
    _8,
}

/// Utilities for creating outline rings with box-shadows.
/// 
/// <https://tailwindcss.com/docs/ring-width>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "ring")]
pub enum RingWidth {
    /// ```css
    /// {
    ///     box-shadow: var(--tw-ring-inset) 0 0 0 calc(0px + var(--tw-ring-offset-width)) var(--tw-ring-color);
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     box-shadow: var(--tw-ring-inset) 0 0 0 calc(1px + var(--tw-ring-offset-width)) var(--tw-ring-color);
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     box-shadow: var(--tw-ring-inset) 0 0 0 calc(2px + var(--tw-ring-offset-width)) var(--tw-ring-color);
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     box-shadow: var(--tw-ring-inset) 0 0 0 calc(3px + var(--tw-ring-offset-width)) var(--tw-ring-color);
    /// }
    /// ```
    #[display(no_prefix)]
    Ring,
    /// ```css
    /// {
    ///     box-shadow: var(--tw-ring-inset) 0 0 0 calc(4px + var(--tw-ring-offset-width)) var(--tw-ring-color);
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     box-shadow: var(--tw-ring-inset) 0 0 0 calc(8px + var(--tw-ring-offset-width)) var(--tw-ring-color);
    /// }
    /// ```
    _8,
    /// ```css
    /// {
    ///     --tw-ring-inset: inset;
    /// }
    /// ```
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
    /// ```css
    /// {
    ///     --tw-ring-offset-width: 0px;
    ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     --tw-ring-offset-width: 1px;
    ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     --tw-ring-offset-width: 2px;
    ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     --tw-ring-offset-width: 4px;
    ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     --tw-ring-offset-width: 8px;
    ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
    /// }
    /// ```
    _8,
}

/// Utilities for setting the color of outline ring offsets.
/// 
/// <https://tailwindcss.com/docs/ring-offset-color>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "ring-offset")]
pub struct RingOffsetColor(Color);
