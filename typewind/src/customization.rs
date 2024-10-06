use typewind_macros::{Display, Parse};

/// <https://tailwindcss.com/docs/customizing-colors>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum ColorTone {
    _50,
    _100,
    _200,
    _300,
    _400,
    _500,
    _600,
    _700,
    _800,
    _900,
    _950,
}

/// <https://tailwindcss.com/docs/customizing-colors>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
pub enum Color {
    Inherit,
    Current,
    Transparent,
    Black,
    White,
    Slate(ColorTone),
    Gray(ColorTone),
    Zinc(ColorTone),
    Neutral(ColorTone),
    Stone(ColorTone),
    Red(ColorTone),
    Orange(ColorTone),
    Amber(ColorTone),
    Yellow(ColorTone),
    Lime(ColorTone),
    Green(ColorTone),
    Emerald(ColorTone),
    Teal(ColorTone),
    Cyan(ColorTone),
    Sky(ColorTone),
    Blue(ColorTone),
    Indigo(ColorTone),
    Violet(ColorTone),
    Purple(ColorTone),
    Fuchsia(ColorTone),
    Pink(ColorTone),
    Rose(ColorTone),
}
