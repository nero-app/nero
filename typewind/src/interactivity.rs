use typewind_macros::{Display, Parse};
use crate::{customization::Color, spacing::Padding};

tailwind_types!(
    AccentColor, Appearance, Cursor, CaretColor, PointerEvents, Resize, ScrollBehavior, 
    ScrollMargin, ScrollPadding, ScrollSnapAlign, ScrollSnapStop, ScrollSnapType, TouchAction, 
    UserSelect, WillChange
);

/// Utilities for controlling the accented color of a form control.
/// 
/// <https://tailwindcss.com/docs/accent-color>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "accent")]
pub struct AccentColor(pub Color);

/// Utilities for suppressing native form control styling.
/// 
/// <https://tailwindcss.com/docs/appearance>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "appearance")]
pub enum Appearance {
    /// ```css
    /// appearance: none;
    /// ```
    None,
    /// ```css
    /// appearance: auto;
    /// ```
    Auto,
}

/// Utilities for controlling the cursor style when hovering over an element.
/// 
/// <https://tailwindcss.com/docs/cursor>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "cursor")]
pub enum Cursor {
    /// ```css
    /// cursor: auto;
    /// ```
    Auto,
    /// ```css
    /// cursor: default;
    /// ```
    Default,
    /// ```css
    /// cursor: pointer;
    /// ```
    Pointer,
    /// ```css
    /// cursor: wait;
    /// ```
    Wait,
    /// ```css
    /// cursor: text;
    /// ```
    Text,
    /// ```css
    /// cursor: move;
    /// ```
    Move,
    /// ```css
    /// cursor: help;
    /// ```
    Help,
    /// ```css
    /// cursor: not-allowed;
    /// ```
    NotAllowed,
    /// ```css
    /// cursor: none;
    /// ```
    None,
    /// ```css
    /// cursor: context-menu;
    /// ```
    ContextMenu,
    /// ```css
    /// cursor: progress;
    /// ```
    Progress,
    /// ```css
    /// cursor: cell;
    /// ```
    Cell,
    /// ```css
    /// cursor: crosshair;
    /// ```
    Crosshair,
    /// ```css
    /// cursor: vertical-text;
    /// ```
    VerticalText,
    /// ```css
    /// cursor: alias;
    /// ```
    Alias,
    /// ```css
    /// cursor: copy;
    /// ```
    Copy,
    /// ```css
    /// cursor: no-drop;
    /// ```
    NoDrop,
    /// ```css
    /// cursor: grab;
    /// ```
    Grab,
    /// ```css
    /// cursor: grabbing;
    /// ```
    Grabbing,
    /// ```css
    /// cursor: all-scroll;
    /// ```
    AllScroll,
    /// ```css
    /// cursor: col-resize;
    /// ```
    ColResize,
    /// ```css
    /// cursor: row-resize;
    /// ```
    RowResize,
    /// ```css
    /// cursor: n-resize;
    /// ```
    NResize,
    /// ```css
    /// cursor: e-resize;
    /// ```
    EResize,
    /// ```css
    /// cursor: s-resize;
    /// ```
    SResize,
    /// ```css
    /// cursor: w-resize;
    /// ```
    WResize,
    /// ```css
    /// cursor: ne-resize;
    /// ```
    NeResize,
    /// ```css
    /// cursor: nw-resize;
    /// ```
    NwResize,
    /// ```css
    /// cursor: se-resize;
    /// ```
    SeResize,
    /// ```css
    /// cursor: sw-resize;
    /// ```
    SwResize,
    /// ```css
    /// cursor: ew-resize;
    /// ```
    EwResize,
    /// ```css
    /// cursor: ns-resize;
    /// ```
    NsResize,
    /// ```css
    /// cursor: nesw-resize;
    /// ```
    NeswResize,
    /// ```css
    /// cursor: nwse-resize;
    /// ```
    NwseResize,
    /// ```css
    /// cursor: zoom-in;
    /// ```
    ZoomIn,
    /// ```css
    /// cursor: zoom-out;
    /// ```
    ZoomOut,
}

/// Utilities for controlling the color of the text input cursor.
/// 
/// <https://tailwindcss.com/docs/caret-color>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "caret")]
pub struct CaretColor(pub Color);

/// Utilities for controlling whether an element responds to pointer events.
/// 
/// <https://tailwindcss.com/docs/pointer-events>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "pointer-events")]
pub enum PointerEvents {
    /// ```css
    /// pointer-events: none;
    /// ```
    None,
    /// ```css
    /// pointer-events: auto;
    /// ```
    Auto,
}

/// Utilities for controlling how an element can be resized.
/// 
/// <https://tailwindcss.com/docs/resize>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "resize")]
pub enum Resize {
    /// ```css
    /// resize: none;
    /// ```
    None,
    /// ```css
    /// resize: vertical;
    /// ```
    Y,
    /// ```css
    /// resize: horizontal;
    /// ```
    X,
    /// ```css
    /// resize: both;
    /// ```
    #[display(no_prefix)]
    Resize,
}

/// Utilities for controlling the scroll behavior of an element.
/// 
/// <https://tailwindcss.com/docs/scroll-behavior>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "scroll")]
pub enum ScrollBehavior {
    /// ```css
    /// scroll-behavior: auto;
    /// ```
    Auto,
    /// ```css
    /// scroll-behavior: smooth;
    /// ```
    Smooth,
}

