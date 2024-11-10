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
    /// ```css
    /// {
    ///     width: 0px;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     width: 1px;
    /// }
    /// ```
    Px,
    /// ```css
    /// {
    ///     width: 0.125rem; /* 2px */
    /// }
    /// ```
    _0_5,
    /// ```css
    /// {
    ///     width: 0.25rem; /* 4px */
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     width: 0.375rem; /* 6px */
    /// }
    /// ```
    _1_5,
    /// ```css
    /// {
    ///     width: 0.5rem; /* 8px */
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     width: 0.625rem; /* 10px */
    /// }
    /// ```
    _2_5,
    /// ```css
    /// {
    ///     width: 0.75rem; /* 12px */
    /// }
    /// ```
    _3,
    /// ```css
    /// {
    ///     width: 0.875rem; /* 14px */
    /// }
    /// ```
    _3_5,
    /// ```css
    /// {
    ///     width: 1rem; /* 16px */
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     width: 1.25rem; /* 20px */
    /// }
    /// ```
    _5,
    /// ```css
    /// {
    ///     width: 1.5rem; /* 24px */
    /// }
    /// ```
    _6,
    /// ```css
    /// {
    ///     width: 1.75rem; /* 28px */
    /// }
    /// ```
    _7,
    /// ```css
    /// {
    ///     width: 2rem; /* 32px */
    /// }
    /// ```
    _8,
    /// ```css
    /// {
    ///     width: 2.25rem; /* 36px */
    /// }
    /// ```
    _9,
    /// ```css
    /// {
    ///     width: 2.5rem; /* 40px */
    /// }
    /// ```
    _10,
    /// ```css
    /// {
    ///     width: 2.75rem; /* 44px */
    /// }
    /// ```
    _11,
    /// ```css
    /// {
    ///     width: 3rem; /* 48px */
    /// }
    /// ```
    _12,
    /// ```css
    /// {
    ///     width: 3.5rem; /* 56px */
    /// }
    /// ```
    _14,
    /// ```css
    /// {
    ///     width: 4rem; /* 64px */
    /// }
    /// ```
    _16,
    /// ```css
    /// {
    ///     width: 5rem; /* 80px */
    /// }
    /// ```
    _20,
    /// ```css
    /// {
    ///     width: 6rem; /* 96px */
    /// }
    /// ```
    _24,
    /// ```css
    /// {
    ///     width: 7rem; /* 112px */
    /// }
    /// ```
    _28,
    /// ```css
    /// {
    ///     width: 8rem; /* 128px */
    /// }
    /// ```
    _32,
    /// ```css
    /// {
    ///     width: 9rem; /* 144px */
    /// }
    /// ```
    _36,
    /// ```css
    /// {
    ///     width: 10rem; /* 160px */
    /// }
    /// ```
    _40,
    /// ```css
    /// {
    ///     width: 11rem; /* 176px */
    /// }
    /// ```
    _44,
    /// ```css
    /// {
    ///     width: 12rem; /* 192px */
    /// }
    /// ```
    _48,
    /// ```css
    /// {
    ///     width: 13rem; /* 208px */
    /// }
    /// ```
    _52,
    /// ```css
    /// {
    ///     width: 14rem; /* 224px */
    /// }
    /// ```
    _56,
    /// ```css
    /// {
    ///     width: 15rem; /* 240px */
    /// }
    /// ```
    _60,
    /// ```css
    /// {
    ///     width: 16rem; /* 256px */
    /// }
    /// ```
    _64,
    /// ```css
    /// {
    ///     width: 18rem; /* 288px */
    /// }
    /// ```
    _72,
    /// ```css
    /// {
    ///     width: 20rem; /* 320px */
    /// }
    /// ```
    _80,
    /// ```css
    /// {
    ///     width: 24rem; /* 384px */
    /// }
    /// ```
    _96,
    /// ```css
    /// {
    ///     width: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     width: 50%;
    /// }
    /// ```
    _1div2,
    /// ```css
    /// {
    ///     width: 33.333333%;
    /// }
    /// ```
    _1div3,
    /// ```css
    /// {
    ///     width: 66.666667%;
    /// }
    /// ```
    _2div3,
    /// ```css
    /// {
    ///     width: 25%;
    /// }
    /// ```
    _1div4,
    /// ```css
    /// {
    ///     width: 50%;
    /// }
    /// ```
    _2div4,
    /// ```css
    /// {
    ///     width: 75%;
    /// }
    /// ```
    _3div4,
    /// ```css
    /// {
    ///     width: 20%;
    /// }
    /// ```
    _1div5,
    /// ```css
    /// {
    ///     width: 40%;
    /// }
    /// ```
    _2div5,
    /// ```css
    /// {
    ///     width: 60%;
    /// }
    /// ```
    _3div5,
    /// ```css
    /// {
    ///     width: 80%;
    /// }
    /// ```
    _4div5,
    /// ```css
    /// {
    ///     width: 16.666667%;
    /// }
    /// ```
    _1div6,
    /// ```css
    /// {
    ///     width: 33.333333%;
    /// }
    /// ```
    _2div6,
    /// ```css
    /// {
    ///     width: 50%;
    /// }
    /// ```
    _3div6,
    /// ```css
    /// {
    ///     width: 66.666667%;
    /// }
    /// ```
    _4div6,
    /// ```css
    /// {
    ///     width: 83.333333%;
    /// }
    /// ```
    _5div6,
    /// ```css
    /// {
    ///     width: 8.333333%;
    /// }
    /// ```
    _1div12,
    /// ```css
    /// {
    ///     width: 16.666667%;
    /// }
    /// ```
    _2div12,
    /// ```css
    /// {
    ///     width: 25%;
    /// }
    /// ```
    _3div12,
    /// ```css
    /// {
    ///     width: 33.333333%;
    /// }
    /// ```
    _4div12,
    /// ```css
    /// {
    ///     width: 41.666667%;
    /// }
    /// ```
    _5div12,
    /// ```css
    /// {
    ///     width: 50%;
    /// }
    /// ```
    _6div12,
    /// ```css
    /// {
    ///     width: 58.333333%;
    /// }
    /// ```
    _7div12,
    /// ```css
    /// {
    ///     width: 66.666667%;
    /// }
    /// ```
    _8div12,
    /// ```css
    /// {
    ///     width: 75%;
    /// }
    /// ```
    _9div12,
    /// ```css
    /// {
    ///     width: 83.333333%;
    /// }
    /// ```
    _10div12,
    /// ```css
    /// {
    ///     width: 91.666667%;
    /// }
    /// ```
    _11div12,
    /// ```css
    /// {
    ///     width: 100%;
    /// }
    /// ```
    Full,
    /// ```css
    /// {
    ///     width: 100vw;
    /// }
    /// ```
    Screen,
    /// ```css
    /// {
    ///     width: 100svw;
    /// }
    /// ```
    Svw,
    /// ```css
    /// {
    ///     width: 100lvw;
    /// }
    /// ```
    Lvw,
    /// ```css
    /// {
    ///     width: 100dvw;
    /// }
    /// ```
    Dvw,
    /// ```css
    /// {
    ///     width: min-content;
    /// }
    /// ```
    Min,
    /// ```css
    /// {
    ///     width: max-content;
    /// }
    /// ```
    Max,
    /// ```css
    /// {
    ///     width: fit-content;
    /// }
    /// ```
    Fit,
}

