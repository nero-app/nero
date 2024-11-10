use typewind_macros::{Display, Parse};
use crate::customization::Color;

tailwind_types!(
    FontFamily, FontSize, FontSmoothing, FontStyle, FontWeight, FontVariantNumeric, 
    LetterSpacing, LineClamp, LineHeight,
    ListStyleImage, ListStylePosition, ListStyleType,
    TextAlign, TextColor, TextDecoration, TextDecorationColor, TextDecorationStyle, TextDecorationThickness, 
    TextUnderlineOffset, TextTransform, TextOverflow, TextWrap, TextIndent,
    VerticalAlign, Whitespace, WordBreak, Hyphens, Content
);

/// Utilities for controlling the font family of an element.
/// 
/// <https://tailwindcss.com/docs/font-family>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "font")]
pub enum FontFamily {
    /// ```css
    /// {
    ///     font-family: ui-sans-serif, system-ui, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";
    /// }
    /// ```
    Sans,
    /// ```css
    /// {
    ///     font-family: ui-serif, Georgia, Cambria, "Times New Roman", Times, serif;
    /// }
    /// ```
    Serif,
    /// ```css
    /// {
    ///     font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
    /// }
    /// ```
    Mono,
}

/// Utilities for controlling the font size of an element.
/// 
/// <https://tailwindcss.com/docs/font-size>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "text")]
pub enum FontSize {
    /// ```css
    /// {
    ///     font-size: 0.75rem; /* 12px */
    ///     line-height: 1rem; /* 16px */
    /// }
    /// ```
    Xs,
    /// ```css
    /// {
    ///     font-size: 0.875rem; /* 14px */
    ///     line-height: 1.25rem; /* 20px */
    /// }
    /// ```
    Sm,
    /// ```css
    /// {
    ///     font-size: 1rem; /* 16px */
    ///     line-height: 1.5rem; /* 24px */
    /// }
    /// ```
    Base,
    /// ```css
    /// {
    ///     font-size: 1.125rem; /* 18px */
    ///     line-height: 1.75rem; /* 28px */
    /// }
    /// ```
    Lg,
    /// ```css
    /// {
    ///     font-size: 1.25rem; /* 20px */
    ///     line-height: 1.75rem; /* 28px */
    /// }
    /// ```
    Xl,
    /// ```css
    /// {
    ///     font-size: 1.5rem; /* 24px */
    ///     line-height: 2rem; /* 32px */
    /// }
    /// ```
    _2xl,
    /// ```css
    /// {
    ///     font-size: 1.875rem; /* 30px */
    ///     line-height: 2.25rem; /* 36px */
    /// }
    /// ```
    _3xl,
    /// ```css
    /// {
    ///     font-size: 2.25rem; /* 36px */
    ///     line-height: 2.5rem; /* 40px */
    /// }
    /// ```
    _4xl,
    /// ```css
    /// {
    ///     font-size: 3rem; /* 48px */
    ///     line-height: 1;
    /// }
    /// ```
    _5xl,
    /// ```css
    /// {
    ///     font-size: 3.75rem; /* 60px */
    ///     line-height: 1;
    /// }
    /// ```
    _6xl,
    /// ```css
    /// {
    ///     font-size: 4.5rem; /* 72px */
    ///     line-height: 1;
    /// }
    /// ```
    _7xl,
    /// ```css
    /// {
    ///     font-size: 6rem; /* 96px */
    ///     line-height: 1;
    /// }
    /// ```
    _8xl,
    /// ```css
    /// {
    ///     font-size: 8rem; /* 128px */
    ///     line-height: 1;
    /// }
    /// ```
    _9xl,
}

/// Utilities for controlling the font smoothing of an element.
///
/// <https://tailwindcss.com/docs/font-smoothing>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum FontSmoothing {
    /// ```css
    /// {
    ///     -webkit-font-smoothing: antialiased;
    ///     -moz-osx-font-smoothing: grayscale;
    /// }
    /// ```
    Antialiased,
    /// ```css
    /// {
    ///     -webkit-font-smoothing: auto;
    ///     -moz-osx-font-smoothing: auto;
    /// }
    /// ```
    SubpixelAntialiased,
}