/// Utilities for controlling the scroll offset around items in a snap container.
/// 
/// <https://tailwindcss.com/docs/scroll-margin>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "scroll", replace(from = "_", to = "."))]
pub enum ScrollMargin {
    /// ```css
    /// scroll-margin: 0px;
    /// ```
    M0,
    /// ```css
    /// scroll-margin-left: 0px;
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 0px;
    /// ```
    Mx0,
    /// ```css
    /// scroll-margin-top: 0px;
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 0px;
    /// ```
    My0,
    /// ```css
    /// scroll-margin-inline-start: 0px;
    /// ```
    Ms0,
    /// ```css
    /// scroll-margin-inline-end: 0px;
    /// ```
    Me0,
    /// ```css
    /// scroll-margin-top: 0px;
    /// ```
    Mt0,
    /// ```css
    /// scroll-margin-right: 0px;
    /// ```
    Mr0,
    /// ```css
    /// scroll-margin-bottom: 0px;
    /// ```
    Mb0,
    /// ```css
    /// scroll-margin-left: 0px;
    /// ```
    Ml0,
    /// ```css
    /// scroll-margin: 1px;
    /// ```
    MPx,
    /// ```css
    /// scroll-margin-left: 1px;
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 1px;
    /// ```
    MxPx,
    /// ```css
    /// scroll-margin-top: 1px;
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 1px;
    /// ```
    MyPx,
    /// ```css
    /// scroll-margin-inline-start: 1px;
    /// ```
    MsPx,
    /// ```css
    /// scroll-margin-inline-end: 1px;
    /// ```
    MePx,
    /// ```css
    /// scroll-margin-top: 1px;
    /// ```
    MtPx,
    /// ```css
    /// scroll-margin-right: 1px;
    /// ```
    MrPx,
    /// ```css
    /// scroll-margin-bottom: 1px;
    /// ```
    MbPx,
    /// ```css
    /// scroll-margin-left: 1px;
    /// ```
    MlPx,
    /// ```css
    /// scroll-margin: 0.125rem; /* 2px */
    /// ```
    M0_5,
    /// ```css
    /// scroll-margin-left: 0.125rem; /* 2px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 0.125rem; /* 2px */
    /// ```
    Mx0_5,
    /// ```css
    /// scroll-margin-top: 0.125rem; /* 2px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 0.125rem; /* 2px */
    /// ```
    My0_5,
    /// ```css
    /// scroll-margin-inline-start: 0.125rem; /* 2px */
    /// ```
    Ms0_5,
    /// ```css
    /// scroll-margin-inline-end: 0.125rem; /* 2px */
    /// ```
    Me0_5,
    /// ```css
    /// scroll-margin-top: 0.125rem; /* 2px */
    /// ```
    Mt0_5,
    /// ```css
    /// scroll-margin-right: 0.125rem; /* 2px */
    /// ```
    Mr0_5,
    /// ```css
    /// scroll-margin-bottom: 0.125rem; /* 2px */
    /// ```
    Mb0_5,
    /// ```css
    /// scroll-margin-left: 0.125rem; /* 2px */
    /// ```
    Ml0_5,
    /// ```css
    /// scroll-margin: 0.25rem; /* 4px */
    /// ```
    M1,
    /// ```css
    /// scroll-margin-left: 0.25rem; /* 4px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 0.25rem; /* 4px */
    /// ```
    Mx1,
    /// ```css
    /// scroll-margin-top: 0.25rem; /* 4px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 0.25rem; /* 4px */
    /// ```
    My1,
    /// ```css
    /// scroll-margin-inline-start: 0.25rem; /* 4px */
    /// ```
    Ms1,
    /// ```css
    /// scroll-margin-inline-end: 0.25rem; /* 4px */
    /// ```
    Me1,
    /// ```css
    /// scroll-margin-top: 0.25rem; /* 4px */
    /// ```
    Mt1,
    /// ```css
    /// scroll-margin-right: 0.25rem; /* 4px */
    /// ```
    Mr1,
    /// ```css
    /// scroll-margin-bottom: 0.25rem; /* 4px */
    /// ```
    Mb1,
    /// ```css
    /// scroll-margin-left: 0.25rem; /* 4px */
    /// ```
    Ml1,
    /// ```css
    /// scroll-margin: 0.375rem; /* 6px */
    /// ```
    M1_5,
    /// ```css
    /// scroll-margin-left: 0.375rem; /* 6px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 0.375rem; /* 6px */
    /// ```
    Mx1_5,
    /// ```css
    /// scroll-margin-top: 0.375rem; /* 6px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 0.375rem; /* 6px */
    /// ```
    My1_5,
    /// ```css
    /// scroll-margin-inline-start: 0.375rem; /* 6px */
    /// ```
    Ms1_5,
    /// ```css
    /// scroll-margin-inline-end: 0.375rem; /* 6px */
    /// ```
    Me1_5,
    /// ```css
    /// scroll-margin-top: 0.375rem; /* 6px */
    /// ```
    Mt1_5,
    /// ```css
    /// scroll-margin-right: 0.375rem; /* 6px */
    /// ```
    Mr1_5,
    /// ```css
    /// scroll-margin-bottom: 0.375rem; /* 6px */
    /// ```
    Mb1_5,
    /// ```css
    /// scroll-margin-left: 0.375rem; /* 6px */
    /// ```
    Ml1_5,
    /// ```css
    /// scroll-margin: 0.5rem; /* 8px */
    /// ```
    M2,
    /// ```css
    /// scroll-margin-left: 0.5rem; /* 8px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 0.5rem; /* 8px */
    /// ```
    Mx2,
    /// ```css
    /// scroll-margin-top: 0.5rem; /* 8px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 0.5rem; /* 8px */
    /// ```
    My2,
    /// ```css
    /// scroll-margin-inline-start: 0.5rem; /* 8px */
    /// ```
    Ms2,
    /// ```css
    /// scroll-margin-inline-end: 0.5rem; /* 8px */
    /// ```
    Me2,
    /// ```css
    /// scroll-margin-top: 0.5rem; /* 8px */
    /// ```
    Mt2,
    /// ```css
    /// scroll-margin-right: 0.5rem; /* 8px */
    /// ```
    Mr2,
    /// ```css
    /// scroll-margin-bottom: 0.5rem; /* 8px */
    /// ```
    Mb2,
    /// ```css
    /// scroll-margin-left: 0.5rem; /* 8px */
    /// ```
    Ml2,
    /// ```css
    /// scroll-margin: 0.625rem; /* 10px */
    /// ```
    M2_5,
    /// ```css
    /// scroll-margin-left: 0.625rem; /* 10px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 0.625rem; /* 10px */
    /// ```
    Mx2_5,
    /// ```css
    /// scroll-margin-top: 0.625rem; /* 10px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 0.625rem; /* 10px */
    /// ```
    My2_5,
    /// ```css
    /// scroll-margin-inline-start: 0.625rem; /* 10px */
    /// ```
    Ms2_5,
    /// ```css
    /// scroll-margin-inline-end: 0.625rem; /* 10px */
    /// ```
    Me2_5,
    /// ```css
    /// scroll-margin-top: 0.625rem; /* 10px */
    /// ```
    Mt2_5,
    /// ```css
    /// scroll-margin-right: 0.625rem; /* 10px */
    /// ```
    Mr2_5,
    /// ```css
    /// scroll-margin-bottom: 0.625rem; /* 10px */
    /// ```
    Mb2_5,
    /// ```css
    /// scroll-margin-left: 0.625rem; /* 10px */
    /// ```
    Ml2_5,
    /// ```css
    /// scroll-margin: 0.75rem; /* 12px */
    /// ```
    M3,
    /// ```css
    /// scroll-margin-left: 0.75rem; /* 12px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 0.75rem; /* 12px */
    /// ```
    Mx3,
    /// ```css
    /// scroll-margin-top: 0.75rem; /* 12px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 0.75rem; /* 12px */
    /// ```
    My3,
    /// ```css
    /// scroll-margin-inline-start: 0.75rem; /* 12px */
    /// ```
    Ms3,
    /// ```css
    /// scroll-margin-inline-end: 0.75rem; /* 12px */
    /// ```
    Me3,
    /// ```css
    /// scroll-margin-top: 0.75rem; /* 12px */
    /// ```
    Mt3,
    /// ```css
    /// scroll-margin-right: 0.75rem; /* 12px */
    /// ```
    Mr3,
    /// ```css
    /// scroll-margin-bottom: 0.75rem; /* 12px */
    /// ```
    Mb3,
    /// ```css
    /// scroll-margin-left: 0.75rem; /* 12px */
    /// ```
    Ml3,
    /// ```css
    /// scroll-margin: 0.875rem; /* 14px */
    /// ```
    M3_5,
    /// ```css
    /// scroll-margin-left: 0.875rem; /* 14px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 0.875rem; /* 14px */
    /// ```
    Mx3_5,
    /// ```css
    /// scroll-margin-top: 0.875rem; /* 14px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 0.875rem; /* 14px */
    /// ```
    My3_5,
    /// ```css
    /// scroll-margin-inline-start: 0.875rem; /* 14px */
    /// ```
    Ms3_5,
    /// ```css
    /// scroll-margin-inline-end: 0.875rem; /* 14px */
    /// ```
    Me3_5,
    /// ```css
    /// scroll-margin-top: 0.875rem; /* 14px */
    /// ```
    Mt3_5,
    /// ```css
    /// scroll-margin-right: 0.875rem; /* 14px */
    /// ```
    Mr3_5,
    /// ```css
    /// scroll-margin-bottom: 0.875rem; /* 14px */
    /// ```
    Mb3_5,
    /// ```css
    /// scroll-margin-left: 0.875rem; /* 14px */
    /// ```
    Ml3_5,
    /// ```css
    /// scroll-margin: 1rem; /* 16px */
    /// ```
    M4,
    /// ```css
    /// scroll-margin-left: 1rem; /* 16px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 1rem; /* 16px */
    /// ```
    Mx4,
    /// ```css
    /// scroll-margin-top: 1rem; /* 16px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 1rem; /* 16px */
    /// ```
    My4,
    /// ```css
    /// scroll-margin-inline-start: 1rem; /* 16px */
    /// ```
    Ms4,
    /// ```css
    /// scroll-margin-inline-end: 1rem; /* 16px */
    /// ```
    Me4,
    /// ```css
    /// scroll-margin-top: 1rem; /* 16px */
    /// ```
    Mt4,
    /// ```css
    /// scroll-margin-right: 1rem; /* 16px */
    /// ```
    Mr4,
    /// ```css
    /// scroll-margin-bottom: 1rem; /* 16px */
    /// ```
    Mb4,
    /// ```css
    /// scroll-margin-left: 1rem; /* 16px */
    /// ```
    Ml4,
    /// ```css
    /// scroll-margin: 1.25rem; /* 20px */
    /// ```
    M5,
    /// ```css
    /// scroll-margin-left: 1.25rem; /* 20px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 1.25rem; /* 20px */
    /// ```
    Mx5,
    /// ```css
    /// scroll-margin-top: 1.25rem; /* 20px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 1.25rem; /* 20px */
    /// ```
    My5,
    /// ```css
    /// scroll-margin-inline-start: 1.25rem; /* 20px */
    /// ```
    Ms5,
    /// ```css
    /// scroll-margin-inline-end: 1.25rem; /* 20px */
    /// ```
    Me5,
    /// ```css
    /// scroll-margin-top: 1.25rem; /* 20px */
    /// ```
    Mt5,
    /// ```css
    /// scroll-margin-right: 1.25rem; /* 20px */
    /// ```
    Mr5,
    /// ```css
    /// scroll-margin-bottom: 1.25rem; /* 20px */
    /// ```
    Mb5,
    /// ```css
    /// scroll-margin-left: 1.25rem; /* 20px */
    /// ```
    Ml5,
    /// ```css
    /// scroll-margin: 1.5rem; /* 24px */
    /// ```
    M6,
    /// ```css
    /// scroll-margin-left: 1.5rem; /* 24px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 1.5rem; /* 24px */
    /// ```
    Mx6,
    /// ```css
    /// scroll-margin-top: 1.5rem; /* 24px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 1.5rem; /* 24px */
    /// ```
    My6,
    /// ```css
    /// scroll-margin-inline-start: 1.5rem; /* 24px */
    /// ```
    Ms6,
    /// ```css
    /// scroll-margin-inline-end: 1.5rem; /* 24px */
    /// ```
    Me6,
    /// ```css
    /// scroll-margin-top: 1.5rem; /* 24px */
    /// ```
    Mt6,
    /// ```css
    /// scroll-margin-right: 1.5rem; /* 24px */
    /// ```
    Mr6,
    /// ```css
    /// scroll-margin-bottom: 1.5rem; /* 24px */
    /// ```
    Mb6,
    /// ```css
    /// scroll-margin-left: 1.5rem; /* 24px */
    /// ```
    Ml6,
    /// ```css
    /// scroll-margin: 1.75rem; /* 28px */
    /// ```
    M7,
    /// ```css
    /// scroll-margin-left: 1.75rem; /* 28px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 1.75rem; /* 28px */
    /// ```
    Mx7,
    /// ```css
    /// scroll-margin-top: 1.75rem; /* 28px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 1.75rem; /* 28px */
    /// ```
    My7,
    /// ```css
    /// scroll-margin-inline-start: 1.75rem; /* 28px */
    /// ```
    Ms7,
    /// ```css
    /// scroll-margin-inline-end: 1.75rem; /* 28px */
    /// ```
    Me7,
    /// ```css
    /// scroll-margin-top: 1.75rem; /* 28px */
    /// ```
    Mt7,
    /// ```css
    /// scroll-margin-right: 1.75rem; /* 28px */
    /// ```
    Mr7,
    /// ```css
    /// scroll-margin-bottom: 1.75rem; /* 28px */
    /// ```
    Mb7,
    /// ```css
    /// scroll-margin-left: 1.75rem; /* 28px */
    /// ```
    Ml7,
    /// ```css
    /// scroll-margin: 2rem; /* 32px */
    /// ```
    M8,
    /// ```css
    /// scroll-margin-left: 2rem; /* 32px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 2rem; /* 32px */
    /// ```
    Mx8,
    /// ```css
    /// scroll-margin-top: 2rem; /* 32px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 2rem; /* 32px */
    /// ```
    My8,
    /// ```css
    /// scroll-margin-inline-start: 2rem; /* 32px */
    /// ```
    Ms8,
    /// ```css
    /// scroll-margin-inline-end: 2rem; /* 32px */
    /// ```
    Me8,
    /// ```css
    /// scroll-margin-top: 2rem; /* 32px */
    /// ```
    Mt8,
    /// ```css
    /// scroll-margin-right: 2rem; /* 32px */
    /// ```
    Mr8,
    /// ```css
    /// scroll-margin-bottom: 2rem; /* 32px */
    /// ```
    Mb8,
    /// ```css
    /// scroll-margin-left: 2rem; /* 32px */
    /// ```
    Ml8,
    /// ```css
    /// scroll-margin: 2.25rem; /* 36px */
    /// ```
    M9,
    /// ```css
    /// scroll-margin-left: 2.25rem; /* 36px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 2.25rem; /* 36px */
    /// ```
    Mx9,
    /// ```css
    /// scroll-margin-top: 2.25rem; /* 36px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 2.25rem; /* 36px */
    /// ```
    My9,
    /// ```css
    /// scroll-margin-inline-start: 2.25rem; /* 36px */
    /// ```
    Ms9,
    /// ```css
    /// scroll-margin-inline-end: 2.25rem; /* 36px */
    /// ```
    Me9,
    /// ```css
    /// scroll-margin-top: 2.25rem; /* 36px */
    /// ```
    Mt9,
    /// ```css
    /// scroll-margin-right: 2.25rem; /* 36px */
    /// ```
    Mr9,
    /// ```css
    /// scroll-margin-bottom: 2.25rem; /* 36px */
    /// ```
    Mb9,
    /// ```css
    /// scroll-margin-left: 2.25rem; /* 36px */
    /// ```
    Ml9,
    /// ```css
    /// scroll-margin: 2.5rem; /* 40px */
    /// ```
    M10,
    /// ```css
    /// scroll-margin-left: 2.5rem; /* 40px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 2.5rem; /* 40px */
    /// ```
    Mx10,
    /// ```css
    /// scroll-margin-top: 2.5rem; /* 40px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 2.5rem; /* 40px */
    /// ```
    My10,
    /// ```css
    /// scroll-margin-inline-start: 2.5rem; /* 40px */
    /// ```
    Ms10,
    /// ```css
    /// scroll-margin-inline-end: 2.5rem; /* 40px */
    /// ```
    Me10,
    /// ```css
    /// scroll-margin-top: 2.5rem; /* 40px */
    /// ```
    Mt10,
    /// ```css
    /// scroll-margin-right: 2.5rem; /* 40px */
    /// ```
    Mr10,
    /// ```css
    /// scroll-margin-bottom: 2.5rem; /* 40px */
    /// ```
    Mb10,
    /// ```css
    /// scroll-margin-left: 2.5rem; /* 40px */
    /// ```
    Ml10,
    /// ```css
    /// scroll-margin: 2.75rem; /* 44px */
    /// ```
    M11,
    /// ```css
    /// scroll-margin-left: 2.75rem; /* 44px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 2.75rem; /* 44px */
    /// ```
    Mx11,
    /// ```css
    /// scroll-margin-top: 2.75rem; /* 44px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 2.75rem; /* 44px */
    /// ```
    My11,
    /// ```css
    /// scroll-margin-inline-start: 2.75rem; /* 44px */
    /// ```
    Ms11,
    /// ```css
    /// scroll-margin-inline-end: 2.75rem; /* 44px */
    /// ```
    Me11,
    /// ```css
    /// scroll-margin-top: 2.75rem; /* 44px */
    /// ```
    Mt11,
    /// ```css
    /// scroll-margin-right: 2.75rem; /* 44px */
    /// ```
    Mr11,
    /// ```css
    /// scroll-margin-bottom: 2.75rem; /* 44px */
    /// ```
    Mb11,
    /// ```css
    /// scroll-margin-left: 2.75rem; /* 44px */
    /// ```
    Ml11,
    /// ```css
    /// scroll-margin: 3rem; /* 48px */
    /// ```
    M12,
    /// ```css
    /// scroll-margin-left: 3rem; /* 48px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 3rem; /* 48px */
    /// ```
    Mx12,
    /// ```css
    /// scroll-margin-top: 3rem; /* 48px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 3rem; /* 48px */
    /// ```
    My12,
    /// ```css
    /// scroll-margin-inline-start: 3rem; /* 48px */
    /// ```
    Ms12,
    /// ```css
    /// scroll-margin-inline-end: 3rem; /* 48px */
    /// ```
    Me12,
    /// ```css
    /// scroll-margin-top: 3rem; /* 48px */
    /// ```
    Mt12,
    /// ```css
    /// scroll-margin-right: 3rem; /* 48px */
    /// ```
    Mr12,
    /// ```css
    /// scroll-margin-bottom: 3rem; /* 48px */
    /// ```
    Mb12,
    /// ```css
    /// scroll-margin-left: 3rem; /* 48px */
    /// ```
    Ml12,
    /// ```css
    /// scroll-margin: 3.5rem; /* 56px */
    /// ```
    M14,
    /// ```css
    /// scroll-margin-left: 3.5rem; /* 56px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 3.5rem; /* 56px */
    /// ```
    Mx14,
    /// ```css
    /// scroll-margin-top: 3.5rem; /* 56px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 3.5rem; /* 56px */
    /// ```
    My14,
    /// ```css
    /// scroll-margin-inline-start: 3.5rem; /* 56px */
    /// ```
    Ms14,
    /// ```css
    /// scroll-margin-inline-end: 3.5rem; /* 56px */
    /// ```
    Me14,
    /// ```css
    /// scroll-margin-top: 3.5rem; /* 56px */
    /// ```
    Mt14,
    /// ```css
    /// scroll-margin-right: 3.5rem; /* 56px */
    /// ```
    Mr14,
    /// ```css
    /// scroll-margin-bottom: 3.5rem; /* 56px */
    /// ```
    Mb14,
    /// ```css
    /// scroll-margin-left: 3.5rem; /* 56px */
    /// ```
    Ml14,
    /// ```css
    /// scroll-margin: 4rem; /* 64px */
    /// ```
    M16,
    /// ```css
    /// scroll-margin-left: 4rem; /* 64px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 4rem; /* 64px */
    /// ```
    Mx16,
    /// ```css
    /// scroll-margin-top: 4rem; /* 64px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 4rem; /* 64px */
    /// ```
    My16,
    /// ```css
    /// scroll-margin-inline-start: 4rem; /* 64px */
    /// ```
    Ms16,
    /// ```css
    /// scroll-margin-inline-end: 4rem; /* 64px */
    /// ```
    Me16,
    /// ```css
    /// scroll-margin-top: 4rem; /* 64px */
    /// ```
    Mt16,
    /// ```css
    /// scroll-margin-right: 4rem; /* 64px */
    /// ```
    Mr16,
    /// ```css
    /// scroll-margin-bottom: 4rem; /* 64px */
    /// ```
    Mb16,
    /// ```css
    /// scroll-margin-left: 4rem; /* 64px */
    /// ```
    Ml16,
    /// ```css
    /// scroll-margin: 5rem; /* 80px */
    /// ```
    M20,
    /// ```css
    /// scroll-margin-left: 5rem; /* 80px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 5rem; /* 80px */
    /// ```
    Mx20,
    /// ```css
    /// scroll-margin-top: 5rem; /* 80px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 5rem; /* 80px */
    /// ```
    My20,
    /// ```css
    /// scroll-margin-inline-start: 5rem; /* 80px */
    /// ```
    Ms20,
    /// ```css
    /// scroll-margin-inline-end: 5rem; /* 80px */
    /// ```
    Me20,
    /// ```css
    /// scroll-margin-top: 5rem; /* 80px */
    /// ```
    Mt20,
    /// ```css
    /// scroll-margin-right: 5rem; /* 80px */
    /// ```
    Mr20,
    /// ```css
    /// scroll-margin-bottom: 5rem; /* 80px */
    /// ```
    Mb20,
    /// ```css
    /// scroll-margin-left: 5rem; /* 80px */
    /// ```
    Ml20,
    /// ```css
    /// scroll-margin: 6rem; /* 96px */
    /// ```
    M24,
    /// ```css
    /// scroll-margin-left: 6rem; /* 96px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 6rem; /* 96px */
    /// ```
    Mx24,
    /// ```css
    /// scroll-margin-top: 6rem; /* 96px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 6rem; /* 96px */
    /// ```
    My24,
    /// ```css
    /// scroll-margin-inline-start: 6rem; /* 96px */
    /// ```
    Ms24,
    /// ```css
    /// scroll-margin-inline-end: 6rem; /* 96px */
    /// ```
    Me24,
    /// ```css
    /// scroll-margin-top: 6rem; /* 96px */
    /// ```
    Mt24,
    /// ```css
    /// scroll-margin-right: 6rem; /* 96px */
    /// ```
    Mr24,
    /// ```css
    /// scroll-margin-bottom: 6rem; /* 96px */
    /// ```
    Mb24,
    /// ```css
    /// scroll-margin-left: 6rem; /* 96px */
    /// ```
    Ml24,
    /// ```css
    /// scroll-margin: 7rem; /* 112px */
    /// ```
    M28,
    /// ```css
    /// scroll-margin-left: 7rem; /* 112px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 7rem; /* 112px */
    /// ```
    Mx28,
    /// ```css
    /// scroll-margin-top: 7rem; /* 112px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 7rem; /* 112px */
    /// ```
    My28,
    /// ```css
    /// scroll-margin-inline-start: 7rem; /* 112px */
    /// ```
    Ms28,
    /// ```css
    /// scroll-margin-inline-end: 7rem; /* 112px */
    /// ```
    Me28,
    /// ```css
    /// scroll-margin-top: 7rem; /* 112px */
    /// ```
    Mt28,
    /// ```css
    /// scroll-margin-right: 7rem; /* 112px */
    /// ```
    Mr28,
    /// ```css
    /// scroll-margin-bottom: 7rem; /* 112px */
    /// ```
    Mb28,
    /// ```css
    /// scroll-margin-left: 7rem; /* 112px */
    /// ```
    Ml28,
    /// ```css
    /// scroll-margin: 8rem; /* 128px */
    /// ```
    M32,
    /// ```css
    /// scroll-margin-left: 8rem; /* 128px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 8rem; /* 128px */
    /// ```
    Mx32,
    /// ```css
    /// scroll-margin-top: 8rem; /* 128px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 8rem; /* 128px */
    /// ```
    My32,
    /// ```css
    /// scroll-margin-inline-start: 8rem; /* 128px */
    /// ```
    Ms32,
    /// ```css
    /// scroll-margin-inline-end: 8rem; /* 128px */
    /// ```
    Me32,
    /// ```css
    /// scroll-margin-top: 8rem; /* 128px */
    /// ```
    Mt32,
    /// ```css
    /// scroll-margin-right: 8rem; /* 128px */
    /// ```
    Mr32,
    /// ```css
    /// scroll-margin-bottom: 8rem; /* 128px */
    /// ```
    Mb32,
    /// ```css
    /// scroll-margin-left: 8rem; /* 128px */
    /// ```
    Ml32,
    /// ```css
    /// scroll-margin: 9rem; /* 144px */
    /// ```
    M36,
    /// ```css
    /// scroll-margin-left: 9rem; /* 144px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 9rem; /* 144px */
    /// ```
    Mx36,
    /// ```css
    /// scroll-margin-top: 9rem; /* 144px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 9rem; /* 144px */
    /// ```
    My36,
    /// ```css
    /// scroll-margin-inline-start: 9rem; /* 144px */
    /// ```
    Ms36,
    /// ```css
    /// scroll-margin-inline-end: 9rem; /* 144px */
    /// ```
    Me36,
    /// ```css
    /// scroll-margin-top: 9rem; /* 144px */
    /// ```
    Mt36,
    /// ```css
    /// scroll-margin-right: 9rem; /* 144px */
    /// ```
    Mr36,
    /// ```css
    /// scroll-margin-bottom: 9rem; /* 144px */
    /// ```
    Mb36,
    /// ```css
    /// scroll-margin-left: 9rem; /* 144px */
    /// ```
    Ml36,
    /// ```css
    /// scroll-margin: 10rem; /* 160px */
    /// ```
    M40,
    /// ```css
    /// scroll-margin-left: 10rem; /* 160px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 10rem; /* 160px */
    /// ```
    Mx40,
    /// ```css
    /// scroll-margin-top: 10rem; /* 160px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 10rem; /* 160px */
    /// ```
    My40,
    /// ```css
    /// scroll-margin-inline-start: 10rem; /* 160px */
    /// ```
    Ms40,
    /// ```css
    /// scroll-margin-inline-end: 10rem; /* 160px */
    /// ```
    Me40,
    /// ```css
    /// scroll-margin-top: 10rem; /* 160px */
    /// ```
    Mt40,
    /// ```css
    /// scroll-margin-right: 10rem; /* 160px */
    /// ```
    Mr40,
    /// ```css
    /// scroll-margin-bottom: 10rem; /* 160px */
    /// ```
    Mb40,
    /// ```css
    /// scroll-margin-left: 10rem; /* 160px */
    /// ```
    Ml40,
    /// ```css
    /// scroll-margin: 11rem; /* 176px */
    /// ```
    M44,
    /// ```css
    /// scroll-margin-left: 11rem; /* 176px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 11rem; /* 176px */
    /// ```
    Mx44,
    /// ```css
    /// scroll-margin-top: 11rem; /* 176px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 11rem; /* 176px */
    /// ```
    My44,
    /// ```css
    /// scroll-margin-inline-start: 11rem; /* 176px */
    /// ```
    Ms44,
    /// ```css
    /// scroll-margin-inline-end: 11rem; /* 176px */
    /// ```
    Me44,
    /// ```css
    /// scroll-margin-top: 11rem; /* 176px */
    /// ```
    Mt44,
    /// ```css
    /// scroll-margin-right: 11rem; /* 176px */
    /// ```
    Mr44,
    /// ```css
    /// scroll-margin-bottom: 11rem; /* 176px */
    /// ```
    Mb44,
    /// ```css
    /// scroll-margin-left: 11rem; /* 176px */
    /// ```
    Ml44,
    /// ```css
    /// scroll-margin: 12rem; /* 192px */
    /// ```
    M48,
    /// ```css
    /// scroll-margin-left: 12rem; /* 192px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 12rem; /* 192px */
    /// ```
    Mx48,
    /// ```css
    /// scroll-margin-top: 12rem; /* 192px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 12rem; /* 192px */
    /// ```
    My48,
    /// ```css
    /// scroll-margin-inline-start: 12rem; /* 192px */
    /// ```
    Ms48,
    /// ```css
    /// scroll-margin-inline-end: 12rem; /* 192px */
    /// ```
    Me48,
    /// ```css
    /// scroll-margin-top: 12rem; /* 192px */
    /// ```
    Mt48,
    /// ```css
    /// scroll-margin-right: 12rem; /* 192px */
    /// ```
    Mr48,
    /// ```css
    /// scroll-margin-bottom: 12rem; /* 192px */
    /// ```
    Mb48,
    /// ```css
    /// scroll-margin-left: 12rem; /* 192px */
    /// ```
    Ml48,
    /// ```css
    /// scroll-margin: 13rem; /* 208px */
    /// ```
    M52,
    /// ```css
    /// scroll-margin-left: 13rem; /* 208px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 13rem; /* 208px */
    /// ```
    Mx52,
    /// ```css
    /// scroll-margin-top: 13rem; /* 208px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 13rem; /* 208px */
    /// ```
    My52,
    /// ```css
    /// scroll-margin-inline-start: 13rem; /* 208px */
    /// ```
    Ms52,
    /// ```css
    /// scroll-margin-inline-end: 13rem; /* 208px */
    /// ```
    Me52,
    /// ```css
    /// scroll-margin-top: 13rem; /* 208px */
    /// ```
    Mt52,
    /// ```css
    /// scroll-margin-right: 13rem; /* 208px */
    /// ```
    Mr52,
    /// ```css
    /// scroll-margin-bottom: 13rem; /* 208px */
    /// ```
    Mb52,
    /// ```css
    /// scroll-margin-left: 13rem; /* 208px */
    /// ```
    Ml52,
    /// ```css
    /// scroll-margin: 14rem; /* 224px */
    /// ```
    M56,
    /// ```css
    /// scroll-margin-left: 14rem; /* 224px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 14rem; /* 224px */
    /// ```
    Mx56,
    /// ```css
    /// scroll-margin-top: 14rem; /* 224px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 14rem; /* 224px */
    /// ```
    My56,
    /// ```css
    /// scroll-margin-inline-start: 14rem; /* 224px */
    /// ```
    Ms56,
    /// ```css
    /// scroll-margin-inline-end: 14rem; /* 224px */
    /// ```
    Me56,
    /// ```css
    /// scroll-margin-top: 14rem; /* 224px */
    /// ```
    Mt56,
    /// ```css
    /// scroll-margin-right: 14rem; /* 224px */
    /// ```
    Mr56,
    /// ```css
    /// scroll-margin-bottom: 14rem; /* 224px */
    /// ```
    Mb56,
    /// ```css
    /// scroll-margin-left: 14rem; /* 224px */
    /// ```
    Ml56,
    /// ```css
    /// scroll-margin: 15rem; /* 240px */
    /// ```
    M60,
    /// ```css
    /// scroll-margin-left: 15rem; /* 240px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 15rem; /* 240px */
    /// ```
    Mx60,
    /// ```css
    /// scroll-margin-top: 15rem; /* 240px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 15rem; /* 240px */
    /// ```
    My60,
    /// ```css
    /// scroll-margin-inline-start: 15rem; /* 240px */
    /// ```
    Ms60,
    /// ```css
    /// scroll-margin-inline-end: 15rem; /* 240px */
    /// ```
    Me60,
    /// ```css
    /// scroll-margin-top: 15rem; /* 240px */
    /// ```
    Mt60,
    /// ```css
    /// scroll-margin-right: 15rem; /* 240px */
    /// ```
    Mr60,
    /// ```css
    /// scroll-margin-bottom: 15rem; /* 240px */
    /// ```
    Mb60,
    /// ```css
    /// scroll-margin-left: 15rem; /* 240px */
    /// ```
    Ml60,
    /// ```css
    /// scroll-margin: 16rem; /* 256px */
    /// ```
    M64,
    /// ```css
    /// scroll-margin-left: 16rem; /* 256px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 16rem; /* 256px */
    /// ```
    Mx64,
    /// ```css
    /// scroll-margin-top: 16rem; /* 256px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 16rem; /* 256px */
    /// ```
    My64,
    /// ```css
    /// scroll-margin-inline-start: 16rem; /* 256px */
    /// ```
    Ms64,
    /// ```css
    /// scroll-margin-inline-end: 16rem; /* 256px */
    /// ```
    Me64,
    /// ```css
    /// scroll-margin-top: 16rem; /* 256px */
    /// ```
    Mt64,
    /// ```css
    /// scroll-margin-right: 16rem; /* 256px */
    /// ```
    Mr64,
    /// ```css
    /// scroll-margin-bottom: 16rem; /* 256px */
    /// ```
    Mb64,
    /// ```css
    /// scroll-margin-left: 16rem; /* 256px */
    /// ```
    Ml64,
    /// ```css
    /// scroll-margin: 18rem; /* 288px */
    /// ```
    M72,
    /// ```css
    /// scroll-margin-left: 18rem; /* 288px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 18rem; /* 288px */
    /// ```
    Mx72,
    /// ```css
    /// scroll-margin-top: 18rem; /* 288px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 18rem; /* 288px */
    /// ```
    My72,
    /// ```css
    /// scroll-margin-inline-start: 18rem; /* 288px */
    /// ```
    Ms72,
    /// ```css
    /// scroll-margin-inline-end: 18rem; /* 288px */
    /// ```
    Me72,
    /// ```css
    /// scroll-margin-top: 18rem; /* 288px */
    /// ```
    Mt72,
    /// ```css
    /// scroll-margin-right: 18rem; /* 288px */
    /// ```
    Mr72,
    /// ```css
    /// scroll-margin-bottom: 18rem; /* 288px */
    /// ```
    Mb72,
    /// ```css
    /// scroll-margin-left: 18rem; /* 288px */
    /// ```
    Ml72,
    /// ```css
    /// scroll-margin: 20rem; /* 320px */
    /// ```
    M80,
    /// ```css
    /// scroll-margin-left: 20rem; /* 320px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 20rem; /* 320px */
    /// ```
    Mx80,
    /// ```css
    /// scroll-margin-top: 20rem; /* 320px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 20rem; /* 320px */
    /// ```
    My80,
    /// ```css
    /// scroll-margin-inline-start: 20rem; /* 320px */
    /// ```
    Ms80,
    /// ```css
    /// scroll-margin-inline-end: 20rem; /* 320px */
    /// ```
    Me80,
    /// ```css
    /// scroll-margin-top: 20rem; /* 320px */
    /// ```
    Mt80,
    /// ```css
    /// scroll-margin-right: 20rem; /* 320px */
    /// ```
    Mr80,
    /// ```css
    /// scroll-margin-bottom: 20rem; /* 320px */
    /// ```
    Mb80,
    /// ```css
    /// scroll-margin-left: 20rem; /* 320px */
    /// ```
    Ml80,
    /// ```css
    /// scroll-margin: 24rem; /* 384px */
    /// ```
    M96,
    /// ```css
    /// scroll-margin-left: 24rem; /* 384px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-right: 24rem; /* 384px */
    /// ```
    Mx96,
    /// ```css
    /// scroll-margin-top: 24rem; /* 384px */
    /// ```
    /// 
    /// ```css
    /// scroll-margin-bottom: 24rem; /* 384px */
    /// ```
    My96,
    /// ```css
    /// scroll-margin-inline-start: 24rem; /* 384px */
    /// ```
    Ms96,
    /// ```css
    /// scroll-margin-inline-end: 24rem; /* 384px */
    /// ```
    Me96,
    /// ```css
    /// scroll-margin-top: 24rem; /* 384px */
    /// ```
    Mt96,
    /// ```css
    /// scroll-margin-right: 24rem; /* 384px */
    /// ```
    Mr96,
    /// ```css
    /// scroll-margin-bottom: 24rem; /* 384px */
    /// ```
    Mb96,
    /// ```css
    /// scroll-margin-left: 24rem; /* 384px */
    /// ```
    Ml96,
}

