use typewind_macros::{Display, Parse};

tailwind_types!(
    Width, MinWidth, MaxWidth, Height, MinHeight, MaxHeight, Size
);

/// Utilities for setting the width of an element.
/// 
/// <https://tailwindcss.com/docs/width>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(
    prefix = "w", 
    replace(from = "_", to = "."),
    replace(from = "div", to = "/")
)]
pub enum Width {
    /// `width: 0px;`
    _0,
    /// `width: 1px;`
    Px,
    /// `width: 0.125rem; /* 2px */`
    _0_5,
    /// `width: 0.25rem; /* 4px */`
    _1,
    /// `width: 0.375rem; /* 6px */`
    _1_5,
    /// `width: 0.5rem; /* 8px */`
    _2,
    /// `width: 0.625rem; /* 10px */`
    _2_5,
    /// `width: 0.75rem; /* 12px */`
    _3,
    /// `width: 0.875rem; /* 14px */`
    _3_5,
    /// `width: 1rem; /* 16px */`
    _4,
    /// `width: 1.25rem; /* 20px */`
    _5,
    /// `width: 1.5rem; /* 24px */`
    _6,
    /// `width: 1.75rem; /* 28px */`
    _7,
    /// `width: 2rem; /* 32px */`
    _8,
    /// `width: 2.25rem; /* 36px */`
    _9,
    /// `width: 2.5rem; /* 40px */`
    _10,
    /// `width: 2.75rem; /* 44px */`
    _11,
    /// `width: 3rem; /* 48px */`
    _12,
    /// `width: 3.5rem; /* 56px */`
    _14,
    /// `width: 4rem; /* 64px */`
    _16,
    /// `width: 5rem; /* 80px */`
    _20,
    /// `width: 6rem; /* 96px */`
    _24,
    /// `width: 7rem; /* 112px */`
    _28,
    /// `width: 8rem; /* 128px */`
    _32,
    /// `width: 9rem; /* 144px */`
    _36,
    /// `width: 10rem; /* 160px */`
    _40,
    /// `width: 11rem; /* 176px */`
    _44,
    /// `width: 12rem; /* 192px */`
    _48,
    /// `width: 13rem; /* 208px */`
    _52,
    /// `width: 14rem; /* 224px */`
    _56,
    /// `width: 15rem; /* 240px */`
    _60,
    /// `width: 16rem; /* 256px */`
    _64,
    /// `width: 18rem; /* 288px */`
    _72,
    /// `width: 20rem; /* 320px */`
    _80,
    /// `width: 24rem; /* 384px */`
    _96,
    /// `width: auto;`
    Auto,
    /// `width: 50%;`
    _1div2,
    /// `width: 33.333333%;`
    _1div3,
    /// `width: 66.666667%;`
    _2div3,
    /// `width: 25%;`
    _1div4,
    /// `width: 50%;`
    _2div4,
    /// `width: 75%;`
    _3div4,
    /// `width: 20%;`
    _1div5,
    /// `width: 40%;`
    _2div5,
    /// `width: 60%;`
    _3div5,
    /// `width: 80%;`
    _4div5,
    /// `width: 16.666667%;`
    _1div6,
    /// `width: 33.333333%;`
    _2div6,
    /// `width: 50%;`
    _3div6,
    /// `width: 66.666667%;`
    _4div6,
    /// `width: 83.333333%;`
    _5div6,
    /// `width: 8.333333%;`
    _1div12,
    /// `width: 16.666667%;`
    _2div12,
    /// `width: 25%;`
    _3div12,
    /// `width: 33.333333%;`
    _4div12,
    /// `width: 41.666667%;`
    _5div12,
    /// `width: 50%;`
    _6div12,
    /// `width: 58.333333%;`
    _7div12,
    /// `width: 66.666667%;`
    _8div12,
    /// `width: 75%;`
    _9div12,
    /// `width: 83.333333%;`
    _10div12,
    /// `width: 91.666667%;`
    _11div12,
    /// `width: 100%;`
    Full,
    /// `width: 100vw;`
    Screen,
    /// `width: 100svw;`
    Svw,
    /// `width: 100lvw;`
    Lvw,
    /// `width: 100dvw;`
    Dvw,
    /// `width: min-content;`
    Min,
    /// `width: max-content;`
    Max,
    /// `width: fit-content;`
    Fit,
}

