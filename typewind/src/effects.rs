use typewind_macros::{Display, Parse};
use crate::customization::Color;

tailwind_types!(
    BoxShadow, BoxShadowColor, Opacity, MixBlendMode, BackgroundBlendMode
);

/// Utilities for controlling the box shadow of an element.
/// 
/// <https://tailwindcss.com/docs/box-shadow>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "shadow")]
pub enum BoxShadow {
    /// ```css
    /// {
    ///     box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);
    /// }
    /// ```
    Sm,
    /// ```css
    /// {
    ///     box-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);
    /// }
    /// ```
    #[display(no_prefix)]
    Shadow,
    /// ```css
    /// {
    ///     box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);
    /// }
    /// ```
    Md,
    /// ```css
    /// {
    ///     box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);
    /// }
    /// ```
    Lg,
    /// ```css
    /// {
    ///     box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);
    /// }
    /// ```
    Xl,
    /// ```css
    /// {
    ///     box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);
    /// }
    /// ```
    _2xl,
    /// ```css
    /// {
    ///     box-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05);
    /// }
    /// ```
    Inner,
    /// ```css
    /// {
    ///     box-shadow: 0 0 #0000;
    /// }
    /// ```
    None,
}

/// Utilities for controlling the color of a box shadow.
/// 
/// <https://tailwindcss.com/docs/box-shadow-color>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "shadow")]
pub struct BoxShadowColor(pub Color);

/// Utilities for controlling the opacity of an element.
/// 
/// <https://tailwindcss.com/docs/opacity>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "opacity")]
pub enum Opacity {
    /// ```css
    /// {
    ///     opacity: 0;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     opacity: 0.05;
    /// }
    /// ```
    _5,
    /// ```css
    /// {
    ///     opacity: 0.1;
    /// }
    /// ```
    _10,
    /// ```css
    /// {
    ///     opacity: 0.15;
    /// }
    /// ```
    _15,
    /// ```css
    /// {
    ///     opacity: 0.2;
    /// }
    /// ```
    _20,
    /// ```css
    /// {
    ///     opacity: 0.25;
    /// }
    /// ```
    _25,
    /// ```css
    /// {
    ///     opacity: 0.3;
    /// }
    /// ```
    _30,
    /// ```css
    /// {
    ///     opacity: 0.35;
    /// }
    /// ```
    _35,
    /// ```css
    /// {
    ///     opacity: 0.4;
    /// }
    /// ```
    _40,
    /// ```css
    /// {
    ///     opacity: 0.45;
    /// }
    /// ```
    _45,
    /// ```css
    /// {
    ///     opacity: 0.5;
    /// }
    /// ```
    _50,
    /// ```css
    /// {
    ///     opacity: 0.55;
    /// }
    /// ```
    _55,
    /// ```css
    /// {
    ///     opacity: 0.6;
    /// }
    /// ```
    _60,
    /// ```css
    /// {
    ///     opacity: 0.65;
    /// }
    /// ```
    _65,
    /// ```css
    /// {
    ///     opacity: 0.7;
    /// }
    /// ```
    _70,
    /// ```css
    /// {
    ///     opacity: 0.75;
    /// }
    /// ```
    _75,
    /// ```css
    /// {
    ///     opacity: 0.8;
    /// }
    /// ```
    _80,
    /// ```css
    /// {
    ///     opacity: 0.85;
    /// }
    /// ```
    _85,
    /// ```css
    /// {
    ///     opacity: 0.9;
    /// }
    /// ```
    _90,
    /// ```css
    /// {
    ///     opacity: 0.95;
    /// }
    /// ```
    _95,
    /// ```css
    /// {
    ///     opacity: 1;
    /// }
    /// ```
    _100,
}