/// Utilities for controlling an element's scroll offset within a snap container.
/// 
/// <https://tailwindcss.com/docs/scroll-padding>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "scroll")]
pub struct ScrollPadding(pub Padding);

/// Utilities for controlling the scroll snap alignment of an element.
/// 
/// <https://tailwindcss.com/docs/scroll-snap-align>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "snap")]
pub enum ScrollSnapAlign {
    /// ```css
    /// scroll-snap-align: start;
    /// ```
    Start,
    /// ```css
    /// scroll-snap-align: end;
    /// ```
    End,
    /// ```css
    /// scroll-snap-align: center;
    /// ```
    Center,
    /// ```css
    /// scroll-snap-align: none;
    /// ```
    AlignNone,
}

/// Utilities for controlling whether you can skip past possible snap positions.
/// 
/// <https://tailwindcss.com/docs/scroll-snap-stop>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "snap")]
pub enum ScrollSnapStop {
    /// ```css
    /// scroll-snap-stop: normal;
    /// ```
    Normal,
    /// ```css
    /// scroll-snap-stop: always;
    /// ```
    Always,
}

/// Utilities for controlling how strictly snap points are enforced in a snap container.
/// 
/// <https://tailwindcss.com/docs/scroll-snap-type>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "snap")]
pub enum ScrollSnapType {
    /// ```css
    /// scroll-snap-type: none;
    /// ```
    None,
    /// ```css
    /// scroll-snap-type: x var(--tw-scroll-snap-strictness);
    /// ```
    X,
    /// ```css
    /// scroll-snap-type: y var(--tw-scroll-snap-strictness);
    /// ```
    Y,
    /// ```css
    /// scroll-snap-type: both var(--tw-scroll-snap-strictness);
    /// ```
    Both,
    /// ```css
    /// --tw-scroll-snap-strictness: mandatory;
    /// ```
    Mandatory,
    /// ```css
    /// --tw-scroll-snap-strictness: proximity;
    /// ```
    Proximity,
}

