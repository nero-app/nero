use typewind_macros::{Display, Parse};

tailwind_types!(ScreenReaders, ForcedColorAdjust);

/// Utilities for improving accessibility with screen readers.
/// 
/// <https://tailwindcss.com/docs/screen-readers>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum ScreenReaders {
    /// ```css
    /// position: absolute;
    /// width: 1px;
    /// height: 1px;
    /// padding: 0;
    /// margin: -1px;
    /// overflow: hidden;
    /// clip: rect(0, 0, 0, 0);
    /// white-space: nowrap;
    /// border-width: 0;
    /// ```
    SrOnly,
    /// ```css
    /// position: static;
    /// width: auto;
    /// height: auto;
    /// padding: 0;
    /// margin: 0;
    /// overflow: visible;
    /// clip: auto;
    /// white-space: normal;
    /// ```
    NotSrOnly,
}

/// Utilities for opting in and out of forced colors.
/// 
/// <https://tailwindcss.com/docs/forced-color-adjust>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "forced-color-adjust")]
pub enum ForcedColorAdjust {
    /// ```css
    /// forced-color-adjust: auto;
    /// ```
    Auto,
    /// ```css
    /// forced-color-adjust: none;
    /// ```
    None,
}
