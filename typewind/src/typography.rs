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
    /// `font-family: ui-sans-serif, system-ui, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";`
    Sans,
    /// `font-family: ui-serif, Georgia, Cambria, "Times New Roman", Times, serif;`
    Serif,
    /// `font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;`
    Mono,
}

/// Utilities for controlling the font size of an element.
/// 
/// <https://tailwindcss.com/docs/font-size>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "text")]
pub enum FontSize {
    /// `font-size: 0.75rem; /* 12px */`
    /// 
    /// `line-height: 1rem; /* 16px */`
    Xs,
    /// `font-size: 0.875rem; /* 14px */`
    /// 
    /// `line-height: 1.25rem; /* 20px */`
    Sm,
    /// `font-size: 1rem; /* 16px */`
    /// 
    /// `line-height: 1.5rem; /* 24px */`
    Base,
    /// `font-size: 1.125rem; /* 18px */`
    /// 
    /// `line-height: 1.75rem; /* 28px */`
    Lg,
    /// `font-size: 1.25rem; /* 20px */`
    /// 
    /// `line-height: 1.75rem; /* 28px */`
    Xl,
    /// `font-size: 1.5rem; /* 24px */`
    /// 
    /// `line-height: 2rem; /* 32px */`
    _2xl,
    /// `font-size: 1.875rem; /* 30px */`
    /// 
    /// `line-height: 2.25rem; /* 36px */`
    _3xl,
    /// `font-size: 2.25rem; /* 36px */`
    /// 
    /// `line-height: 2.5rem; /* 40px */`
    _4xl,
    /// `font-size: 3rem; /* 48px */`
    /// 
    /// `line-height: 1;`
    _5xl,
    /// `font-size: 3.75rem; /* 60px */`
    /// 
    /// `line-height: 1;`
    _6xl,
    /// `font-size: 4.5rem; /* 72px */`
    /// 
    /// `line-height: 1;`
    _7xl,
    /// `font-size: 6rem; /* 96px */`
    /// 
    /// `line-height: 1;`
    _8xl,
    /// `font-size: 8rem; /* 128px */`
    /// 
    /// `line-height: 1;`
    _9xl,
}

/// Utilities for controlling the font smoothing of an element.
///
/// <https://tailwindcss.com/docs/font-smoothing>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum FontSmoothing {
    /// `-webkit-font-smoothing: antialiased;`
    /// 
    /// `-moz-osx-font-smoothing: grayscale;`
    Antialiased,
    /// `-webkit-font-smoothing: auto;`
    /// 
    /// `-moz-osx-font-smoothing: auto;`
    SubpixelAntialiased,
}

/// Utilities for controlling the style of text.
///
/// <https://tailwindcss.com/docs/font-style>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum FontStyle {
    /// `font-style: italic;`
    Italic,
    /// `font-style: normal;`
    NotItalic,
}

/// Utilities for controlling the font weight of an element.
///
/// <https://tailwindcss.com/docs/font-weight>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "font")]
pub enum FontWeight {
    /// `font-weight: 100;`
    Thin,
    /// `font-weight: 200;`
    ExtraLight,
    /// `font-weight: 300;`
    Light,
    /// `font-weight: 400;`
    Normal,
    /// `font-weight: 500;`
    Medium,
    /// `font-weight: 600;`
    Semibold,
    /// `font-weight: 700;`
    Bold,
    /// `font-weight: 800;`
    Extrabold,
    /// `font-weight: 900;`
    Black,
}

/// Utilities for controlling the variant of numbers.
///
/// <https://tailwindcss.com/docs/font-variant-numeric>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum FontVariantNumeric {
    /// `font-variant-numeric: normal;`
    NormalNums,
    /// `font-variant-numeric: ordinal;`
    Ordinal,
    /// `font-variant-numeric: slashed-zero;`
    SlashedZero,
    /// `font-variant-numeric: lining-nums;`
    LiningNums,
    /// `font-variant-numeric: oldstyle-nums;`
    OldstyleNums,
    /// `font-variant-numeric: proportional-nums;`
    ProportionalNums,
    /// `font-variant-numeric: tabular-nums;`
    TabularNums,
    /// `font-variant-numeric: diagonal-fractions;`
    DiagonalFractions,
    /// `font-variant-numeric: stacked-fractions;`
    StackedFractions,
}

