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
    /// `background-attachment: fixed;`
    Fixed,
    /// `background-attachment: local;`
    Local,
    /// `background-attachment: scroll;`
    Scroll,
}

/// Utilities for controlling the bounding box of an element's background.
/// 
/// <https://tailwindcss.com/docs/background-clip>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "bg-clip")]
pub enum BackgroundClip {
    /// `background-clip: border-box;`
    Border,
    /// `background-clip: padding-box;`
    Padding,
    /// `background-clip: content-box;`
    Content,
    /// `background-clip: text;`
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
    /// `background-origin: border-box;`
    Border,
    /// `background-origin: padding-box;`
    Padding,
    /// `background-origin: content-box;`
    Content,
}

/// Utilities for controlling the position of an element's background image.
/// 
/// <https://tailwindcss.com/docs/background-position>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "bg")]
pub enum BackgroundPosition {
    /// `background-position: bottom;`
    Bottom,
    /// `background-position: center;`
    Center,
    /// `background-position: left;`
    Left,
    /// `background-position: left bottom;`
    LeftBottom,
    /// `background-position: left top;`
    LeftTop,
    /// `background-position: right;`
    Right,
    /// `background-position: right bottom;`
    RightBottom,
    /// `background-position: right top;`
    RightTop,
    /// `background-position: top;`
    Top,
}

/// Utilities for controlling the repetition of an element's background image.
/// 
/// <https://tailwindcss.com/docs/background-repeat>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "bg")]
pub enum BackgroundRepeat {
    /// `background-repeat: repeat;`
    Repeat,
    /// `background-repeat: no-repeat;`
    NoRepeat,
    /// `background-repeat: repeat-x;`
    RepeatX,
    /// `background-repeat: repeat-y;`
    RepeatY,
    /// `background-repeat: round;`
    RepeatRound,
    /// `background-repeat: space;`
    RepeatSpace,
}

/// Utilities for controlling the background size of an element's background image.
/// 
/// <https://tailwindcss.com/docs/background-size>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "bg")]
pub enum BackgroundSize {
    /// `background-size: auto;`
    Auto,
    /// `background-size: cover;`
    Cover,
    /// `background-size: contain;`
    Contain,
}

/// Utilities for controlling an element's background image.
/// 
/// <https://tailwindcss.com/docs/background-image>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "bg")]
pub enum BackgroundImage {
    /// `background-image: none;`
    None,
    /// `background-image: linear-gradient(to top, var(--tw-gradient-stops));`
    GradientToT,
    /// `background-image: linear-gradient(to top right, var(--tw-gradient-stops));`
    GradientToTr,
    /// `background-image: linear-gradient(to right, var(--tw-gradient-stops));`
    GradientToR,
    /// `background-image: linear-gradient(to bottom right, var(--tw-gradient-stops));`
    GradientToBr,
    /// `background-image: linear-gradient(to bottom, var(--tw-gradient-stops));`
    GradientToB,
    /// `background-image: linear-gradient(to bottom left, var(--tw-gradient-stops));`
    GradientToBl,
    /// `background-image: linear-gradient(to left, var(--tw-gradient-stops));`
    GradientToL,
    /// `background-image: linear-gradient(to top left, var(--tw-gradient-stops));`
    GradientToTl,
}

// TODO: Gradient Color Stops