/// Utilities for controlling the style of text.
///
/// <https://tailwindcss.com/docs/font-style>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum FontStyle {
    /// ```css
    /// {
    ///     font-style: italic;
    /// }
    /// ```
    Italic,
    /// ```css
    /// {
    ///     font-style: normal;
    /// }
    /// ```
    NotItalic,
}

/// Utilities for controlling the font weight of an element.
///
/// <https://tailwindcss.com/docs/font-weight>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "font")]
pub enum FontWeight {
    /// ```css
    /// {
    ///     font-weight: 100;
    /// }
    /// ```
    Thin,
    /// ```css
    /// {
    ///     font-weight: 200;
    /// }
    /// ```
    ExtraLight,
    /// ```css
    /// {
    ///     font-weight: 300;
    /// }
    /// ```
    Light,
    /// ```css
    /// {
    ///     font-weight: 400;
    /// }
    /// ```
    Normal,
    /// ```css
    /// {
    ///     font-weight: 500;
    /// }
    /// ```
    Medium,
    /// ```css
    /// {
    ///     font-weight: 600;
    /// }
    /// ```
    Semibold,
    /// ```css
    /// {
    ///     font-weight: 700;
    /// }
    /// ```
    Bold,
    /// ```css
    /// {
    ///     font-weight: 800;
    /// }
    /// ```
    Extrabold,
    /// ```css
    /// {
    ///     font-weight: 900;
    /// }
    /// ```
    Black,
}

/// Utilities for controlling the variant of numbers.
///
/// <https://tailwindcss.com/docs/font-variant-numeric>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum FontVariantNumeric {
    /// ```css
    /// {
    ///     font-variant-numeric: normal;
    /// }
    /// ```
    NormalNums,
    /// ```css
    /// {
    ///     font-variant-numeric: ordinal;
    /// }
    /// ```
    Ordinal,
    /// ```css
    /// {
    ///     font-variant-numeric: slashed-zero;
    /// }
    /// ```
    SlashedZero,
    /// ```css
    /// {
    ///     font-variant-numeric: lining-nums;
    /// }
    /// ```
    LiningNums,
    /// ```css
    /// {
    ///     font-variant-numeric: oldstyle-nums;
    /// }
    /// ```
    OldstyleNums,
    /// ```css
    /// {
    ///     font-variant-numeric: proportional-nums;
    /// }
    /// ```
    ProportionalNums,
    /// ```css
    /// {
    ///     font-variant-numeric: tabular-nums;
    /// }
    /// ```
    TabularNums,
    /// ```css
    /// {
    ///     font-variant-numeric: diagonal-fractions;
    /// }
    /// ```
    DiagonalFractions,
    /// ```css
    /// {
    ///     font-variant-numeric: stacked-fractions;
    /// }
    /// ```
    StackedFractions,
}

/// Utilities for controlling the tracking (letter spacing) of an element.
///
/// <https://tailwindcss.com/docs/letter-spacing>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "tracking")]
pub enum LetterSpacing {
    /// ```css
    /// {
    ///     letter-spacing: -0.05em;
    /// }
    /// ```
    Tighter,
    /// ```css
    /// {
    ///     letter-spacing: -0.025em;
    /// }
    /// ```
    Tight,
    /// ```css
    /// {
    ///     letter-spacing: 0em;
    /// }
    /// ```
    Normal,
    /// ```css
    /// {
    ///     letter-spacing: 0.025em;
    /// }
    /// ```
    Wide,
    /// ```css
    /// {
    ///     letter-spacing: 0.05em;
    /// }
    /// ```
    Wider,
    /// ```css
    /// {
    ///     letter-spacing: 0.1em;
    /// }
    /// ```
    Widest,
}