/// Utilities for setting the minimum width of an element.
/// 
/// <https://tailwindcss.com/docs/min-width>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "min-w", replace(from = "_", to = "."))]
pub enum MinWidth {
    /// `min-width: 0px;`
    _0,
    /// `min-width: 0.25rem; /* 4px */`
    _1,
    /// `min-width: 0.5rem; /* 8px */`
    _2,
    /// `min-width: 0.75rem; /* 12px */`
    _3,
    /// `min-width: 1rem; /* 16px */`
    _4,
    /// `min-width: 1.25rem; /* 20px */`
    _5,
    /// `min-width: 1.5rem; /* 24px */`
    _6,
    /// `min-width: 1.75rem; /* 28px */`
    _7,
    /// `min-width: 2rem; /* 32px */`
    _8,
    /// `min-width: 2.25rem; /* 36px */`
    _9,
    /// `min-width: 2.5rem; /* 40px */`
    _10,
    /// `min-width: 2.75rem; /* 44px */`
    _11,
    /// `min-width: 3rem; /* 48px */`
    _12,
    /// `min-width: 3.5rem; /* 56px */`
    _14,
    /// `min-width: 4rem; /* 64px */`
    _16,
    /// `min-width: 5rem; /* 80px */`
    _20,
    /// `min-width: 6rem; /* 96px */`
    _24,
    /// `min-width: 7rem; /* 112px */`
    _28,
    /// `min-width: 8rem; /* 128px */`
    _32,
    /// `min-width: 9rem; /* 144px */`
    _36,
    /// `min-width: 10rem; /* 160px */`
    _40,
    /// `min-width: 11rem; /* 176px */`
    _44,
    /// `min-width: 12rem; /* 192px */`
    _48,
    /// `min-width: 13rem; /* 208px */`
    _52,
    /// `min-width: 14rem; /* 224px */`
    _56,
    /// `min-width: 15rem; /* 240px */`
    _60,
    /// `min-width: 16rem; /* 256px */`
    _64,
    /// `min-width: 18rem; /* 288px */`
    _72,
    /// `min-width: 20rem; /* 320px */`
    _80,
    /// `min-width: 24rem; /* 384px */`
    _96,
    /// `min-width: 1px;`
    Px,
    /// `min-width: 0.125rem; /* 2px */`
    _0_5,
    /// `min-width: 0.375rem; /* 6px */`
    _1_5,
    /// `min-width: 0.625rem; /* 10px */`
    _2_5,
    /// `min-width: 0.875rem; /* 14px */`
    _3_5,
    /// `min-width: 100%;`
    Full,
    /// `min-width: min-content;`
    Min,
    /// `min-width: max-content;`
    Max,
    /// `min-width: fit-content;`
    Fit,
}

