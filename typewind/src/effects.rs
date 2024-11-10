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
    /// `box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);`
    Sm,
    /// `box-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);`
    #[display(no_prefix)]
    Shadow,
    /// `box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);`
    Md,
    /// `box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);`
    Lg,
    /// `box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);`
    Xl,
    /// `box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);`
    _2xl,
    /// `box-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05);`
    Inner,
    /// `box-shadow: 0 0 #0000;`
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
    /// `opacity: 0;`
    _0,
    /// `opacity: 0.05;`
    _5,
    /// `opacity: 0.1;`
    _10,
    /// `opacity: 0.15;`
    _15,
    /// `opacity: 0.2;`
    _20,
    /// `opacity: 0.25;`
    _25,
    /// `opacity: 0.3;`
    _30,
    /// `opacity: 0.35;`
    _35,
    /// `opacity: 0.4;`
    _40,
    /// `opacity: 0.45;`
    _45,
    /// `opacity: 0.5;`
    _50,
    /// `opacity: 0.55;`
    _55,
    /// `opacity: 0.6;`
    _60,
    /// `opacity: 0.65;`
    _65,
    /// `opacity: 0.7;`
    _70,
    /// `opacity: 0.75;`
    _75,
    /// `opacity: 0.8;`
    _80,
    /// `opacity: 0.85;`
    _85,
    /// `opacity: 0.9;`
    _90,
    /// `opacity: 0.95;`
    _95,
    /// `opacity: 1;`
    _100,
}

/// Utilities for controlling how an element should blend with the background.
/// 
/// <https://tailwindcss.com/docs/mix-blend-mode>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "mix-blend")]
pub enum MixBlendMode {
    /// `mix-blend-mode: normal;`
    Normal,
    /// `mix-blend-mode: multiply;`
    Multiply,
    /// `mix-blend-mode: screen;`
    Screen,
    /// `mix-blend-mode: overlay;`
    Overlay,
    /// `mix-blend-mode: darken;`
    Darken,
    /// `mix-blend-mode: lighten;`
    Lighten,
    /// `mix-blend-mode: color-dodge;`
    ColorDodge,
    /// `mix-blend-mode: color-burn;`
    ColorBurn,
    /// `mix-blend-mode: hard-light;`
    HardLight,
    /// `mix-blend-mode: soft-light;`
    SoftLight,
    /// `mix-blend-mode: difference;`
    Difference,
    /// `mix-blend-mode: exclusion;`
    Exclusion,
    /// `mix-blend-mode: hue;`
    Hue,
    /// `mix-blend-mode: saturation;`
    Saturation,
    /// `mix-blend-mode: color;`
    Color,
    /// `mix-blend-mode: luminosity;`
    Luminosity,
    /// `mix-blend-mode: plus-darker;`
    PlusDarker,
    /// `mix-blend-mode: plus-lighter;`
    PlusLighter,
}

/// Utilities for controlling how an element's background image should blend with its background color.
/// 
/// <https://tailwindcss.com/docs/background-blend-mode>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "bg-blend")]
pub enum BackgroundBlendMode {
    /// `background-blend-mode: normal;`
    Normal,
    /// `background-blend-mode: multiply;`
    Multiply,
    /// `background-blend-mode: screen;`
    Screen,
    /// `background-blend-mode: overlay;`
    Overlay,
    /// `background-blend-mode: darken;`
    Darken,
    /// `background-blend-mode: lighten;`
    Lighten,
    /// `background-blend-mode: color-dodge;`
    ColorDodge,
    /// `background-blend-mode: color-burn;`
    ColorBurn,
    /// `background-blend-mode: hard-light;`
    HardLight,
    /// `background-blend-mode: soft-light;`
    SoftLight,
    /// `background-blend-mode: difference;`
    Difference,
    /// `background-blend-mode: exclusion;`
    Exclusion,
    /// `background-blend-mode: hue;`
    Hue,
    /// `background-blend-mode: saturation;`
    Saturation,
    /// `background-blend-mode: color;`
    Color,
    /// `background-blend-mode: luminosity;`
    Luminosity,
}