/// Utilities for clamping text to a specific number of lines.
///
/// <https://tailwindcss.com/docs/line-clamp>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "line-clamp")]
pub enum LineClamp {
    /// ```css
    /// {
    ///     overflow: hidden;
    ///     display: -webkit-box;
    ///     -webkit-box-orient: vertical;
    ///     -webkit-line-clamp: 1;
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     overflow: hidden;
    ///     display: -webkit-box;
    ///     -webkit-box-orient: vertical;
    ///     -webkit-line-clamp: 2;
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     overflow: hidden;
    ///     display: -webkit-box;
    ///     -webkit-box-orient: vertical;
    ///     -webkit-line-clamp: 3;
    /// }
    /// ```
    _3,
    /// ```css
    /// {
    ///     overflow: hidden;
    ///     display: -webkit-box;
    ///     -webkit-box-orient: vertical;
    ///     -webkit-line-clamp: 4;
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     overflow: hidden;
    ///     display: -webkit-box;
    ///     -webkit-box-orient: vertical;
    ///     -webkit-line-clamp: 5;
    /// }
    /// ```
    _5,
    /// ```css
    /// {
    ///     overflow: hidden;
    ///     display: -webkit-box;
    ///     -webkit-box-orient: vertical;
    ///     -webkit-line-clamp: 6;
    /// }
    /// ```
    _6,
    /// ```css
    /// {
    ///     overflow: visible;
    ///     display: block;
    ///     -webkit-box-orient: horizontal;
    ///     -webkit-line-clamp: none;
    /// }
    /// ```
    None,
}

/// Utilities for controlling the leading (line height) of an element.
/// 
/// <https://tailwindcss.com/docs/line-height>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "leading")]
pub enum LineHeight {
    /// ```css
    /// {
    ///     line-height: .75rem; /* 12px */
    /// }
    /// ```
    _3,
    /// ```css
    /// {
    ///     line-height: 1rem; /* 16px */
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     line-height: 1.25rem; /* 20px */
    /// }
    /// ```
    _5,
    /// ```css
    /// {
    ///     line-height: 1.5rem; /* 24px */
    /// }
    /// ```
    _6,
    /// ```css
    /// {
    ///     line-height: 1.75rem; /* 28px */
    /// }
    /// ```
    _7,
    /// ```css
    /// {
    ///     line-height: 2rem; /* 32px */
    /// }
    /// ```
    _8,
    /// ```css
    /// {
    ///     line-height: 2.25rem; /* 36px */
    /// }
    /// ```
    _9,
    /// ```css
    /// {
    ///     line-height: 2.5rem; /* 40px */
    /// }
    /// ```
    _10,
    /// ```css
    /// {
    ///     line-height: 1;
    /// }
    /// ```
    None,
    /// ```css
    /// {
    ///     line-height: 1.25;
    /// }
    /// ```
    Tight,
    /// ```css
    /// {
    ///     line-height: 1.375;
    /// }
    /// ```
    Snug,
    /// ```css
    /// {
    ///     line-height: 1.5;
    /// }
    /// ```
    Normal,
    /// ```css
    /// {
    ///     line-height: 1.625;
    /// }
    /// ```
    Relaxed,
    /// ```css
    /// {
    ///     line-height: 2;
    /// }
    /// ```
    Loose,
}

/// Utilities for controlling the marker images for list items.
/// 
/// <https://tailwindcss.com/docs/list-style-image>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "list-image")]
pub enum ListStyleImage {
    /// ```css
    /// {
    ///     list-style-image: none;
    /// }
    /// ```
    None,
}

/// Utilities for controlling the position of bullets/numbers in lists.
/// 
/// <https://tailwindcss.com/docs/list-style-position>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "list")]
pub enum ListStylePosition {
    /// ```css
    /// {
    ///     list-style-position: inside;
    /// }
    /// ```
    Inside,
    /// ```css
    /// {
    ///     list-style-position: outside;
    /// }
    /// ```
    Outside,
}

/// Utilities for controlling the bullet/number style of a list.
/// 
/// <https://tailwindcss.com/docs/list-style-type>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "list")]
pub enum ListStyleType {
    /// ```css
    /// {
    ///     list-style-type: none;
    /// }
    /// ```
    None,
    /// ```css
    /// {
    ///     list-style-type: disc;
    /// }
    /// ```
    Disc,
    /// ```css
    /// {
    ///     list-style-type: decimal;
    /// }
    /// ```
    Decimal,
}