/// Utilities for setting the maximum width of an element.
/// 
/// <https://tailwindcss.com/docs/max-width>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "max-w", replace(from = "_", to = "."))]
pub enum MaxWidth {
    /// `max-width: 0px;`
    _0,
    /// `max-width: 1px;`
    Px,
    /// `max-width: 0.125rem; /* 2px */`
    _0_5,
    /// `max-width: 0.25rem; /* 4px */`
    _1,
    /// `max-width: 0.375rem; /* 6px */`
    _1_5,
    /// `max-width: 0.5rem; /* 8px */`
    _2,
    /// `max-width: 0.625rem; /* 10px */`
    _2_5,
    /// `max-width: 0.75rem; /* 12px */`
    _3,
    /// `max-width: 0.875rem; /* 14px */`
    _3_5,
    /// `max-width: 1rem; /* 16px */`
    _4,
    /// `max-width: 1.25rem; /* 20px */`
    _5,
    /// `max-width: 1.5rem; /* 24px */`
    _6,
    /// `max-width: 1.75rem; /* 28px */`
    _7,
    /// `max-width: 2rem; /* 32px */`
    _8,
    /// `max-width: 2.25rem; /* 36px */`
    _9,
    /// `max-width: 2.5rem; /* 40px */`
    _10,
    /// `max-width: 2.75rem; /* 44px */`
    _11,
    /// `max-width: 3rem; /* 48px */`
    _12,
    /// `max-width: 3.5rem; /* 56px */`
    _14,
    /// `max-width: 4rem; /* 64px */`
    _16,
    /// `max-width: 5rem; /* 80px */`
    _20,
    /// `max-width: 6rem; /* 96px */`
    _24,
    /// `max-width: 7rem; /* 112px */`
    _28,
    /// `max-width: 8rem; /* 128px */`
    _32,
    /// `max-width: 9rem; /* 144px */`
    _36,
    /// `max-width: 10rem; /* 160px */`
    _40,
    /// `max-width: 11rem; /* 176px */`
    _44,
    /// `max-width: 12rem; /* 192px */`
    _48,
    /// `max-width: 13rem; /* 208px */`
    _52,
    /// `max-width: 14rem; /* 224px */`
    _56,
    /// `max-width: 15rem; /* 240px */`
    _60,
    /// `max-width: 16rem; /* 256px */`
    _64,
    /// `max-width: 18rem; /* 288px */`
    _72,
    /// `max-width: 20rem; /* 320px */`
    _80,
    /// `max-width: 24rem; /* 384px */`
    _96,
    /// `max-width: none;`
    None,
    /// `max-width: 20rem; /* 320px */`
    Xs,
    /// `max-width: 24rem; /* 384px */`
    Sm,
    /// `max-width: 28rem; /* 448px */`
    Md,
    /// `max-width: 32rem; /* 512px */`
    Lg,
    /// `max-width: 36rem; /* 576px */`
    Xl,
    /// `max-width: 42rem; /* 672px */`
    _2xl,
    /// `max-width: 48rem; /* 768px */`
    _3xl,
    /// `max-width: 56rem; /* 896px */`
    _4xl,
    /// `max-width: 64rem; /* 1024px */`
    _5xl,
    /// `max-width: 72rem; /* 1152px */`
    _6xl,
    /// `max-width: 80rem; /* 1280px */`
    _7xl,
    /// `max-width: 100%;`
    Full,
    /// `max-width: min-content;`
    Min,
    /// `max-width: max-content;`
    Max,
    /// `max-width: fit-content;`
    Fit,
    /// `max-width: 65ch;`
    Prose,
    /// `max-width: 640px;`
    ScreenSm,
    /// `max-width: 768px;`
    ScreenMd,
    /// `max-width: 1024px;`
    ScreenLg,
    /// `max-width: 1280px;`
    ScreenXl,
    /// `max-width: 1536px;`
    Screen2xl,
}

