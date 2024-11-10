use typewind_macros::{Display, Parse};

tailwind_types!(Padding, Margin, SpaceBetween);

/// Utilities for controlling an element's padding.
/// 
/// <https://tailwindcss.com/docs/padding>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(replace(from = "_", to = "."))]
pub enum Padding {
    /// ```css
    /// {
    ///     padding: 0px;
    /// }
    /// ```
    P0,
    /// ```css
    /// {
    ///     padding-left: 0px;
    ///     padding-right: 0px;
    /// }
    /// ```
    Px0,
    /// ```css
    /// {
    ///     padding-top: 0px;
    ///     padding-bottom: 0px;
    /// }
    /// ```
    Py0,
    /// ```css
    /// {
    ///     padding-inline-start: 0px;
    /// }
    /// ```
    Ps0,
    /// ```css
    /// {
    ///     padding-inline-end: 0px;
    /// }
    /// ```
    Pe0,
    /// ```css
    /// {
    ///     padding-top: 0px;
    /// }
    /// ```
    Pt0,
    /// ```css
    /// {
    ///     padding-right: 0px;
    /// }
    /// ```
    Pr0,
    /// ```css
    /// {
    ///     padding-bottom: 0px;
    /// }
    /// ```
    Pb0,
    /// ```css
    /// {
    ///     padding-left: 0px;
    /// }
    /// ```
    Pl0,
    /// ```css
    /// {
    ///     padding: 1px;
    /// }
    /// ```
    PPx,
    /// ```css
    /// {
    ///     padding-left: 1px;
    ///     padding-right: 1px;
    /// }
    /// ```
    PxPx,
    /// ```css
    /// {
    ///     padding-top: 1px;
    ///     padding-bottom: 1px;
    /// }
    /// ```
    PyPx,
    /// ```css
    /// {
    ///     padding-inline-start: 1px;
    /// }
    /// ```
    PsPx,
    /// ```css
    /// {
    ///     padding-inline-end: 1px;
    /// }
    /// ```
    PePx,
    /// ```css
    /// {
    ///     padding-top: 1px;
    /// }
    /// ```
    PtPx,
    /// ```css
    /// {
    ///     padding-right: 1px;
    /// }
    /// ```
    PrPx,
    /// ```css
    /// {
    ///     padding-bottom: 1px;
    /// }
    /// ```
    PbPx,
    /// ```css
    /// {
    ///     padding-left: 1px;
    /// }
    /// ```
    PlPx,
    /// ```css
    /// {
    ///     padding: 0.125rem; /* 2px */
    /// }
    /// ```
    P0_5,
    /// ```css
    /// {
    ///     padding-left: 0.125rem; /* 2px */
    ///     padding-right: 0.125rem; /* 2px */
    /// }
    /// ```
    Px0_5,
    /// ```css
    /// {
    ///     padding-top: 0.125rem; /* 2px */
    ///     padding-bottom: 0.125rem; /* 2px */
    /// }
    /// ```
    Py0_5,
    /// ```css
    /// {
    ///     padding-inline-start: 0.125rem; /* 2px */
    /// }
    /// ```
    Ps0_5,
    /// ```css
    /// {
    ///     padding-inline-end: 0.125rem; /* 2px */
    /// }
    /// ```
    Pe0_5,
    /// ```css
    /// {
    ///     padding-top: 0.125rem; /* 2px */
    /// }
    /// ```
    Pt0_5,
    /// ```css
    /// {
    ///     padding-right: 0.125rem; /* 2px */
    /// }
    /// ```
    Pr0_5,
    /// ```css
    /// {
    ///     padding-bottom: 0.125rem; /* 2px */
    /// }
    /// ```
    Pb0_5,
    /// ```css
    /// {
    ///     padding-left: 0.125rem; /* 2px */
    /// }
    /// ```
    Pl0_5,
    /// ```css
    /// {
    ///     padding: 0.25rem; /* 4px */
    /// }
    /// ```
    P1,
    /// ```css
    /// {
    ///     padding-left: 0.25rem; /* 4px */
    ///     padding-right: 0.25rem; /* 4px */
    /// }
    /// ```
    Px1,
    /// ```css
    /// {
    ///     padding-top: 0.25rem; /* 4px */
    ///     padding-bottom: 0.25rem; /* 4px */
    /// }
    /// ```
    Py1,
    /// ```css
    /// {
    ///     padding-inline-start: 0.25rem; /* 4px */
    /// }
    /// ```
    Ps1,
    /// ```css
    /// {
    ///     padding-inline-end: 0.25rem; /* 4px */
    /// }
    /// ```
    Pe1,
    /// ```css
    /// {
    ///     padding-top: 0.25rem; /* 4px */
    /// }
    /// ```
    Pt1,
    /// ```css
    /// {
    ///     padding-right: 0.25rem; /* 4px */
    /// }
    /// ```
    Pr1,
    /// ```css
    /// {
    ///     padding-bottom: 0.25rem; /* 4px */
    /// }
    /// ```
    Pb1,
    /// ```css
    /// {
    ///     padding-left: 0.25rem; /* 4px */
    /// }
    /// ```
    Pl1,
    /// ```css
    /// {
    ///     padding: 0.375rem; /* 6px */
    /// }
    /// ```
    P1_5,
    /// ```css
    /// {
    ///     padding-left: 0.375rem; /* 6px */
    ///     padding-right: 0.375rem; /* 6px */
    /// }
    /// ```
    Px1_5,
    /// ```css
    /// {
    ///     padding-top: 0.375rem; /* 6px */
    ///     padding-bottom: 0.375rem; /* 6px */
    /// }
    /// ```
    Py1_5,
    /// ```css
    /// {
    ///     padding-inline-start: 0.375rem; /* 6px */
    /// }
    /// ```
    Ps1_5,
    /// ```css
    /// {
    ///     padding-inline-end: 0.375rem; /* 6px */
    /// }
    /// ```
    Pe1_5,
    /// ```css
    /// {
    ///     padding-top: 0.375rem; /* 6px */
    /// }
    /// ```
    Pt1_5,
    /// ```css
    /// {
    ///     padding-right: 0.375rem; /* 6px */
    /// }
    /// ```
    Pr1_5,
    /// ```css
    /// {
    ///     padding-bottom: 0.375rem; /* 6px */
    /// }
    /// ```
    Pb1_5,
    /// ```css
    /// {
    ///     padding-left: 0.375rem; /* 6px */
    /// }
    /// ```
    Pl1_5,
    /// ```css
    /// {
    ///     padding: 0.5rem; /* 8px */
    /// }
    /// ```
    P2,
    /// ```css
    /// {
    ///     padding-left: 0.5rem; /* 8px */
    ///     padding-right: 0.5rem; /* 8px */
    /// }
    /// ```
    Px2,
    /// ```css
    /// {
    ///     padding-top: 0.5rem; /* 8px */
    ///     padding-bottom: 0.5rem; /* 8px */
    /// }
    /// ```
    Py2,
    /// ```css
    /// {
    ///     padding-inline-start: 0.5rem; /* 8px */
    /// }
    /// ```
    Ps2,
    /// ```css
    /// {
    ///     padding-inline-end: 0.5rem; /* 8px */
    /// }
    /// ```
    Pe2,
    /// ```css
    /// {
    ///     padding-top: 0.5rem; /* 8px */
    /// }
    /// ```
    Pt2,
    /// ```css
    /// {
    ///     padding-right: 0.5rem; /* 8px */
    /// }
    /// ```
    Pr2,
    /// ```css
    /// {
    ///     padding-bottom: 0.5rem; /* 8px */
    /// }
    /// ```
    Pb2,
    /// ```css
    /// {
    ///     padding-left: 0.5rem; /* 8px */
    /// }
    /// ```
    Pl2,
    /// ```css
    /// {
    ///     padding: 0.625rem; /* 10px */
    /// }
    /// ```
    P2_5,
    /// ```css
    /// {
    ///     padding-left: 0.625rem; /* 10px */
    ///     padding-right: 0.625rem; /* 10px */
    /// }
    /// ```
    Px2_5,
    /// ```css
    /// {
    ///     padding-top: 0.625rem; /* 10px */
    ///     padding-bottom: 0.625rem; /* 10px */
    /// }
    /// ```
    Py2_5,
    /// ```css
    /// {
    ///     padding-inline-start: 0.625rem; /* 10px */
    /// }
    /// ```
    Ps2_5,
    /// ```css
    /// {
    ///     padding-inline-end: 0.625rem; /* 10px */
    /// }
    /// ```
    Pe2_5,
    /// ```css
    /// {
    ///     padding-top: 0.625rem; /* 10px */
    /// }
    /// ```
    Pt2_5,
    /// ```css
    /// {
    ///     padding-right: 0.625rem; /* 10px */
    /// }
    /// ```
    Pr2_5,
    /// ```css
    /// {
    ///     padding-bottom: 0.625rem; /* 10px */
    /// }
    /// ```
    Pb2_5,
    /// ```css
    /// {
    ///     padding-left: 0.625rem; /* 10px */
    /// }
    /// ```
    Pl2_5,
    /// ```css
    /// {
    ///     padding: 0.75rem; /* 12px */
    /// }
    /// ```
    P3,
    /// ```css
    /// {
    ///     padding-left: 0.75rem; /* 12px */
    ///     padding-right: 0.75rem; /* 12px */
    /// }
    /// ```
    Px3,
    /// ```css
    /// {
    ///     padding-top: 0.75rem; /* 12px */
    ///     padding-bottom: 0.75rem; /* 12px */
    /// }
    /// ```
    Py3,
    /// ```css
    /// {
    ///     padding-inline-start: 0.75rem; /* 12px */
    /// }
    /// ```
    Ps3,
    /// ```css
    /// {
    ///     padding-inline-end: 0.75rem; /* 12px */
    /// }
    /// ```
    Pe3,
    /// ```css
    /// {
    ///     padding-top: 0.75rem; /* 12px */
    /// }
    /// ```
    Pt3,
    /// ```css
    /// {
    ///     padding-right: 0.75rem; /* 12px */
    /// }
    /// ```
    Pr3,
    /// ```css
    /// {
    ///     padding-bottom: 0.75rem; /* 12px */
    /// }
    /// ```
    Pb3,
    /// ```css
    /// {
    ///     padding-left: 0.75rem; /* 12px */
    /// }
    /// ```
    Pl3,
    /// ```css
    /// {
    ///     padding: 0.875rem; /* 14px */
    /// }
    /// ```
    P3_5,
    /// ```css
    /// {
    ///     padding-left: 0.875rem; /* 14px */
    ///     padding-right: 0.875rem; /* 14px */
    /// }
    /// ```
    Px3_5,
    /// ```css
    /// {
    ///     padding-top: 0.875rem; /* 14px */
    ///     padding-bottom: 0.875rem; /* 14px */
    /// }
    /// ```
    Py3_5,
    /// ```css
    /// {
    ///     padding-inline-start: 0.875rem; /* 14px */
    /// }
    /// ```
    Ps3_5,
    /// ```css
    /// {
    ///     padding-inline-end: 0.875rem; /* 14px */
    /// }
    /// ```
    Pe3_5,
    /// ```css
    /// {
    ///     padding-top: 0.875rem; /* 14px */
    /// }
    /// ```
    Pt3_5,
    /// ```css
    /// {
    ///     padding-right: 0.875rem; /* 14px */
    /// }
    /// ```
    Pr3_5,
    /// ```css
    /// {
    ///     padding-bottom: 0.875rem; /* 14px */
    /// }
    /// ```
    Pb3_5,
    /// ```css
    /// {
    ///     padding-left: 0.875rem; /* 14px */
    /// }
    /// ```
    Pl3_5,
    /// ```css
    /// {
    ///     padding: 1rem; /* 16px */
    /// }
    /// ```
    P4,
    /// ```css
    /// {
    ///     padding-left: 1rem; /* 16px */
    ///     padding-right: 1rem; /* 16px */
    /// }
    /// ```
    Px4,
    /// ```css
    /// {
    ///     padding-top: 1rem; /* 16px */
    ///     padding-bottom: 1rem; /* 16px */
    /// }
    /// ```
    Py4,
    /// ```css
    /// {
    ///     padding-inline-start: 1rem; /* 16px */
    /// }
    /// ```
    Ps4,
    /// ```css
    /// {
    ///     padding-inline-end: 1rem; /* 16px */
    /// }
    /// ```
    Pe4,
    /// ```css
    /// {
    ///     padding-top: 1rem; /* 16px */
    /// }
    /// ```
    Pt4,
    /// ```css
    /// {
    ///     padding-right: 1rem; /* 16px */
    /// }
    /// ```
    Pr4,
    /// ```css
    /// {
    ///     padding-bottom: 1rem; /* 16px */
    /// }
    /// ```
    Pb4,
    /// ```css
    /// {
    ///     padding-left: 1rem; /* 16px */
    /// }
    /// ```
    Pl4,
    /// ```css
    /// {
    ///     padding: 1.25rem; /* 20px */
    /// }
    /// ```
    P5,
    /// ```css
    /// {
    ///     padding-left: 1.25rem; /* 20px */
    ///     padding-right: 1.25rem; /* 20px */
    /// }
    /// ```
    Px5,
    /// ```css
    /// {
    ///     padding-top: 1.25rem; /* 20px */
    ///     padding-bottom: 1.25rem; /* 20px */
    /// }
    /// ```
    Py5,
    /// ```css
    /// {
    ///     padding-inline-start: 1.25rem; /* 20px */
    /// }
    /// ```
    Ps5,
    /// ```css
    /// {
    ///     padding-inline-end: 1.25rem; /* 20px */
    /// }
    /// ```
    Pe5,
    /// ```css
    /// {
    ///     padding-top: 1.25rem; /* 20px */
    /// }
    /// ```
    Pt5,
    /// ```css
    /// {
    ///     padding-right: 1.25rem; /* 20px */
    /// }
    /// ```
    Pr5,
    /// ```css
    /// {
    ///     padding-bottom: 1.25rem; /* 20px */
    /// }
    /// ```
    Pb5,
    /// ```css
    /// {
    ///     padding-left: 1.25rem; /* 20px */
    /// }
    /// ```
    Pl5,
    /// ```css
    /// {
    ///     padding: 1.5rem; /* 24px */
    /// }
    /// ```
    P6,
    /// ```css
    /// {
    ///     padding-left: 1.5rem; /* 24px */
    ///     padding-right: 1.5rem; /* 24px */
    /// }
    /// ```
    Px6,
    /// ```css
    /// {
    ///     padding-top: 1.5rem; /* 24px */
    ///     padding-bottom: 1.5rem; /* 24px */
    /// }
    /// ```
    Py6,
    /// ```css
    /// {
    ///     padding-inline-start: 1.5rem; /* 24px */
    /// }
    /// ```
    Ps6,
    /// ```css
    /// {
    ///     padding-inline-end: 1.5rem; /* 24px */
    /// }
    /// ```
    Pe6,
    /// ```css
    /// {
    ///     padding-top: 1.5rem; /* 24px */
    /// }
    /// ```
    Pt6,
    /// ```css
    /// {
    ///     padding-right: 1.5rem; /* 24px */
    /// }
    /// ```
    Pr6,
    /// ```css
    /// {
    ///     padding-bottom: 1.5rem; /* 24px */
    /// }
    /// ```
    Pb6,
    /// ```css
    /// {
    ///     padding-left: 1.5rem; /* 24px */
    /// }
    /// ```
    Pl6,
    /// ```css
    /// {
    ///     padding: 1.75rem; /* 28px */
    /// }
    /// ```
    P7,
    /// ```css
    /// {
    ///     padding-left: 1.75rem; /* 28px */
    ///     padding-right: 1.75rem; /* 28px */
    /// }
    /// ```
    Px7,
    /// ```css
    /// {
    ///     padding-top: 1.75rem; /* 28px */
    ///     padding-bottom: 1.75rem; /* 28px */
    /// }
    /// ```
    Py7,
    /// ```css
    /// {
    ///     padding-inline-start: 1.75rem; /* 28px */
    /// }
    /// ```
    Ps7,
    /// ```css
    /// {
    ///     padding-inline-end: 1.75rem; /* 28px */
    /// }
    /// ```
    Pe7,
    /// ```css
    /// {
    ///     padding-top: 1.75rem; /* 28px */
    /// }
    /// ```
    Pt7,
    /// ```css
    /// {
    ///     padding-right: 1.75rem; /* 28px */
    /// }
    /// ```
    Pr7,
    /// ```css
    /// {
    ///     padding-bottom: 1.75rem; /* 28px */
    /// }
    /// ```
    Pb7,
    /// ```css
    /// {
    ///     padding-left: 1.75rem; /* 28px */
    /// }
    /// ```
    Pl7,
    /// ```css
    /// {
    ///     padding: 2rem; /* 32px */
    /// }
    /// ```
    P8,
    /// ```css
    /// {
    ///     padding-left: 2rem; /* 32px */
    ///     padding-right: 2rem; /* 32px */
    /// }
    /// ```
    Px8,
    /// ```css
    /// {
    ///     padding-top: 2rem; /* 32px */
    ///     padding-bottom: 2rem; /* 32px */
    /// }
    /// ```
    Py8,
    /// ```css
    /// {
    ///     padding-inline-start: 2rem; /* 32px */
    /// }
    /// ```
    Ps8,
    /// ```css
    /// {
    ///     padding-inline-end: 2rem; /* 32px */
    /// }
    /// ```
    Pe8,
    /// ```css
    /// {
    ///     padding-top: 2rem; /* 32px */
    /// }
    /// ```
    Pt8,
    /// ```css
    /// {
    ///     padding-right: 2rem; /* 32px */
    /// }
    /// ```
    Pr8,
    /// ```css
    /// {
    ///     padding-bottom: 2rem; /* 32px */
    /// }
    /// ```
    Pb8,
    /// ```css
    /// {
    ///     padding-left: 2rem; /* 32px */
    /// }
    /// ```
    Pl8,
    /// ```css
    /// {
    ///     padding: 2.25rem; /* 36px */
    /// }
    /// ```
    P9,
    /// ```css
    /// {
    ///     padding-left: 2.25rem; /* 36px */
    ///     padding-right: 2.25rem; /* 36px */
    /// }
    /// ```
    Px9,
    /// ```css
    /// {
    ///     padding-top: 2.25rem; /* 36px */
    ///     padding-bottom: 2.25rem; /* 36px */
    /// }
    /// ```
    Py9,
    /// ```css
    /// {
    ///     padding-inline-start: 2.25rem; /* 36px */
    /// }
    /// ```
    Ps9,
    /// ```css
    /// {
    ///     padding-inline-end: 2.25rem; /* 36px */
    /// }
    /// ```
    Pe9,
    /// ```css
    /// {
    ///     padding-top: 2.25rem; /* 36px */
    /// }
    /// ```
    Pt9,
    /// ```css
    /// {
    ///     padding-right: 2.25rem; /* 36px */
    /// }
    /// ```
    Pr9,
    /// ```css
    /// {
    ///     padding-bottom: 2.25rem; /* 36px */
    /// }
    /// ```
    Pb9,
    /// ```css
    /// {
    ///     padding-left: 2.25rem; /* 36px */
    /// }
    /// ```
    Pl9,
    /// ```css
    /// {
    ///     padding: 2.5rem; /* 40px */
    /// }
    /// ```
    P10,
    /// ```css
    /// {
    ///     padding-left: 2.5rem; /* 40px */
    ///     padding-right: 2.5rem; /* 40px */
    /// }
    /// ```
    Px10,
    /// ```css
    /// {
    ///     padding-top: 2.5rem; /* 40px */
    ///     padding-bottom: 2.5rem; /* 40px */
    /// }
    /// ```
    Py10,
    /// ```css
    /// {
    ///     padding-inline-start: 2.5rem; /* 40px */
    /// }
    /// ```
    Ps10,
    /// ```css
    /// {
    ///     padding-inline-end: 2.5rem; /* 40px */
    /// }
    /// ```
    Pe10,
    /// ```css
    /// {
    ///     padding-top: 2.5rem; /* 40px */
    /// }
    /// ```
    Pt10,
    /// ```css
    /// {
    ///     padding-right: 2.5rem; /* 40px */
    /// }
    /// ```
    Pr10,
    /// ```css
    /// {
    ///     padding-bottom: 2.5rem; /* 40px */
    /// }
    /// ```
    Pb10,
    /// ```css
    /// {
    ///     padding-left: 2.5rem; /* 40px */
    /// }
    /// ```
    Pl10,
    /// ```css
    /// {
    ///     padding: 2.75rem; /* 44px */
    /// }
    /// ```
    P11,
    /// ```css
    /// {
    ///     padding-left: 2.75rem; /* 44px */
    ///     padding-right: 2.75rem; /* 44px */
    /// }
    /// ```
    Px11,
    /// ```css
    /// {
    ///     padding-top: 2.75rem; /* 44px */
    ///     padding-bottom: 2.75rem; /* 44px */
    /// }
    /// ```
    Py11,
    /// ```css
    /// {
    ///     padding-inline-start: 2.75rem; /* 44px */
    /// }
    /// ```
    Ps11,
    /// ```css
    /// {
    ///     padding-inline-end: 2.75rem; /* 44px */
    /// }
    /// ```
    Pe11,
    /// ```css
    /// {
    ///     padding-top: 2.75rem; /* 44px */
    /// }
    /// ```
    Pt11,
    /// ```css
    /// {
    ///     padding-right: 2.75rem; /* 44px */
    /// }
    /// ```
    Pr11,
    /// ```css
    /// {
    ///     padding-bottom: 2.75rem; /* 44px */
    /// }
    /// ```
    Pb11,
    /// ```css
    /// {
    ///     padding-left: 2.75rem; /* 44px */
    /// }
    /// ```
    Pl11,
    /// ```css
    /// {
    ///     padding: 3rem; /* 48px */
    /// }
    /// ```
    P12,
    /// ```css
    /// {
    ///     padding-left: 3rem; /* 48px */
    ///     padding-right: 3rem; /* 48px */
    /// }
    /// ```
    Px12,
    /// ```css
    /// {
    ///     padding-top: 3rem; /* 48px */
    ///     padding-bottom: 3rem; /* 48px */
    /// }
    /// ```
    Py12,
    /// ```css
    /// {
    ///     padding-inline-start: 3rem; /* 48px */
    /// }
    /// ```
    Ps12,
    /// ```css
    /// {
    ///     padding-inline-end: 3rem; /* 48px */
    /// }
    /// ```
    Pe12,
    /// ```css
    /// {
    ///     padding-top: 3rem; /* 48px */
    /// }
    /// ```
    Pt12,
    /// ```css
    /// {
    ///     padding-right: 3rem; /* 48px */
    /// }
    /// ```
    Pr12,
    /// ```css
    /// {
    ///     padding-bottom: 3rem; /* 48px */
    /// }
    /// ```
    Pb12,
    /// ```css
    /// {
    ///     padding-left: 3rem; /* 48px */
    /// }
    /// ```
    Pl12,
    /// ```css
    /// {
    ///     padding: 3.5rem; /* 56px */
    /// }
    /// ```
    P14,
    /// ```css
    /// {
    ///     padding-left: 3.5rem; /* 56px */
    ///     padding-right: 3.5rem; /* 56px */
    /// }
    /// ```
    Px14,
    /// ```css
    /// {
    ///     padding-top: 3.5rem; /* 56px */
    ///     padding-bottom: 3.5rem; /* 56px */
    /// }
    /// ```
    Py14,
    /// ```css
    /// {
    ///     padding-inline-start: 3.5rem; /* 56px */
    /// }
    /// ```
    Ps14,
    /// ```css
    /// {
    ///     padding-inline-end: 3.5rem; /* 56px */
    /// }
    /// ```
    Pe14,
    /// ```css
    /// {
    ///     padding-top: 3.5rem; /* 56px */
    /// }
    /// ```
    Pt14,
    /// ```css
    /// {
    ///     padding-right: 3.5rem; /* 56px */
    /// }
    /// ```
    Pr14,
    /// ```css
    /// {
    ///     padding-bottom: 3.5rem; /* 56px */
    /// }
    /// ```
    Pb14,
    /// ```css
    /// {
    ///     padding-left: 3.5rem; /* 56px */
    /// }
    /// ```
    Pl14,
    /// ```css
    /// {
    ///     padding: 4rem; /* 64px */
    /// }
    /// ```
    P16,
    /// ```css
    /// {
    ///     padding-left: 4rem; /* 64px */
    ///     padding-right: 4rem; /* 64px */
    /// }
    /// ```
    Px16,
    /// ```css
    /// {
    ///     padding-top: 4rem; /* 64px */
    ///     padding-bottom: 4rem; /* 64px */
    /// }
    /// ```
    Py16,
    /// ```css
    /// {
    ///     padding-inline-start: 4rem; /* 64px */
    /// }
    /// ```
    Ps16,
    /// ```css
    /// {
    ///     padding-inline-end: 4rem; /* 64px */
    /// }
    /// ```
    Pe16,
    /// ```css
    /// {
    ///     padding-top: 4rem; /* 64px */
    /// }
    /// ```
    Pt16,
    /// ```css
    /// {
    ///     padding-right: 4rem; /* 64px */
    /// }
    /// ```
    Pr16,
    /// ```css
    /// {
    ///     padding-bottom: 4rem; /* 64px */
    /// }
    /// ```
    Pb16,
    /// ```css
    /// {
    ///     padding-left: 4rem; /* 64px */
    /// }
    /// ```
    Pl16,
    /// ```css
    /// {
    ///     padding: 5rem; /* 80px */
    /// }
    /// ```
    P20,
    /// ```css
    /// {
    ///     padding-left: 5rem; /* 80px */
    ///     padding-right: 5rem; /* 80px */
    /// }
    /// ```
    Px20,
    /// ```css
    /// {
    ///     padding-top: 5rem; /* 80px */
    ///     padding-bottom: 5rem; /* 80px */
    /// }
    /// ```
    Py20,
    /// ```css
    /// {
    ///     padding-inline-start: 5rem; /* 80px */
    /// }
    /// ```
    Ps20,
    /// ```css
    /// {
    ///     padding-inline-end: 5rem; /* 80px */
    /// }
    /// ```
    Pe20,
    /// ```css
    /// {
    ///     padding-top: 5rem; /* 80px */
    /// }
    /// ```
    Pt20,
    /// ```css
    /// {
    ///     padding-right: 5rem; /* 80px */
    /// }
    /// ```
    Pr20,
    /// ```css
    /// {
    ///     padding-bottom: 5rem; /* 80px */
    /// }
    /// ```
    Pb20,
    /// ```css
    /// {
    ///     padding-left: 5rem; /* 80px */
    /// }
    /// ```
    Pl20,
    /// ```css
    /// {
    ///     padding: 6rem; /* 96px */
    /// }
    /// ```
    P24,
    /// ```css
    /// {
    ///     padding-left: 6rem; /* 96px */
    ///     padding-right: 6rem; /* 96px */
    /// }
    /// ```
    Px24,
    /// ```css
    /// {
    ///     padding-top: 6rem; /* 96px */
    ///     padding-bottom: 6rem; /* 96px */
    /// }
    /// ```
    Py24,
    /// ```css
    /// {
    ///     padding-inline-start: 6rem; /* 96px */
    /// }
    /// ```
    Ps24,
    /// ```css
    /// {
    ///     padding-inline-end: 6rem; /* 96px */
    /// }
    /// ```
    Pe24,
    /// ```css
    /// {
    ///     padding-top: 6rem; /* 96px */
    /// }
    /// ```
    Pt24,
    /// ```css
    /// {
    ///     padding-right: 6rem; /* 96px */
    /// }
    /// ```
    Pr24,
    /// ```css
    /// {
    ///     padding-bottom: 6rem; /* 96px */
    /// }
    /// ```
    Pb24,
    /// ```css
    /// {
    ///     padding-left: 6rem; /* 96px */
    /// }
    /// ```
    Pl24,
    /// ```css
    /// {
    ///     padding: 7rem; /* 112px */
    /// }
    /// ```
    P28,
    /// ```css
    /// {
    ///     padding-left: 7rem; /* 112px */
    ///     padding-right: 7rem; /* 112px */
    /// }
    /// ```
    Px28,
    /// ```css
    /// {
    ///     padding-top: 7rem; /* 112px */
    ///     padding-bottom: 7rem; /* 112px */
    /// }
    /// ```
    Py28,
    /// ```css
    /// {
    ///     padding-inline-start: 7rem; /* 112px */
    /// }
    /// ```
    Ps28,
    /// ```css
    /// {
    ///     padding-inline-end: 7rem; /* 112px */
    /// }
    /// ```
    Pe28,
    /// ```css
    /// {
    ///     padding-top: 7rem; /* 112px */
    /// }
    /// ```
    Pt28,
    /// ```css
    /// {
    ///     padding-right: 7rem; /* 112px */
    /// }
    /// ```
    Pr28,
    /// ```css
    /// {
    ///     padding-bottom: 7rem; /* 112px */
    /// }
    /// ```
    Pb28,
    /// ```css
    /// {
    ///     padding-left: 7rem; /* 112px */
    /// }
    /// ```
    Pl28,
    /// ```css
    /// {
    ///     padding: 8rem; /* 128px */
    /// }
    /// ```
    P32,
    /// ```css
    /// {
    ///     padding-left: 8rem; /* 128px */
    ///     padding-right: 8rem; /* 128px */
    /// }
    /// ```
    Px32,
    /// ```css
    /// {
    ///     padding-top: 8rem; /* 128px */
    ///     padding-bottom: 8rem; /* 128px */
    /// }
    /// ```
    Py32,
    /// ```css
    /// {
    ///     padding-inline-start: 8rem; /* 128px */
    /// }
    /// ```
    Ps32,
    /// ```css
    /// {
    ///     padding-inline-end: 8rem; /* 128px */
    /// }
    /// ```
    Pe32,
    /// ```css
    /// {
    ///     padding-top: 8rem; /* 128px */
    /// }
    /// ```
    Pt32,
    /// ```css
    /// {
    ///     padding-right: 8rem; /* 128px */
    /// }
    /// ```
    Pr32,
    /// ```css
    /// {
    ///     padding-bottom: 8rem; /* 128px */
    /// }
    /// ```
    Pb32,
    /// ```css
    /// {
    ///     padding-left: 8rem; /* 128px */
    /// }
    /// ```
    Pl32,
    /// ```css
    /// {
    ///     padding: 9rem; /* 144px */
    /// }
    /// ```
    P36,
    /// ```css
    /// {
    ///     padding-left: 9rem; /* 144px */
    ///     padding-right: 9rem; /* 144px */
    /// }
    /// ```
    Px36,
    /// ```css
    /// {
    ///     padding-top: 9rem; /* 144px */
    ///     padding-bottom: 9rem; /* 144px */
    /// }
    /// ```
    Py36,
    /// ```css
    /// {
    ///     padding-inline-start: 9rem; /* 144px */
    /// }
    /// ```
    Ps36,
    /// ```css
    /// {
    ///     padding-inline-end: 9rem; /* 144px */
    /// }
    /// ```
    Pe36,
    /// ```css
    /// {
    ///     padding-top: 9rem; /* 144px */
    /// }
    /// ```
    Pt36,
    /// ```css
    /// {
    ///     padding-right: 9rem; /* 144px */
    /// }
    /// ```
    Pr36,
    /// ```css
    /// {
    ///     padding-bottom: 9rem; /* 144px */
    /// }
    /// ```
    Pb36,
    /// ```css
    /// {
    ///     padding-left: 9rem; /* 144px */
    /// }
    /// ```
    Pl36,
    /// ```css
    /// {
    ///     padding: 10rem; /* 160px */
    /// }
    /// ```
    P40,
    /// ```css
    /// {
    ///     padding-left: 10rem; /* 160px */
    ///     padding-right: 10rem; /* 160px */
    /// }
    /// ```
    Px40,
    /// ```css
    /// {
    ///     padding-top: 10rem; /* 160px */
    ///     padding-bottom: 10rem; /* 160px */
    /// }
    /// ```
    Py40,
    /// ```css
    /// {
    ///     padding-inline-start: 10rem; /* 160px */
    /// }
    /// ```
    Ps40,
    /// ```css
    /// {
    ///     padding-inline-end: 10rem; /* 160px */
    /// }
    /// ```
    Pe40,
    /// ```css
    /// {
    ///     padding-top: 10rem; /* 160px */
    /// }
    /// ```
    Pt40,
    /// ```css
    /// {
    ///     padding-right: 10rem; /* 160px */
    /// }
    /// ```
    Pr40,
    /// ```css
    /// {
    ///     padding-bottom: 10rem; /* 160px */
    /// }
    /// ```
    Pb40,
    /// ```css
    /// {
    ///     padding-left: 10rem; /* 160px */
    /// }
    /// ```
    Pl40,
    /// ```css
    /// {
    ///     padding: 11rem; /* 176px */
    /// }
    /// ```
    P44,
    /// ```css
    /// {
    ///     padding-left: 11rem; /* 176px */
    ///     padding-right: 11rem; /* 176px */
    /// }
    /// ```
    Px44,
    /// ```css
    /// {
    ///     padding-top: 11rem; /* 176px */
    ///     padding-bottom: 11rem; /* 176px */
    /// }
    /// ```
    Py44,
    /// ```css
    /// {
    ///     padding-inline-start: 11rem; /* 176px */
    /// }
    /// ```
    Ps44,
    /// ```css
    /// {
    ///     padding-inline-end: 11rem; /* 176px */
    /// }
    /// ```
    Pe44,
    /// ```css
    /// {
    ///     padding-top: 11rem; /* 176px */
    /// }
    /// ```
    Pt44,
    /// ```css
    /// {
    ///     padding-right: 11rem; /* 176px */
    /// }
    /// ```
    Pr44,
    /// ```css
    /// {
    ///     padding-bottom: 11rem; /* 176px */
    /// }
    /// ```
    Pb44,
    /// ```css
    /// {
    ///     padding-left: 11rem; /* 176px */
    /// }
    /// ```
    Pl44,
    /// ```css
    /// {
    ///     padding: 12rem; /* 192px */
    /// }
    /// ```
    P48,
    /// ```css
    /// {
    ///     padding-left: 12rem; /* 192px */
    ///     padding-right: 12rem; /* 192px */
    /// }
    /// ```
    Px48,
    /// ```css
    /// {
    ///     padding-top: 12rem; /* 192px */
    ///     padding-bottom: 12rem; /* 192px */
    /// }
    /// ```
    Py48,
    /// ```css
    /// {
    ///     padding-inline-start: 12rem; /* 192px */
    /// }
    /// ```
    Ps48,
    /// ```css
    /// {
    ///     padding-inline-end: 12rem; /* 192px */
    /// }
    /// ```
    Pe48,
    /// ```css
    /// {
    ///     padding-top: 12rem; /* 192px */
    /// }
    /// ```
    Pt48,
    /// ```css
    /// {
    ///     padding-right: 12rem; /* 192px */
    /// }
    /// ```
    Pr48,
    /// ```css
    /// {
    ///     padding-bottom: 12rem; /* 192px */
    /// }
    /// ```
    Pb48,
    /// ```css
    /// {
    ///     padding-left: 12rem; /* 192px */
    /// }
    /// ```
    Pl48,
    /// ```css
    /// {
    ///     padding: 13rem; /* 208px */
    /// }
    /// ```
    P52,
    /// ```css
    /// {
    ///     padding-left: 13rem; /* 208px */
    ///     padding-right: 13rem; /* 208px */
    /// }
    /// ```
    Px52,
    /// ```css
    /// {
    ///     padding-top: 13rem; /* 208px */
    ///     padding-bottom: 13rem; /* 208px */
    /// }
    /// ```
    Py52,
    /// ```css
    /// {
    ///     padding-inline-start: 13rem; /* 208px */
    /// }
    /// ```
    Ps52,
    /// ```css
    /// {
    ///     padding-inline-end: 13rem; /* 208px */
    /// }
    /// ```
    Pe52,
    /// ```css
    /// {
    ///     padding-top: 13rem; /* 208px */
    /// }
    /// ```
    Pt52,
    /// ```css
    /// {
    ///     padding-right: 13rem; /* 208px */
    /// }
    /// ```
    Pr52,
    /// ```css
    /// {
    ///     padding-bottom: 13rem; /* 208px */
    /// }
    /// ```
    Pb52,
    /// ```css
    /// {
    ///     padding-left: 13rem; /* 208px */
    /// }
    /// ```
    Pl52,
    /// ```css
    /// {
    ///     padding: 14rem; /* 224px */
    /// }
    /// ```
    P56,
    /// ```css
    /// {
    ///     padding-left: 14rem; /* 224px */
    ///     padding-right: 14rem; /* 224px */
    /// }
    /// ```
    Px56,
    /// ```css
    /// {
    ///     padding-top: 14rem; /* 224px */
    ///     padding-bottom: 14rem; /* 224px */
    /// }
    /// ```
    Py56,
    /// ```css
    /// {
    ///     padding-inline-start: 14rem; /* 224px */
    /// }
    /// ```
    Ps56,
    /// ```css
    /// {
    ///     padding-inline-end: 14rem; /* 224px */
    /// }
    /// ```
    Pe56,
    /// ```css
    /// {
    ///     padding-top: 14rem; /* 224px */
    /// }
    /// ```
    Pt56,
    /// ```css
    /// {
    ///     padding-right: 14rem; /* 224px */
    /// }
    /// ```
    Pr56,
    /// ```css
    /// {
    ///     padding-bottom: 14rem; /* 224px */
    /// }
    /// ```
    Pb56,
    /// ```css
    /// {
    ///     padding-left: 14rem; /* 224px */
    /// }
    /// ```
    Pl56,
    /// ```css
    /// {
    ///     padding: 15rem; /* 240px */
    /// }
    /// ```
    P60,
    /// ```css
    /// {
    ///     padding-left: 15rem; /* 240px */
    ///     padding-right: 15rem; /* 240px */
    /// }
    /// ```
    Px60,
    /// ```css
    /// {
    ///     padding-top: 15rem; /* 240px */
    ///     padding-bottom: 15rem; /* 240px */
    /// }
    /// ```
    Py60,
    /// ```css
    /// {
    ///     padding-inline-start: 15rem; /* 240px */
    /// }
    /// ```
    Ps60,
    /// ```css
    /// {
    ///     padding-inline-end: 15rem; /* 240px */
    /// }
    /// ```
    Pe60,
    /// ```css
    /// {
    ///     padding-top: 15rem; /* 240px */
    /// }
    /// ```
    Pt60,
    /// ```css
    /// {
    ///     padding-right: 15rem; /* 240px */
    /// }
    /// ```
    Pr60,
    /// ```css
    /// {
    ///     padding-bottom: 15rem; /* 240px */
    /// }
    /// ```
    Pb60,
    /// ```css
    /// {
    ///     padding-left: 15rem; /* 240px */
    /// }
    /// ```
    Pl60,
    /// ```css
    /// {
    ///     padding: 16rem; /* 256px */
    /// }
    /// ```
    P64,
    /// ```css
    /// {
    ///     padding-left: 16rem; /* 256px */
    ///     padding-right: 16rem; /* 256px */
    /// }
    /// ```
    Px64,
    /// ```css
    /// {
    ///     padding-top: 16rem; /* 256px */
    ///     padding-bottom: 16rem; /* 256px */
    /// }
    /// ```
    Py64,
    /// ```css
    /// {
    ///     padding-inline-start: 16rem; /* 256px */
    /// }
    /// ```
    Ps64,
    /// ```css
    /// {
    ///     padding-inline-end: 16rem; /* 256px */
    /// }
    /// ```
    Pe64,
    /// ```css
    /// {
    ///     padding-top: 16rem; /* 256px */
    /// }
    /// ```
    Pt64,
    /// ```css
    /// {
    ///     padding-right: 16rem; /* 256px */
    /// }
    /// ```
    Pr64,
    /// ```css
    /// {
    ///     padding-bottom: 16rem; /* 256px */
    /// }
    /// ```
    Pb64,
    /// ```css
    /// {
    ///     padding-left: 16rem; /* 256px */
    /// }
    /// ```
    Pl64,
    /// ```css
    /// {
    ///     padding: 18rem; /* 288px */
    /// }
    /// ```
    P72,
    /// ```css
    /// {
    ///     padding-left: 18rem; /* 288px */
    ///     padding-right: 18rem; /* 288px */
    /// }
    /// ```
    Px72,
    /// ```css
    /// {
    ///     padding-top: 18rem; /* 288px */
    ///     padding-bottom: 18rem; /* 288px */
    /// }
    /// ```
    Py72,
    /// ```css
    /// {
    ///     padding-inline-start: 18rem; /* 288px */
    /// }
    /// ```
    Ps72,
    /// ```css
    /// {
    ///     padding-inline-end: 18rem; /* 288px */
    /// }
    /// ```
    Pe72,
    /// ```css
    /// {
    ///     padding-top: 18rem; /* 288px */
    /// }
    /// ```
    Pt72,
    /// ```css
    /// {
    ///     padding-right: 18rem; /* 288px */
    /// }
    /// ```
    Pr72,
    /// ```css
    /// {
    ///     padding-bottom: 18rem; /* 288px */
    /// }
    /// ```
    Pb72,
    /// ```css
    /// {
    ///     padding-left: 18rem; /* 288px */
    /// }
    /// ```
    Pl72,
    /// ```css
    /// {
    ///     padding: 20rem; /* 320px */
    /// }
    /// ```
    P80,
    /// ```css
    /// {
    ///     padding-left: 20rem; /* 320px */
    ///     padding-right: 20rem; /* 320px */
    /// }
    /// ```
    Px80,
    /// ```css
    /// {
    ///     padding-top: 20rem; /* 320px */
    ///     padding-bottom: 20rem; /* 320px */
    /// }
    /// ```
    Py80,
    /// ```css
    /// {
    ///     padding-inline-start: 20rem; /* 320px */
    /// }
    /// ```
    Ps80,
    /// ```css
    /// {
    ///     padding-inline-end: 20rem; /* 320px */
    /// }
    /// ```
    Pe80,
    /// ```css
    /// {
    ///     padding-top: 20rem; /* 320px */
    /// }
    /// ```
    Pt80,
    /// ```css
    /// {
    ///     padding-right: 20rem; /* 320px */
    /// }
    /// ```
    Pr80,
    /// ```css
    /// {
    ///     padding-bottom: 20rem; /* 320px */
    /// }
    /// ```
    Pb80,
    /// ```css
    /// {
    ///     padding-left: 20rem; /* 320px */
    /// }
    /// ```
    Pl80,
    /// ```css
    /// {
    ///     padding: 24rem; /* 384px */
    /// }
    /// ```
    P96,
    /// ```css
    /// {
    ///     padding-left: 24rem; /* 384px */
    ///     padding-right: 24rem; /* 384px */
    /// }
    /// ```
    Px96,
    /// ```css
    /// {
    ///     padding-top: 24rem; /* 384px */
    ///     padding-bottom: 24rem; /* 384px */
    /// }
    /// ```
    Py96,
    /// ```css
    /// {
    ///     padding-inline-start: 24rem; /* 384px */
    /// }
    /// ```
    Ps96,
    /// ```css
    /// {
    ///     padding-inline-end: 24rem; /* 384px */
    /// }
    /// ```
    Pe96,
    /// ```css
    /// {
    ///     padding-top: 24rem; /* 384px */
    /// }
    /// ```
    Pt96,
    /// ```css
    /// {
    ///     padding-right: 24rem; /* 384px */
    /// }
    /// ```
    Pr96,
    /// ```css
    /// {
    ///     padding-bottom: 24rem; /* 384px */
    /// }
    /// ```
    Pb96,
    /// ```css
    /// {
    ///     padding-left: 24rem; /* 384px */
    /// }
    /// ```
    Pl96,
}