/// Utilities for controlling the alignment of text.
/// 
/// <https://tailwindcss.com/docs/text-align>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "text")]
pub enum TextAlign {
    /// ```css
    /// {
    ///     text-align: left;
    /// }
    /// ```
    Left,
    /// ```css
    /// {
    ///     text-align: center;
    /// }
    /// ```
    Center,
    /// ```css
    /// {
    ///     text-align: right;
    /// }
    /// ```
    Right,
    /// ```css
    /// {
    ///     text-align: justify;
    /// }
    /// ```
    Justify,
    /// ```css
    /// {
    ///     text-align: start;
    /// }
    /// ```
    Start,
    /// ```css
    /// {
    ///     text-align: end;
    /// }
    /// ```
    End,
}

/// Utilities for controlling the text color of an element.
///
/// <https://tailwindcss.com/docs/text-color>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "text")]
pub struct TextColor(pub Color);

/// Utilities for controlling the decoration of text.
/// 
/// <https://tailwindcss.com/docs/text-decoration>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum TextDecoration {
    /// ```css
    /// {
    ///     text-decoration-line: underline;
    /// }
    /// ```
    Underline,
    /// ```css
    /// {
    ///     text-decoration-line: overline;
    /// }
    /// ```
    Overline,
    /// ```css
    /// {
    ///     text-decoration-line: line-through;
    /// }
    /// ```
    LineThrough,
    /// ```css
    /// {
    ///     text-decoration-line: none;
    /// }
    /// ```
    NoUnderline,
}

/// Utilities for controlling the color of text decorations.
/// 
/// <https://tailwindcss.com/docs/text-decoration-color>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "decoration")]
pub struct TextDecorationColor(pub Color);

/// Utilities for controlling the style of text decorations.
/// 
/// <https://tailwindcss.com/docs/text-decoration-style>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "decoration")]
pub enum TextDecorationStyle {
    /// ```css
    /// {
    ///     text-decoration-style: solid;
    /// }
    /// ```
    Solid,
    /// ```css
    /// {
    ///     text-decoration-style: double;
    /// }
    /// ```
    Double,
    /// ```css
    /// {
    ///     text-decoration-style: dotted;
    /// }
    /// ```
    Dotted,
    /// ```css
    /// {
    ///     text-decoration-style: dashed;
    /// }
    /// ```
    Dashed,
    /// ```css
    /// {
    ///     text-decoration-style: wavy;
    /// }
    /// ```
    Wavy,
}

/// Utilities for controlling the thickness of text decorations.
/// 
/// <https://tailwindcss.com/docs/text-decoration-thickness>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "decoration")]
pub enum TextDecorationThickness {
    /// ```css
    /// {
    ///     text-decoration-thickness: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     text-decoration-thickness: from-font;
    /// }
    /// ```
    FromFont,
    /// ```css
    /// {
    ///     text-decoration-thickness: 0px;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     text-decoration-thickness: 1px;
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     text-decoration-thickness: 2px;
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     text-decoration-thickness: 4px;
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     text-decoration-thickness: 8px;
    /// }
    /// ```
    _8,
}

/// Utilities for controlling the offset of a text underline.
///
/// <https://tailwindcss.com/docs/text-underline-offset>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "underline-offset")]
pub enum TextUnderlineOffset {
    /// ```css
    /// {
    ///     text-underline-offset: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     text-underline-offset: 0px;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     text-underline-offset: 1px;
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     text-underline-offset: 2px;
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     text-underline-offset: 4px;
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     text-underline-offset: 8px;
    /// }
    /// ```
    _8,
}

/// Utilities for controlling the transformation of text.
/// 
/// <https://tailwindcss.com/docs/text-transform>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum TextTransform {
    /// ```css
    /// {
    ///     text-transform: uppercase;
    /// }
    /// ```
    Uppercase,
    /// ```css
    /// {
    ///     text-transform: lowercase;
    /// }
    /// ```
    Lowercase,
    /// ```css
    /// {
    ///     text-transform: capitalize;
    /// }
    /// ```
    Capitalize,
    /// ```css
    /// {
    ///     text-transform: none;
    /// }
    /// ```
    NormalCase,
}

/// Utilities for controlling text overflow in an element.
/// 
/// <https://tailwindcss.com/docs/text-overflow>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum TextOverflow {
    /// ```css
    /// {
    ///     overflow: hidden;
    ///     text-overflow: ellipsis;
    ///     white-space: nowrap;
    /// }
    /// ```
    Truncate,
    /// ```css
    /// {
    ///     text-overflow: ellipsis;
    /// }
    /// ```
    TextEllipsis,
    /// ```css
    /// {
    ///     text-overflow: clip;
    /// }
    /// ```
    TextClip
}