/// Utilities for setting the height of an element.
/// 
/// <https://tailwindcss.com/docs/height>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(
    prefix = "h", 
    replace(from = "_", to = "."),
    replace(from = "div", to = "/")
)]
pub enum Height {
    /// `height: 0px;`
    _0,
    /// `height: 1px;`
    Px,
    /// `height: 0.125rem; /* 2px */`
    _0_5,
    /// `height: 0.25rem; /* 4px */`
    _1,
    /// `height: 0.375rem; /* 6px */`
    _1_5,
    /// `height: 0.5rem; /* 8px */`
    _2,
    /// `height: 0.625rem; /* 10px */`
    _2_5,
    /// `height: 0.75rem; /* 12px */`
    _3,
    /// `height: 0.875rem; /* 14px */`
    _3_5,
    /// `height: 1rem; /* 16px */`
    _4,
    /// `height: 1.25rem; /* 20px */`
    _5,
    /// `height: 1.5rem; /* 24px */`
    _6,
    /// `height: 1.75rem; /* 28px */`
    _7,
    /// `height: 2rem; /* 32px */`
    _8,
    /// `height: 2.25rem; /* 36px */`
    _9,
    /// `height: 2.5rem; /* 40px */`
    _10,
    /// `height: 2.75rem; /* 44px */`
    _11,
    /// `height: 3rem; /* 48px */`
    _12,
    /// `height: 3.5rem; /* 56px */`
    _14,
    /// `height: 4rem; /* 64px */`
    _16,
    /// `height: 5rem; /* 80px */`
    _20,
    /// `height: 6rem; /* 96px */`
    _24,
    /// `height: 7rem; /* 112px */`
    _28,
    /// `height: 8rem; /* 128px */`
    _32,
    /// `height: 9rem; /* 144px */`
    _36,
    /// `height: 10rem; /* 160px */`
    _40,
    /// `height: 11rem; /* 176px */`
    _44,
    /// `height: 12rem; /* 192px */`
    _48,
    /// `height: 13rem; /* 208px */`
    _52,
    /// `height: 14rem; /* 224px */`
    _56,
    /// `height: 15rem; /* 240px */`
    _60,
    /// `height: 16rem; /* 256px */`
    _64,
    /// `height: 18rem; /* 288px */`
    _72,
    /// `height: 20rem; /* 320px */`
    _80,
    /// `height: 24rem; /* 384px */`
    _96,
    /// `height: auto;`
    Auto,
    /// `height: 50%;`
    _1div2,
    /// `height: 33.333333%;`
    _1div3,
    /// `height: 66.666667%;`
    _2div3,
    /// `height: 25%;`
    _1div4,
    /// `height: 50%;`
    _2div4,
    /// `height: 75%;`
    _3div4,
    /// `height: 20%;`
    _1div5,
    /// `height: 40%;`
    _2div5,
    /// `height: 60%;`
    _3div5,
    /// `height: 80%;`
    _4div5,
    /// `height: 16.666667%;`
    _1div6,
    /// `height: 33.333333%;`
    _2div6,
    /// `height: 50%;`
    _3div6,
    /// `height: 66.666667%;`
    _4div6,
    /// `height: 83.333333%;`
    _5div6,
    /// `height: 100%;`
    Full,
    /// `height: 100vh;`
    Screen,
    /// `height: 100svh;`
    Svh,
    /// `height: 100lvh;`
    Lvh,
    /// `height: 100dvh;`
    Dvh,
    /// `height: min-content;`
    Min,
    /// `height: max-content;`
    Max,
    /// `height: fit-content;`
    Fit,
}

/// Utilities for setting the minimum height of an element.
/// 
/// <https://tailwindcss.com/docs/min-height>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "min-h", replace(from = "_", to = "."))]
pub enum MinHeight {
    /// `min-height: 0px;`
    _0,
    /// `min-height: 0.25rem; /* 4px */`
    _1,
    /// `min-height: 0.5rem; /* 8px */`
    _2,
    /// `min-height: 0.75rem; /* 12px */`
    _3,
    /// `min-height: 1rem; /* 16px */`
    _4,
    /// `min-height: 1.25rem; /* 20px */`
    _5,
    /// `min-height: 1.5rem; /* 24px */`
    _6,
    /// `min-height: 1.75rem; /* 28px */`
    _7,
    /// `min-height: 2rem; /* 32px */`
    _8,
    /// `min-height: 2.25rem; /* 36px */`
    _9,
    /// `min-height: 2.5rem; /* 40px */`
    _10,
    /// `min-height: 2.75rem; /* 44px */`
    _11,
    /// `min-height: 3rem; /* 48px */`
    _12,
    /// `min-height: 3.5rem; /* 56px */`
    _14,
    /// `min-height: 4rem; /* 64px */`
    _16,
    /// `min-height: 5rem; /* 80px */`
    _20,
    /// `min-height: 6rem; /* 96px */`
    _24,
    /// `min-height: 7rem; /* 112px */`
    _28,
    /// `min-height: 8rem; /* 128px */`
    _32,
    /// `min-height: 9rem; /* 144px */`
    _36,
    /// `min-height: 10rem; /* 160px */`
    _40,
    /// `min-height: 11rem; /* 176px */`
    _44,
    /// `min-height: 12rem; /* 192px */`
    _48,
    /// `min-height: 13rem; /* 208px */`
    _52,
    /// `min-height: 14rem; /* 224px */`
    _56,
    /// `min-height: 15rem; /* 240px */`
    _60,
    /// `min-height: 16rem; /* 256px */`
    _64,
    /// `min-height: 18rem; /* 288px */`
    _72,
    /// `min-height: 20rem; /* 320px */`
    _80,
    /// `min-height: 24rem; /* 384px */`
    _96,
    /// `min-height: 1px;`
    Px,
    /// `min-height: 0.125rem; /* 2px */`
    _0_5,
    /// `min-height: 0.375rem; /* 6px */`
    _1_5,
    /// `min-height: 0.625rem; /* 10px */`
    _2_5,
    /// `min-height: 0.875rem; /* 14px */`
    _3_5,
    /// `min-height: 100%;`
    Full,
    /// `min-height: 100vh;`
    Screen,
    /// `min-height: 100svh;`
    Svh,
    /// `min-height: 100lvh;`
    Lvh,
    /// `min-height: 100dvh;`
    Dvh,
    /// `min-height: min-content;`
    Min,
    /// `min-height: max-content;`
    Max,
    /// `min-height: fit-content;`
    Fit,
}

