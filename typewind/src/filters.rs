use typewind_macros::{Display, Parse};
use crate::effects::Opacity;

tailwind_types!(
    Blur, Brightness, Contrast, DropShadow, Grayscale, HueRotate, Invert, Saturate, Sepia,
    BackdropBlur, BackdropBrightness, BackdropContrast, BackdropGrayscale, BackdropHueRotate, 
    BackdropInvert, BackdropOpacity, BackdropSaturate, BackdropSepia
);

/// Utilities for applying blur filters to an element.
/// 
/// <https://tailwindcss.com/docs/blur>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "blur")]
pub enum Blur {
    /// `filter:  ;`
    None,
    /// `filter: blur(4px);`
    Sm,
    /// `filter: blur(8px);`
    #[display(no_prefix)]
    Blur,
    /// `filter: blur(12px);`
    Md,
    /// `filter: blur(16px);`
    Lg,
    /// `filter: blur(24px);`
    Xl,
    /// `filter: blur(40px);`
    _2xl,
    /// `filter: blur(64px);`
    _3xl,
}

/// Utilities for applying brightness filters to an element.
/// 
/// <https://tailwindcss.com/docs/brightness>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "brightness")]
pub enum Brightness {
    /// `filter: brightness(0);`
    _0,
    /// `filter: brightness(.5);`
    _50,
    /// `filter: brightness(.75);`
    _75,
    /// `filter: brightness(.9);`
    _90,
    /// `filter: brightness(.95);`
    _95,
    /// `filter: brightness(1);`
    _100,
    /// `filter: brightness(1.05);`
    _105,
    /// `filter: brightness(1.1);`
    _110,
    /// `filter: brightness(1.25);`
    _125,
    /// `filter: brightness(1.5);`
    _150,
    /// `filter: brightness(2);`
    _200,
}

/// Utilities for applying contrast filters to an element.
/// 
/// <https://tailwindcss.com/docs/contrast>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "contrast")]
pub enum Contrast {
    /// `filter: contrast(0);`
    _0,
    /// `filter: contrast(.5);`
    _50,
    /// `filter: contrast(.75);`
    _75,
    /// `filter: contrast(1);`
    _100,
    /// `filter: contrast(1.25);`
    _125,
    /// `filter: contrast(1.5);`
    _150,
    /// `filter: contrast(2);`
    _200,
}

/// Utilities for applying drop-shadow filters to an element.
/// 
/// <https://tailwindcss.com/docs/drop-shadow>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "drop-shadow")]
pub enum DropShadow {
    /// `filter: drop-shadow(0 1px 1px rgb(0 0 0 / 0.05));`
    Sm,
    /// `filter: drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06));`
    #[display(no_prefix)]
    DropShadow,
    /// `filter: drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06));`
    Md,
    /// `filter: drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1));`
    Lg,
    /// `filter: drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08));`
    Xl,
    /// `filter: drop-shadow(0 25px 25px rgb(0 0 0 / 0.15));`
    _2xl,
    /// `filter: drop-shadow(0 0 #0000);`
    None,
}

/// Utilities for applying grayscale filters to an element.
/// 
/// <https://tailwindcss.com/docs/grayscale>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "grayscale")]
pub enum Grayscale {
    /// `filter: grayscale(0);`
    _0,
    /// `filter: grayscale(100%);`
    #[display(no_prefix)]
    Grayscale,
}

/// Utilities for applying hue-rotate filters to an element.
/// 
/// <https://tailwindcss.com/docs/hue-rotate>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "hue-rotate")]
pub enum HueRotate {
    /// `filter: hue-rotate(0deg);`
    _0,
    /// `filter: hue-rotate(15deg);`
    _15,
    /// `filter: hue-rotate(30deg);`
    _30,
    /// `filter: hue-rotate(60deg);`
    _60,
    /// `filter: hue-rotate(90deg);`
    _90,
    /// `filter: hue-rotate(180deg);`
    _180,
}

/// Utilities for applying invert filters to an element.
/// 
/// <https://tailwindcss.com/docs/invert>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "invert")]
pub enum Invert {
    /// `filter: invert(0);`
    _0,
    /// `filter: invert(100%);`
    #[display(no_prefix)]
    Invert,
}

/// Utilities for applying saturation filters to an element.
/// 
/// <https://tailwindcss.com/docs/saturate>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "saturate")]
pub enum Saturate {
    /// `filter: saturate(0);`
    _0,
    /// `filter: saturate(.5);`
    _50,
    /// `filter: saturate(1);`
    _100,
    /// `filter: saturate(1.5);`
    _150,
    /// `filter: saturate(2);`
    _200,
}

/// Utilities for applying sepia filters to an element.
/// 
/// <https://tailwindcss.com/docs/sepia>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "sepia")]
pub enum Sepia {
    /// `filter: sepia(0);`
    _0,
    /// `filter: sepia(100%);`
    #[display(no_prefix)]
    Sepia,
}

/// Utilities for applying backdrop blur filters to an element.
/// 
/// <https://tailwindcss.com/docs/backdrop-blur>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "backdrop")]
pub struct BackdropBlur(pub Blur);

/// Utilities for applying backdrop brightness filters to an element.
/// 
/// <https://tailwindcss.com/docs/backdrop-brightness>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "backdrop")]
pub struct BackdropBrightness(pub Brightness);

/// Utilities for applying backdrop contrast filters to an element.
/// 
/// <https://tailwindcss.com/docs/backdrop-contrast>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "backdrop")]
pub struct BackdropContrast(pub Contrast);

/// Utilities for applying backdrop grayscale filters to an element.
/// 
/// <https://tailwindcss.com/docs/backdrop-grayscale>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "backdrop")]
pub struct BackdropGrayscale(pub Grayscale);

/// Utilities for applying backdrop hue-rotate filters to an element.
/// 
/// <https://tailwindcss.com/docs/backdrop-hue-rotate>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "backdrop")]
pub struct BackdropHueRotate(pub HueRotate);

/// Utilities for applying backdrop invert filters to an element.
/// 
/// <https://tailwindcss.com/docs/backdrop-invert>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "backdrop")]
pub struct BackdropInvert(pub Invert);

/// Utilities for applying backdrop opacity filters to an element.
/// 
/// <https://tailwindcss.com/docs/backdrop-opacity>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "backdrop")]
pub struct BackdropOpacity(pub Opacity); // this should be as - ?

/// Utilities for applying backdrop saturation filters to an element.
/// 
/// <https://tailwindcss.com/docs/backdrop-saturate>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "backdrop")]
pub struct BackdropSaturate(pub Saturate);

/// Utilities for applying backdrop sepia filters to an element.
/// 
/// <https://tailwindcss.com/docs/backdrop-sepia>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "backdrop")]
pub struct BackdropSepia(pub Sepia);