/// Utilities for controlling how text wraps within an element.
/// 
/// <https://tailwindcss.com/docs/text-wrap>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "text")]
pub enum TextWrap {
    /// ```css
    /// {
    ///     text-wrap: wrap;
    /// }
    /// ```
    Wrap,
    /// ```css
    /// {
    ///     text-wrap: nowrap;
    /// }
    /// ```
    Nowrap,
    /// ```css
    /// {
    ///     text-wrap: balance;
    /// }
    /// ```
    Balance,
    /// ```css
    /// {
    ///     text-wrap: pretty;
    /// }
    /// ```
    Pretty,
}

/// Utilities for controlling the amount of empty space shown before text in a block.
/// 
/// <https://tailwindcss.com/docs/text-indent>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "indent", replace(from = "_", to = "."))] 
pub enum TextIndent {
    /// ```css
    /// {
    ///     text-indent: 0px;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     text-indent: 1px;
    /// }
    /// ```
    Px,
    /// ```css
    /// {
    ///     text-indent: 0.125rem; /* 2px */
    /// }
    /// ```
    _0_5,
    /// ```css
    /// {
    ///     text-indent: 0.25rem; /* 4px */
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     text-indent: 0.375rem; /* 6px */
    /// }
    /// ```
    _1_5,
    /// ```css
    /// {
    ///     text-indent: 0.5rem; /* 8px */
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     text-indent: 0.625rem; /* 10px */
    /// }
    /// ```
    _2_5,
    /// ```css
    /// {
    ///     text-indent: 0.75rem; /* 12px */
    /// }
    /// ```
    _3,
    /// ```css
    /// {
    ///     text-indent: 0.875rem; /* 14px */
    /// }
    /// ```
    _3_5,
    /// ```css
    /// {
    ///     text-indent: 1rem; /* 16px */
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     text-indent: 1.25rem; /* 20px */
    /// }
    /// ```
    _5,
    /// ```css
    /// {
    ///     text-indent: 1.5rem; /* 24px */
    /// }
    /// ```
    _6,
    /// ```css
    /// {
    ///     text-indent: 1.75rem; /* 28px */
    /// }
    /// ```
    _7,
    /// ```css
    /// {
    ///     text-indent: 2rem; /* 32px */
    /// }
    /// ```
    _8,
    /// ```css
    /// {
    ///     text-indent: 2.25rem; /* 36px */
    /// }
    /// ```
    _9,
    /// ```css
    /// {
    ///     text-indent: 2.5rem; /* 40px */
    /// }
    /// ```
    _10,
    /// ```css
    /// {
    ///     text-indent: 2.75rem; /* 44px */
    /// }
    /// ```
    _11,
    /// ```css
    /// {
    ///     text-indent: 3rem; /* 48px */
    /// }
    /// ```
    _12,
    /// ```css
    /// {
    ///     text-indent: 3.5rem; /* 56px */
    /// }
    /// ```
    _14,
    /// ```css
    /// {
    ///     text-indent: 4rem; /* 64px */
    /// }
    /// ```
    _16,
    /// ```css
    /// {
    ///     text-indent: 5rem; /* 80px */
    /// }
    /// ```
    _20,
    /// ```css
    /// {
    ///     text-indent: 6rem; /* 96px */
    /// }
    /// ```
    _24,
    /// ```css
    /// {
    ///     text-indent: 7rem; /* 112px */
    /// }
    /// ```
    _28,
    /// ```css
    /// {
    ///     text-indent: 8rem; /* 128px */
    /// }
    /// ```
    _32,
    /// ```css
    /// {
    ///     text-indent: 9rem; /* 144px */
    /// }
    /// ```
    _36,
    /// ```css
    /// {
    ///     text-indent: 10rem; /* 160px */
    /// }
    /// ```
    _40,
    /// ```css
    /// {
    ///     text-indent: 11rem; /* 176px */
    /// }
    /// ```
    _44,
    /// ```css
    /// {
    ///     text-indent: 12rem; /* 192px */
    /// }
    /// ```
    _48,
    /// ```css
    /// {
    ///     text-indent: 13rem; /* 208px */
    /// }
    /// ```
    _52,
    /// ```css
    /// {
    ///     text-indent: 14rem; /* 224px */
    /// }
    /// ```
    _56,
    /// ```css
    /// {
    ///     text-indent: 15rem; /* 240px */
    /// }
    /// ```
    _60,
    /// ```css
    /// {
    ///     text-indent: 16rem; /* 256px */
    /// }
    /// ```
    _64,
    /// ```css
    /// {
    ///     text-indent: 18rem; /* 288px */
    /// }
    /// ```
    _72,
    /// ```css
    /// {
    ///     text-indent: 20rem; /* 320px */
    /// }
    /// ```
    _80,
    /// ```css
    /// {
    ///     text-indent: 24rem; /* 384px */
    /// }
    /// ```
    _96,
}