/// Utilities for controlling the tracking (letter spacing) of an element.
///
/// <https://tailwindcss.com/docs/letter-spacing>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "tracking")]
pub enum LetterSpacing {
    /// `letter-spacing: -0.05em;`
    Tighter,
    /// `letter-spacing: -0.025em;`
    Tight,
    /// `letter-spacing: 0em;`
    Normal,
    /// `letter-spacing: 0.025em;`
    Wide,
    /// `letter-spacing: 0.05em;`
    Wider,
    /// `letter-spacing: 0.1em;`
    Widest,
}

/// Utilities for clamping text to a specific number of lines.
///
/// <https://tailwindcss.com/docs/line-clamp>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "line-clamp")]
pub enum LineClamp {
    /// `overflow: hidden;`
    /// 
    /// `display: -webkit-box;`
    /// 
    /// `-webkit-box-orient: vertical;`
    /// 
    /// `-webkit-line-clamp: 1;`
    _1,
    /// `overflow: hidden;`
    /// 
    /// `display: -webkit-box;`
    /// 
    /// `-webkit-box-orient: vertical;`
    /// 
    /// `-webkit-line-clamp: 2;`
    _2,
    /// `overflow: hidden;`
    /// 
    /// `display: -webkit-box;`
    /// 
    /// `-webkit-box-orient: vertical;`
    /// 
    /// `-webkit-line-clamp: 3;`
    _3,
    /// `overflow: hidden;`
    /// 
    /// `display: -webkit-box;`
    /// 
    /// `-webkit-box-orient: vertical;`
    /// 
    /// `-webkit-line-clamp: 4;`
    _4,
    /// `overflow: hidden;`
    /// 
    /// `display: -webkit-box;`
    /// 
    /// `-webkit-box-orient: vertical;`
    /// 
    /// `-webkit-line-clamp: 5;`
    _5,
    /// `overflow: hidden;`
    /// 
    /// `display: -webkit-box;`
    /// 
    /// `-webkit-box-orient: vertical;`
    /// 
    /// `-webkit-line-clamp: 6;`
    _6,
    /// `overflow: visible;`
    /// 
    /// `display: block;`
    /// 
    /// `-webkit-box-orient: horizontal;`
    /// 
    /// `-webkit-line-clamp: none;`
    None,
}

/// Utilities for controlling the leading (line height) of an element.
/// 
/// <https://tailwindcss.com/docs/line-height>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "leading")]
pub enum LineHeight {
    /// `line-height: .75rem; /* 12px */`
    _3,
    /// `line-height: 1rem; /* 16px */`
    _4,
    /// `line-height: 1.25rem; /* 20px */`
    _5,
    /// `line-height: 1.5rem; /* 24px */`
    _6,
    /// `line-height: 1.75rem; /* 28px */`
    _7,
    /// `line-height: 2rem; /* 32px */`
    _8,
    /// `line-height: 2.25rem; /* 36px */`
    _9,
    /// `line-height: 2.5rem; /* 40px */`
    _10,
    /// `line-height: 1;`
    None,
    /// `line-height: 1.25;`
    Tight,
    /// `line-height: 1.375;`
    Snug,
    /// `line-height: 1.5;`
    Normal,
    /// `line-height: 1.625;`
    Relaxed,
    /// `line-height: 2;`
    Loose,
}

/// Utilities for controlling the marker images for list items.
/// 
/// <https://tailwindcss.com/docs/list-style-image>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "list-image")]
pub enum ListStyleImage {
    /// `list-style-image: none;`
    None,
}

/// Utilities for controlling the position of bullets/numbers in lists.
/// 
/// <https://tailwindcss.com/docs/list-style-position>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "list")]
pub enum ListStylePosition {
    /// `list-style-position: inside;`
    Inside,
    /// `list-style-position: outside;`
    Outside,
}