/// Utilities for controlling how an element can be scrolled and zoomed on touchscreens.
/// 
/// <https://tailwindcss.com/docs/touch-action>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "touch")]
pub enum TouchAction {
    /// ```css
    /// touch-action: auto;
    /// ```
    Auto,
    /// ```css
    /// touch-action: none;
    /// ```
    None,
    /// ```css
    /// touch-action: pan-x;
    /// ```
    PanX,
    /// ```css
    /// touch-action: pan-left;
    /// ```
    PanLeft,
    /// ```css
    /// touch-action: pan-right;
    /// ```
    PanRight,
    /// ```css
    /// touch-action: pan-y;
    /// ```
    PanY,
    /// ```css
    /// touch-action: pan-up;
    /// ```
    PanUp,
    /// ```css
    /// touch-action: pan-down;
    /// ```
    PanDown,
    /// ```css
    /// touch-action: pinch-zoom;
    /// ```
    PinchZoom,
    /// ```css
    /// touch-action: manipulation;
    /// ```
    Manipulation,
}

/// Utilities for controlling whether the user can select text in an element.
/// 
/// <https://tailwindcss.com/docs/user-select>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "select")]
pub enum UserSelect {
    /// ```css
    /// user-select: none;
    /// ```
    None,
    /// ```css
    /// user-select: text;
    /// ```
    Text,
    /// ```css
    /// user-select: all;
    /// ```
    All,
    /// ```css
    /// user-select: auto;
    /// ```
    Auto,
}

/// Utilities for optimizing upcoming animations of elements that are expected to change.
/// 
/// <https://tailwindcss.com/docs/will-change>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "will-change")]
pub enum WillChange {
    /// ```css
    /// will-change: auto;
    /// ```
    Auto,
    /// ```css
    /// will-change: scroll-position;
    /// ```
    Scroll,
    /// ```css
    /// will-change: contents;
    /// ```
    Contents,
    /// ```css
    /// will-change: transform;
    /// ```
    Transform,
}