/// Utilities for setting the maximum height of an element.
/// 
/// <https://tailwindcss.com/docs/max-height>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "max-h", replace(from = "_", to = "."))]
pub enum MaxHeight {
    /// `max-height: 0px;`
    _0,
    /// `max-height: 1px;`
    Px,
    /// `max-height: 0.125rem; /* 2px */`
    _0_5,
    /// `max-height: 0.25rem; /* 4px */`
    _1,
    /// `max-height: 0.375rem; /* 6px */`
    _1_5,
    /// `max-height: 0.5rem; /* 8px */`
    _2,
    /// `max-height: 0.625rem; /* 10px */`
    _2_5,
    /// `max-height: 0.75rem; /* 12px */`
    _3,
    /// `max-height: 0.875rem; /* 14px */`
    _3_5,
    /// `max-height: 1rem; /* 16px */`
    _4,
    /// `max-height: 1.25rem; /* 20px */`
    _5,
    /// `max-height: 1.5rem; /* 24px */`
    _6,
    /// `max-height: 1.75rem; /* 28px */`
    _7,
    /// `max-height: 2rem; /* 32px */`
    _8,
    /// `max-height: 2.25rem; /* 36px */`
    _9,
    /// `max-height: 2.5rem; /* 40px */`
    _10,
    /// `max-height: 2.75rem; /* 44px */`
    _11,
    /// `max-height: 3rem; /* 48px */`
    _12,
    /// `max-height: 3.5rem; /* 56px */`
    _14,
    /// `max-height: 4rem; /* 64px */`
    _16,
    /// `max-height: 5rem; /* 80px */`
    _20,
    /// `max-height: 6rem; /* 96px */`
    _24,
    /// `max-height: 7rem; /* 112px */`
    _28,
    /// `max-height: 8rem; /* 128px */`
    _32,
    /// `max-height: 9rem; /* 144px */`
    _36,
    /// `max-height: 10rem; /* 160px */`
    _40,
    /// `max-height: 11rem; /* 176px */`
    _44,
    /// `max-height: 12rem; /* 192px */`
    _48,
    /// `max-height: 13rem; /* 208px */`
    _52,
    /// `max-height: 14rem; /* 224px */`
    _56,
    /// `max-height: 15rem; /* 240px */`
    _60,
    /// `max-height: 16rem; /* 256px */`
    _64,
    /// `max-height: 18rem; /* 288px */`
    _72,
    /// `max-height: 20rem; /* 320px */`
    _80,
    /// `max-height: 24rem; /* 384px */`
    _96,
    /// `max-height: none;`
    None,
    /// `max-height: 100%;`
    Full,
    /// `max-height: 100vh;`
    Screen,
    /// `max-height: 100svh;`
    Svh,
    /// `max-height: 100lvh;`
    Lvh,
    /// `max-height: 100dvh;`
    Dvh,
    /// `max-height: min-content;`
    Min,
    /// `max-height: max-content;`
    Max,
    /// `max-height: fit-content;`
    Fit,
}