/// Utilities for controlling the bullet/number style of a list.
/// 
/// <https://tailwindcss.com/docs/list-style-type>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "list")]
pub enum ListStyleType {
    /// `list-style-type: none;`
    None,
    /// `list-style-type: disc;`
    Disc,
    /// `list-style-type: decimal;`
    Decimal,
}

/// Utilities for controlling the alignment of text.
/// 
/// <https://tailwindcss.com/docs/text-align>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "text")]
pub enum TextAlign {
    /// `text-align: left;`
    Left,
    /// `text-align: center;`
    Center,
    /// `text-align: right;`
    Right,
    /// `text-align: justify;`
    Justify,
    /// `text-align: start;`
    Start,
    /// `text-align: end;`
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
    /// `text-decoration-line: underline;`
    Underline,
    /// `text-decoration-line: overline;`
    Overline,
    /// `text-decoration-line: line-through;`
    LineThrough,
    /// `text-decoration-line: none;`
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
    /// `text-decoration-style: solid;`
    Solid,
    /// `text-decoration-style: double;`
    Double,
    /// `text-decoration-style: dotted;`
    Dotted,
    /// `text-decoration-style: dashed;`
    Dashed,
    /// `text-decoration-style: wavy;`
    Wavy,
}

/// Utilities for controlling the thickness of text decorations.
/// 
/// <https://tailwindcss.com/docs/text-decoration-thickness>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "decoration")]
pub enum TextDecorationThickness {
    /// `text-decoration-thickness: auto;`
    Auto,
    /// `text-decoration-thickness: from-font;`
    FromFont,
    /// `text-decoration-thickness: 0px;`
    _0,
    /// `text-decoration-thickness: 1px;`
    _1,
    /// `text-decoration-thickness: 2px;`
    _2,
    /// `text-decoration-thickness: 4px;`
    _4,
    /// `text-decoration-thickness: 8px;`
    _8,
}

/// Utilities for controlling the offset of a text underline.
///
/// <https://tailwindcss.com/docs/text-underline-offset>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "underline-offset")]
pub enum TextUnderlineOffset {
    /// `text-underline-offset: auto;`
    Auto,
    /// `text-underline-offset: 0px;`
    _0,
    /// `text-underline-offset: 1px;`
    _1,
    /// `text-underline-offset: 2px;`
    _2,
    /// `text-underline-offset: 4px;`
    _4,
    /// `text-underline-offset: 8px;`
    _8,
}

/// Utilities for controlling the transformation of text.
/// 
/// <https://tailwindcss.com/docs/text-transform>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum TextTransform {
    /// `text-transform: uppercase;`
    Uppercase,
    /// `text-transform: lowercase;`
    Lowercase,
    /// `text-transform: capitalize;`
    Capitalize,
    /// `text-transform: none;`
    NormalCase,
}

/// Utilities for controlling text overflow in an element.
/// 
/// <https://tailwindcss.com/docs/text-overflow>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum TextOverflow {
    /// `overflow: hidden;`
    /// 
    /// `text-overflow: ellipsis;`
    /// 
    /// `white-space: nowrap;`
    Truncate,
    /// `text-overflow: ellipsis;`
    TextEllipsis,
    /// `text-overflow: clip;`
    TextClip
}

/// Utilities for controlling how text wraps within an element.
/// 
/// <https://tailwindcss.com/docs/text-wrap>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "text")]
pub enum TextWrap {
    /// `text-wrap: wrap;`
    Wrap,
    /// `text-wrap: nowrap;`
    Nowrap,
    /// `text-wrap: balance;`
    Balance,
    /// `text-wrap: pretty;`
    Pretty,
}