/// Utilities for setting the minimum width of an element.
/// 
/// <https://tailwindcss.com/docs/min-width>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "min-w", replace(from = "_", to = "."))]
pub enum MinWidth {
    /// ```css
    /// {
    ///     min-width: 0px;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     min-width: 0.25rem; /* 4px */
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     min-width: 0.5rem; /* 8px */
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     min-width: 0.75rem; /* 12px */
    /// }
    /// ```
    _3,
    /// ```css
    /// {
    ///     min-width: 1rem; /* 16px */
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     min-width: 1.25rem; /* 20px */
    /// }
    /// ```
    _5,
    /// ```css
    /// {
    ///     min-width: 1.5rem; /* 24px */
    /// }
    /// ```
    _6,
    /// ```css
    /// {
    ///     min-width: 1.75rem; /* 28px */
    /// }
    /// ```
    _7,
    /// ```css
    /// {
    ///     min-width: 2rem; /* 32px */
    /// }
    /// ```
    _8,
    /// ```css
    /// {
    ///     min-width: 2.25rem; /* 36px */
    /// }
    /// ```
    _9,
    /// ```css
    /// {
    ///     min-width: 2.5rem; /* 40px */
    /// }
    /// ```
    _10,
    /// ```css
    /// {
    ///     min-width: 2.75rem; /* 44px */
    /// }
    /// ```
    _11,
    /// ```css
    /// {
    ///     min-width: 3rem; /* 48px */
    /// }
    /// ```
    _12,
    /// ```css
    /// {
    ///     min-width: 3.5rem; /* 56px */
    /// }
    /// ```
    _14,
    /// ```css
    /// {
    ///     min-width: 4rem; /* 64px */
    /// }
    /// ```
    _16,
    /// ```css
    /// {
    ///     min-width: 5rem; /* 80px */
    /// }
    /// ```
    _20,
    /// ```css
    /// {
    ///     min-width: 6rem; /* 96px */
    /// }
    /// ```
    _24,
    /// ```css
    /// {
    ///     min-width: 7rem; /* 112px */
    /// }
    /// ```
    _28,
    /// ```css
    /// {
    ///     min-width: 8rem; /* 128px */
    /// }
    /// ```
    _32,
    /// ```css
    /// {
    ///     min-width: 9rem; /* 144px */
    /// }
    /// ```
    _36,
    /// ```css
    /// {
    ///     min-width: 10rem; /* 160px */
    /// }
    /// ```
    _40,
    /// ```css
    /// {
    ///     min-width: 11rem; /* 176px */
    /// }
    /// ```
    _44,
    /// ```css
    /// {
    ///     min-width: 12rem; /* 192px */
    /// }
    /// ```
    _48,
    /// ```css
    /// {
    ///     min-width: 13rem; /* 208px */
    /// }
    /// ```
    _52,
    /// ```css
    /// {
    ///     min-width: 14rem; /* 224px */
    /// }
    /// ```
    _56,
    /// ```css
    /// {
    ///     min-width: 15rem; /* 240px */
    /// }
    /// ```
    _60,
    /// ```css
    /// {
    ///     min-width: 16rem; /* 256px */
    /// }
    /// ```
    _64,
    /// ```css
    /// {
    ///     min-width: 18rem; /* 288px */
    /// }
    /// ```
    _72,
    /// ```css
    /// {
    ///     min-width: 20rem; /* 320px */
    /// }
    /// ```
    _80,
    /// ```css
    /// {
    ///     min-width: 24rem; /* 384px */
    /// }
    /// ```
    _96,
    /// ```css
    /// {
    ///     min-width: 1px;
    /// }
    /// ```
    Px,
    /// ```css
    /// {
    ///     min-width: 0.125rem; /* 2px */
    /// }
    /// ```
    _0_5,
    /// ```css
    /// {
    ///     min-width: 0.375rem; /* 6px */
    /// }
    /// ```
    _1_5,
    /// ```css
    /// {
    ///     min-width: 0.625rem; /* 10px */
    /// }
    /// ```
    _2_5,
    /// ```css
    /// {
    ///     min-width: 0.875rem; /* 14px */
    /// }
    /// ```
    _3_5,
    /// ```css
    /// {
    ///     min-width: 100%;
    /// }
    /// ```
    Full,
    /// ```css
    /// {
    ///     min-width: min-content;
    /// }
    /// ```
    Min,
    /// ```css
    /// {
    ///     min-width: max-content;
    /// }
    /// ```
    Max,
    /// ```css
    /// {
    ///     min-width: fit-content;
    /// }
    /// ```
    Fit,
}

