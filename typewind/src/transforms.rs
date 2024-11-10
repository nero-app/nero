use typewind_macros::{Display, Parse};

tailwind_types!(Scale, Rotate, Translate, Skew, TransformOrigin);

/// Utilities for scaling elements with transform.
/// 
/// <https://tailwindcss.com/docs/scale>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "scale")]
pub enum Scale {
    /// ```css
    /// {
    ///     transform: scale(0);
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     transform: scaleX(0);
    /// }
    /// ```
    X0,
    /// ```css
    /// {
    ///     transform: scaleY(0);
    /// }
    /// ```
    Y0,
    /// ```css
    /// {
    ///     transform: scale(.5);
    /// }
    /// ```
    _50,
    /// ```css
    /// {
    ///     transform: scaleX(.5);
    /// }
    /// ```
    X50,
    /// ```css
    /// {
    ///     transform: scaleY(.5);
    /// }
    /// ```
    Y50,
    /// ```css
    /// {
    ///     transform: scale(.75);
    /// }
    /// ```
    _75,
    /// ```css
    /// {
    ///     transform: scaleX(.75);
    /// }
    /// ```
    X75,
    /// ```css
    /// {
    ///     transform: scaleY(.75);
    /// }
    /// ```
    Y75,
    /// ```css
    /// {
    ///     transform: scale(.9);
    /// }
    /// ```
    _90,
    /// ```css
    /// {
    ///     transform: scaleX(.9);
    /// }
    /// ```
    X90,
    /// ```css
    /// {
    ///     transform: scaleY(.9);
    /// }
    /// ```
    Y90,
    /// ```css
    /// {
    ///     transform: scale(.95);
    /// }
    /// ```
    _95,
    /// ```css
    /// {
    ///     transform: scaleX(.95);
    /// }
    /// ```
    X95,
    /// ```css
    /// {
    ///     transform: scaleY(.95);
    /// }
    /// ```
    Y95,
    /// ```css
    /// {
    ///     transform: scale(1);
    /// }
    /// ```
    _100,
    /// ```css
    /// {
    ///     transform: scaleX(1);
    /// }
    /// ```
    X100,
    /// ```css
    /// {
    ///     transform: scaleY(1);
    /// }
    /// ```
    Y100,
    /// ```css
    /// {
    ///     transform: scale(1.05);
    /// }
    /// ```
    _105,
    /// ```css
    /// {
    ///     transform: scaleX(1.05);
    /// }
    /// ```
    X105,
    /// ```css
    /// {
    ///     transform: scaleY(1.05);
    /// }
    /// ```
    Y105,
    /// ```css
    /// {
    ///     transform: scale(1.1);
    /// }
    /// ```
    _110,
    /// ```css
    /// {
    ///     transform: scaleX(1.1);
    /// }
    /// ```
    X110,
    /// ```css
    /// {
    ///     transform: scaleY(1.1);
    /// }
    /// ```
    Y110,
    /// ```css
    /// {
    ///     transform: scale(1.25);
    /// }
    /// ```
    _125,
    /// ```css
    /// {
    ///     transform: scaleX(1.25);
    /// }
    /// ```
    X125,
    /// ```css
    /// {
    ///     transform: scaleY(1.25);
    /// }
    /// ```
    Y125,
    /// ```css
    /// {
    ///     transform: scale(1.5);
    /// }
    /// ```
    _150,
    /// ```css
    /// {
    ///     transform: scaleX(1.5);
    /// }
    /// ```
    X150,
    /// ```css
    /// {
    ///     transform: scaleY(1.5);
    /// }
    /// ```
    Y150,
}

/// Utilities for rotating elements with transform.
/// 
/// <https://tailwindcss.com/docs/rotate>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "rotate")]
pub enum Rotate {
    /// ```css
    /// {
    ///     transform: rotate(0deg);
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     transform: rotate(1deg);
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     transform: rotate(2deg);
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     transform: rotate(3deg);
    /// }
    /// ```
    _3,
    /// ```css
    /// {
    ///     transform: rotate(6deg);
    /// }
    /// ```
    _6,
    /// ```css
    /// {
    ///     transform: rotate(12deg);
    /// }
    /// ```
    _12,
    /// ```css
    /// {
    ///     transform: rotate(45deg);
    /// }
    /// ```
    _45,
    /// ```css
    /// {
    ///     transform: rotate(90deg);
    /// }
    /// ```
    _90,
    /// ```css
    /// {
    ///     transform: rotate(180deg);
    /// }
    /// ```
    _180,
}

