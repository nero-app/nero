use typewind_macros::{Display, Parse};
use crate::customization::Color;

tailwind_types!(Fill, Stroke, StrokeWidth);

/// Utilities for optimizing upcoming animations of elements that are expected to change.
/// 
/// <https://tailwindcss.com/docs/will-change>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "fill")]
pub struct Fill(pub Color);

/// Utilities for styling the stroke of SVG elements.
/// 
/// <https://tailwindcss.com/docs/stroke>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "stroke")]
pub struct Stroke(pub Color);

/// Utilities for styling the stroke width of SVG elements.
/// 
/// <https://tailwindcss.com/docs/stroke-width>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "stroke")]
pub enum StrokeWidth {
    /// ```css
    /// {
    ///     stroke-width: 0;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     stroke-width: 1;
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     stroke-width: 2;
    /// }
    /// ```
    _2,
}