/// Utilities for controlling an element's margin.
/// 
/// <https://tailwindcss.com/docs/margin>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(replace(from = "_", to = "."))]
pub enum Margin {
    /// ```css
    /// {
    ///     margin: 0px;
    /// }
    /// ```
    M0,
    /// ```css
    /// {
    ///     margin-left: 0px;
    ///     margin-right: 0px;
    /// }
    /// ```
    Mx0,
    /// ```css
    /// {
    ///     margin-top: 0px;
    ///     margin-bottom: 0px;
    /// }
    /// ```
    My0,
    /// ```css
    /// {
    ///     margin-inline-start: 0px;
    /// }
    /// ```
    Ms0,
    /// ```css
    /// {
    ///     margin-inline-end: 0px;
    /// }
    /// ```
    Me0,
    /// ```css
    /// {
    ///     margin-top: 0px;
    /// }
    /// ```
    Mt0,
    /// ```css
    /// {
    ///     margin-right: 0px;
    /// }
    /// ```
    Mr0,
    /// ```css
    /// {
    ///     margin-bottom: 0px;
    /// }
    /// ```
    Mb0,
    /// ```css
    /// {
    ///     margin-left: 0px;
    /// }
    /// ```
    Ml0,
    /// ```css
    /// {
    ///     margin: 1px;
    /// }
    /// ```
    MPx,
    /// ```css
    /// {
    ///     margin-left: 1px;
    ///     margin-right: 1px;
    /// }
    /// ```
    MxPx,
    /// ```css
    /// {
    ///     margin-top: 1px;
    ///     margin-bottom: 1px;
    /// }
    /// ```
    MyPx,
    /// ```css
    /// {
    ///     margin-inline-start: 1px;
    /// }
    /// ```
    MsPx,
    /// ```css
    /// {
    ///     margin-inline-end: 1px;
    /// }
    /// ```
    MePx,
    /// ```css
    /// {
    ///     margin-top: 1px;
    /// }
    /// ```
    MtPx,
    /// ```css
    /// {
    ///     margin-right: 1px;
    /// }
    /// ```
    MrPx,
    /// ```css
    /// {
    ///     margin-bottom: 1px;
    /// }
    /// ```
    MbPx,
    /// ```css
    /// {
    ///     margin-left: 1px;
    /// }
    /// ```
    MlPx,
    /// ```css
    /// {
    ///     margin: 0.125rem; /* 2px */
    /// }
    /// ```
    M0_5,
    /// ```css
    /// {
    ///     margin-left: 0.125rem; /* 2px */
    ///     margin-right: 0.125rem; /* 2px */
    /// }
    /// ```
    Mx0_5,
    /// ```css
    /// {
    ///     margin-top: 0.125rem; /* 2px */
    ///     margin-bottom: 0.125rem; /* 2px */
    /// }
    /// ```
    My0_5,
    /// ```css
    /// {
    ///     margin-inline-start: 0.125rem; /* 2px */
    /// }
    /// ```
    Ms0_5,
    /// ```css
    /// {
    ///     margin-inline-end: 0.125rem; /* 2px */
    /// }
    /// ```
    Me0_5,
    /// ```css
    /// {
    ///     margin-top: 0.125rem; /* 2px */
    /// }
    /// ```
    Mt0_5,
    /// ```css
    /// {
    ///     margin-right: 0.125rem; /* 2px */
    /// }
    /// ```
    Mr0_5,
    /// ```css
    /// {
    ///     margin-bottom: 0.125rem; /* 2px */
    /// }
    /// ```
    Mb0_5,
    /// ```css
    /// {
    ///     margin-left: 0.125rem; /* 2px */
    /// }
    /// ```
    Ml0_5,
    /// ```css
    /// {
    ///     margin: 0.25rem; /* 4px */
    /// }
    /// ```
    M1,
    /// ```css
    /// {
    ///     margin-left: 0.25rem; /* 4px */
    ///     margin-right: 0.25rem; /* 4px */
    /// }
    /// ```
    Mx1,
    /// ```css
    /// {
    ///     margin-top: 0.25rem; /* 4px */
    ///     margin-bottom: 0.25rem; /* 4px */
    /// }
    /// ```
    My1,
    /// ```css
    /// {
    ///     margin-inline-start: 0.25rem; /* 4px */
    /// }
    /// ```
    Ms1,
    /// ```css
    /// {
    ///     margin-inline-end: 0.25rem; /* 4px */
    /// }
    /// ```
    Me1,
    /// ```css
    /// {
    ///     margin-top: 0.25rem; /* 4px */
    /// }
    /// ```
    Mt1,
    /// ```css
    /// {
    ///     margin-right: 0.25rem; /* 4px */
    /// }
    /// ```
    Mr1,
    /// ```css
    /// {
    ///     margin-bottom: 0.25rem; /* 4px */
    /// }
    /// ```
    Mb1,
    /// ```css
    /// {
    ///     margin-left: 0.25rem; /* 4px */
    /// }
    /// ```
    Ml1,
    /// ```css
    /// {
    ///     margin: 0.375rem; /* 6px */
    /// }
    /// ```
    M1_5,
    /// ```css
    /// {
    ///     margin-left: 0.375rem; /* 6px */
    ///     margin-right: 0.375rem; /* 6px */
    /// }
    /// ```
    Mx1_5,
    /// ```css
    /// {
    ///     margin-top: 0.375rem; /* 6px */
    ///     margin-bottom: 0.375rem; /* 6px */
    /// }
    /// ```
    My1_5,
    /// ```css
    /// {
    ///     margin-inline-start: 0.375rem; /* 6px */
    /// }
    /// ```
    Ms1_5,
    /// ```css
    /// {
    ///     margin-inline-end: 0.375rem; /* 6px */
    /// }
    /// ```
    Me1_5,
    /// ```css
    /// {
    ///     margin-top: 0.375rem; /* 6px */
    /// }
    /// ```
    Mt1_5,
    /// ```css
    /// {
    ///     margin-right: 0.375rem; /* 6px */
    /// }
    /// ```
    Mr1_5,
    /// ```css
    /// {
    ///     margin-bottom: 0.375rem; /* 6px */
    /// }
    /// ```
    Mb1_5,
    /// ```css
    /// {
    ///     margin-left: 0.375rem; /* 6px */
    /// }
    /// ```
    Ml1_5,
    /// ```css
    /// {
    ///     margin: 0.5rem; /* 8px */
    /// }
    /// ```
    M2,
    /// ```css
    /// {
    ///     margin-left: 0.5rem; /* 8px */
    ///     margin-right: 0.5rem; /* 8px */
    /// }
    /// ```
    Mx2,
    /// ```css
    /// {
    ///     margin-top: 0.5rem; /* 8px */
    ///     margin-bottom: 0.5rem; /* 8px */
    /// }
    /// ```
    My2,
    /// ```css
    /// {
    ///     margin-inline-start: 0.5rem; /* 8px */
    /// }
    /// ```
    Ms2,
    /// ```css
    /// {
    ///     margin-inline-end: 0.5rem; /* 8px */
    /// }
    /// ```
    Me2,
    /// ```css
    /// {
    ///     margin-top: 0.5rem; /* 8px */
    /// }
    /// ```
    Mt2,
    /// ```css
    /// {
    ///     margin-right: 0.5rem; /* 8px */
    /// }
    /// ```
    Mr2,
    /// ```css
    /// {
    ///     margin-bottom: 0.5rem; /* 8px */
    /// }
    /// ```
    Mb2,
    /// ```css
    /// {
    ///     margin-left: 0.5rem; /* 8px */
    /// }
    /// ```
    Ml2,
    /// ```css
    /// {
    ///     margin: 0.625rem; /* 10px */
    /// }
    /// ```
    M2_5,
    /// ```css
    /// {
    ///     margin-left: 0.625rem; /* 10px */
    ///     margin-right: 0.625rem; /* 10px */
    /// }
    /// ```
    Mx2_5,
    /// ```css
    /// {
    ///     margin-top: 0.625rem; /* 10px */
    ///     margin-bottom: 0.625rem; /* 10px */
    /// }
    /// ```
    My2_5,
    /// ```css
    /// {
    ///     margin-inline-start: 0.625rem; /* 10px */
    /// }
    /// ```
    Ms2_5,
    /// ```css
    /// {
    ///     margin-inline-end: 0.625rem; /* 10px */
    /// }
    /// ```
    Me2_5,
    /// ```css
    /// {
    ///     margin-top: 0.625rem; /* 10px */
    /// }
    /// ```
    Mt2_5,
    /// ```css
    /// {
    ///     margin-right: 0.625rem; /* 10px */
    /// }
    /// ```
    Mr2_5,
    /// ```css
    /// {
    ///     margin-bottom: 0.625rem; /* 10px */
    /// }
    /// ```
    Mb2_5,
    /// ```css
    /// {
    ///     margin-left: 0.625rem; /* 10px */
    /// }
    /// ```
    Ml2_5,
    /// ```css
    /// {
    ///     margin: 0.75rem; /* 12px */
    /// }
    /// ```
    M3,
    /// ```css
    /// {
    ///     margin-left: 0.75rem; /* 12px */
    ///     margin-right: 0.75rem; /* 12px */
    /// }
    /// ```
    Mx3,
    /// ```css
    /// {
    ///     margin-top: 0.75rem; /* 12px */
    ///     margin-bottom: 0.75rem; /* 12px */
    /// }
    /// ```
    My3,
    /// ```css
    /// {
    ///     margin-inline-start: 0.75rem; /* 12px */
    /// }
    /// ```
    Ms3,
    /// ```css
    /// {
    ///     margin-inline-end: 0.75rem; /* 12px */
    /// }
    /// ```
    Me3,
    /// ```css
    /// {
    ///     margin-top: 0.75rem; /* 12px */
    /// }
    /// ```
    Mt3,
    /// ```css
    /// {
    ///     margin-right: 0.75rem; /* 12px */
    /// }
    /// ```
    Mr3,
    /// ```css
    /// {
    ///     margin-bottom: 0.75rem; /* 12px */
    /// }
    /// ```
    Mb3,
    /// ```css
    /// {
    ///     margin-left: 0.75rem; /* 12px */
    /// }
    /// ```
    Ml3,
    /// ```css
    /// {
    ///     margin: 0.875rem; /* 14px */
    /// }
    /// ```
    M3_5,
    /// ```css
    /// {
    ///     margin-left: 0.875rem; /* 14px */
    ///     margin-right: 0.875rem; /* 14px */
    /// }
    /// ```
    Mx3_5,
    /// ```css
    /// {
    ///     margin-top: 0.875rem; /* 14px */
    ///     margin-bottom: 0.875rem; /* 14px */
    /// }
    /// ```
    My3_5,
    /// ```css
    /// {
    ///     margin-inline-start: 0.875rem; /* 14px */
    /// }
    /// ```
    Ms3_5,
    /// ```css
    /// {
    ///     margin-inline-end: 0.875rem; /* 14px */
    /// }
    /// ```
    Me3_5,
    /// ```css
    /// {
    ///     margin-top: 0.875rem; /* 14px */
    /// }
    /// ```
    Mt3_5,
    /// ```css
    /// {
    ///     margin-right: 0.875rem; /* 14px */
    /// }
    /// ```
    Mr3_5,
    /// ```css
    /// {
    ///     margin-bottom: 0.875rem; /* 14px */
    /// }
    /// ```
    Mb3_5,
    /// ```css
    /// {
    ///     margin-left: 0.875rem; /* 14px */
    /// }
    /// ```
    Ml3_5,
    /// ```css
    /// {
    ///     margin: 1rem; /* 16px */
    /// }
    /// ```
    M4,
    /// ```css
    /// {
    ///     margin-left: 1rem; /* 16px */
    ///     margin-right: 1rem; /* 16px */
    /// }
    /// ```
    Mx4,
    /// ```css
    /// {
    ///     margin-top: 1rem; /* 16px */
    ///     margin-bottom: 1rem; /* 16px */
    /// }
    /// ```
    My4,
    /// ```css
    /// {
    ///     margin-inline-start: 1rem; /* 16px */
    /// }
    /// ```
    Ms4,
    /// ```css
    /// {
    ///     margin-inline-end: 1rem; /* 16px */
    /// }
    /// ```
    Me4,
    /// ```css
    /// {
    ///     margin-top: 1rem; /* 16px */
    /// }
    /// ```
    Mt4,
    /// ```css
    /// {
    ///     margin-right: 1rem; /* 16px */
    /// }
    /// ```
    Mr4,
    /// ```css
    /// {
    ///     margin-bottom: 1rem; /* 16px */
    /// }
    /// ```
    Mb4,
    /// ```css
    /// {
    ///     margin-left: 1rem; /* 16px */
    /// }
    /// ```
    Ml4,
    /// ```css
    /// {
    ///     margin: 1.25rem; /* 20px */
    /// }
    /// ```
    M5,
    /// ```css
    /// {
    ///     margin-left: 1.25rem; /* 20px */
    ///     margin-right: 1.25rem; /* 20px */
    /// }
    /// ```
    Mx5,
    /// ```css
    /// {
    ///     margin-top: 1.25rem; /* 20px */
    ///     margin-bottom: 1.25rem; /* 20px */
    /// }
    /// ```
    My5,
    /// ```css
    /// {
    ///     margin-inline-start: 1.25rem; /* 20px */
    /// }
    /// ```
    Ms5,
    /// ```css
    /// {
    ///     margin-inline-end: 1.25rem; /* 20px */
    /// }
    /// ```
    Me5,
    /// ```css
    /// {
    ///     margin-top: 1.25rem; /* 20px */
    /// }
    /// ```
    Mt5,
    /// ```css
    /// {
    ///     margin-right: 1.25rem; /* 20px */
    /// }
    /// ```
    Mr5,
    /// ```css
    /// {
    ///     margin-bottom: 1.25rem; /* 20px */
    /// }
    /// ```
    Mb5,
    /// ```css
    /// {
    ///     margin-left: 1.25rem; /* 20px */
    /// }
    /// ```
    Ml5,
    /// ```css
    /// {
    ///     margin: 1.5rem; /* 24px */
    /// }
    /// ```
    M6,
    /// ```css
    /// {
    ///     margin-left: 1.5rem; /* 24px */
    ///     margin-right: 1.5rem; /* 24px */
    /// }
    /// ```
    Mx6,
    /// ```css
    /// {
    ///     margin-top: 1.5rem; /* 24px */
    ///     margin-bottom: 1.5rem; /* 24px */
    /// }
    /// ```
    My6,
    /// ```css
    /// {
    ///     margin-inline-start: 1.5rem; /* 24px */
    /// }
    /// ```
    Ms6,
    /// ```css
    /// {
    ///     margin-inline-end: 1.5rem; /* 24px */
    /// }
    /// ```
    Me6,
    /// ```css
    /// {
    ///     margin-top: 1.5rem; /* 24px */
    /// }
    /// ```
    Mt6,
    /// ```css
    /// {
    ///     margin-right: 1.5rem; /* 24px */
    /// }
    /// ```
    Mr6,
    /// ```css
    /// {
    ///     margin-bottom: 1.5rem; /* 24px */
    /// }
    /// ```
    Mb6,
    /// ```css
    /// {
    ///     margin-left: 1.5rem; /* 24px */
    /// }
    /// ```
    Ml6,
    /// ```css
    /// {
    ///     margin: 1.75rem; /* 28px */
    /// }
    /// ```
    M7,
    /// ```css
    /// {
    ///     margin-left: 1.75rem; /* 28px */
    ///     margin-right: 1.75rem; /* 28px */
    /// }
    /// ```
    Mx7,
    /// ```css
    /// {
    ///     margin-top: 1.75rem; /* 28px */
    ///     margin-bottom: 1.75rem; /* 28px */
    /// }
    /// ```
    My7,
    /// ```css
    /// {
    ///     margin-inline-start: 1.75rem; /* 28px */
    /// }
    /// ```
    Ms7,
    /// ```css
    /// {
    ///     margin-inline-end: 1.75rem; /* 28px */
    /// }
    /// ```
    Me7,
    /// ```css
    /// {
    ///     margin-top: 1.75rem; /* 28px */
    /// }
    /// ```
    Mt7,
    /// ```css
    /// {
    ///     margin-right: 1.75rem; /* 28px */
    /// }
    /// ```
    Mr7,
    /// ```css
    /// {
    ///     margin-bottom: 1.75rem; /* 28px */
    /// }
    /// ```
    Mb7,
    /// ```css
    /// {
    ///     margin-left: 1.75rem; /* 28px */
    /// }
    /// ```
    Ml7,
    /// ```css
    /// {
    ///     margin: 2rem; /* 32px */
    /// }
    /// ```
    M8,
    /// ```css
    /// {
    ///     margin-left: 2rem; /* 32px */
    ///     margin-right: 2rem; /* 32px */
    /// }
    /// ```
    Mx8,
    /// ```css
    /// {
    ///     margin-top: 2rem; /* 32px */
    ///     margin-bottom: 2rem; /* 32px */
    /// }
    /// ```
    My8,
    /// ```css
    /// {
    ///     margin-inline-start: 2rem; /* 32px */
    /// }
    /// ```
    Ms8,
    /// ```css
    /// {
    ///     margin-inline-end: 2rem; /* 32px */
    /// }
    /// ```
    Me8,
    /// ```css
    /// {
    ///     margin-top: 2rem; /* 32px */
    /// }
    /// ```
    Mt8,
    /// ```css
    /// {
    ///     margin-right: 2rem; /* 32px */
    /// }
    /// ```
    Mr8,
    /// ```css
    /// {
    ///     margin-bottom: 2rem; /* 32px */
    /// }
    /// ```
    Mb8,
    /// ```css
    /// {
    ///     margin-left: 2rem; /* 32px */
    /// }
    /// ```
    Ml8,
    /// ```css
    /// {
    ///     margin: 2.25rem; /* 36px */
    /// }
    /// ```
    M9,
    /// ```css
    /// {
    ///     margin-left: 2.25rem; /* 36px */
    ///     margin-right: 2.25rem; /* 36px */
    /// }
    /// ```
    Mx9,
    /// ```css
    /// {
    ///     margin-top: 2.25rem; /* 36px */
    ///     margin-bottom: 2.25rem; /* 36px */
    /// }
    /// ```
    My9,
    /// ```css
    /// {
    ///     margin-inline-start: 2.25rem; /* 36px */
    /// }
    /// ```
    Ms9,
    /// ```css
    /// {
    ///     margin-inline-end: 2.25rem; /* 36px */
    /// }
    /// ```
    Me9,
    /// ```css
    /// {
    ///     margin-top: 2.25rem; /* 36px */
    /// }
    /// ```
    Mt9,
    /// ```css
    /// {
    ///     margin-right: 2.25rem; /* 36px */
    /// }
    /// ```
    Mr9,
    /// ```css
    /// {
    ///     margin-bottom: 2.25rem; /* 36px */
    /// }
    /// ```
    Mb9,
    /// ```css
    /// {
    ///     margin-left: 2.25rem; /* 36px */
    /// }
    /// ```
    Ml9,
    /// ```css
    /// {
    ///     margin: 2.5rem; /* 40px */
    /// }
    /// ```
    M10,
    /// ```css
    /// {
    ///     margin-left: 2.5rem; /* 40px */
    ///     margin-right: 2.5rem; /* 40px */
    /// }
    /// ```
    Mx10,
    /// ```css
    /// {
    ///     margin-top: 2.5rem; /* 40px */
    ///     margin-bottom: 2.5rem; /* 40px */
    /// }
    /// ```
    My10,
    /// ```css
    /// {
    ///     margin-inline-start: 2.5rem; /* 40px */
    /// }
    /// ```
    Ms10,
    /// ```css
    /// {
    ///     margin-inline-end: 2.5rem; /* 40px */
    /// }
    /// ```
    Me10,
    /// ```css
    /// {
    ///     margin-top: 2.5rem; /* 40px */
    /// }
    /// ```
    Mt10,
    /// ```css
    /// {
    ///     margin-right: 2.5rem; /* 40px */
    /// }
    /// ```
    Mr10,
    /// ```css
    /// {
    ///     margin-bottom: 2.5rem; /* 40px */
    /// }
    /// ```
    Mb10,
    /// ```css
    /// {
    ///     margin-left: 2.5rem; /* 40px */
    /// }
    /// ```
    Ml10,
    /// ```css
    /// {
    ///     margin: 2.75rem; /* 44px */
    /// }
    /// ```
    M11,
    /// ```css
    /// {
    ///     margin-left: 2.75rem; /* 44px */
    ///     margin-right: 2.75rem; /* 44px */
    /// }
    /// ```
    Mx11,
    /// ```css
    /// {
    ///     margin-top: 2.75rem; /* 44px */
    ///     margin-bottom: 2.75rem; /* 44px */
    /// }
    /// ```
    My11,
    /// ```css
    /// {
    ///     margin-inline-start: 2.75rem; /* 44px */
    /// }
    /// ```
    Ms11,
    /// ```css
    /// {
    ///     margin-inline-end: 2.75rem; /* 44px */
    /// }
    /// ```
    Me11,
    /// ```css
    /// {
    ///     margin-top: 2.75rem; /* 44px */
    /// }
    /// ```
    Mt11,
    /// ```css
    /// {
    ///     margin-right: 2.75rem; /* 44px */
    /// }
    /// ```
    Mr11,
    /// ```css
    /// {
    ///     margin-bottom: 2.75rem; /* 44px */
    /// }
    /// ```
    Mb11,
    /// ```css
    /// {
    ///     margin-left: 2.75rem; /* 44px */
    /// }
    /// ```
    Ml11,
    /// ```css
    /// {
    ///     margin: 3rem; /* 48px */
    /// }
    /// ```
    M12,
    /// ```css
    /// {
    ///     margin-left: 3rem; /* 48px */
    ///     margin-right: 3rem; /* 48px */
    /// }
    /// ```
    Mx12,
    /// ```css
    /// {
    ///     margin-top: 3rem; /* 48px */
    ///     margin-bottom: 3rem; /* 48px */
    /// }
    /// ```
    My12,
    /// ```css
    /// {
    ///     margin-inline-start: 3rem; /* 48px */
    /// }
    /// ```
    Ms12,
    /// ```css
    /// {
    ///     margin-inline-end: 3rem; /* 48px */
    /// }
    /// ```
    Me12,
    /// ```css
    /// {
    ///     margin-top: 3rem; /* 48px */
    /// }
    /// ```
    Mt12,
    /// ```css
    /// {
    ///     margin-right: 3rem; /* 48px */
    /// }
    /// ```
    Mr12,
    /// ```css
    /// {
    ///     margin-bottom: 3rem; /* 48px */
    /// }
    /// ```
    Mb12,
    /// ```css
    /// {
    ///     margin-left: 3rem; /* 48px */
    /// }
    /// ```
    Ml12,
    /// ```css
    /// {
    ///     margin: 3.5rem; /* 56px */
    /// }
    /// ```
    M14,
    /// ```css
    /// {
    ///     margin-left: 3.5rem; /* 56px */
    ///     margin-right: 3.5rem; /* 56px */
    /// }
    /// ```
    Mx14,
    /// ```css
    /// {
    ///     margin-top: 3.5rem; /* 56px */
    ///     margin-bottom: 3.5rem; /* 56px */
    /// }
    /// ```
    My14,
    /// ```css
    /// {
    ///     margin-inline-start: 3.5rem; /* 56px */
    /// }
    /// ```
    Ms14,
    /// ```css
    /// {
    ///     margin-inline-end: 3.5rem; /* 56px */
    /// }
    /// ```
    Me14,
    /// ```css
    /// {
    ///     margin-top: 3.5rem; /* 56px */
    /// }
    /// ```
    Mt14,
    /// ```css
    /// {
    ///     margin-right: 3.5rem; /* 56px */
    /// }
    /// ```
    Mr14,
    /// ```css
    /// {
    ///     margin-bottom: 3.5rem; /* 56px */
    /// }
    /// ```
    Mb14,
    /// ```css
    /// {
    ///     margin-left: 3.5rem; /* 56px */
    /// }
    /// ```
    Ml14,
    /// ```css
    /// {
    ///     margin: 4rem; /* 64px */
    /// }
    /// ```
    M16,
    /// ```css
    /// {
    ///     margin-left: 4rem; /* 64px */
    ///     margin-right: 4rem; /* 64px */
    /// }
    /// ```
    Mx16,
    /// ```css
    /// {
    ///     margin-top: 4rem; /* 64px */
    ///     margin-bottom: 4rem; /* 64px */
    /// }
    /// ```
    My16,
    /// ```css
    /// {
    ///     margin-inline-start: 4rem; /* 64px */
    /// }
    /// ```
    Ms16,
    /// ```css
    /// {
    ///     margin-inline-end: 4rem; /* 64px */
    /// }
    /// ```
    Me16,
    /// ```css
    /// {
    ///     margin-top: 4rem; /* 64px */
    /// }
    /// ```
    Mt16,
    /// ```css
    /// {
    ///     margin-right: 4rem; /* 64px */
    /// }
    /// ```
    Mr16,
    /// ```css
    /// {
    ///     margin-bottom: 4rem; /* 64px */
    /// }
    /// ```
    Mb16,
    /// ```css
    /// {
    ///     margin-left: 4rem; /* 64px */
    /// }
    /// ```
    Ml16,
    /// ```css
    /// {
    ///     margin: 5rem; /* 80px */
    /// }
    /// ```
    M20,
    /// ```css
    /// {
    ///     margin-left: 5rem; /* 80px */
    ///     margin-right: 5rem; /* 80px */
    /// }
    /// ```
    Mx20,
    /// ```css
    /// {
    ///     margin-top: 5rem; /* 80px */
    ///     margin-bottom: 5rem; /* 80px */
    /// }
    /// ```
    My20,
    /// ```css
    /// {
    ///     margin-inline-start: 5rem; /* 80px */
    /// }
    /// ```
    Ms20,
    /// ```css
    /// {
    ///     margin-inline-end: 5rem; /* 80px */
    /// }
    /// ```
    Me20,
    /// ```css
    /// {
    ///     margin-top: 5rem; /* 80px */
    /// }
    /// ```
    Mt20,
    /// ```css
    /// {
    ///     margin-right: 5rem; /* 80px */
    /// }
    /// ```
    Mr20,
    /// ```css
    /// {
    ///     margin-bottom: 5rem; /* 80px */
    /// }
    /// ```
    Mb20,
    /// ```css
    /// {
    ///     margin-left: 5rem; /* 80px */
    /// }
    /// ```
    Ml20,
    /// ```css
    /// {
    ///     margin: 6rem; /* 96px */
    /// }
    /// ```
    M24,
    /// ```css
    /// {
    ///     margin-left: 6rem; /* 96px */
    ///     margin-right: 6rem; /* 96px */
    /// }
    /// ```
    Mx24,
    /// ```css
    /// {
    ///     margin-top: 6rem; /* 96px */
    ///     margin-bottom: 6rem; /* 96px */
    /// }
    /// ```
    My24,
    /// ```css
    /// {
    ///     margin-inline-start: 6rem; /* 96px */
    /// }
    /// ```
    Ms24,
    /// ```css
    /// {
    ///     margin-inline-end: 6rem; /* 96px */
    /// }
    /// ```
    Me24,
    /// ```css
    /// {
    ///     margin-top: 6rem; /* 96px */
    /// }
    /// ```
    Mt24,
    /// ```css
    /// {
    ///     margin-right: 6rem; /* 96px */
    /// }
    /// ```
    Mr24,
    /// ```css
    /// {
    ///     margin-bottom: 6rem; /* 96px */
    /// }
    /// ```
    Mb24,
    /// ```css
    /// {
    ///     margin-left: 6rem; /* 96px */
    /// }
    /// ```
    Ml24,
    /// ```css
    /// {
    ///     margin: 7rem; /* 112px */
    /// }
    /// ```
    M28,
    /// ```css
    /// {
    ///     margin-left: 7rem; /* 112px */
    ///     margin-right: 7rem; /* 112px */
    /// }
    /// ```
    Mx28,
    /// ```css
    /// {
    ///     margin-top: 7rem; /* 112px */
    ///     margin-bottom: 7rem; /* 112px */
    /// }
    /// ```
    My28,
    /// ```css
    /// {
    ///     margin-inline-start: 7rem; /* 112px */
    /// }
    /// ```
    Ms28,
    /// ```css
    /// {
    ///     margin-inline-end: 7rem; /* 112px */
    /// }
    /// ```
    Me28,
    /// ```css
    /// {
    ///     margin-top: 7rem; /* 112px */
    /// }
    /// ```
    Mt28,
    /// ```css
    /// {
    ///     margin-right: 7rem; /* 112px */
    /// }
    /// ```
    Mr28,
    /// ```css
    /// {
    ///     margin-bottom: 7rem; /* 112px */
    /// }
    /// ```
    Mb28,
    /// ```css
    /// {
    ///     margin-left: 7rem; /* 112px */
    /// }
    /// ```
    Ml28,
    /// ```css
    /// {
    ///     margin: 8rem; /* 128px */
    /// }
    /// ```
    M32,
    /// ```css
    /// {
    ///     margin-left: 8rem; /* 128px */
    ///     margin-right: 8rem; /* 128px */
    /// }
    /// ```
    Mx32,
    /// ```css
    /// {
    ///     margin-top: 8rem; /* 128px */
    ///     margin-bottom: 8rem; /* 128px */
    /// }
    /// ```
    My32,
    /// ```css
    /// {
    ///     margin-inline-start: 8rem; /* 128px */
    /// }
    /// ```
    Ms32,
    /// ```css
    /// {
    ///     margin-inline-end: 8rem; /* 128px */
    /// }
    /// ```
    Me32,
    /// ```css
    /// {
    ///     margin-top: 8rem; /* 128px */
    /// }
    /// ```
    Mt32,
    /// ```css
    /// {
    ///     margin-right: 8rem; /* 128px */
    /// }
    /// ```
    Mr32,
    /// ```css
    /// {
    ///     margin-bottom: 8rem; /* 128px */
    /// }
    /// ```
    Mb32,
    /// ```css
    /// {
    ///     margin-left: 8rem; /* 128px */
    /// }
    /// ```
    Ml32,
    /// ```css
    /// {
    ///     margin: 9rem; /* 144px */
    /// }
    /// ```
    M36,
    /// ```css
    /// {
    ///     margin-left: 9rem; /* 144px */
    ///     margin-right: 9rem; /* 144px */
    /// }
    /// ```
    Mx36,
    /// ```css
    /// {
    ///     margin-top: 9rem; /* 144px */
    ///     margin-bottom: 9rem; /* 144px */
    /// }
    /// ```
    My36,
    /// ```css
    /// {
    ///     margin-inline-start: 9rem; /* 144px */
    /// }
    /// ```
    Ms36,
    /// ```css
    /// {
    ///     margin-inline-end: 9rem; /* 144px */
    /// }
    /// ```
    Me36,
    /// ```css
    /// {
    ///     margin-top: 9rem; /* 144px */
    /// }
    /// ```
    Mt36,
    /// ```css
    /// {
    ///     margin-right: 9rem; /* 144px */
    /// }
    /// ```
    Mr36,
    /// ```css
    /// {
    ///     margin-bottom: 9rem; /* 144px */
    /// }
    /// ```
    Mb36,
    /// ```css
    /// {
    ///     margin-left: 9rem; /* 144px */
    /// }
    /// ```
    Ml36,
    /// ```css
    /// {
    ///     margin: 10rem; /* 160px */
    /// }
    /// ```
    M40,
    /// ```css
    /// {
    ///     margin-left: 10rem; /* 160px */
    ///     margin-right: 10rem; /* 160px */
    /// }
    /// ```
    Mx40,
    /// ```css
    /// {
    ///     margin-top: 10rem; /* 160px */
    ///     margin-bottom: 10rem; /* 160px */
    /// }
    /// ```
    My40,
    /// ```css
    /// {
    ///     margin-inline-start: 10rem; /* 160px */
    /// }
    /// ```
    Ms40,
    /// ```css
    /// {
    ///     margin-inline-end: 10rem; /* 160px */
    /// }
    /// ```
    Me40,
    /// ```css
    /// {
    ///     margin-top: 10rem; /* 160px */
    /// }
    /// ```
    Mt40,
    /// ```css
    /// {
    ///     margin-right: 10rem; /* 160px */
    /// }
    /// ```
    Mr40,
    /// ```css
    /// {
    ///     margin-bottom: 10rem; /* 160px */
    /// }
    /// ```
    Mb40,
    /// ```css
    /// {
    ///     margin-left: 10rem; /* 160px */
    /// }
    /// ```
    Ml40,
    /// ```css
    /// {
    ///     margin: 11rem; /* 176px */
    /// }
    /// ```
    M44,
    /// ```css
    /// {
    ///     margin-left: 11rem; /* 176px */
    ///     margin-right: 11rem; /* 176px */
    /// }
    /// ```
    Mx44,
    /// ```css
    /// {
    ///     margin-top: 11rem; /* 176px */
    ///     margin-bottom: 11rem; /* 176px */
    /// }
    /// ```
    My44,
    /// ```css
    /// {
    ///     margin-inline-start: 11rem; /* 176px */
    /// }
    /// ```
    Ms44,
    /// ```css
    /// {
    ///     margin-inline-end: 11rem; /* 176px */
    /// }
    /// ```
    Me44,
    /// ```css
    /// {
    ///     margin-top: 11rem; /* 176px */
    /// }
    /// ```
    Mt44,
    /// ```css
    /// {
    ///     margin-right: 11rem; /* 176px */
    /// }
    /// ```
    Mr44,
    /// ```css
    /// {
    ///     margin-bottom: 11rem; /* 176px */
    /// }
    /// ```
    Mb44,
    /// ```css
    /// {
    ///     margin-left: 11rem; /* 176px */
    /// }
    /// ```
    Ml44,
    /// ```css
    /// {
    ///     margin: 12rem; /* 192px */
    /// }
    /// ```
    M48,
    /// ```css
    /// {
    ///     margin-left: 12rem; /* 192px */
    ///     margin-right: 12rem; /* 192px */
    /// }
    /// ```
    Mx48,
    /// ```css
    /// {
    ///     margin-top: 12rem; /* 192px */
    ///     margin-bottom: 12rem; /* 192px */
    /// }
    /// ```
    My48,
    /// ```css
    /// {
    ///     margin-inline-start: 12rem; /* 192px */
    /// }
    /// ```
    Ms48,
    /// ```css
    /// {
    ///     margin-inline-end: 12rem; /* 192px */
    /// }
    /// ```
    Me48,
    /// ```css
    /// {
    ///     margin-top: 12rem; /* 192px */
    /// }
    /// ```
    Mt48,
    /// ```css
    /// {
    ///     margin-right: 12rem; /* 192px */
    /// }
    /// ```
    Mr48,
    /// ```css
    /// {
    ///     margin-bottom: 12rem; /* 192px */
    /// }
    /// ```
    Mb48,
    /// ```css
    /// {
    ///     margin-left: 12rem; /* 192px */
    /// }
    /// ```
    Ml48,
    /// ```css
    /// {
    ///     margin: 13rem; /* 208px */
    /// }
    /// ```
    M52,
    /// ```css
    /// {
    ///     margin-left: 13rem; /* 208px */
    ///     margin-right: 13rem; /* 208px */
    /// }
    /// ```
    Mx52,
    /// ```css
    /// {
    ///     margin-top: 13rem; /* 208px */
    ///     margin-bottom: 13rem; /* 208px */
    /// }
    /// ```
    My52,
    /// ```css
    /// {
    ///     margin-inline-start: 13rem; /* 208px */
    /// }
    /// ```
    Ms52,
    /// ```css
    /// {
    ///     margin-inline-end: 13rem; /* 208px */
    /// }
    /// ```
    Me52,
    /// ```css
    /// {
    ///     margin-top: 13rem; /* 208px */
    /// }
    /// ```
    Mt52,
    /// ```css
    /// {
    ///     margin-right: 13rem; /* 208px */
    /// }
    /// ```
    Mr52,
    /// ```css
    /// {
    ///     margin-bottom: 13rem; /* 208px */
    /// }
    /// ```
    Mb52,
    /// ```css
    /// {
    ///     margin-left: 13rem; /* 208px */
    /// }
    /// ```
    Ml52,
    /// ```css
    /// {
    ///     margin: 14rem; /* 224px */
    /// }
    /// ```
    M56,
    /// ```css
    /// {
    ///     margin-left: 14rem; /* 224px */
    ///     margin-right: 14rem; /* 224px */
    /// }
    /// ```
    Mx56,
    /// ```css
    /// {
    ///     margin-top: 14rem; /* 224px */
    ///     margin-bottom: 14rem; /* 224px */
    /// }
    /// ```
    My56,
    /// ```css
    /// {
    ///     margin-inline-start: 14rem; /* 224px */
    /// }
    /// ```
    Ms56,
    /// ```css
    /// {
    ///     margin-inline-end: 14rem; /* 224px */
    /// }
    /// ```
    Me56,
    /// ```css
    /// {
    ///     margin-top: 14rem; /* 224px */
    /// }
    /// ```
    Mt56,
    /// ```css
    /// {
    ///     margin-right: 14rem; /* 224px */
    /// }
    /// ```
    Mr56,
    /// ```css
    /// {
    ///     margin-bottom: 14rem; /* 224px */
    /// }
    /// ```
    Mb56,
    /// ```css
    /// {
    ///     margin-left: 14rem; /* 224px */
    /// }
    /// ```
    Ml56,
    /// ```css
    /// {
    ///     margin: 15rem; /* 240px */
    /// }
    /// ```
    M60,
    /// ```css
    /// {
    ///     margin-left: 15rem; /* 240px */
    ///     margin-right: 15rem; /* 240px */
    /// }
    /// ```
    Mx60,
    /// ```css
    /// {
    ///     margin-top: 15rem; /* 240px */
    ///     margin-bottom: 15rem; /* 240px */
    /// }
    /// ```
    My60,
    /// ```css
    /// {
    ///     margin-inline-start: 15rem; /* 240px */
    /// }
    /// ```
    Ms60,
    /// ```css
    /// {
    ///     margin-inline-end: 15rem; /* 240px */
    /// }
    /// ```
    Me60,
    /// ```css
    /// {
    ///     margin-top: 15rem; /* 240px */
    /// }
    /// ```
    Mt60,
    /// ```css
    /// {
    ///     margin-right: 15rem; /* 240px */
    /// }
    /// ```
    Mr60,
    /// ```css
    /// {
    ///     margin-bottom: 15rem; /* 240px */
    /// }
    /// ```
    Mb60,
    /// ```css
    /// {
    ///     margin-left: 15rem; /* 240px */
    /// }
    /// ```
    Ml60,
    /// ```css
    /// {
    ///     margin: 16rem; /* 256px */
    /// }
    /// ```
    M64,
    /// ```css
    /// {
    ///     margin-left: 16rem; /* 256px */
    ///     margin-right: 16rem; /* 256px */
    /// }
    /// ```
    Mx64,
    /// ```css
    /// {
    ///     margin-top: 16rem; /* 256px */
    ///     margin-bottom: 16rem; /* 256px */
    /// }
    /// ```
    My64,
    /// ```css
    /// {
    ///     margin-inline-start: 16rem; /* 256px */
    /// }
    /// ```
    Ms64,
    /// ```css
    /// {
    ///     margin-inline-end: 16rem; /* 256px */
    /// }
    /// ```
    Me64,
    /// ```css
    /// {
    ///     margin-top: 16rem; /* 256px */
    /// }
    /// ```
    Mt64,
    /// ```css
    /// {
    ///     margin-right: 16rem; /* 256px */
    /// }
    /// ```
    Mr64,
    /// ```css
    /// {
    ///     margin-bottom: 16rem; /* 256px */
    /// }
    /// ```
    Mb64,
    /// ```css
    /// {
    ///     margin-left: 16rem; /* 256px */
    /// }
    /// ```
    Ml64,
    /// ```css
    /// {
    ///     margin: 18rem; /* 288px */
    /// }
    /// ```
    M72,
    /// ```css
    /// {
    ///     margin-left: 18rem; /* 288px */
    ///     margin-right: 18rem; /* 288px */
    /// }
    /// ```
    Mx72,
    /// ```css
    /// {
    ///     margin-top: 18rem; /* 288px */
    ///     margin-bottom: 18rem; /* 288px */
    /// }
    /// ```
    My72,
    /// ```css
    /// {
    ///     margin-inline-start: 18rem; /* 288px */
    /// }
    /// ```
    Ms72,
    /// ```css
    /// {
    ///     margin-inline-end: 18rem; /* 288px */
    /// }
    /// ```
    Me72,
    /// ```css
    /// {
    ///     margin-top: 18rem; /* 288px */
    /// }
    /// ```
    Mt72,
    /// ```css
    /// {
    ///     margin-right: 18rem; /* 288px */
    /// }
    /// ```
    Mr72,
    /// ```css
    /// {
    ///     margin-bottom: 18rem; /* 288px */
    /// }
    /// ```
    Mb72,
    /// ```css
    /// {
    ///     margin-left: 18rem; /* 288px */
    /// }
    /// ```
    Ml72,
    /// ```css
    /// {
    ///     margin: 20rem; /* 320px */
    /// }
    /// ```
    M80,
    /// ```css
    /// {
    ///     margin-left: 20rem; /* 320px */
    ///     margin-right: 20rem; /* 320px */
    /// }
    /// ```
    Mx80,
    /// ```css
    /// {
    ///     margin-top: 20rem; /* 320px */
    ///     margin-bottom: 20rem; /* 320px */
    /// }
    /// ```
    My80,
    /// ```css
    /// {
    ///     margin-inline-start: 20rem; /* 320px */
    /// }
    /// ```
    Ms80,
    /// ```css
    /// {
    ///     margin-inline-end: 20rem; /* 320px */
    /// }
    /// ```
    Me80,
    /// ```css
    /// {
    ///     margin-top: 20rem; /* 320px */
    /// }
    /// ```
    Mt80,
    /// ```css
    /// {
    ///     margin-right: 20rem; /* 320px */
    /// }
    /// ```
    Mr80,
    /// ```css
    /// {
    ///     margin-bottom: 20rem; /* 320px */
    /// }
    /// ```
    Mb80,
    /// ```css
    /// {
    ///     margin-left: 20rem; /* 320px */
    /// }
    /// ```
    Ml80,
    /// ```css
    /// {
    ///     margin: 24rem; /* 384px */
    /// }
    /// ```
    M96,
    /// ```css
    /// {
    ///     margin-left: 24rem; /* 384px */
    ///     margin-right: 24rem; /* 384px */
    /// }
    /// ```
    Mx96,
    /// ```css
    /// {
    ///     margin-top: 24rem; /* 384px */
    ///     margin-bottom: 24rem; /* 384px */
    /// }
    /// ```
    My96,
    /// ```css
    /// {
    ///     margin-inline-start: 24rem; /* 384px */
    /// }
    /// ```
    Ms96,
    /// ```css
    /// {
    ///     margin-inline-end: 24rem; /* 384px */
    /// }
    /// ```
    Me96,
    /// ```css
    /// {
    ///     margin-top: 24rem; /* 384px */
    /// }
    /// ```
    Mt96,
    /// ```css
    /// {
    ///     margin-right: 24rem; /* 384px */
    /// }
    /// ```
    Mr96,
    /// ```css
    /// {
    ///     margin-bottom: 24rem; /* 384px */
    /// }
    /// ```
    Mb96,
    /// ```css
    /// {
    ///     margin-left: 24rem; /* 384px */
    /// }
    /// ```
    Ml96,
    /// ```css
    /// {
    ///     margin: auto;
    /// }
    /// ```
    MAuto,
    /// ```css
    /// {
    ///     margin-left: auto;
    ///     margin-right: auto;
    /// }
    /// ```
    MxAuto,
    /// ```css
    /// {
    ///     margin-top: auto;
    ///     margin-bottom: auto;
    /// }
    /// ```
    MyAuto,
    /// ```css
    /// {
    ///     margin-inline-start: auto;
    /// }
    /// ```
    MsAuto,
    /// ```css
    /// {
    ///     margin-inline-end: auto;
    /// }
    /// ```
    MeAuto,
    /// ```css
    /// {
    ///     margin-top: auto;
    /// }
    /// ```
    MtAuto,
    /// ```css
    /// {
    ///     margin-right: auto;
    /// }
    /// ```
    MrAuto,
    /// ```css
    /// {
    ///     margin-bottom: auto;
    /// }
    /// ```
    MbAuto,
    /// ```css
    /// {
    ///     margin-left: auto;
    /// }
    /// ```
    MlAuto,
}