/// Utilities for translating elements with transform.
/// 
/// <https://tailwindcss.com/docs/translate>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "translate")]
pub enum Translate {
    /// ```css
    /// {
    ///     transform: translateX(0px);
    /// }
    /// ```
    X0,
    /// ```css
    /// {
    ///     transform: translateY(0px);
    /// }
    /// ```
    Y0,
    /// ```css
    /// {
    ///     transform: translateX(1px);
    /// }
    /// ```
    XPx,
    /// ```css
    /// {
    ///     transform: translateY(1px);
    /// }
    /// ```
    YPx,
    /// ```css
    /// {
    ///     transform: translateX(0.125rem);
    /// }
    /// ```
    X0_5,
    /// ```css
    /// {
    ///     transform: translateY(0.125rem);
    /// }
    /// ```
    Y0_5,
    /// ```css
    /// {
    ///     transform: translateX(0.25rem);
    /// }
    /// ```
    X1,
    /// ```css
    /// {
    ///     transform: translateY(0.25rem);
    /// }
    /// ```
    Y1,
    /// ```css
    /// {
    ///     transform: translateX(0.375rem);
    /// }
    /// ```
    X1_5,
    /// ```css
    /// {
    ///     transform: translateY(0.375rem);
    /// }
    /// ```
    Y1_5,
    /// ```css
    /// {
    ///     transform: translateX(0.5rem);
    /// }
    /// ```
    X2,
    /// ```css
    /// {
    ///     transform: translateY(0.5rem);
    /// }
    /// ```
    Y2,
    /// ```css
    /// {
    ///     transform: translateX(0.625rem);
    /// }
    /// ```
    X2_5,
    /// ```css
    /// {
    ///     transform: translateY(0.625rem);
    /// }
    /// ```
    Y2_5,
    /// ```css
    /// {
    ///     transform: translateX(0.75rem);
    /// }
    /// ```
    X3,
    /// ```css
    /// {
    ///     transform: translateY(0.75rem);
    /// }
    /// ```
    Y3,
    /// ```css
    /// {
    ///     transform: translateX(0.875rem);
    /// }
    /// ```
    X3_5,
    /// ```css
    /// {
    ///     transform: translateY(0.875rem);
    /// }
    /// ```
    Y3_5,
    /// ```css
    /// {
    ///     transform: translateX(1rem);
    /// }
    /// ```
    X4,
    /// ```css
    /// {
    ///     transform: translateY(1rem);
    /// }
    /// ```
    Y4,
    /// ```css
    /// {
    ///     transform: translateX(1.25rem);
    /// }
    /// ```
    X5,
    /// ```css
    /// {
    ///     transform: translateY(1.25rem);
    /// }
    /// ```
    Y5,
    /// ```css
    /// {
    ///     transform: translateX(1.5rem);
    /// }
    /// ```
    X6,
    /// ```css
    /// {
    ///     transform: translateY(1.5rem);
    /// }
    /// ```
    Y6,
    /// ```css
    /// {
    ///     transform: translateX(1.75rem);
    /// }
    /// ```
    X7,
    /// ```css
    /// {
    ///     transform: translateY(1.75rem);
    /// }
    /// ```
    Y7,
    /// ```css
    /// {
    ///     transform: translateX(2rem);
    /// }
    /// ```
    X8,
    /// ```css
    /// {
    ///     transform: translateY(2rem);
    /// }
    /// ```
    Y8,
    /// ```css
    /// {
    ///     transform: translateX(2.25rem);
    /// }
    /// ```
    X9,
    /// ```css
    /// {
    ///     transform: translateY(2.25rem);
    /// }
    /// ```
    Y9,
    /// ```css
    /// {
    ///     transform: translateX(2.5rem);
    /// }
    /// ```
    X10,
    /// ```css
    /// {
    ///     transform: translateY(2.5rem);
    /// }
    /// ```
    Y10,
    /// ```css
    /// {
    ///     transform: translateX(2.75rem);
    /// }
    /// ```
    X11,
    /// ```css
    /// {
    ///     transform: translateY(2.75rem);
    /// }
    /// ```
    Y11,
    /// ```css
    /// {
    ///     transform: translateX(3rem);
    /// }
    /// ```
    X12,
    /// ```css
    /// {
    ///     transform: translateY(3rem);
    /// }
    /// ```
    Y12,
    /// ```css
    /// {
    ///     transform: translateX(3.5rem);
    /// }
    /// ```
    X14,
    /// ```css
    /// {
    ///     transform: translateY(3.5rem);
    /// }
    /// ```
    Y14,
    /// ```css
    /// {
    ///     transform: translateX(4rem);
    /// }
    /// ```
    X16,
    /// ```css
    /// {
    ///     transform: translateY(4rem);
    /// }
    /// ```
    Y16,
    /// ```css
    /// {
    ///     transform: translateX(5rem);
    /// }
    /// ```
    X20,
    /// ```css
    /// {
    ///     transform: translateY(5rem);
    /// }
    /// ```
    Y20,
    /// ```css
    /// {
    ///     transform: translateX(6rem);
    /// }
    /// ```
    X24,
    /// ```css
    /// {
    ///     transform: translateY(6rem);
    /// }
    /// ```
    Y24,
    /// ```css
    /// {
    ///     transform: translateX(7rem);
    /// }
    /// ```
    X28,
    /// ```css
    /// {
    ///     transform: translateY(7rem);
    /// }
    /// ```
    Y28,
    /// ```css
    /// {
    ///     transform: translateX(8rem);
    /// }
    /// ```
    X32,
    /// ```css
    /// {
    ///     transform: translateY(8rem);
    /// }
    /// ```
    Y32,
    /// ```css
    /// {
    ///     transform: translateX(9rem);
    /// }
    /// ```
    X36,
    /// ```css
    /// {
    ///     transform: translateY(9rem);
    /// }
    /// ```
    Y36,
    /// ```css
    /// {
    ///     transform: translateX(10rem);
    /// }
    /// ```
    X40,
    /// ```css
    /// {
    ///     transform: translateY(10rem);
    /// }
    /// ```
    Y40,
    /// ```css
    /// {
    ///     transform: translateX(11rem);
    /// }
    /// ```
    X44,
    /// ```css
    /// {
    ///     transform: translateY(11rem);
    /// }
    /// ```
    Y44,
    /// ```css
    /// {
    ///     transform: translateX(12rem);
    /// }
    /// ```
    X48,
    /// ```css
    /// {
    ///     transform: translateY(12rem);
    /// }
    /// ```
    Y48,
    /// ```css
    /// {
    ///     transform: translateX(13rem);
    /// }
    /// ```
    X52,
    /// ```css
    /// {
    ///     transform: translateY(13rem);
    /// }
    /// ```
    Y52,
    /// ```css
    /// {
    ///     transform: translateX(14rem);
    /// }
    /// ```
    X56,
    /// ```css
    /// {
    ///     transform: translateY(14rem);
    /// }
    /// ```
    Y56,
    /// ```css
    /// {
    ///     transform: translateX(15rem);
    /// }
    /// ```
    X60,
    /// ```css
    /// {
    ///     transform: translateY(15rem);
    /// }
    /// ```
    Y60,
    /// ```css
    /// {
    ///     transform: translateX(16rem);
    /// }
    /// ```
    X64,
    /// ```css
    /// {
    ///     transform: translateY(16rem);
    /// }
    /// ```
    Y64,
    /// ```css
    /// {
    ///     transform: translateX(18rem);
    /// }
    /// ```
    X72,
    /// ```css
    /// {
    ///     transform: translateY(18rem);
    /// }
    /// ```
    Y72,
    /// ```css
    /// {
    ///     transform: translateX(20rem);
    /// }
    /// ```
    X80,
    /// ```css
    /// {
    ///     transform: translateY(20rem);
    /// }
    /// ```
    Y80,
    /// ```css
    /// {
    ///     transform: translateX(24rem);
    /// }
    /// ```
    X96,
    /// ```css
    /// {
    ///     transform: translateY(24rem);
    /// }
    /// ```
    Y96,
    /// ```css
    /// {
    ///     transform: translateX(50%);
    /// }
    /// ```
    X1over2,
    /// ```css
    /// {
    ///     transform: translateX(33.333333%);
    /// }
    /// ```
    X1over3,
    /// ```css
    /// {
    ///     transform: translateX(66.666667%);
    /// }
    /// ```
    X2over3,
    /// ```css
    /// {
    ///     transform: translateX(25%);
    /// }
    /// ```
    X1over4,
    /// ```css
    /// {
    ///     transform: translateX(50%);
    /// }
    /// ```
    X2over4,
    /// ```css
    /// {
    ///     transform: translateX(75%);
    /// }
    /// ```
    X3over4,
    /// ```css
    /// {
    ///     transform: translateX(100%);
    /// }
    /// ```
    XFull,
    /// ```css
    /// {
    ///     transform: translateY(50%);
    /// }
    /// ```
    Y1over2,
    /// ```css
    /// {
    ///     transform: translateY(33.333333%);
    /// }
    /// ```
    Y1over3,
    /// ```css
    /// {
    ///     transform: translateY(66.666667%);
    /// }
    /// ```
    Y2over3,
    /// ```css
    /// {
    ///     transform: translateY(25%);
    /// }
    /// ```
    Y1over4,
    /// ```css
    /// {
    ///     transform: translateY(50%);
    /// }
    /// ```
    Y2over4,
    /// ```css
    /// {
    ///     transform: translateY(75%);
    /// }
    /// ```
    Y3over4,
    /// ```css
    /// {
    ///     transform: translateY(100%);
    /// }
    /// ```
    YFull,
}

