use typewind_macros::{Display, Parse};

use crate::customization::Color;

tailwind_types!(
    BackgroundAttachment, BackgroundClip, BackgroundColor, BackgroundOrigin,
    BackgroundPosition, BackgroundRepeat, BackgroundSize, BackgroundImage
);

/// Utilities for controlling how a background image behaves when scrolling.
/// 
/// <https://tailwindcss.com/docs/background-attachment>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "bg")]
pub enum BackgroundAttachment {
    /// ```css
    /// {
    ///     background-attachment: fixed;
    /// }
    /// ```
    Fixed,
    /// ```css
    /// {
    ///     background-attachment: local;
    /// }
    /// ```
    Local,
    /// ```css
    /// {
    ///     background-attachment: scroll;
    /// }
    /// ```
    Scroll,
}

/// Utilities for controlling the bounding box of an element's background.
/// 
/// <https://tailwindcss.com/docs/background-clip>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "bg-clip")]
pub enum BackgroundClip {
    /// ```css
    /// {
    ///     background-clip: border-box;
    /// }
    /// ```
    Border,
    /// ```css
    /// {
    ///     background-clip: padding-box;
    /// }
    /// ```
    Padding,
    /// ```css
    /// {
    ///     background-clip: content-box;
    /// }
    /// ```
    Content,
    /// ```css
    /// {
    ///     background-clip: text;
    /// }
    /// ```
    Text,
}

/// Utilities for controlling an element's background color.
/// 
/// <https://tailwindcss.com/docs/background-color>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "bg")]
pub struct BackgroundColor(pub Color);

/// Utilities for controlling how an element's background is positioned relative to borders, padding, and content.
/// 
/// <https://tailwindcss.com/docs/background-origin>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "bg-origin")]
pub enum BackgroundOrigin {
    /// ```css
    /// {
    ///     background-origin: border-box;
    /// }
    /// ```
    Border,
    /// ```css
    /// {
    ///     background-origin: padding-box;
    /// }
    /// ```
    Padding,
    /// ```css
    /// {
    ///     background-origin: content-box;
    /// }
    /// ```
    Content,
}

/// Utilities for controlling the position of an element's background image.
/// 
/// <https://tailwindcss.com/docs/background-position>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "bg")]
pub enum BackgroundPosition {
    /// ```css
    /// {
    ///     background-position: bottom;
    /// }
    /// ```
    Bottom,
    /// ```css
    /// {
    ///     background-position: center;
    /// }
    /// ```
    Center,
    /// ```css
    /// {
    ///     background-position: left;
    /// }
    /// ```
    Left,
    /// ```css
    /// {
    ///     background-position: left bottom;
    /// }
    /// ```
    LeftBottom,
    /// ```css
    /// {
    ///     background-position: left top;
    /// }
    /// ```
    LeftTop,
    /// ```css
    /// {
    ///     background-position: right;
    /// }
    /// ```
    Right,
    /// ```css
    /// {
    ///     background-position: right bottom;
    /// }
    /// ```
    RightBottom,
    /// ```css
    /// {
    ///     background-position: right top;
    /// }
    /// ```
    RightTop,
    /// ```css
    /// {
    ///     background-position: top;
    /// }
    /// ```
    Top,
}

/// Utilities for controlling the repetition of an element's background image.
/// 
/// <https://tailwindcss.com/docs/background-repeat>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "bg")]
pub enum BackgroundRepeat {
    /// ```css
    /// {
    ///     background-repeat: repeat;
    /// }
    /// ```
    Repeat,
    /// ```css
    /// {
    ///     background-repeat: no-repeat;
    /// }
    /// ```
    NoRepeat,
    /// ```css
    /// {
    ///     background-repeat: repeat-x;
    /// }
    /// ```
    RepeatX,
    /// ```css
    /// {
    ///     background-repeat: repeat-y;
    /// }
    /// ```
    RepeatY,
    /// ```css
    /// {
    ///     background-repeat: round;
    /// }
    /// ```
    RepeatRound,
    /// ```css
    /// {
    ///     background-repeat: space;
    /// }
    /// ```
    RepeatSpace,
}

/// Utilities for controlling the background size of an element's background image.
/// 
/// <https://tailwindcss.com/docs/background-size>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "bg")]
pub enum BackgroundSize {
    /// ```css
    /// {
    ///     background-size: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     background-size: cover;
    /// }
    /// ```
    Cover,
    /// ```css
    /// {
    ///     background-size: contain;
    /// }
    /// ```
    Contain,
}

/// Utilities for controlling an element's background image.
/// 
/// <https://tailwindcss.com/docs/background-image>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "bg")]
pub enum BackgroundImage {
    /// ```css
    /// {
    ///     background-image: none;
    /// }
    /// ```
    None,
    /// ```css
    /// {
    ///     background-image: linear-gradient(to top, var(--tw-gradient-stops));
    /// }
    /// ```
    GradientToT,
    /// ```css
    /// {
    ///     background-image: linear-gradient(to top right, var(--tw-gradient-stops));
    /// }
    /// ```
    GradientToTr,
    /// ```css
    /// {
    ///     background-image: linear-gradient(to right, var(--tw-gradient-stops));
    /// }
    /// ```
    GradientToR,
    /// ```css
    /// {
    ///     background-image: linear-gradient(to bottom right, var(--tw-gradient-stops));
    /// }
    /// ```
    GradientToBr,
    /// ```css
    /// {
    ///     background-image: linear-gradient(to bottom, var(--tw-gradient-stops));
    /// }
    /// ```
    GradientToB,
    /// ```css
    /// {
    ///     background-image: linear-gradient(to bottom left, var(--tw-gradient-stops));
    /// }
    /// ```
    GradientToBl,
    /// ```css
    /// {
    ///     background-image: linear-gradient(to left, var(--tw-gradient-stops));
    /// }
    /// ```
    GradientToL,
    /// ```css
    /// {
    ///     background-image: linear-gradient(to top left, var(--tw-gradient-stops));
    /// }
    /// ```
    GradientToTl,
}

// TODO: Gradient Color Stops