/// Utilities for setting the maximum width of an element.
/// 
/// <https://tailwindcss.com/docs/max-width>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "max-w", replace(from = "_", to = "."))]
pub enum MaxWidth {
    /// ```css
    /// {
    ///     max-width: 0px;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     max-width: 1px;
    /// }
    /// ```
    Px,
    /// ```css
    /// {
    ///     max-width: 0.125rem; /* 2px */
    /// }
    /// ```
    _0_5,
    /// ```css
    /// {
    ///     max-width: 0.25rem; /* 4px */
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     max-width: 0.375rem; /* 6px */
    /// }
    /// ```
    _1_5,
    /// ```css
    /// {
    ///     max-width: 0.5rem; /* 8px */
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     max-width: 0.625rem; /* 10px */
    /// }
    /// ```
    _2_5,
    /// ```css
    /// {
    ///     max-width: 0.75rem; /* 12px */
    /// }
    /// ```
    _3,
    /// ```css
    /// {
    ///     max-width: 0.875rem; /* 14px */
    /// }
    /// ```
    _3_5,
    /// ```css
    /// {
    ///     max-width: 1rem; /* 16px */
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     max-width: 1.25rem; /* 20px */
    /// }
    /// ```
    _5,
    /// ```css
    /// {
    ///     max-width: 1.5rem; /* 24px */
    /// }
    /// ```
    _6,
    /// ```css
    /// {
    ///     max-width: 1.75rem; /* 28px */
    /// }
    /// ```
    _7,
    /// ```css
    /// {
    ///     max-width: 2rem; /* 32px */
    /// }
    /// ```
    _8,
    /// ```css
    /// {
    ///     max-width: 2.25rem; /* 36px */
    /// }
    /// ```
    _9,
    /// ```css
    /// {
    ///     max-width: 2.5rem; /* 40px */
    /// }
    /// ```
    _10,
    /// ```css
    /// {
    ///     max-width: 2.75rem; /* 44px */
    /// }
    /// ```
    _11,
    /// ```css
    /// {
    ///     max-width: 3rem; /* 48px */
    /// }
    /// ```
    _12,
    /// ```css
    /// {
    ///     max-width: 3.5rem; /* 56px */
    /// }
    /// ```
    _14,
    /// ```css
    /// {
    ///     max-width: 4rem; /* 64px */
    /// }
    /// ```
    _16,
    /// ```css
    /// {
    ///     max-width: 5rem; /* 80px */
    /// }
    /// ```
    _20,
    /// ```css
    /// {
    ///     max-width: 6rem; /* 96px */
    /// }
    /// ```
    _24,
    /// ```css
    /// {
    ///     max-width: 7rem; /* 112px */
    /// }
    /// ```
    _28,
    /// ```css
    /// {
    ///     max-width: 8rem; /* 128px */
    /// }
    /// ```
    _32,
    /// ```css
    /// {
    ///     max-width: 9rem; /* 144px */
    /// }
    /// ```
    _36,
    /// ```css
    /// {
    ///     max-width: 10rem; /* 160px */
    /// }
    /// ```
    _40,
    /// ```css
    /// {
    ///     max-width: 11rem; /* 176px */
    /// }
    /// ```
    _44,
    /// ```css
    /// {
    ///     max-width: 12rem; /* 192px */
    /// }
    /// ```
    _48,
    /// ```css
    /// {
    ///     max-width: 13rem; /* 208px */
    /// }
    /// ```
    _52,
    /// ```css
    /// {
    ///     max-width: 14rem; /* 224px */
    /// }
    /// ```
    _56,
    /// ```css
    /// {
    ///     max-width: 15rem; /* 240px */
    /// }
    /// ```
    _60,
    /// ```css
    /// {
    ///     max-width: 16rem; /* 256px */
    /// }
    /// ```
    _64,
    /// ```css
    /// {
    ///     max-width: 18rem; /* 288px */
    /// }
    /// ```
    _72,
    /// ```css
    /// {
    ///     max-width: 20rem; /* 320px */
    /// }
    /// ```
    _80,
    /// ```css
    /// {
    ///     max-width: 24rem; /* 384px */
    /// }
    /// ```
    _96,
    /// ```css
    /// {
    ///     max-width: none;
    /// }
    /// ```
    None,
    /// ```css
    /// {
    ///     max-width: 20rem; /* 320px */
    /// }
    /// ```
    Xs,
    /// ```css
    /// {
    ///     max-width: 24rem; /* 384px */
    /// }
    /// ```
    Sm,
    /// ```css
    /// {
    ///     max-width: 28rem; /* 448px */
    /// }
    /// ```
    Md,
    /// ```css
    /// {
    ///     max-width: 32rem; /* 512px */
    /// }
    /// ```
    Lg,
    /// ```css
    /// {
    ///     max-width: 36rem; /* 576px */
    /// }
    /// ```
    Xl,
    /// ```css
    /// {
    ///     max-width: 42rem; /* 672px */
    /// }
    /// ```
    _2xl,
    /// ```css
    /// {
    ///     max-width: 48rem; /* 768px */
    /// }
    /// ```
    _3xl,
    /// ```css
    /// {
    ///     max-width: 56rem; /* 896px */
    /// }
    /// ```
    _4xl,
    /// ```css
    /// {
    ///     max-width: 64rem; /* 1024px */
    /// }
    /// ```
    _5xl,
    /// ```css
    /// {
    ///     max-width: 72rem; /* 1152px */
    /// }
    /// ```
    _6xl,
    /// ```css
    /// {
    ///     max-width: 80rem; /* 1280px */
    /// }
    /// ```
    _7xl,
    /// ```css
    /// {
    ///     max-width: 100%;
    /// }
    /// ```
    Full,
    /// ```css
    /// {
    ///     max-width: min-content;
    /// }
    /// ```
    Min,
    /// ```css
    /// {
    ///     max-width: max-content;
    /// }
    /// ```
    Max,
    /// ```css
    /// {
    ///     max-width: fit-content;
    /// }
    /// ```
    Fit,
    /// ```css
    /// {
    ///     max-width: 65ch;
    /// }
    /// ```
    Prose,
    /// ```css
    /// {
    ///     max-width: 640px;
    /// }
    /// ```
    ScreenSm,
    /// ```css
    /// {
    ///     max-width: 768px;
    /// }
    /// ```
    ScreenMd,
    /// ```css
    /// {
    ///     max-width: 1024px;
    /// }
    /// ```
    ScreenLg,
    /// ```css
    /// {
    ///     max-width: 1280px;
    /// }
    /// ```
    ScreenXl,
    /// ```css
    /// {
    ///     max-width: 1536px;
    /// }
    /// ```
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
    /// ```css
    /// {
    ///     height: 0px;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     height: 1px;
    /// }
    /// ```
    Px,
    /// ```css
    /// {
    ///     height: 0.125rem; /* 2px */
    /// }
    /// ```
    _0_5,
    /// ```css
    /// {
    ///     height: 0.25rem; /* 4px */
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     height: 0.375rem; /* 6px */
    /// }
    /// ```
    _1_5,
    /// ```css
    /// {
    ///     height: 0.5rem; /* 8px */
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     height: 0.625rem; /* 10px */
    /// }
    /// ```
    _2_5,
    /// ```css
    /// {
    ///     height: 0.75rem; /* 12px */
    /// }
    /// ```
    _3,
    /// ```css
    /// {
    ///     height: 0.875rem; /* 14px */
    /// }
    /// ```
    _3_5,
    /// ```css
    /// {
    ///     height: 1rem; /* 16px */
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     height: 1.25rem; /* 20px */
    /// }
    /// ```
    _5,
    /// ```css
    /// {
    ///     height: 1.5rem; /* 24px */
    /// }
    /// ```
    _6,
    /// ```css
    /// {
    ///     height: 1.75rem; /* 28px */
    /// }
    /// ```
    _7,
    /// ```css
    /// {
    ///     height: 2rem; /* 32px */
    /// }
    /// ```
    _8,
    /// ```css
    /// {
    ///     height: 2.25rem; /* 36px */
    /// }
    /// ```
    _9,
    /// ```css
    /// {
    ///     height: 2.5rem; /* 40px */
    /// }
    /// ```
    _10,
    /// ```css
    /// {
    ///     height: 2.75rem; /* 44px */
    /// }
    /// ```
    _11,
    /// ```css
    /// {
    ///     height: 3rem; /* 48px */
    /// }
    /// ```
    _12,
    /// ```css
    /// {
    ///     height: 3.5rem; /* 56px */
    /// }
    /// ```
    _14,
    /// ```css
    /// {
    ///     height: 4rem; /* 64px */
    /// }
    /// ```
    _16,
    /// ```css
    /// {
    ///     height: 5rem; /* 80px */
    /// }
    /// ```
    _20,
    /// ```css
    /// {
    ///     height: 6rem; /* 96px */
    /// }
    /// ```
    _24,
    /// ```css
    /// {
    ///     height: 7rem; /* 112px */
    /// }
    /// ```
    _28,
    /// ```css
    /// {
    ///     height: 8rem; /* 128px */
    /// }
    /// ```
    _32,
    /// ```css
    /// {
    ///     height: 9rem; /* 144px */
    /// }
    /// ```
    _36,
    /// ```css
    /// {
    ///     height: 10rem; /* 160px */
    /// }
    /// ```
    _40,
    /// ```css
    /// {
    ///     height: 11rem; /* 176px */
    /// }
    /// ```
    _44,
    /// ```css
    /// {
    ///     height: 12rem; /* 192px */
    /// }
    /// ```
    _48,
    /// ```css
    /// {
    ///     height: 13rem; /* 208px */
    /// }
    /// ```
    _52,
    /// ```css
    /// {
    ///     height: 14rem; /* 224px */
    /// }
    /// ```
    _56,
    /// ```css
    /// {
    ///     height: 15rem; /* 240px */
    /// }
    /// ```
    _60,
    /// ```css
    /// {
    ///     height: 16rem; /* 256px */
    /// }
    /// ```
    _64,
    /// ```css
    /// {
    ///     height: 18rem; /* 288px */
    /// }
    /// ```
    _72,
    /// ```css
    /// {
    ///     height: 20rem; /* 320px */
    /// }
    /// ```
    _80,
    /// ```css
    /// {
    ///     height: 24rem; /* 384px */
    /// }
    /// ```
    _96,
    /// ```css
    /// {
    ///     height: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     height: 50%;
    /// }
    /// ```
    _1div2,
    /// ```css
    /// {
    ///     height: 33.333333%;
    /// }
    /// ```
    _1div3,
    /// ```css
    /// {
    ///     height: 66.666667%;
    /// }
    /// ```
    _2div3,
    /// ```css
    /// {
    ///     height: 25%;
    /// }
    /// ```
    _1div4,
    /// ```css
    /// {
    ///     height: 50%;
    /// }
    /// ```
    _2div4,
    /// ```css
    /// {
    ///     height: 75%;
    /// }
    /// ```
    _3div4,
    /// ```css
    /// {
    ///     height: 20%;
    /// }
    /// ```
    _1div5,
    /// ```css
    /// {
    ///     height: 40%;
    /// }
    /// ```
    _2div5,
    /// ```css
    /// {
    ///     height: 60%;
    /// }
    /// ```
    _3div5,
    /// ```css
    /// {
    ///     height: 80%;
    /// }
    /// ```
    _4div5,
    /// ```css
    /// {
    ///     height: 16.666667%;
    /// }
    /// ```
    _1div6,
    /// ```css
    /// {
    ///     height: 33.333333%;
    /// }
    /// ```
    _2div6,
    /// ```css
    /// {
    ///     height: 50%;
    /// }
    /// ```
    _3div6,
    /// ```css
    /// {
    ///     height: 66.666667%;
    /// }
    /// ```
    _4div6,
    /// ```css
    /// {
    ///     height: 83.333333%;
    /// }
    /// ```
    _5div6,
    /// ```css
    /// {
    ///     height: 100%;
    /// }
    /// ```
    Full,
    /// ```css
    /// {
    ///     height: 100vh;
    /// }
    /// ```
    Screen,
    /// ```css
    /// {
    ///     height: 100svh;
    /// }
    /// ```
    Svh,
    /// ```css
    /// {
    ///     height: 100lvh;
    /// }
    /// ```
    Lvh,
    /// ```css
    /// {
    ///     height: 100dvh;
    /// }
    /// ```
    Dvh,
    /// ```css
    /// {
    ///     height: min-content;
    /// }
    /// ```
    Min,
    /// ```css
    /// {
    ///     height: max-content;
    /// }
    /// ```
    Max,
    /// ```css
    /// {
    ///     height: fit-content;
    /// }
    /// ```
    Fit,
}