/// Utilities for skewing elements with transform.
/// 
/// <https://tailwindcss.com/docs/skew>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "skew")]
pub enum Skew {
    /// ```css
    /// {
    ///     transform: skewX(0deg);
    /// }
    /// ```
    X0,
    /// ```css
    /// {
    ///     transform: skewY(0deg);
    /// }
    /// ```
    Y0,
    /// ```css
    /// {
    ///     transform: skewX(1deg);
    /// }
    /// ```
    X1,
    /// ```css
    /// {
    ///     transform: skewY(1deg);
    /// }
    /// ```
    Y1,
    /// ```css
    /// {
    ///     transform: skewX(2deg);
    /// }
    /// ```
    X2,
    /// ```css
    /// {
    ///     transform: skewY(2deg);
    /// }
    /// ```
    Y2,
    /// ```css
    /// {
    ///     transform: skewX(3deg);
    /// }
    /// ```
    X3,
    /// ```css
    /// {
    ///     transform: skewY(3deg);
    /// }
    /// ```
    Y3,
    /// ```css
    /// {
    ///     transform: skewX(6deg);
    /// }
    /// ```
    X6,
    /// ```css
    /// {
    ///     transform: skewY(6deg);
    /// }
    /// ```
    Y6,
    /// ```css
    /// {
    ///     transform: skewX(12deg);
    /// }
    /// ```
    X12,
    /// ```css
    /// {
    ///     transform: skewY(12deg);
    /// }
    /// ```
    Y12,
}

/// Utilities for specifying the origin for an element's transformations.
/// 
/// <https://tailwindcss.com/docs/transform-origin>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "origin")]
pub enum TransformOrigin {
    /// ```css
    /// {
    ///     transform-origin: center;
    /// }
    /// ```
    Center,
    /// ```css
    /// {
    ///     transform-origin: top;
    /// }
    /// ```
    Top,
    /// ```css
    /// {
    ///     transform-origin: top right;
    /// }
    /// ```
    TopRight,
    /// ```css
    /// {
    ///     transform-origin: right;
    /// }
    /// ```
    Right,
    /// ```css
    /// {
    ///     transform-origin: bottom right;
    /// }
    /// ```
    BottomRight,
    /// ```css
    /// {
    ///     transform-origin: bottom;
    /// }
    /// ```
    Bottom,
    /// ```css
    /// {
    ///     transform-origin: bottom left;
    /// }
    /// ```
    BottomLeft,
    /// ```css
    /// {
    ///     transform-origin: left;
    /// }
    /// ```
    Left,
    /// ```css
    /// {
    ///     transform-origin: top left;
    /// }
    /// ```
    TopLeft,
}