/// Utilities for controlling the vertical alignment of an inline or table-cell box.
///
/// <https://tailwindcss.com/docs/vertical-align>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "align")]
pub enum VerticalAlign {
    /// ```css
    /// {
    ///     vertical-align: baseline;
    /// }
    /// ```
    Baseline,
    /// ```css
    /// {
    ///     vertical-align: top;
    /// }
    /// ```
    Top,
    /// ```css
    /// {
    ///     vertical-align: middle;
    /// }
    /// ```
    Middle,
    /// ```css
    /// {
    ///     vertical-align: bottom;
    /// }
    /// ```
    Bottom,
    /// ```css
    /// {
    ///     vertical-align: text-top;
    /// }
    /// ```
    TextTop,
    /// ```css
    /// {
    ///     vertical-align: text-bottom;
    /// }
    /// ```
    TextBottom,
    /// ```css
    /// {
    ///     vertical-align: sub;
    /// }
    /// ```
    Sub,
    /// ```css
    /// {
    ///     vertical-align: super;
    /// }
    /// ```
    Super,
}

/// Utilities for controlling an element's white-space property.
/// 
/// <https://tailwindcss.com/docs/whitespace>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "whitespace")]
pub enum Whitespace {
    /// ```css
    /// {
    ///     white-space: normal;
    /// }
    /// ```
    Normal,
    /// ```css
    /// {
    ///     white-space: nowrap;
    /// }
    /// ```
    Nowrap,
    /// ```css
    /// {
    ///     white-space: pre;
    /// }
    /// ```
    Pre,
    /// ```css
    /// {
    ///     white-space: pre-line;
    /// }
    /// ```
    PreLine,
    /// ```css
    /// {
    ///     white-space: pre-wrap;
    /// }
    /// ```
    PreWrap,
    /// ```css
    /// {
    ///     white-space: break-spaces;
    /// }
    /// ```
    BreakSpaces,
}

/// Utilities for controlling word breaks in an element.
/// 
/// <https://tailwindcss.com/docs/word-break>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "break")]
pub enum WordBreak {
    /// ```css
    /// {
    ///     overflow-wrap: normal;
    ///     word-break: normal;
    /// }
    /// ```
    Normal,
    /// ```css
    /// {
    ///     overflow-wrap: break-word;
    /// }
    /// ```
    Words,
    /// ```css
    /// {
    ///     word-break: break-all;
    /// }
    /// ```
    All,
    /// ```css
    /// {
    ///     word-break: keep-all;
    /// }
    /// ```
    Keep,
}

/// Utilities for controlling how words should be hyphenated.
/// 
/// <https://tailwindcss.com/docs/hyphens>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "hyphens")]
pub enum Hyphens {
    /// ```css
    /// {
    ///     hyphens: none;
    /// }
    /// ```
    None,
    /// ```css
    /// {
    ///     hyphens: manual;
    /// }
    /// ```
    Manual,
    /// ```css
    /// {
    ///     hyphens: auto;
    /// }
    /// ```
    Auto,
}

/// Utilities for controlling the content of the before and after pseudo-elements.
/// 
/// <https://tailwindcss.com/docs/content>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "content")]
pub enum Content {
    /// ```css
    /// {
    ///     content: none;
    /// }
    /// ```
    None,
}