/// Utilities for setting the minimum height of an element.
/// 
/// <https://tailwindcss.com/docs/min-height>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "min-h", replace(from = "_", to = "."))]
pub enum MinHeight {
    /// ```css
    /// {
    ///     min-height: 0px;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     min-height: 0.25rem; /* 4px */
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     min-height: 0.5rem; /* 8px */
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     min-height: 0.75rem; /* 12px */
    /// }
    /// ```
    _3,
    /// ```css
    /// {
    ///     min-height: 1rem; /* 16px */
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     min-height: 1.25rem; /* 20px */
    /// }
    /// ```
    _5,
    /// ```css
    /// {
    ///     min-height: 1.5rem; /* 24px */
    /// }
    /// ```
    _6,
    /// ```css
    /// {
    ///     min-height: 1.75rem; /* 28px */
    /// }
    /// ```
    _7,
    /// ```css
    /// {
    ///     min-height: 2rem; /* 32px */
    /// }
    /// ```
    _8,
    /// ```css
    /// {
    ///     min-height: 2.25rem; /* 36px */
    /// }
    /// ```
    _9,
    /// ```css
    /// {
    ///     min-height: 2.5rem; /* 40px */
    /// }
    /// ```
    _10,
    /// ```css
    /// {
    ///     min-height: 2.75rem; /* 44px */
    /// }
    /// ```
    _11,
    /// ```css
    /// {
    ///     min-height: 3rem; /* 48px */
    /// }
    /// ```
    _12,
    /// ```css
    /// {
    ///     min-height: 3.5rem; /* 56px */
    /// }
    /// ```
    _14,
    /// ```css
    /// {
    ///     min-height: 4rem; /* 64px */
    /// }
    /// ```
    _16,
    /// ```css
    /// {
    ///     min-height: 5rem; /* 80px */
    /// }
    /// ```
    _20,
    /// ```css
    /// {
    ///     min-height: 6rem; /* 96px */
    /// }
    /// ```
    _24,
    /// ```css
    /// {
    ///     min-height: 7rem; /* 112px */
    /// }
    /// ```
    _28,
    /// ```css
    /// {
    ///     min-height: 8rem; /* 128px */
    /// }
    /// ```
    _32,
    /// ```css
    /// {
    ///     min-height: 9rem; /* 144px */
    /// }
    /// ```
    _36,
    /// ```css
    /// {
    ///     min-height: 10rem; /* 160px */
    /// }
    /// ```
    _40,
    /// ```css
    /// {
    ///     min-height: 11rem; /* 176px */
    /// }
    /// ```
    _44,
    /// ```css
    /// {
    ///     min-height: 12rem; /* 192px */
    /// }
    /// ```
    _48,
    /// ```css
    /// {
    ///     min-height: 13rem; /* 208px */
    /// }
    /// ```
    _52,
    /// ```css
    /// {
    ///     min-height: 14rem; /* 224px */
    /// }
    /// ```
    _56,
    /// ```css
    /// {
    ///     min-height: 15rem; /* 240px */
    /// }
    /// ```
    _60,
    /// ```css
    /// {
    ///     min-height: 16rem; /* 256px */
    /// }
    /// ```
    _64,
    /// ```css
    /// {
    ///     min-height: 18rem; /* 288px */
    /// }
    /// ```
    _72,
    /// ```css
    /// {
    ///     min-height: 20rem; /* 320px */
    /// }
    /// ```
    _80,
    /// ```css
    /// {
    ///     min-height: 24rem; /* 384px */
    /// }
    /// ```
    _96,
    /// ```css
    /// {
    ///     min-height: 1px;
    /// }
    /// ```
    Px,
    /// ```css
    /// {
    ///     min-height: 0.125rem; /* 2px */
    /// }
    /// ```
    _0_5,
    /// ```css
    /// {
    ///     min-height: 0.375rem; /* 6px */
    /// }
    /// ```
    _1_5,
    /// ```css
    /// {
    ///     min-height: 0.625rem; /* 10px */
    /// }
    /// ```
    _2_5,
    /// ```css
    /// {
    ///     min-height: 0.875rem; /* 14px */
    /// }
    /// ```
    _3_5,
    /// ```css
    /// {
    ///     min-height: 100%;
    /// }
    /// ```
    Full,
    /// ```css
    /// {
    ///     min-height: 100vh;
    /// }
    /// ```
    Screen,
    /// ```css
    /// {
    ///     min-height: 100svh;
    /// }
    /// ```
    Svh,
    /// ```css
    /// {
    ///     min-height: 100lvh;
    /// }
    /// ```
    Lvh,
    /// ```css
    /// {
    ///     min-height: 100dvh;
    /// }
    /// ```
    Dvh,
    /// ```css
    /// {
    ///     min-height: min-content;
    /// }
    /// ```
    Min,
    /// ```css
    /// {
    ///     min-height: max-content;
    /// }
    /// ```
    Max,
    /// ```css
    /// {
    ///     min-height: fit-content;
    /// }
    /// ```
    Fit,
}