/// Utilities for setting the width and height of an element at the same time.
/// 
/// <https://tailwindcss.com/docs/size>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(
    prefix = "size", 
    replace(from = "_", to = "."),
    replace(from = "div", to = "/")
)]
pub enum Size {
    /// `width: 0px;`
    /// 
    /// `height: 0px;`
    _0,
    /// `width: 1px;`
    /// 
    /// `height: 1px;`
    Px,
    /// `width: 0.125rem; /* 2px */`
    /// 
    /// `height: 0.125rem; /* 2px */`
    _0_5,
    /// `width: 0.25rem; /* 4px */`
    /// 
    /// `height: 0.25rem; /* 4px */`
    _1,
    /// `width: 0.375rem; /* 6px */`
    /// 
    /// `height: 0.375rem; /* 6px */`
    _1_5,
    /// `width: 0.5rem; /* 8px */`
    /// 
    /// `height: 0.5rem; /* 8px */`
    _2,
    /// `width: 0.625rem; /* 10px */`
    /// 
    /// `height: 0.625rem; /* 10px */`
    _2_5,
    /// `width: 0.75rem; /* 12px */`
    /// 
    /// `height: 0.75rem; /* 12px */`
    _3,
    /// `width: 0.875rem; /* 14px */`
    /// 
    /// `height: 0.875rem; /* 14px */`
    _3_5,
    /// `width: 1rem; /* 16px */`
    /// 
    /// `height: 1rem; /* 16px */`
    _4,
    /// `width: 1.25rem; /* 20px */`
    /// 
    /// `height: 1.25rem; /* 20px */`
    _5,
    /// `width: 1.5rem; /* 24px */`
    /// 
    /// `height: 1.5rem; /* 24px */`
    _6,
    /// `width: 1.75rem; /* 28px */`
    /// 
    /// `height: 1.75rem; /* 28px */`
    _7,
    /// `width: 2rem; /* 32px */`
    /// 
    /// `height: 2rem; /* 32px */`
    _8,
    /// `width: 2.25rem; /* 36px */`
    /// 
    /// `height: 2.25rem; /* 36px */`
    _9,
    /// `width: 2.5rem; /* 40px */`
    /// 
    /// `height: 2.5rem; /* 40px */`
    _10,
    /// `width: 2.75rem; /* 44px */`
    /// 
    /// `height: 2.75rem; /* 44px */`
    _11,
    /// `width: 3rem; /* 48px */`
    /// 
    /// `height: 3rem; /* 48px */`
    _12,
    /// `width: 3.5rem; /* 56px */`
    /// 
    /// `height: 3.5rem; /* 56px */`
    _14,
    /// `width: 4rem; /* 64px */`
    /// 
    /// `height: 4rem; /* 64px */`
    _16,
    /// `width: 5rem; /* 80px */`
    /// 
    /// `height: 5rem; /* 80px */`
    _20,
    /// `width: 6rem; /* 96px */`
    /// 
    /// `height: 6rem; /* 96px */`
    _24,
    /// `width: 7rem; /* 112px */`
    /// 
    /// `height: 7rem; /* 112px */`
    _28,
    /// `width: 8rem; /* 128px */`
    /// 
    /// `height: 8rem; /* 128px */`
    _32,
    /// `width: 9rem; /* 144px */`
    /// 
    /// `height: 9rem; /* 144px */`
    _36,
    /// `width: 10rem; /* 160px */`
    /// 
    /// `height: 10rem; /* 160px */`
    _40,
    /// `width: 11rem; /* 176px */`
    /// 
    /// `height: 11rem; /* 176px */`
    _44,
    /// `width: 12rem; /* 192px */`
    /// 
    /// `height: 12rem; /* 192px */`
    _48,
    /// `width: 13rem; /* 208px */`
    /// 
    /// `height: 13rem; /* 208px */`
    _52,
    /// `width: 14rem; /* 224px */`
    /// 
    /// `height: 14rem; /* 224px */`
    _56,
    /// `width: 15rem; /* 240px */`
    /// 
    /// `height: 15rem; /* 240px */`
    _60,
    /// `width: 16rem; /* 256px */`
    /// 
    /// `height: 16rem; /* 256px */`
    _64,
    /// `width: 18rem; /* 288px */`
    /// 
    /// `height: 18rem; /* 288px */`
    _72,
    /// `width: 20rem; /* 320px */`
    /// 
    /// `height: 20rem; /* 320px */`
    _80,
    /// `width: 24rem; /* 384px */`
    /// 
    /// `height: 24rem; /* 384px */`
    _96,
    /// `width: auto;`
    /// 
    /// `height: auto;`
    Auto,
    /// `width: 50%;`
    /// 
    /// `height: 50%;`
    _1div2,
    /// `width: 33.333333%;`
    /// 
    /// `height: 33.333333%;`
    _1div3,
    /// `width: 66.666667%;`
    /// 
    /// `height: 66.666667%;`
    _2div3,
    /// `width: 25%;`
    /// 
    /// `height: 25%;`
    _1div4,
    /// `width: 50%;`
    /// 
    /// `height: 50%;`
    _2div4,
    /// `width: 75%;`
    /// 
    /// `height: 75%;`
    _3div4,
    /// `width: 20%;`
    /// 
    /// `height: 20%;`
    _1div5,
    /// `width: 40%;`
    /// 
    /// `height: 40%;`
    _2div5,
    /// `width: 60%;`
    /// 
    /// `height: 60%;`
    _3div5,
    /// `width: 80%;`
    /// 
    /// `height: 80%;`
    _4div5,
    /// `width: 16.666667%;`
    /// 
    /// `height: 16.666667%;`
    _1div6,
    /// `width: 33.333333%;`
    /// 
    /// `height: 33.333333%;`
    _2div6,
    /// `width: 50%;`
    /// 
    /// `height: 50%;`
    _3div6,
    /// `width: 66.666667%;`
    /// 
    /// `height: 66.666667%;`
    _4div6,
    /// `width: 83.333333%;`
    /// 
    /// `height: 83.333333%;`
    _5div6,
    /// `width: 8.333333%;`
    /// 
    /// `height: 8.333333%;`
    _1div12,
    /// `width: 16.666667%;`
    /// 
    /// `height: 16.666667%;`
    _2div12,
    /// `width: 25%;`
    /// 
    /// `height: 25%;`
    _3div12,
    /// `width: 33.333333%;`
    /// 
    /// `height: 33.333333%;`
    _4div12,
    /// `width: 41.666667%;`
    /// 
    /// `height: 41.666667%;`
    _5div12,
    /// `width: 50%;`
    /// 
    /// `height: 50%;`
    _6div12,
    /// `width: 58.333333%;`
    /// 
    /// `height: 58.333333%;`
    _7div12,
    /// `width: 66.666667%;`
    /// 
    /// `height: 66.666667%;`
    _8div12,
    /// `width: 75%;`
    /// 
    /// `height: 75%;`
    _9div12,
    /// `width: 83.333333%;`
    /// 
    /// `height: 83.333333%;`
    _10div12,
    /// `width: 91.666667%;`
    /// 
    /// `height: 91.666667%;`
    _11div12,
    /// `width: 100%;`
    /// 
    /// `height: 100%;`
    Full,
    /// `width: min-content;`
    /// 
    /// `height: min-content;`
    Min,
    /// `width: max-content;`
    /// 
    /// `height: max-content;`
    Max,
    /// `width: fit-content;`
    /// 
    /// `height: fit-content;`
    Fit,
}