/// Utilities for controlling the amount of empty space shown before text in a block.
/// 
/// <https://tailwindcss.com/docs/text-indent>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "indent", replace(from = "_", to = "."))] 
pub enum TextIndent {
    /// `text-indent: 0px;`
    _0,
    /// `text-indent: 1px;`
    Px,
    /// `text-indent: 0.125rem; /* 2px */`
    _0_5,
    /// `text-indent: 0.25rem; /* 4px */`
    _1,
    /// `text-indent: 0.375rem; /* 6px */`
    _1_5,
    /// `text-indent: 0.5rem; /* 8px */`
    _2,
    /// `text-indent: 0.625rem; /* 10px */`
    _2_5,
    /// `text-indent: 0.75rem; /* 12px */`
    _3,
    /// `text-indent: 0.875rem; /* 14px */`
    _3_5,
    /// `text-indent: 1rem; /* 16px */`
    _4,
    /// `text-indent: 1.25rem; /* 20px */`
    _5,
    /// `text-indent: 1.5rem; /* 24px */`
    _6,
    /// `text-indent: 1.75rem; /* 28px */`
    _7,
    /// `text-indent: 2rem; /* 32px */`
    _8,
    /// `text-indent: 2.25rem; /* 36px */`
    _9,
    /// `text-indent: 2.5rem; /* 40px */`
    _10,
    /// `text-indent: 2.75rem; /* 44px */`
    _11,
    /// `text-indent: 3rem; /* 48px */`
    _12,
    /// `text-indent: 3.5rem; /* 56px */`
    _14,
    /// `text-indent: 4rem; /* 64px */`
    _16,
    /// `text-indent: 5rem; /* 80px */`
    _20,
    /// `text-indent: 6rem; /* 96px */`
    _24,
    /// `text-indent: 7rem; /* 112px */`
    _28,
    /// `text-indent: 8rem; /* 128px */`
    _32,
    /// `text-indent: 9rem; /* 144px */`
    _36,
    /// `text-indent: 10rem; /* 160px */`
    _40,
    /// `text-indent: 11rem; /* 176px */`
    _44,
    /// `text-indent: 12rem; /* 192px */`
    _48,
    /// `text-indent: 13rem; /* 208px */`
    _52,
    /// `text-indent: 14rem; /* 224px */`
    _56,
    /// `text-indent: 15rem; /* 240px */`
    _60,
    /// `text-indent: 16rem; /* 256px */`
    _64,
    /// `text-indent: 18rem; /* 288px */`
    _72,
    /// `text-indent: 20rem; /* 320px */`
    _80,
    /// `text-indent: 24rem; /* 384px */`
    _96,
}

/// Utilities for controlling the vertical alignment of an inline or table-cell box.
///
/// <https://tailwindcss.com/docs/vertical-align>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "align")]
pub enum VerticalAlign {
    /// `vertical-align: baseline;`
    Baseline,
    /// `vertical-align: top;`
    Top,
    /// `vertical-align: middle;`
    Middle,
    /// `vertical-align: bottom;`
    Bottom,
    /// `vertical-align: text-top;`
    TextTop,
    /// `vertical-align: text-bottom;`
    TextBottom,
    /// `vertical-align: sub;`
    Sub,
    /// `vertical-align: super;`
    Super,
}

/// Utilities for controlling an element's white-space property.
/// 
/// <https://tailwindcss.com/docs/whitespace>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "whitespace")]
pub enum Whitespace {
    /// `white-space: normal;`
    Normal,
    /// `white-space: nowrap;`
    Nowrap,
    /// `white-space: pre;`
    Pre,
    /// `white-space: pre-line;`
    PreLine,
    /// `white-space: pre-wrap;`
    PreWrap,
    /// `white-space: break-spaces;`
    BreakSpaces,
}

/// Utilities for controlling word breaks in an element.
/// 
/// <https://tailwindcss.com/docs/word-break>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "break")]
pub enum WordBreak {
    /// `overflow-wrap: normal;`
    /// 
    /// `word-break: normal;`
    Normal,
    /// `overflow-wrap: break-word;`
    Words,
    /// `word-break: break-all;`
    All,
    /// `word-break: keep-all;`
    Keep,
}

/// Utilities for controlling how words should be hyphenated.
/// 
/// <https://tailwindcss.com/docs/hyphens>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "hyphens")]
pub enum Hyphens {
    /// `hyphens: none;`
    None,
    /// `hyphens: manual;`
    Manual,
    /// `hyphens: auto;`
    Auto,
}

/// Utilities for controlling the content of the before and after pseudo-elements.
/// 
/// <https://tailwindcss.com/docs/content>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "content")]
pub enum Content {
    /// `content: none;`
    None,
}