/// Utilities for setting the maximum height of an element.
/// 
/// <https://tailwindcss.com/docs/max-height>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "max-h", replace(from = "_", to = "."))]
pub enum MaxHeight {
    /// ```css
    /// {
    ///     max-height: 0px;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     max-height: 1px;
    /// }
    /// ```
    Px,
    /// ```css
    /// {
    ///     max-height: 0.125rem; /* 2px */
    /// }
    /// ```
    _0_5,
    /// ```css
    /// {
    ///     max-height: 0.25rem; /* 4px */
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     max-height: 0.375rem; /* 6px */
    /// }
    /// ```
    _1_5,
    /// ```css
    /// {
    ///     max-height: 0.5rem; /* 8px */
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     max-height: 0.625rem; /* 10px */
    /// }
    /// ```
    _2_5,
    /// ```css
    /// {
    ///     max-height: 0.75rem; /* 12px */
    /// }
    /// ```
    _3,
    /// ```css
    /// {
    ///     max-height: 0.875rem; /* 14px */
    /// }
    /// ```
    _3_5,
    /// ```css
    /// {
    ///     max-height: 1rem; /* 16px */
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     max-height: 1.25rem; /* 20px */
    /// }
    /// ```
    _5,
    /// ```css
    /// {
    ///     max-height: 1.5rem; /* 24px */
    /// }
    /// ```
    _6,
    /// ```css
    /// {
    ///     max-height: 1.75rem; /* 28px */
    /// }
    /// ```
    _7,
    /// ```css
    /// {
    ///     max-height: 2rem; /* 32px */
    /// }
    /// ```
    _8,
    /// ```css
    /// {
    ///     max-height: 2.25rem; /* 36px */
    /// }
    /// ```
    _9,
    /// ```css
    /// {
    ///     max-height: 2.5rem; /* 40px */
    /// }
    /// ```
    _10,
    /// ```css
    /// {
    ///     max-height: 2.75rem; /* 44px */
    /// }
    /// ```
    _11,
    /// ```css
    /// {
    ///     max-height: 3rem; /* 48px */
    /// }
    /// ```
    _12,
    /// ```css
    /// {
    ///     max-height: 3.5rem; /* 56px */
    /// }
    /// ```
    _14,
    /// ```css
    /// {
    ///     max-height: 4rem; /* 64px */
    /// }
    /// ```
    _16,
    /// ```css
    /// {
    ///     max-height: 5rem; /* 80px */
    /// }
    /// ```
    _20,
    /// ```css
    /// {
    ///     max-height: 6rem; /* 96px */
    /// }
    /// ```
    _24,
    /// ```css
    /// {
    ///     max-height: 7rem; /* 112px */
    /// }
    /// ```
    _28,
    /// ```css
    /// {
    ///     max-height: 8rem; /* 128px */
    /// }
    /// ```
    _32,
    /// ```css
    /// {
    ///     max-height: 9rem; /* 144px */
    /// }
    /// ```
    _36,
    /// ```css
    /// {
    ///     max-height: 10rem; /* 160px */
    /// }
    /// ```
    _40,
    /// ```css
    /// {
    ///     max-height: 11rem; /* 176px */
    /// }
    /// ```
    _44,
    /// ```css
    /// {
    ///     max-height: 12rem; /* 192px */
    /// }
    /// ```
    _48,
    /// ```css
    /// {
    ///     max-height: 13rem; /* 208px */
    /// }
    /// ```
    _52,
    /// ```css
    /// {
    ///     max-height: 14rem; /* 224px */
    /// }
    /// ```
    _56,
    /// ```css
    /// {
    ///     max-height: 15rem; /* 240px */
    /// }
    /// ```
    _60,
    /// ```css
    /// {
    ///     max-height: 16rem; /* 256px */
    /// }
    /// ```
    _64,
    /// ```css
    /// {
    ///     max-height: 18rem; /* 288px */
    /// }
    /// ```
    _72,
    /// ```css
    /// {
    ///     max-height: 20rem; /* 320px */
    /// }
    /// ```
    _80,
    /// ```css
    /// {
    ///     max-height: 24rem; /* 384px */
    /// }
    /// ```
    _96,
    /// ```css
    /// {
    ///     max-height: none;
    /// }
    /// ```
    None,
    /// ```css
    /// {
    ///     max-height: 100%;
    /// }
    /// ```
    Full,
    /// ```css
    /// {
    ///     max-height: 100vh;
    /// }
    /// ```
    Screen,
    /// ```css
    /// {
    ///     max-height: 100svh;
    /// }
    /// ```
    Svh,
    /// ```css
    /// {
    ///     max-height: 100lvh;
    /// }
    /// ```
    Lvh,
    /// ```css
    /// {
    ///     max-height: 100dvh;
    /// }
    /// ```
    Dvh,
    /// ```css
    /// {
    ///     max-height: min-content;
    /// }
    /// ```
    Min,
    /// ```css
    /// {
    ///     max-height: max-content;
    /// }
    /// ```
    Max,
    /// ```css
    /// {
    ///     max-height: fit-content;
    /// }
    /// ```
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
    /// ```css
    /// {
    ///     width: 0px;
    ///     height: 0px;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     width: 1px;
    ///     height: 1px;
    /// }
    /// ```
    Px,
    /// ```css
    /// {
    ///     width: 0.125rem; /* 2px */
    ///     height: 0.125rem; /* 2px */
    /// }
    /// ```
    _0_5,
    /// ```css
    /// {
    ///     width: 0.25rem; /* 4px */
    ///     height: 0.25rem; /* 4px */
    /// }
    /// ```
    _1,
    /// ```css
    /// {
    ///     width: 0.375rem; /* 6px */
    ///     height: 0.375rem; /* 6px */
    /// }
    /// ```
    _1_5,
    /// ```css
    /// {
    ///     width: 0.5rem; /* 8px */
    ///     height: 0.5rem; /* 8px */
    /// }
    /// ```
    _2,
    /// ```css
    /// {
    ///     width: 0.625rem; /* 10px */
    ///     height: 0.625rem; /* 10px */
    /// }
    /// ```
    _2_5,
    /// ```css
    /// {
    ///     width: 0.75rem; /* 12px */
    ///     height: 0.75rem; /* 12px */
    /// }
    /// ```
    _3,
    /// ```css
    /// {
    ///     width: 0.875rem; /* 14px */
    ///     height: 0.875rem; /* 14px */
    /// }
    /// ```
    _3_5,
    /// ```css
    /// {
    ///     width: 1rem; /* 16px */
    ///     height: 1rem; /* 16px */
    /// }
    /// ```
    _4,
    /// ```css
    /// {
    ///     width: 1.25rem; /* 20px */
    ///     height: 1.25rem; /* 20px */
    /// }
    /// ```
    _5,
    /// ```css
    /// {
    ///     width: 1.5rem; /* 24px */
    ///     height: 1.5rem; /* 24px */
    /// }
    /// ```
    _6,
    /// ```css
    /// {
    ///     width: 1.75rem; /* 28px */
    ///     height: 1.75rem; /* 28px */
    /// }
    /// ```
    _7,
    /// ```css
    /// {
    ///     width: 2rem; /* 32px */
    ///     height: 2rem; /* 32px */
    /// }
    /// ```
    _8,
    /// ```css
    /// {
    ///     width: 2.25rem; /* 36px */
    ///     height: 2.25rem; /* 36px */
    /// }
    /// ```
    _9,
    /// ```css
    /// {
    ///     width: 2.5rem; /* 40px */
    ///     height: 2.5rem; /* 40px */
    /// }
    /// ```
    _10,
    /// ```css
    /// {
    ///     width: 2.75rem; /* 44px */
    ///     height: 2.75rem; /* 44px */
    /// }
    /// ```
    _11,
    /// ```css
    /// {
    ///     width: 3rem; /* 48px */
    ///     height: 3rem; /* 48px */
    /// }
    /// ```
    _12,
    /// ```css
    /// {
    ///     width: 3.5rem; /* 56px */
    ///     height: 3.5rem; /* 56px */
    /// }
    /// ```
    _14,
    /// ```css
    /// {
    ///     width: 4rem; /* 64px */
    ///     height: 4rem; /* 64px */
    /// }
    /// ```
    _16,
    /// ```css
    /// {
    ///     width: 5rem; /* 80px */
    ///     height: 5rem; /* 80px */
    /// }
    /// ```
    _20,
    /// ```css
    /// {
    ///     width: 6rem; /* 96px */
    ///     height: 6rem; /* 96px */
    /// }
    /// ```
    _24,
    /// ```css
    /// {
    ///     width: 7rem; /* 112px */
    ///     height: 7rem; /* 112px */
    /// }
    /// ```
    _28,
    /// ```css
    /// {
    ///     width: 8rem; /* 128px */
    ///     height: 8rem; /* 128px */
    /// }
    /// ```
    _32,
    /// ```css
    /// {
    ///     width: 9rem; /* 144px */
    ///     height: 9rem; /* 144px */
    /// }
    /// ```
    _36,
    /// ```css
    /// {
    ///     width: 10rem; /* 160px */
    ///     height: 10rem; /* 160px */
    /// }
    /// ```
    _40,
    /// ```css
    /// {
    ///     width: 11rem; /* 176px */
    ///     height: 11rem; /* 176px */
    /// }
    /// ```
    _44,
    /// ```css
    /// {
    ///     width: 12rem; /* 192px */
    ///     height: 12rem; /* 192px */
    /// }
    /// ```
    _48,
    /// ```css
    /// {
    ///     width: 13rem; /* 208px */
    ///     height: 13rem; /* 208px */
    /// }
    /// ```
    _52,
    /// ```css
    /// {
    ///     width: 14rem; /* 224px */
    ///     height: 14rem; /* 224px */
    /// }
    /// ```
    _56,
    /// ```css
    /// {
    ///     width: 15rem; /* 240px */
    ///     height: 15rem; /* 240px */
    /// }
    /// ```
    _60,
    /// ```css
    /// {
    ///     width: 16rem; /* 256px */
    ///     height: 16rem; /* 256px */
    /// }
    /// ```
    _64,
    /// ```css
    /// {
    ///     width: 18rem; /* 288px */
    ///     height: 18rem; /* 288px */
    /// }
    /// ```
    _72,
    /// ```css
    /// {
    ///     width: 20rem; /* 320px */
    ///     height: 20rem; /* 320px */
    /// }
    /// ```
    _80,
    /// ```css
    /// {
    ///     width: 24rem; /* 384px */
    ///     height: 24rem; /* 384px */
    /// }
    /// ```
    _96,
    /// ```css
    /// {
    ///     width: auto;
    ///     height: auto;
    /// }
    /// ```
    Auto,
    /// ```css
    /// {
    ///     width: 50%;
    ///     height: 50%;
    /// }
    /// ```
    _1div2,
    /// ```css
    /// {
    ///     width: 33.333333%;
    ///     height: 33.333333%;
    /// }
    /// ```
    _1div3,
    /// ```css
    /// {
    ///     width: 66.666667%;
    ///     height: 66.666667%;
    /// }
    /// ```
    _2div3,
    /// ```css
    /// {
    ///     width: 25%;
    ///     height: 25%;
    /// }
    /// ```
    _1div4,
    /// ```css
    /// {
    ///     width: 50%;
    ///     height: 50%;
    /// }
    /// ```
    _2div4,
    /// ```css
    /// {
    ///     width: 75%;
    ///     height: 75%;
    /// }
    /// ```
    _3div4,
    /// ```css
    /// {
    ///     width: 20%;
    ///     height: 20%;
    /// }
    /// ```
    _1div5,
    /// ```css
    /// {
    ///     width: 40%;
    ///     height: 40%;
    /// }
    /// ```
    _2div5,
    /// ```css
    /// {
    ///     width: 60%;
    ///     height: 60%;
    /// }
    /// ```
    _3div5,
    /// ```css
    /// {
    ///     width: 80%;
    ///     height: 80%;
    /// }
    /// ```
    _4div5,
    /// ```css
    /// {
    ///     width: 16.666667%;
    ///     height: 16.666667%;
    /// }
    /// ```
    _1div6,
    /// ```css
    /// {
    ///     width: 33.333333%;
    ///     height: 33.333333%;
    /// }
    /// ```
    _2div6,
    /// ```css
    /// {
    ///     width: 50%;
    ///     height: 50%;
    /// }
    /// ```
    _3div6,
    /// ```css
    /// {
    ///     width: 66.666667%;
    ///     height: 66.666667%;
    /// }
    /// ```
    _4div6,
    /// ```css
    /// {
    ///     width: 83.333333%;
    ///     height: 83.333333%;
    /// }
    /// ```
    _5div6,
    /// ```css
    /// {
    ///     width: 8.333333%;
    ///     height: 8.333333%;
    /// }
    /// ```
    _1div12,
    /// ```css
    /// {
    ///     width: 16.666667%;
    ///     height: 16.666667%;
    /// }
    /// ```
    _2div12,
    /// ```css
    /// {
    ///     width: 25%;
    ///     height: 25%;
    /// }
    /// ```
    _3div12,
    /// ```css
    /// {
    ///     width: 33.333333%;
    ///     height: 33.333333%;
    /// }
    /// ```
    _4div12,
    /// ```css
    /// {
    ///     width: 41.666667%;
    ///     height: 41.666667%;
    /// }
    /// ```
    _5div12,
    /// ```css
    /// {
    ///     width: 50%;
    ///     height: 50%;
    /// }
    /// ```
    _6div12,
    /// ```css
    /// {
    ///     width: 58.333333%;
    ///     height: 58.333333%;
    /// }
    /// ```
    _7div12,
    /// ```css
    /// {
    ///     width: 66.666667%;
    ///     height: 66.666667%;
    /// }
    /// ```
    _8div12,
    /// ```css
    /// {
    ///     width: 75%;
    ///     height: 75%;
    /// }
    /// ```
    _9div12,
    /// ```css
    /// {
    ///     width: 83.333333%;
    ///     height: 83.333333%;
    /// }
    /// ```
    _10div12,
    /// ```css
    /// {
    ///     width: 91.666667%;
    ///     height: 91.666667%;
    /// }
    /// ```
    _11div12,
    /// ```css
    /// {
    ///     width: 100%;
    ///     height: 100%;
    /// }
    /// ```
    Full,
    /// ```css
    /// {
    ///     width: min-content;
    ///     height: min-content;
    /// }
    /// ```
    Min,
    /// ```css
    /// {
    ///     width: max-content;
    ///     height: max-content;
    /// }
    /// ```
    Max,
    /// ```css
    /// {
    ///     width: fit-content;
    ///     height: fit-content;
    /// }
    /// ```
    Fit,
}