/// Utilities for controlling how an element should blend with the background.
/// 
/// <https://tailwindcss.com/docs/mix-blend-mode>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "mix-blend")]
pub enum MixBlendMode {
    /// ```css
    /// {
    ///     mix-blend-mode: normal;
    /// }
    /// ```
    Normal,
    /// ```css
    /// {
    ///     mix-blend-mode: multiply;
    /// }
    /// ```
    Multiply,
    /// ```css
    /// {
    ///     mix-blend-mode: screen;
    /// }
    /// ```
    Screen,
    /// ```css
    /// {
    ///     mix-blend-mode: overlay;
    /// }
    /// ```
    Overlay,
    /// ```css
    /// {
    ///     mix-blend-mode: darken;
    /// }
    /// ```
    Darken,
    /// ```css
    /// {
    ///     mix-blend-mode: lighten;
    /// }
    /// ```
    Lighten,
    /// ```css
    /// {
    ///     mix-blend-mode: color-dodge;
    /// }
    /// ```
    ColorDodge,
    /// ```css
    /// {
    ///     mix-blend-mode: color-burn;
    /// }
    /// ```
    ColorBurn,
    /// ```css
    /// {
    ///     mix-blend-mode: hard-light;
    /// }
    /// ```
    HardLight,
    /// ```css
    /// {
    ///     mix-blend-mode: soft-light;
    /// }
    /// ```
    SoftLight,
    /// ```css
    /// {
    ///     mix-blend-mode: difference;
    /// }
    /// ```
    Difference,
    /// ```css
    /// {
    ///     mix-blend-mode: exclusion;
    /// }
    /// ```
    Exclusion,
    /// ```css
    /// {
    ///     mix-blend-mode: hue;
    /// }
    /// ```
    Hue,
    /// ```css
    /// {
    ///     mix-blend-mode: saturation;
    /// }
    /// ```
    Saturation,
    /// ```css
    /// {
    ///     mix-blend-mode: color;
    /// }
    /// ```
    Color,
    /// ```css
    /// {
    ///     mix-blend-mode: luminosity;
    /// }
    /// ```
    Luminosity,
    /// ```css
    /// {
    ///     mix-blend-mode: plus-darker;
    /// }
    /// ```
    PlusDarker,
    /// ```css
    /// {
    ///     mix-blend-mode: plus-lighter;
    /// }
    /// ```
    PlusLighter,
}

/// Utilities for controlling how an element's background image should blend with its background color.
/// 
/// <https://tailwindcss.com/docs/background-blend-mode>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "bg-blend")]
pub enum BackgroundBlendMode {
    /// ```css
    /// {
    ///     background-blend-mode: normal;
    /// }
    /// ```
    Normal,
    /// ```css
    /// {
    ///     background-blend-mode: multiply;
    /// }
    /// ```
    Multiply,
    /// ```css
    /// {
    ///     background-blend-mode: screen;
    /// }
    /// ```
    Screen,
    /// ```css
    /// {
    ///     background-blend-mode: overlay;
    /// }
    /// ```
    Overlay,
    /// ```css
    /// {
    ///     background-blend-mode: darken;
    /// }
    /// ```
    Darken,
    /// ```css
    /// {
    ///     background-blend-mode: lighten;
    /// }
    /// ```
    Lighten,
    /// ```css
    /// {
    ///     background-blend-mode: color-dodge;
    /// }
    /// ```
    ColorDodge,
    /// ```css
    /// {
    ///     background-blend-mode: color-burn;
    /// }
    /// ```
    ColorBurn,
    /// ```css
    /// {
    ///     background-blend-mode: hard-light;
    /// }
    /// ```
    HardLight,
    /// ```css
    /// {
    ///     background-blend-mode: soft-light;
    /// }
    /// ```
    SoftLight,
    /// ```css
    /// {
    ///     background-blend-mode: difference;
    /// }
    /// ```
    Difference,
    /// ```css
    /// {
    ///     background-blend-mode: exclusion;
    /// }
    /// ```
    Exclusion,
    /// ```css
    /// {
    ///     background-blend-mode: hue;
    /// }
    /// ```
    Hue,
    /// ```css
    /// {
    ///     background-blend-mode: saturation;
    /// }
    /// ```
    Saturation,
    /// ```css
    /// {
    ///     background-blend-mode: color;
    /// }
    /// ```
    Color,
    /// ```css
    /// {
    ///     background-blend-mode: luminosity;
    /// }
    /// ```
    Luminosity,
}