/// Utilities for controlling the space between child elements.
/// 
/// <https://tailwindcss.com/docs/space>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "space", replace(from = "_", to = "."))]
pub enum SpaceBetween {
    /// ```css
    /// {
    ///     margin-left: 0px;
    /// }
    /// ```
    X0,
    /// ```css
    /// {
    ///     margin-top: 0px;
    /// }
    /// ```
    Y0,
    /// ```css
    /// {
    ///     margin-left: 0.125rem; /* 2px */
    /// }
    /// ```
    X0_5,
    /// ```css
    /// {
    ///     margin-top: 0.125rem; /* 2px */
    /// }
    /// ```
    Y0_5,
    /// ```css
    /// {
    ///     margin-left: 0.25rem; /* 4px */
    /// }
    /// ```
    X1,
    /// ```css
    /// {
    ///     margin-top: 0.25rem; /* 4px */
    /// }
    /// ```
    Y1,
    /// ```css
    /// {
    ///     margin-left: 0.375rem; /* 6px */
    /// }
    /// ```
    X1_5,
    /// ```css
    /// {
    ///     margin-top: 0.375rem; /* 6px */
    /// }
    /// ```
    Y1_5,
    /// ```css
    /// {
    ///     margin-left: 0.5rem; /* 8px */
    /// }
    /// ```
    X2,
    /// ```css
    /// {
    ///     margin-top: 0.5rem; /* 8px */
    /// }
    /// ```
    Y2,
    /// ```css
    /// {
    ///     margin-left: 0.625rem; /* 10px */
    /// }
    /// ```
    X2_5,
    /// ```css
    /// {
    ///     margin-top: 0.625rem; /* 10px */
    /// }
    /// ```
    Y2_5,
    /// ```css
    /// {
    ///     margin-left: 0.75rem; /* 12px */
    /// }
    /// ```
    X3,
    /// ```css
    /// {
    ///     margin-top: 0.75rem; /* 12px */
    /// }
    /// ```
    Y3,
    /// ```css
    /// {
    ///     margin-left: 0.875rem; /* 14px */
    /// }
    /// ```
    X3_5,
    /// ```css
    /// {
    ///     margin-top: 0.875rem; /* 14px */
    /// }
    /// ```
    Y3_5,
    /// ```css
    /// {
    ///     margin-left: 1rem; /* 16px */
    /// }
    /// ```
    X4,
    /// ```css
    /// {
    ///     margin-top: 1rem; /* 16px */
    /// }
    /// ```
    Y4,
    /// ```css
    /// {
    ///     margin-left: 1.25rem; /* 20px */
    /// }
    /// ```
    X5,
    /// ```css
    /// {
    ///     margin-top: 1.25rem; /* 20px */
    /// }
    /// ```
    Y5,
    /// ```css
    /// {
    ///     margin-left: 1.5rem; /* 24px */
    /// }
    /// ```
    X6,
    /// ```css
    /// {
    ///     margin-top: 1.5rem; /* 24px */
    /// }
    /// ```
    Y6,
    /// ```css
    /// {
    ///     margin-left: 1.75rem; /* 28px */
    /// }
    /// ```
    X7,
    /// ```css
    /// {
    ///     margin-top: 1.75rem; /* 28px */
    /// }
    /// ```
    Y7,
    /// ```css
    /// {
    ///     margin-left: 2rem; /* 32px */
    /// }
    /// ```
    X8,
    /// ```css
    /// {
    ///     margin-top: 2rem; /* 32px */
    /// }
    /// ```
    Y8,
    /// ```css
    /// {
    ///     margin-left: 2.25rem; /* 36px */
    /// }
    /// ```
    X9,
    /// ```css
    /// {
    ///     margin-top: 2.25rem; /* 36px */
    /// }
    /// ```
    Y9,
    /// ```css
    /// {
    ///     margin-left: 2.5rem; /* 40px */
    /// }
    /// ```
    X10,
    /// ```css
    /// {
    ///     margin-top: 2.5rem; /* 40px */
    /// }
    /// ```
    Y10,
    /// ```css
    /// {
    ///     margin-left: 2.75rem; /* 44px */
    /// }
    /// ```
    X11,
    /// ```css
    /// {
    ///     margin-top: 2.75rem; /* 44px */
    /// }
    /// ```
    Y11,
    /// ```css
    /// {
    ///     margin-left: 3rem; /* 48px */
    /// }
    /// ```
    X12,
    /// ```css
    /// {
    ///     margin-top: 3rem; /* 48px */
    /// }
    /// ```
    Y12,
    /// ```css
    /// {
    ///     margin-left: 3.5rem; /* 56px */
    /// }
    /// ```
    X14,
    /// ```css
    /// {
    ///     margin-top: 3.5rem; /* 56px */
    /// }
    /// ```
    Y14,
    /// ```css
    /// {
    ///     margin-left: 4rem; /* 64px */
    /// }
    /// ```
    X16,
    /// ```css
    /// {
    ///     margin-top: 4rem; /* 64px */
    /// }
    /// ```
    Y16,
    /// ```css
    /// {
    ///     margin-left: 5rem; /* 80px */
    /// }
    /// ```
    X20,
    /// ```css
    /// {
    ///     margin-top: 5rem; /* 80px */
    /// }
    /// ```
    Y20,
    /// ```css
    /// {
    ///     margin-left: 6rem; /* 96px */
    /// }
    /// ```
    X24,
    /// ```css
    /// {
    ///     margin-top: 6rem; /* 96px */
    /// }
    /// ```
    Y24,
    /// ```css
    /// {
    ///     margin-left: 7rem; /* 112px */
    /// }
    /// ```
    X28,
    /// ```css
    /// {
    ///     margin-top: 7rem; /* 112px */
    /// }
    /// ```
    Y28,
    /// ```css
    /// {
    ///     margin-left: 8rem; /* 128px */
    /// }
    /// ```
    X32,
    /// ```css
    /// {
    ///     margin-top: 8rem; /* 128px */
    /// }
    /// ```
    Y32,
    /// ```css
    /// {
    ///     margin-left: 9rem; /* 144px */
    /// }
    /// ```
    X36,
    /// ```css
    /// {
    ///     margin-top: 9rem; /* 144px */
    /// }
    /// ```
    Y36,
    /// ```css
    /// {
    ///     margin-left: 10rem; /* 160px */
    /// }
    /// ```
    X40,
    /// ```css
    /// {
    ///     margin-top: 10rem; /* 160px */
    /// }
    /// ```
    Y40,
    /// ```css
    /// {
    ///     margin-left: 11rem; /* 176px */
    /// }
    /// ```
    X44,
    /// ```css
    /// {
    ///     margin-top: 11rem; /* 176px */
    /// }
    /// ```
    Y44,
    /// ```css
    /// {
    ///     margin-left: 12rem; /* 192px */
    /// }
    /// ```
    X48,
    /// ```css
    /// {
    ///     margin-top: 12rem; /* 192px */
    /// }
    /// ```
    Y48,
    /// ```css
    /// {
    ///     margin-left: 13rem; /* 208px */
    /// }
    /// ```
    X52,
    /// ```css
    /// {
    ///     margin-top: 13rem; /* 208px */
    /// }
    /// ```
    Y52,
    /// ```css
    /// {
    ///     margin-left: 14rem; /* 224px */
    /// }
    /// ```
    X56,
    /// ```css
    /// {
    ///     margin-top: 14rem; /* 224px */
    /// }
    /// ```
    Y56,
    /// ```css
    /// {
    ///     margin-left: 15rem; /* 240px */
    /// }
    /// ```
    X60,
    /// ```css
    /// {
    ///     margin-top: 15rem; /* 240px */
    /// }
    /// ```
    Y60,
    /// ```css
    /// {
    ///     margin-left: 16rem; /* 256px */
    /// }
    /// ```
    X64,
    /// ```css
    /// {
    ///     margin-top: 16rem; /* 256px */
    /// }
    /// ```
    Y64,
    /// ```css
    /// {
    ///     margin-left: 18rem; /* 288px */
    /// }
    /// ```
    X72,
    /// ```css
    /// {
    ///     margin-top: 18rem; /* 288px */
    /// }
    /// ```
    Y72,
    /// ```css
    /// {
    ///     margin-left: 20rem; /* 320px */
    /// }
    /// ```
    X80,
    /// ```css
    /// {
    ///     margin-top: 20rem; /* 320px */
    /// }
    /// ```
    Y80,
    /// ```css
    /// {
    ///     margin-left: 24rem; /* 384px */
    /// }
    /// ```
    X96,
    /// ```css
    /// {
    ///     margin-top: 24rem; /* 384px */
    /// }
    /// ```
    Y96,
    /// ```css
    /// {
    ///     margin-left: 1px;
    /// }
    /// ```
    XPx,
    /// ```css
    /// {
    ///     margin-top: 1px;
    /// }
    /// ```
    YPx,
    /// ```css
    /// {
    ///     --tw-space-y-reverse: 1;
    /// }
    /// ```
    YReverse,
    /// ```css
    /// {
    ///     --tw-space-x-reverse: 1;
    /// }
    /// ```
    XReverse,
}
