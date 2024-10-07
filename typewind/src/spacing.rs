use typewind_macros::{Display, Parse};

tailwind_types!(Padding, Margin, SpaceBetween);

/// Utilities for controlling an element's padding.
/// 
/// <https://tailwindcss.com/docs/padding>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(replace(from = "_", to = "."))]
pub enum Padding {
    /// `padding: 0px;`
    P0,
    /// `padding-left: 0px;`
    /// 
    /// `padding-right: 0px;`
    Px0,
    /// `padding-top: 0px;`
    /// 
    /// `padding-bottom: 0px;`
    Py0,
    /// `padding-inline-start: 0px;`
    Ps0,
    /// `padding-inline-end: 0px;`
    Pe0,
    /// `padding-top: 0px;`
    Pt0,
    /// `padding-right: 0px;`
    Pr0,
    /// `padding-bottom: 0px;`
    Pb0,
    /// `padding-left: 0px;`
    Pl0,
    /// `padding: 1px;`
    PPx,
    /// `padding-left: 1px;`
    /// 
    /// `padding-right: 1px;`
    PxPx,
    /// `padding-top: 1px;`
    /// 
    /// `padding-bottom: 1px;`
    PyPx,
    /// `padding-inline-start: 1px;`
    PsPx,
    /// `padding-inline-end: 1px;`
    PePx,
    /// `padding-top: 1px;`
    PtPx,
    /// `padding-right: 1px;`
    PrPx,
    /// `padding-bottom: 1px;`
    PbPx,
    /// `padding-left: 1px;`
    PlPx,
    /// `padding: 0.125rem; /* 2px */`
    P0_5,
    /// `padding-left: 0.125rem; /* 2px */`
    /// 
    /// `padding-right: 0.125rem; /* 2px */`
    Px0_5,
    /// `padding-top: 0.125rem; /* 2px */`
    /// 
    /// `padding-bottom: 0.125rem; /* 2px */`
    Py0_5,
    /// `padding-inline-start: 0.125rem; /* 2px */`
    Ps0_5,
    /// `padding-inline-end: 0.125rem; /* 2px */`
    Pe0_5,
    /// `padding-top: 0.125rem; /* 2px */`
    Pt0_5,
    /// `padding-right: 0.125rem; /* 2px */`
    Pr0_5,
    /// `padding-bottom: 0.125rem; /* 2px */`
    Pb0_5,
    /// `padding-left: 0.125rem; /* 2px */`
    Pl0_5,
    /// `padding: 0.25rem; /* 4px */`
    P1,
    /// `padding-left: 0.25rem; /* 4px */`
    /// 
    /// `padding-right: 0.25rem; /* 4px */`
    Px1,
    /// `padding-top: 0.25rem; /* 4px */`
    /// 
    /// `padding-bottom: 0.25rem; /* 4px */`
    Py1,
    /// `padding-inline-start: 0.25rem; /* 4px */`
    Ps1,
    /// `padding-inline-end: 0.25rem; /* 4px */`
    Pe1,
    /// `padding-top: 0.25rem; /* 4px */`
    Pt1,
    /// `padding-right: 0.25rem; /* 4px */`
    Pr1,
    /// `padding-bottom: 0.25rem; /* 4px */`
    Pb1,
    /// `padding-left: 0.25rem; /* 4px */`
    Pl1,
    /// `padding: 0.375rem; /* 6px */`
    P1_5,
    /// `padding-left: 0.375rem; /* 6px */`
    /// 
    /// `padding-right: 0.375rem; /* 6px */`
    Px1_5,
    /// `padding-top: 0.375rem; /* 6px */`
    /// 
    /// `padding-bottom: 0.375rem; /* 6px */`
    Py1_5,
    /// `padding-inline-start: 0.375rem; /* 6px */`
    Ps1_5,
    /// `padding-inline-end: 0.375rem; /* 6px */`
    Pe1_5,
    /// `padding-top: 0.375rem; /* 6px */`
    Pt1_5,
    /// `padding-right: 0.375rem; /* 6px */`
    Pr1_5,
    /// `padding-bottom: 0.375rem; /* 6px */`
    Pb1_5,
    /// `padding-left: 0.375rem; /* 6px */`
    Pl1_5,
    /// `padding: 0.5rem; /* 8px */`
    P2,
    /// `padding-left: 0.5rem; /* 8px */`
    /// 
    /// `padding-right: 0.5rem; /* 8px */`
    Px2,
    /// `padding-top: 0.5rem; /* 8px */`
    /// 
    /// `padding-bottom: 0.5rem; /* 8px */`
    Py2,
    /// `padding-inline-start: 0.5rem; /* 8px */`
    Ps2,
    /// `padding-inline-end: 0.5rem; /* 8px */`
    Pe2,
    /// `padding-top: 0.5rem; /* 8px */`
    Pt2,
    /// `padding-right: 0.5rem; /* 8px */`
    Pr2,
    /// `padding-bottom: 0.5rem; /* 8px */`
    Pb2,
    /// `padding-left: 0.5rem; /* 8px */`
    Pl2,
    /// `padding: 0.625rem; /* 10px */`
    P2_5,
    /// `padding-left: 0.625rem; /* 10px */`
    /// 
    /// `padding-right: 0.625rem; /* 10px */`
    Px2_5,
    /// `padding-top: 0.625rem; /* 10px */`
    /// 
    /// `padding-bottom: 0.625rem; /* 10px */`
    Py2_5,
    /// `padding-inline-start: 0.625rem; /* 10px */`
    Ps2_5,
    /// `padding-inline-end: 0.625rem; /* 10px */`
    Pe2_5,
    /// `padding-top: 0.625rem; /* 10px */`
    Pt2_5,
    /// `padding-right: 0.625rem; /* 10px */`
    Pr2_5,
    /// `padding-bottom: 0.625rem; /* 10px */`
    Pb2_5,
    /// `padding-left: 0.625rem; /* 10px */`
    Pl2_5,
    /// `padding: 0.75rem; /* 12px */`
    P3,
    /// `padding-left: 0.75rem; /* 12px */`
    /// 
    /// `padding-right: 0.75rem; /* 12px */`
    Px3,
    /// `padding-top: 0.75rem; /* 12px */`
    /// 
    /// `padding-bottom: 0.75rem; /* 12px */`
    Py3,
    /// `padding-inline-start: 0.75rem; /* 12px */`
    Ps3,
    /// `padding-inline-end: 0.75rem; /* 12px */`
    Pe3,
    /// `padding-top: 0.75rem; /* 12px */`
    Pt3,
    /// `padding-right: 0.75rem; /* 12px */`
    Pr3,
    /// `padding-bottom: 0.75rem; /* 12px */`
    Pb3,
    /// `padding-left: 0.75rem; /* 12px */`
    Pl3,
    /// `padding: 0.875rem; /* 14px */`
    P3_5,
    /// `padding-left: 0.875rem; /* 14px */`
    /// 
    /// `padding-right: 0.875rem; /* 14px */`
    Px3_5,
    /// `padding-top: 0.875rem; /* 14px */`
    /// 
    /// `padding-bottom: 0.875rem; /* 14px */`
    Py3_5,
    /// `padding-inline-start: 0.875rem; /* 14px */`
    Ps3_5,
    /// `padding-inline-end: 0.875rem; /* 14px */`
    Pe3_5,
    /// `padding-top: 0.875rem; /* 14px */`
    Pt3_5,
    /// `padding-right: 0.875rem; /* 14px */`
    Pr3_5,
    /// `padding-bottom: 0.875rem; /* 14px */`
    Pb3_5,
    /// `padding-left: 0.875rem; /* 14px */`
    Pl3_5,
    /// `padding: 1rem; /* 16px */`
    P4,
    /// `padding-left: 1rem; /* 16px */`
    /// 
    /// `padding-right: 1rem; /* 16px */`
    Px4,
    /// `padding-top: 1rem; /* 16px */`
    /// 
    /// `padding-bottom: 1rem; /* 16px */`
    Py4,
    /// `padding-inline-start: 1rem; /* 16px */`
    Ps4,
    /// `padding-inline-end: 1rem; /* 16px */`
    Pe4,
    /// `padding-top: 1rem; /* 16px */`
    Pt4,
    /// `padding-right: 1rem; /* 16px */`
    Pr4,
    /// `padding-bottom: 1rem; /* 16px */`
    Pb4,
    /// `padding-left: 1rem; /* 16px */`
    Pl4,
    /// `padding: 1.25rem; /* 20px */`
    P5,
    /// `padding-left: 1.25rem; /* 20px */`
    /// 
    /// `padding-right: 1.25rem; /* 20px */`
    Px5,
    /// `padding-top: 1.25rem; /* 20px */`
    /// 
    /// `padding-bottom: 1.25rem; /* 20px */`
    Py5,
    /// `padding-inline-start: 1.25rem; /* 20px */`
    Ps5,
    /// `padding-inline-end: 1.25rem; /* 20px */`
    Pe5,
    /// `padding-top: 1.25rem; /* 20px */`
    Pt5,
    /// `padding-right: 1.25rem; /* 20px */`
    Pr5,
    /// `padding-bottom: 1.25rem; /* 20px */`
    Pb5,
    /// `padding-left: 1.25rem; /* 20px */`
    Pl5,
    /// `padding: 1.5rem; /* 24px */`
    P6,
    /// `padding-left: 1.5rem; /* 24px */`
    /// 
    /// `padding-right: 1.5rem; /* 24px */`
    Px6,
    /// `padding-top: 1.5rem; /* 24px */`
    /// 
    /// `padding-bottom: 1.5rem; /* 24px */`
    Py6,
    /// `padding-inline-start: 1.5rem; /* 24px */`
    Ps6,
    /// `padding-inline-end: 1.5rem; /* 24px */`
    Pe6,
    /// `padding-top: 1.5rem; /* 24px */`
    Pt6,
    /// `padding-right: 1.5rem; /* 24px */`
    Pr6,
    /// `padding-bottom: 1.5rem; /* 24px */`
    Pb6,
    /// `padding-left: 1.5rem; /* 24px */`
    Pl6,
    /// `padding: 1.75rem; /* 28px */`
    P7,
    /// `padding-left: 1.75rem; /* 28px */`
    /// 
    /// `padding-right: 1.75rem; /* 28px */`
    Px7,
    /// `padding-top: 1.75rem; /* 28px */`
    /// 
    /// `padding-bottom: 1.75rem; /* 28px */`
    Py7,
    /// `padding-inline-start: 1.75rem; /* 28px */`
    Ps7,
    /// `padding-inline-end: 1.75rem; /* 28px */`
    Pe7,
    /// `padding-top: 1.75rem; /* 28px */`
    Pt7,
    /// `padding-right: 1.75rem; /* 28px */`
    Pr7,
    /// `padding-bottom: 1.75rem; /* 28px */`
    Pb7,
    /// `padding-left: 1.75rem; /* 28px */`
    Pl7,
    /// `padding: 2rem; /* 32px */`
    P8,
    /// `padding-left: 2rem; /* 32px */`
    /// 
    /// `padding-right: 2rem; /* 32px */`
    Px8,
    /// `padding-top: 2rem; /* 32px */`
    /// 
    /// `padding-bottom: 2rem; /* 32px */`
    Py8,
    /// `padding-inline-start: 2rem; /* 32px */`
    Ps8,
    /// `padding-inline-end: 2rem; /* 32px */`
    Pe8,
    /// `padding-top: 2rem; /* 32px */`
    Pt8,
    /// `padding-right: 2rem; /* 32px */`
    Pr8,
    /// `padding-bottom: 2rem; /* 32px */`
    Pb8,
    /// `padding-left: 2rem; /* 32px */`
    Pl8,
    /// `padding: 2.25rem; /* 36px */`
    P9,
    /// `padding-left: 2.25rem; /* 36px */`
    /// 
    /// `padding-right: 2.25rem; /* 36px */`
    Px9,
    /// `padding-top: 2.25rem; /* 36px */`
    /// 
    /// `padding-bottom: 2.25rem; /* 36px */`
    Py9,
    /// `padding-inline-start: 2.25rem; /* 36px */`
    Ps9,
    /// `padding-inline-end: 2.25rem; /* 36px */`
    Pe9,
    /// `padding-top: 2.25rem; /* 36px */`
    Pt9,
    /// `padding-right: 2.25rem; /* 36px */`
    Pr9,
    /// `padding-bottom: 2.25rem; /* 36px */`
    Pb9,
    /// `padding-left: 2.25rem; /* 36px */`
    Pl9,
    /// `padding: 2.5rem; /* 40px */`
    P10,
    /// `padding-left: 2.5rem; /* 40px */`
    /// 
    /// `padding-right: 2.5rem; /* 40px */`
    Px10,
    /// `padding-top: 2.5rem; /* 40px */`
    /// 
    /// `padding-bottom: 2.5rem; /* 40px */`
    Py10,
    /// `padding-inline-start: 2.5rem; /* 40px */`
    Ps10,
    /// `padding-inline-end: 2.5rem; /* 40px */`
    Pe10,
    /// `padding-top: 2.5rem; /* 40px */`
    Pt10,
    /// `padding-right: 2.5rem; /* 40px */`
    Pr10,
    /// `padding-bottom: 2.5rem; /* 40px */`
    Pb10,
    /// `padding-left: 2.5rem; /* 40px */`
    Pl10,
    /// `padding: 2.75rem; /* 44px */`
    P11,
    /// `padding-left: 2.75rem; /* 44px */`
    /// 
    /// `padding-right: 2.75rem; /* 44px */`
    Px11,
    /// `padding-top: 2.75rem; /* 44px */`
    /// 
    /// `padding-bottom: 2.75rem; /* 44px */`
    Py11,
    /// `padding-inline-start: 2.75rem; /* 44px */`
    Ps11,
    /// `padding-inline-end: 2.75rem; /* 44px */`
    Pe11,
    /// `padding-top: 2.75rem; /* 44px */`
    Pt11,
    /// `padding-right: 2.75rem; /* 44px */`
    Pr11,
    /// `padding-bottom: 2.75rem; /* 44px */`
    Pb11,
    /// `padding-left: 2.75rem; /* 44px */`
    Pl11,
    /// `padding: 3rem; /* 48px */`
    P12,
    /// `padding-left: 3rem; /* 48px */`
    /// 
    /// `padding-right: 3rem; /* 48px */`
    Px12,
    /// `padding-top: 3rem; /* 48px */`
    /// 
    /// `padding-bottom: 3rem; /* 48px */`
    Py12,
    /// `padding-inline-start: 3rem; /* 48px */`
    Ps12,
    /// `padding-inline-end: 3rem; /* 48px */`
    Pe12,
    /// `padding-top: 3rem; /* 48px */`
    Pt12,
    /// `padding-right: 3rem; /* 48px */`
    Pr12,
    /// `padding-bottom: 3rem; /* 48px */`
    Pb12,
    /// `padding-left: 3rem; /* 48px */`
    Pl12,
    /// `padding: 3.5rem; /* 56px */`
    P14,
    /// `padding-left: 3.5rem; /* 56px */`
    /// 
    /// `padding-right: 3.5rem; /* 56px */`
    Px14,
    /// `padding-top: 3.5rem; /* 56px */`
    /// 
    /// `padding-bottom: 3.5rem; /* 56px */`
    Py14,
    /// `padding-inline-start: 3.5rem; /* 56px */`
    Ps14,
    /// `padding-inline-end: 3.5rem; /* 56px */`
    Pe14,
    /// `padding-top: 3.5rem; /* 56px */`
    Pt14,
    /// `padding-right: 3.5rem; /* 56px */`
    Pr14,
    /// `padding-bottom: 3.5rem; /* 56px */`
    Pb14,
    /// `padding-left: 3.5rem; /* 56px */`
    Pl14,
    /// `padding: 4rem; /* 64px */`
    P16,
    /// `padding-left: 4rem; /* 64px */`
    /// 
    /// `padding-right: 4rem; /* 64px */`
    Px16,
    /// `padding-top: 4rem; /* 64px */`
    /// 
    /// `padding-bottom: 4rem; /* 64px */`
    Py16,
    /// `padding-inline-start: 4rem; /* 64px */`
    Ps16,
    /// `padding-inline-end: 4rem; /* 64px */`
    Pe16,
    /// `padding-top: 4rem; /* 64px */`
    Pt16,
    /// `padding-right: 4rem; /* 64px */`
    Pr16,
    /// `padding-bottom: 4rem; /* 64px */`
    Pb16,
    /// `padding-left: 4rem; /* 64px */`
    Pl16,
    /// `padding: 5rem; /* 80px */`
    P20,
    /// `padding-left: 5rem; /* 80px */`
    /// 
    /// `padding-right: 5rem; /* 80px */`
    Px20,
    /// `padding-top: 5rem; /* 80px */`
    /// 
    /// `padding-bottom: 5rem; /* 80px */`
    Py20,
    /// `padding-inline-start: 5rem; /* 80px */`
    Ps20,
    /// `padding-inline-end: 5rem; /* 80px */`
    Pe20,
    /// `padding-top: 5rem; /* 80px */`
    Pt20,
    /// `padding-right: 5rem; /* 80px */`
    Pr20,
    /// `padding-bottom: 5rem; /* 80px */`
    Pb20,
    /// `padding-left: 5rem; /* 80px */`
    Pl20,
    /// `padding: 6rem; /* 96px */`
    P24,
    /// `padding-left: 6rem; /* 96px */`
    /// 
    /// `padding-right: 6rem; /* 96px */`
    Px24,
    /// `padding-top: 6rem; /* 96px */`
    /// 
    /// `padding-bottom: 6rem; /* 96px */`
    Py24,
    /// `padding-inline-start: 6rem; /* 96px */`
    Ps24,
    /// `padding-inline-end: 6rem; /* 96px */`
    Pe24,
    /// `padding-top: 6rem; /* 96px */`
    Pt24,
    /// `padding-right: 6rem; /* 96px */`
    Pr24,
    /// `padding-bottom: 6rem; /* 96px */`
    Pb24,
    /// `padding-left: 6rem; /* 96px */`
    Pl24,
    /// `padding: 7rem; /* 112px */`
    P28,
    /// `padding-left: 7rem; /* 112px */`
    /// 
    /// `padding-right: 7rem; /* 112px */`
    Px28,
    /// `padding-top: 7rem; /* 112px */`
    /// 
    /// `padding-bottom: 7rem; /* 112px */`
    Py28,
    /// `padding-inline-start: 7rem; /* 112px */`
    Ps28,
    /// `padding-inline-end: 7rem; /* 112px */`
    Pe28,
    /// `padding-top: 7rem; /* 112px */`
    Pt28,
    /// `padding-right: 7rem; /* 112px */`
    Pr28,
    /// `padding-bottom: 7rem; /* 112px */`
    Pb28,
    /// `padding-left: 7rem; /* 112px */`
    Pl28,
    /// `padding: 8rem; /* 128px */`
    P32,
    /// `padding-left: 8rem; /* 128px */`
    /// 
    /// `padding-right: 8rem; /* 128px */`
    Px32,
    /// `padding-top: 8rem; /* 128px */`
    /// 
    /// `padding-bottom: 8rem; /* 128px */`
    Py32,
    /// `padding-inline-start: 8rem; /* 128px */`
    Ps32,
    /// `padding-inline-end: 8rem; /* 128px */`
    Pe32,
    /// `padding-top: 8rem; /* 128px */`
    Pt32,
    /// `padding-right: 8rem; /* 128px */`
    Pr32,
    /// `padding-bottom: 8rem; /* 128px */`
    Pb32,
    /// `padding-left: 8rem; /* 128px */`
    Pl32,
    /// `padding: 9rem; /* 144px */`
    P36,
    /// `padding-left: 9rem; /* 144px */`
    /// 
    /// `padding-right: 9rem; /* 144px */`
    Px36,
    /// `padding-top: 9rem; /* 144px */`
    /// 
    /// `padding-bottom: 9rem; /* 144px */`
    Py36,
    /// `padding-inline-start: 9rem; /* 144px */`
    Ps36,
    /// `padding-inline-end: 9rem; /* 144px */`
    Pe36,
    /// `padding-top: 9rem; /* 144px */`
    Pt36,
    /// `padding-right: 9rem; /* 144px */`
    Pr36,
    /// `padding-bottom: 9rem; /* 144px */`
    Pb36,
    /// `padding-left: 9rem; /* 144px */`
    Pl36,
    /// `padding: 10rem; /* 160px */`
    P40,
    /// `padding-left: 10rem; /* 160px */`
    /// 
    /// `padding-right: 10rem; /* 160px */`
    Px40,
    /// `padding-top: 10rem; /* 160px */`
    /// 
    /// `padding-bottom: 10rem; /* 160px */`
    Py40,
    /// `padding-inline-start: 10rem; /* 160px */`
    Ps40,
    /// `padding-inline-end: 10rem; /* 160px */`
    Pe40,
    /// `padding-top: 10rem; /* 160px */`
    Pt40,
    /// `padding-right: 10rem; /* 160px */`
    Pr40,
    /// `padding-bottom: 10rem; /* 160px */`
    Pb40,
    /// `padding-left: 10rem; /* 160px */`
    Pl40,
    /// `padding: 11rem; /* 176px */`
    P44,
    /// `padding-left: 11rem; /* 176px */`
    /// 
    /// `padding-right: 11rem; /* 176px */`
    Px44,
    /// `padding-top: 11rem; /* 176px */`
    /// 
    /// `padding-bottom: 11rem; /* 176px */`
    Py44,
    /// `padding-inline-start: 11rem; /* 176px */`
    Ps44,
    /// `padding-inline-end: 11rem; /* 176px */`
    Pe44,
    /// `padding-top: 11rem; /* 176px */`
    Pt44,
    /// `padding-right: 11rem; /* 176px */`
    Pr44,
    /// `padding-bottom: 11rem; /* 176px */`
    Pb44,
    /// `padding-left: 11rem; /* 176px */`
    Pl44,
    /// `padding: 12rem; /* 192px */`
    P48,
    /// `padding-left: 12rem; /* 192px */`
    /// 
    /// `padding-right: 12rem; /* 192px */`
    Px48,
    /// `padding-top: 12rem; /* 192px */`
    /// 
    /// `padding-bottom: 12rem; /* 192px */`
    Py48,
    /// `padding-inline-start: 12rem; /* 192px */`
    Ps48,
    /// `padding-inline-end: 12rem; /* 192px */`
    Pe48,
    /// `padding-top: 12rem; /* 192px */`
    Pt48,
    /// `padding-right: 12rem; /* 192px */`
    Pr48,
    /// `padding-bottom: 12rem; /* 192px */`
    Pb48,
    /// `padding-left: 12rem; /* 192px */`
    Pl48,
    /// `padding: 13rem; /* 208px */`
    P52,
    /// `padding-left: 13rem; /* 208px */`
    /// 
    /// `padding-right: 13rem; /* 208px */`
    Px52,
    /// `padding-top: 13rem; /* 208px */`
    /// 
    /// `padding-bottom: 13rem; /* 208px */`
    Py52,
    /// `padding-inline-start: 13rem; /* 208px */`
    Ps52,
    /// `padding-inline-end: 13rem; /* 208px */`
    Pe52,
    /// `padding-top: 13rem; /* 208px */`
    Pt52,
    /// `padding-right: 13rem; /* 208px */`
    Pr52,
    /// `padding-bottom: 13rem; /* 208px */`
    Pb52,
    /// `padding-left: 13rem; /* 208px */`
    Pl52,
    /// `padding: 14rem; /* 224px */`
    P56,
    /// `padding-left: 14rem; /* 224px */`
    /// 
    /// `padding-right: 14rem; /* 224px */`
    Px56,
    /// `padding-top: 14rem; /* 224px */`
    /// 
    /// `padding-bottom: 14rem; /* 224px */`
    Py56,
    /// `padding-inline-start: 14rem; /* 224px */`
    Ps56,
    /// `padding-inline-end: 14rem; /* 224px */`
    Pe56,
    /// `padding-top: 14rem; /* 224px */`
    Pt56,
    /// `padding-right: 14rem; /* 224px */`
    Pr56,
    /// `padding-bottom: 14rem; /* 224px */`
    Pb56,
    /// `padding-left: 14rem; /* 224px */`
    Pl56,
    /// `padding: 15rem; /* 240px */`
    P60,
    /// `padding-left: 15rem; /* 240px */`
    /// 
    /// `padding-right: 15rem; /* 240px */`
    Px60,
    /// `padding-top: 15rem; /* 240px */`
    /// 
    /// `padding-bottom: 15rem; /* 240px */`
    Py60,
    /// `padding-inline-start: 15rem; /* 240px */`
    Ps60,
    /// `padding-inline-end: 15rem; /* 240px */`
    Pe60,
    /// `padding-top: 15rem; /* 240px */`
    Pt60,
    /// `padding-right: 15rem; /* 240px */`
    Pr60,
    /// `padding-bottom: 15rem; /* 240px */`
    Pb60,
    /// `padding-left: 15rem; /* 240px */`
    Pl60,
    /// `padding: 16rem; /* 256px */`
    P64,
    /// `padding-left: 16rem; /* 256px */`
    /// 
    /// `padding-right: 16rem; /* 256px */`
    Px64,
    /// `padding-top: 16rem; /* 256px */`
    /// 
    /// `padding-bottom: 16rem; /* 256px */`
    Py64,
    /// `padding-inline-start: 16rem; /* 256px */`
    Ps64,
    /// `padding-inline-end: 16rem; /* 256px */`
    Pe64,
    /// `padding-top: 16rem; /* 256px */`
    Pt64,
    /// `padding-right: 16rem; /* 256px */`
    Pr64,
    /// `padding-bottom: 16rem; /* 256px */`
    Pb64,
    /// `padding-left: 16rem; /* 256px */`
    Pl64,
    /// `padding: 18rem; /* 288px */`
    P72,
    /// `padding-left: 18rem; /* 288px */`
    /// 
    /// `padding-right: 18rem; /* 288px */`
    Px72,
    /// `padding-top: 18rem; /* 288px */`
    /// 
    /// `padding-bottom: 18rem; /* 288px */`
    Py72,
    /// `padding-inline-start: 18rem; /* 288px */`
    Ps72,
    /// `padding-inline-end: 18rem; /* 288px */`
    Pe72,
    /// `padding-top: 18rem; /* 288px */`
    Pt72,
    /// `padding-right: 18rem; /* 288px */`
    Pr72,
    /// `padding-bottom: 18rem; /* 288px */`
    Pb72,
    /// `padding-left: 18rem; /* 288px */`
    Pl72,
    /// `padding: 20rem; /* 320px */`
    P80,
    /// `padding-left: 20rem; /* 320px */`
    /// 
    /// `padding-right: 20rem; /* 320px */`
    Px80,
    /// `padding-top: 20rem; /* 320px */`
    /// 
    /// `padding-bottom: 20rem; /* 320px */`
    Py80,
    /// `padding-inline-start: 20rem; /* 320px */`
    Ps80,
    /// `padding-inline-end: 20rem; /* 320px */`
    Pe80,
    /// `padding-top: 20rem; /* 320px */`
    Pt80,
    /// `padding-right: 20rem; /* 320px */`
    Pr80,
    /// `padding-bottom: 20rem; /* 320px */`
    Pb80,
    /// `padding-left: 20rem; /* 320px */`
    Pl80,
    /// `padding: 24rem; /* 384px */`
    P96,
    /// `padding-left: 24rem; /* 384px */`
    /// 
    /// `padding-right: 24rem; /* 384px */`
    Px96,
    /// `padding-top: 24rem; /* 384px */`
    /// 
    /// `padding-bottom: 24rem; /* 384px */`
    Py96,
    /// `padding-inline-start: 24rem; /* 384px */`
    Ps96,
    /// `padding-inline-end: 24rem; /* 384px */`
    Pe96,
    /// `padding-top: 24rem; /* 384px */`
    Pt96,
    /// `padding-right: 24rem; /* 384px */`
    Pr96,
    /// `padding-bottom: 24rem; /* 384px */`
    Pb96,
    /// `padding-left: 24rem; /* 384px */`
    Pl96,
}

/// Utilities for controlling an element's margin.
/// 
/// <https://tailwindcss.com/docs/margin>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(replace(from = "_", to = "."))]
pub enum Margin {
    /// `margin: 0px;`
    M0,
    /// `margin-left: 0px;`
    /// 
    /// `margin-right: 0px;`
    Mx0,
    /// `margin-top: 0px;`
    /// 
    /// `margin-bottom: 0px;`
    My0,
    /// `margin-inline-start: 0px;`
    Ms0,
    /// `margin-inline-end: 0px;`
    Me0,
    /// `margin-top: 0px;`
    Mt0,
    /// `margin-right: 0px;`
    Mr0,
    /// `margin-bottom: 0px;`
    Mb0,
    /// `margin-left: 0px;`
    Ml0,
    /// `margin: 1px;`
    MPx,
    /// `margin-left: 1px;`
    /// 
    /// `margin-right: 1px;`
    MxPx,
    /// `margin-top: 1px;`
    /// 
    /// `margin-bottom: 1px;`
    MyPx,
    /// `margin-inline-start: 1px;`
    MsPx,
    /// `margin-inline-end: 1px;`
    MePx,
    /// `margin-top: 1px;`
    MtPx,
    /// `margin-right: 1px;`
    MrPx,
    /// `margin-bottom: 1px;`
    MbPx,
    /// `margin-left: 1px;`
    MlPx,
    /// `margin: 0.125rem; /* 2px */`
    M0_5,
    /// `margin-left: 0.125rem; /* 2px */`
    /// 
    /// `margin-right: 0.125rem; /* 2px */`
    Mx0_5,
    /// `margin-top: 0.125rem; /* 2px */`
    /// 
    /// `margin-bottom: 0.125rem; /* 2px */`
    My0_5,
    /// `margin-inline-start: 0.125rem; /* 2px */`
    Ms0_5,
    /// `margin-inline-end: 0.125rem; /* 2px */`
    Me0_5,
    /// `margin-top: 0.125rem; /* 2px */`
    Mt0_5,
    /// `margin-right: 0.125rem; /* 2px */`
    Mr0_5,
    /// `margin-bottom: 0.125rem; /* 2px */`
    Mb0_5,
    /// `margin-left: 0.125rem; /* 2px */`
    Ml0_5,
    /// `margin: 0.25rem; /* 4px */`
    M1,
    /// `margin-left: 0.25rem; /* 4px */`
    /// 
    /// `margin-right: 0.25rem; /* 4px */`
    Mx1,
    /// `margin-top: 0.25rem; /* 4px */`
    /// 
    /// `margin-bottom: 0.25rem; /* 4px */`
    My1,
    /// `margin-inline-start: 0.25rem; /* 4px */`
    Ms1,
    /// `margin-inline-end: 0.25rem; /* 4px */`
    Me1,
    /// `margin-top: 0.25rem; /* 4px */`
    Mt1,
    /// `margin-right: 0.25rem; /* 4px */`
    Mr1,
    /// `margin-bottom: 0.25rem; /* 4px */`
    Mb1,
    /// `margin-left: 0.25rem; /* 4px */`
    Ml1,
    /// `margin: 0.375rem; /* 6px */`
    M1_5,
    /// `margin-left: 0.375rem; /* 6px */`
    /// 
    /// `margin-right: 0.375rem; /* 6px */`
    Mx1_5,
    /// `margin-top: 0.375rem; /* 6px */`
    /// 
    /// `margin-bottom: 0.375rem; /* 6px */`
    My1_5,
    /// `margin-inline-start: 0.375rem; /* 6px */`
    Ms1_5,
    /// `margin-inline-end: 0.375rem; /* 6px */`
    Me1_5,
    /// `margin-top: 0.375rem; /* 6px */`
    Mt1_5,
    /// `margin-right: 0.375rem; /* 6px */`
    Mr1_5,
    /// `margin-bottom: 0.375rem; /* 6px */`
    Mb1_5,
    /// `margin-left: 0.375rem; /* 6px */`
    Ml1_5,
    /// `margin: 0.5rem; /* 8px */`
    M2,
    /// `margin-left: 0.5rem; /* 8px */`
    /// 
    /// `margin-right: 0.5rem; /* 8px */`
    Mx2,
    /// `margin-top: 0.5rem; /* 8px */`
    /// 
    /// `margin-bottom: 0.5rem; /* 8px */`
    My2,
    /// `margin-inline-start: 0.5rem; /* 8px */`
    Ms2,
    /// `margin-inline-end: 0.5rem; /* 8px */`
    Me2,
    /// `margin-top: 0.5rem; /* 8px */`
    Mt2,
    /// `margin-right: 0.5rem; /* 8px */`
    Mr2,
    /// `margin-bottom: 0.5rem; /* 8px */`
    Mb2,
    /// `margin-left: 0.5rem; /* 8px */`
    Ml2,
    /// `margin: 0.625rem; /* 10px */`
    M2_5,
    /// `margin-left: 0.625rem; /* 10px */`
    /// 
    /// `margin-right: 0.625rem; /* 10px */`
    Mx2_5,
    /// `margin-top: 0.625rem; /* 10px */`
    /// 
    /// `margin-bottom: 0.625rem; /* 10px */`
    My2_5,
    /// `margin-inline-start: 0.625rem; /* 10px */`
    Ms2_5,
    /// `margin-inline-end: 0.625rem; /* 10px */`
    Me2_5,
    /// `margin-top: 0.625rem; /* 10px */`
    Mt2_5,
    /// `margin-right: 0.625rem; /* 10px */`
    Mr2_5,
    /// `margin-bottom: 0.625rem; /* 10px */`
    Mb2_5,
    /// `margin-left: 0.625rem; /* 10px */`
    Ml2_5,
    /// `margin: 0.75rem; /* 12px */`
    M3,
    /// `margin-left: 0.75rem; /* 12px */`
    /// 
    /// `margin-right: 0.75rem; /* 12px */`
    Mx3,
    /// `margin-top: 0.75rem; /* 12px */`
    /// 
    /// `margin-bottom: 0.75rem; /* 12px */`
    My3,
    /// `margin-inline-start: 0.75rem; /* 12px */`
    Ms3,
    /// `margin-inline-end: 0.75rem; /* 12px */`
    Me3,
    /// `margin-top: 0.75rem; /* 12px */`
    Mt3,
    /// `margin-right: 0.75rem; /* 12px */`
    Mr3,
    /// `margin-bottom: 0.75rem; /* 12px */`
    Mb3,
    /// `margin-left: 0.75rem; /* 12px */`
    Ml3,
    /// `margin: 0.875rem; /* 14px */`
    M3_5,
    /// `margin-left: 0.875rem; /* 14px */`
    /// 
    /// `margin-right: 0.875rem; /* 14px */`
    Mx3_5,
    /// `margin-top: 0.875rem; /* 14px */`
    /// 
    /// `margin-bottom: 0.875rem; /* 14px */`
    My3_5,
    /// `margin-inline-start: 0.875rem; /* 14px */`
    Ms3_5,
    /// `margin-inline-end: 0.875rem; /* 14px */`
    Me3_5,
    /// `margin-top: 0.875rem; /* 14px */`
    Mt3_5,
    /// `margin-right: 0.875rem; /* 14px */`
    Mr3_5,
    /// `margin-bottom: 0.875rem; /* 14px */`
    Mb3_5,
    /// `margin-left: 0.875rem; /* 14px */`
    Ml3_5,
    /// `margin: 1rem; /* 16px */`
    M4,
    /// `margin-left: 1rem; /* 16px */`
    /// 
    /// `margin-right: 1rem; /* 16px */`
    Mx4,
    /// `margin-top: 1rem; /* 16px */`
    /// 
    /// `margin-bottom: 1rem; /* 16px */`
    My4,
    /// `margin-inline-start: 1rem; /* 16px */`
    Ms4,
    /// `margin-inline-end: 1rem; /* 16px */`
    Me4,
    /// `margin-top: 1rem; /* 16px */`
    Mt4,
    /// `margin-right: 1rem; /* 16px */`
    Mr4,
    /// `margin-bottom: 1rem; /* 16px */`
    Mb4,
    /// `margin-left: 1rem; /* 16px */`
    Ml4,
    /// `margin: 1.25rem; /* 20px */`
    M5,
    /// `margin-left: 1.25rem; /* 20px */`
    /// 
    /// `margin-right: 1.25rem; /* 20px */`
    Mx5,
    /// `margin-top: 1.25rem; /* 20px */`
    /// 
    /// `margin-bottom: 1.25rem; /* 20px */`
    My5,
    /// `margin-inline-start: 1.25rem; /* 20px */`
    Ms5,
    /// `margin-inline-end: 1.25rem; /* 20px */`
    Me5,
    /// `margin-top: 1.25rem; /* 20px */`
    Mt5,
    /// `margin-right: 1.25rem; /* 20px */`
    Mr5,
    /// `margin-bottom: 1.25rem; /* 20px */`
    Mb5,
    /// `margin-left: 1.25rem; /* 20px */`
    Ml5,
    /// `margin: 1.5rem; /* 24px */`
    M6,
    /// `margin-left: 1.5rem; /* 24px */`
    /// 
    /// `margin-right: 1.5rem; /* 24px */`
    Mx6,
    /// `margin-top: 1.5rem; /* 24px */`
    /// 
    /// `margin-bottom: 1.5rem; /* 24px */`
    My6,
    /// `margin-inline-start: 1.5rem; /* 24px */`
    Ms6,
    /// `margin-inline-end: 1.5rem; /* 24px */`
    Me6,
    /// `margin-top: 1.5rem; /* 24px */`
    Mt6,
    /// `margin-right: 1.5rem; /* 24px */`
    Mr6,
    /// `margin-bottom: 1.5rem; /* 24px */`
    Mb6,
    /// `margin-left: 1.5rem; /* 24px */`
    Ml6,
    /// `margin: 1.75rem; /* 28px */`
    M7,
    /// `margin-left: 1.75rem; /* 28px */`
    /// 
    /// `margin-right: 1.75rem; /* 28px */`
    Mx7,
    /// `margin-top: 1.75rem; /* 28px */`
    /// 
    /// `margin-bottom: 1.75rem; /* 28px */`
    My7,
    /// `margin-inline-start: 1.75rem; /* 28px */`
    Ms7,
    /// `margin-inline-end: 1.75rem; /* 28px */`
    Me7,
    /// `margin-top: 1.75rem; /* 28px */`
    Mt7,
    /// `margin-right: 1.75rem; /* 28px */`
    Mr7,
    /// `margin-bottom: 1.75rem; /* 28px */`
    Mb7,
    /// `margin-left: 1.75rem; /* 28px */`
    Ml7,
    /// `margin: 2rem; /* 32px */`
    M8,
    /// `margin-left: 2rem; /* 32px */`
    /// 
    /// `margin-right: 2rem; /* 32px */`
    Mx8,
    /// `margin-top: 2rem; /* 32px */`
    /// 
    /// `margin-bottom: 2rem; /* 32px */`
    My8,
    /// `margin-inline-start: 2rem; /* 32px */`
    Ms8,
    /// `margin-inline-end: 2rem; /* 32px */`
    Me8,
    /// `margin-top: 2rem; /* 32px */`
    Mt8,
    /// `margin-right: 2rem; /* 32px */`
    Mr8,
    /// `margin-bottom: 2rem; /* 32px */`
    Mb8,
    /// `margin-left: 2rem; /* 32px */`
    Ml8,
    /// `margin: 2.25rem; /* 36px */`
    M9,
    /// `margin-left: 2.25rem; /* 36px */`
    /// 
    /// `margin-right: 2.25rem; /* 36px */`
    Mx9,
    /// `margin-top: 2.25rem; /* 36px */`
    /// 
    /// `margin-bottom: 2.25rem; /* 36px */`
    My9,
    /// `margin-inline-start: 2.25rem; /* 36px */`
    Ms9,
    /// `margin-inline-end: 2.25rem; /* 36px */`
    Me9,
    /// `margin-top: 2.25rem; /* 36px */`
    Mt9,
    /// `margin-right: 2.25rem; /* 36px */`
    Mr9,
    /// `margin-bottom: 2.25rem; /* 36px */`
    Mb9,
    /// `margin-left: 2.25rem; /* 36px */`
    Ml9,
    /// `margin: 2.5rem; /* 40px */`
    M10,
    /// `margin-left: 2.5rem; /* 40px */`
    /// 
    /// `margin-right: 2.5rem; /* 40px */`
    Mx10,
    /// `margin-top: 2.5rem; /* 40px */`
    /// 
    /// `margin-bottom: 2.5rem; /* 40px */`
    My10,
    /// `margin-inline-start: 2.5rem; /* 40px */`
    Ms10,
    /// `margin-inline-end: 2.5rem; /* 40px */`
    Me10,
    /// `margin-top: 2.5rem; /* 40px */`
    Mt10,
    /// `margin-right: 2.5rem; /* 40px */`
    Mr10,
    /// `margin-bottom: 2.5rem; /* 40px */`
    Mb10,
    /// `margin-left: 2.5rem; /* 40px */`
    Ml10,
    /// `margin: 2.75rem; /* 44px */`
    M11,
    /// `margin-left: 2.75rem; /* 44px */`
    /// 
    /// `margin-right: 2.75rem; /* 44px */`
    Mx11,
    /// `margin-top: 2.75rem; /* 44px */`
    /// 
    /// `margin-bottom: 2.75rem; /* 44px */`
    My11,
    /// `margin-inline-start: 2.75rem; /* 44px */`
    Ms11,
    /// `margin-inline-end: 2.75rem; /* 44px */`
    Me11,
    /// `margin-top: 2.75rem; /* 44px */`
    Mt11,
    /// `margin-right: 2.75rem; /* 44px */`
    Mr11,
    /// `margin-bottom: 2.75rem; /* 44px */`
    Mb11,
    /// `margin-left: 2.75rem; /* 44px */`
    Ml11,
    /// `margin: 3rem; /* 48px */`
    M12,
    /// `margin-left: 3rem; /* 48px */`
    /// 
    /// `margin-right: 3rem; /* 48px */`
    Mx12,
    /// `margin-top: 3rem; /* 48px */`
    /// 
    /// `margin-bottom: 3rem; /* 48px */`
    My12,
    /// `margin-inline-start: 3rem; /* 48px */`
    Ms12,
    /// `margin-inline-end: 3rem; /* 48px */`
    Me12,
    /// `margin-top: 3rem; /* 48px */`
    Mt12,
    /// `margin-right: 3rem; /* 48px */`
    Mr12,
    /// `margin-bottom: 3rem; /* 48px */`
    Mb12,
    /// `margin-left: 3rem; /* 48px */`
    Ml12,
    /// `margin: 3.5rem; /* 56px */`
    M14,
    /// `margin-left: 3.5rem; /* 56px */`
    /// 
    /// `margin-right: 3.5rem; /* 56px */`
    Mx14,
    /// `margin-top: 3.5rem; /* 56px */`
    /// 
    /// `margin-bottom: 3.5rem; /* 56px */`
    My14,
    /// `margin-inline-start: 3.5rem; /* 56px */`
    Ms14,
    /// `margin-inline-end: 3.5rem; /* 56px */`
    Me14,
    /// `margin-top: 3.5rem; /* 56px */`
    Mt14,
    /// `margin-right: 3.5rem; /* 56px */`
    Mr14,
    /// `margin-bottom: 3.5rem; /* 56px */`
    Mb14,
    /// `margin-left: 3.5rem; /* 56px */`
    Ml14,
    /// `margin: 4rem; /* 64px */`
    M16,
    /// `margin-left: 4rem; /* 64px */`
    /// 
    /// `margin-right: 4rem; /* 64px */`
    Mx16,
    /// `margin-top: 4rem; /* 64px */`
    /// 
    /// `margin-bottom: 4rem; /* 64px */`
    My16,
    /// `margin-inline-start: 4rem; /* 64px */`
    Ms16,
    /// `margin-inline-end: 4rem; /* 64px */`
    Me16,
    /// `margin-top: 4rem; /* 64px */`
    Mt16,
    /// `margin-right: 4rem; /* 64px */`
    Mr16,
    /// `margin-bottom: 4rem; /* 64px */`
    Mb16,
    /// `margin-left: 4rem; /* 64px */`
    Ml16,
    /// `margin: 5rem; /* 80px */`
    M20,
    /// `margin-left: 5rem; /* 80px */`
    /// 
    /// `margin-right: 5rem; /* 80px */`
    Mx20,
    /// `margin-top: 5rem; /* 80px */`
    /// 
    /// `margin-bottom: 5rem; /* 80px */`
    My20,
    /// `margin-inline-start: 5rem; /* 80px */`
    Ms20,
    /// `margin-inline-end: 5rem; /* 80px */`
    Me20,
    /// `margin-top: 5rem; /* 80px */`
    Mt20,
    /// `margin-right: 5rem; /* 80px */`
    Mr20,
    /// `margin-bottom: 5rem; /* 80px */`
    Mb20,
    /// `margin-left: 5rem; /* 80px */`
    Ml20,
    /// `margin: 6rem; /* 96px */`
    M24,
    /// `margin-left: 6rem; /* 96px */`
    /// 
    /// `margin-right: 6rem; /* 96px */`
    Mx24,
    /// `margin-top: 6rem; /* 96px */`
    /// 
    /// `margin-bottom: 6rem; /* 96px */`
    My24,
    /// `margin-inline-start: 6rem; /* 96px */`
    Ms24,
    /// `margin-inline-end: 6rem; /* 96px */`
    Me24,
    /// `margin-top: 6rem; /* 96px */`
    Mt24,
    /// `margin-right: 6rem; /* 96px */`
    Mr24,
    /// `margin-bottom: 6rem; /* 96px */`
    Mb24,
    /// `margin-left: 6rem; /* 96px */`
    Ml24,
    /// `margin: 7rem; /* 112px */`
    M28,
    /// `margin-left: 7rem; /* 112px */`
    /// 
    /// `margin-right: 7rem; /* 112px */`
    Mx28,
    /// `margin-top: 7rem; /* 112px */`
    /// 
    /// `margin-bottom: 7rem; /* 112px */`
    My28,
    /// `margin-inline-start: 7rem; /* 112px */`
    Ms28,
    /// `margin-inline-end: 7rem; /* 112px */`
    Me28,
    /// `margin-top: 7rem; /* 112px */`
    Mt28,
    /// `margin-right: 7rem; /* 112px */`
    Mr28,
    /// `margin-bottom: 7rem; /* 112px */`
    Mb28,
    /// `margin-left: 7rem; /* 112px */`
    Ml28,
    /// `margin: 8rem; /* 128px */`
    M32,
    /// `margin-left: 8rem; /* 128px */`
    /// 
    /// `margin-right: 8rem; /* 128px */`
    Mx32,
    /// `margin-top: 8rem; /* 128px */`
    /// 
    /// `margin-bottom: 8rem; /* 128px */`
    My32,
    /// `margin-inline-start: 8rem; /* 128px */`
    Ms32,
    /// `margin-inline-end: 8rem; /* 128px */`
    Me32,
    /// `margin-top: 8rem; /* 128px */`
    Mt32,
    /// `margin-right: 8rem; /* 128px */`
    Mr32,
    /// `margin-bottom: 8rem; /* 128px */`
    Mb32,
    /// `margin-left: 8rem; /* 128px */`
    Ml32,
    /// `margin: 9rem; /* 144px */`
    M36,
    /// `margin-left: 9rem; /* 144px */`
    /// 
    /// `margin-right: 9rem; /* 144px */`
    Mx36,
    /// `margin-top: 9rem; /* 144px */`
    /// 
    /// `margin-bottom: 9rem; /* 144px */`
    My36,
    /// `margin-inline-start: 9rem; /* 144px */`
    Ms36,
    /// `margin-inline-end: 9rem; /* 144px */`
    Me36,
    /// `margin-top: 9rem; /* 144px */`
    Mt36,
    /// `margin-right: 9rem; /* 144px */`
    Mr36,
    /// `margin-bottom: 9rem; /* 144px */`
    Mb36,
    /// `margin-left: 9rem; /* 144px */`
    Ml36,
    /// `margin: 10rem; /* 160px */`
    M40,
    /// `margin-left: 10rem; /* 160px */`
    /// 
    /// `margin-right: 10rem; /* 160px */`
    Mx40,
    /// `margin-top: 10rem; /* 160px */`
    /// 
    /// `margin-bottom: 10rem; /* 160px */`
    My40,
    /// `margin-inline-start: 10rem; /* 160px */`
    Ms40,
    /// `margin-inline-end: 10rem; /* 160px */`
    Me40,
    /// `margin-top: 10rem; /* 160px */`
    Mt40,
    /// `margin-right: 10rem; /* 160px */`
    Mr40,
    /// `margin-bottom: 10rem; /* 160px */`
    Mb40,
    /// `margin-left: 10rem; /* 160px */`
    Ml40,
    /// `margin: 11rem; /* 176px */`
    M44,
    /// `margin-left: 11rem; /* 176px */`
    /// 
    /// `margin-right: 11rem; /* 176px */`
    Mx44,
    /// `margin-top: 11rem; /* 176px */`
    /// 
    /// `margin-bottom: 11rem; /* 176px */`
    My44,
    /// `margin-inline-start: 11rem; /* 176px */`
    Ms44,
    /// `margin-inline-end: 11rem; /* 176px */`
    Me44,
    /// `margin-top: 11rem; /* 176px */`
    Mt44,
    /// `margin-right: 11rem; /* 176px */`
    Mr44,
    /// `margin-bottom: 11rem; /* 176px */`
    Mb44,
    /// `margin-left: 11rem; /* 176px */`
    Ml44,
    /// `margin: 12rem; /* 192px */`
    M48,
    /// `margin-left: 12rem; /* 192px */`
    /// 
    /// `margin-right: 12rem; /* 192px */`
    Mx48,
    /// `margin-top: 12rem; /* 192px */`
    /// 
    /// `margin-bottom: 12rem; /* 192px */`
    My48,
    /// `margin-inline-start: 12rem; /* 192px */`
    Ms48,
    /// `margin-inline-end: 12rem; /* 192px */`
    Me48,
    /// `margin-top: 12rem; /* 192px */`
    Mt48,
    /// `margin-right: 12rem; /* 192px */`
    Mr48,
    /// `margin-bottom: 12rem; /* 192px */`
    Mb48,
    /// `margin-left: 12rem; /* 192px */`
    Ml48,
    /// `margin: 13rem; /* 208px */`
    M52,
    /// `margin-left: 13rem; /* 208px */`
    /// 
    /// `margin-right: 13rem; /* 208px */`
    Mx52,
    /// `margin-top: 13rem; /* 208px */`
    /// 
    /// `margin-bottom: 13rem; /* 208px */`
    My52,
    /// `margin-inline-start: 13rem; /* 208px */`
    Ms52,
    /// `margin-inline-end: 13rem; /* 208px */`
    Me52,
    /// `margin-top: 13rem; /* 208px */`
    Mt52,
    /// `margin-right: 13rem; /* 208px */`
    Mr52,
    /// `margin-bottom: 13rem; /* 208px */`
    Mb52,
    /// `margin-left: 13rem; /* 208px */`
    Ml52,
    /// `margin: 14rem; /* 224px */`
    M56,
    /// `margin-left: 14rem; /* 224px */`
    /// 
    /// `margin-right: 14rem; /* 224px */`
    Mx56,
    /// `margin-top: 14rem; /* 224px */`
    /// 
    /// `margin-bottom: 14rem; /* 224px */`
    My56,
    /// `margin-inline-start: 14rem; /* 224px */`
    Ms56,
    /// `margin-inline-end: 14rem; /* 224px */`
    Me56,
    /// `margin-top: 14rem; /* 224px */`
    Mt56,
    /// `margin-right: 14rem; /* 224px */`
    Mr56,
    /// `margin-bottom: 14rem; /* 224px */`
    Mb56,
    /// `margin-left: 14rem; /* 224px */`
    Ml56,
    /// `margin: 15rem; /* 240px */`
    M60,
    /// `margin-left: 15rem; /* 240px */`
    /// 
    /// `margin-right: 15rem; /* 240px */`
    Mx60,
    /// `margin-top: 15rem; /* 240px */`
    /// 
    /// `margin-bottom: 15rem; /* 240px */`
    My60,
    /// `margin-inline-start: 15rem; /* 240px */`
    Ms60,
    /// `margin-inline-end: 15rem; /* 240px */`
    Me60,
    /// `margin-top: 15rem; /* 240px */`
    Mt60,
    /// `margin-right: 15rem; /* 240px */`
    Mr60,
    /// `margin-bottom: 15rem; /* 240px */`
    Mb60,
    /// `margin-left: 15rem; /* 240px */`
    Ml60,
    /// `margin: 16rem; /* 256px */`
    M64,
    /// `margin-left: 16rem; /* 256px */`
    /// 
    /// `margin-right: 16rem; /* 256px */`
    Mx64,
    /// `margin-top: 16rem; /* 256px */`
    /// 
    /// `margin-bottom: 16rem; /* 256px */`
    My64,
    /// `margin-inline-start: 16rem; /* 256px */`
    Ms64,
    /// `margin-inline-end: 16rem; /* 256px */`
    Me64,
    /// `margin-top: 16rem; /* 256px */`
    Mt64,
    /// `margin-right: 16rem; /* 256px */`
    Mr64,
    /// `margin-bottom: 16rem; /* 256px */`
    Mb64,
    /// `margin-left: 16rem; /* 256px */`
    Ml64,
    /// `margin: 18rem; /* 288px */`
    M72,
    /// `margin-left: 18rem; /* 288px */`
    /// 
    /// `margin-right: 18rem; /* 288px */`
    Mx72,
    /// `margin-top: 18rem; /* 288px */`
    /// 
    /// `margin-bottom: 18rem; /* 288px */`
    My72,
    /// `margin-inline-start: 18rem; /* 288px */`
    Ms72,
    /// `margin-inline-end: 18rem; /* 288px */`
    Me72,
    /// `margin-top: 18rem; /* 288px */`
    Mt72,
    /// `margin-right: 18rem; /* 288px */`
    Mr72,
    /// `margin-bottom: 18rem; /* 288px */`
    Mb72,
    /// `margin-left: 18rem; /* 288px */`
    Ml72,
    /// `margin: 20rem; /* 320px */`
    M80,
    /// `margin-left: 20rem; /* 320px */`
    /// 
    /// `margin-right: 20rem; /* 320px */`
    Mx80,
    /// `margin-top: 20rem; /* 320px */`
    /// 
    /// `margin-bottom: 20rem; /* 320px */`
    My80,
    /// `margin-inline-start: 20rem; /* 320px */`
    Ms80,
    /// `margin-inline-end: 20rem; /* 320px */`
    Me80,
    /// `margin-top: 20rem; /* 320px */`
    Mt80,
    /// `margin-right: 20rem; /* 320px */`
    Mr80,
    /// `margin-bottom: 20rem; /* 320px */`
    Mb80,
    /// `margin-left: 20rem; /* 320px */`
    Ml80,
    /// `margin: 24rem; /* 384px */`
    M96,
    /// `margin-left: 24rem; /* 384px */`
    /// 
    /// `margin-right: 24rem; /* 384px */`
    Mx96,
    /// `margin-top: 24rem; /* 384px */`
    /// 
    /// `margin-bottom: 24rem; /* 384px */`
    My96,
    /// `margin-inline-start: 24rem; /* 384px */`
    Ms96,
    /// `margin-inline-end: 24rem; /* 384px */`
    Me96,
    /// `margin-top: 24rem; /* 384px */`
    Mt96,
    /// `margin-right: 24rem; /* 384px */`
    Mr96,
    /// `margin-bottom: 24rem; /* 384px */`
    Mb96,
    /// `margin-left: 24rem; /* 384px */`
    Ml96,
    /// `margin: auto;`
    MAuto,
    /// `margin-left: auto;`
    /// 
    /// `margin-right: auto;`
    MxAuto,
    /// `margin-top: auto;`
    /// 
    /// `margin-bottom: auto;`
    MyAuto,
    /// `margin-inline-start: auto;`
    MsAuto,
    /// `margin-inline-end: auto;`
    MeAuto,
    /// `margin-top: auto;`
    MtAuto,
    /// `margin-right: auto;`
    MrAuto,
    /// `margin-bottom: auto;`
    MbAuto,
    /// `margin-left: auto;`
    MlAuto,
}

/// Utilities for controlling the space between child elements.
/// 
/// <https://tailwindcss.com/docs/space>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "space", replace(from = "_", to = "."))]
pub enum SpaceBetween {
    /// `margin-left: 0px;`
    X0,
    /// `margin-top: 0px;`
    Y0,
    /// `margin-left: 0.125rem; /* 2px */`
    X0_5,
    /// `margin-top: 0.125rem; /* 2px */`
    Y0_5,
    /// `margin-left: 0.25rem; /* 4px */`
    X1,
    /// `margin-top: 0.25rem; /* 4px */`
    Y1,
    /// `margin-left: 0.375rem; /* 6px */`
    X1_5,
    /// `margin-top: 0.375rem; /* 6px */`
    Y1_5,
    /// `margin-left: 0.5rem; /* 8px */`
    X2,
    /// `margin-top: 0.5rem; /* 8px */`
    Y2,
    /// `margin-left: 0.625rem; /* 10px */`
    X2_5,
    /// `margin-top: 0.625rem; /* 10px */`
    Y2_5,
    /// `margin-left: 0.75rem; /* 12px */`
    X3,
    /// `margin-top: 0.75rem; /* 12px */`
    Y3,
    /// `margin-left: 0.875rem; /* 14px */`
    X3_5,
    /// `margin-top: 0.875rem; /* 14px */`
    Y3_5,
    /// `margin-left: 1rem; /* 16px */`
    X4,
    /// `margin-top: 1rem; /* 16px */`
    Y4,
    /// `margin-left: 1.25rem; /* 20px */`
    X5,
    /// `margin-top: 1.25rem; /* 20px */`
    Y5,
    /// `margin-left: 1.5rem; /* 24px */`
    X6,
    /// `margin-top: 1.5rem; /* 24px */`
    Y6,
    /// `margin-left: 1.75rem; /* 28px */`
    X7,
    /// `margin-top: 1.75rem; /* 28px */`
    Y7,
    /// `margin-left: 2rem; /* 32px */`
    X8,
    /// `margin-top: 2rem; /* 32px */`
    Y8,
    /// `margin-left: 2.25rem; /* 36px */`
    X9,
    /// `margin-top: 2.25rem; /* 36px */`
    Y9,
    /// `margin-left: 2.5rem; /* 40px */`
    X10,
    /// `margin-top: 2.5rem; /* 40px */`
    Y10,
    /// `margin-left: 2.75rem; /* 44px */`
    X11,
    /// `margin-top: 2.75rem; /* 44px */`
    Y11,
    /// `margin-left: 3rem; /* 48px */`
    X12,
    /// `margin-top: 3rem; /* 48px */`
    Y12,
    /// `margin-left: 3.5rem; /* 56px */`
    X14,
    /// `margin-top: 3.5rem; /* 56px */`
    Y14,
    /// `margin-left: 4rem; /* 64px */`
    X16,
    /// `margin-top: 4rem; /* 64px */`
    Y16,
    /// `margin-left: 5rem; /* 80px */`
    X20,
    /// `margin-top: 5rem; /* 80px */`
    Y20,
    /// `margin-left: 6rem; /* 96px */`
    X24,
    /// `margin-top: 6rem; /* 96px */`
    Y24,
    /// `margin-left: 7rem; /* 112px */`
    X28,
    /// `margin-top: 7rem; /* 112px */`
    Y28,
    /// `margin-left: 8rem; /* 128px */`
    X32,
    /// `margin-top: 8rem; /* 128px */`
    Y32,
    /// `margin-left: 9rem; /* 144px */`
    X36,
    /// `margin-top: 9rem; /* 144px */`
    Y36,
    /// `margin-left: 10rem; /* 160px */`
    X40,
    /// `margin-top: 10rem; /* 160px */`
    Y40,
    /// `margin-left: 11rem; /* 176px */`
    X44,
    /// `margin-top: 11rem; /* 176px */`
    Y44,
    /// `margin-left: 12rem; /* 192px */`
    X48,
    /// `margin-top: 12rem; /* 192px */`
    Y48,
    /// `margin-left: 13rem; /* 208px */`
    X52,
    /// `margin-top: 13rem; /* 208px */`
    Y52,
    /// `margin-left: 14rem; /* 224px */`
    X56,
    /// `margin-top: 14rem; /* 224px */`
    Y56,
    /// `margin-left: 15rem; /* 240px */`
    X60,
    /// `margin-top: 15rem; /* 240px */`
    Y60,
    /// `margin-left: 16rem; /* 256px */`
    X64,
    /// `margin-top: 16rem; /* 256px */`
    Y64,
    /// `margin-left: 18rem; /* 288px */`
    X72,
    /// `margin-top: 18rem; /* 288px */`
    Y72,
    /// `margin-left: 20rem; /* 320px */`
    X80,
    /// `margin-top: 20rem; /* 320px */`
    Y80,
    /// `margin-left: 24rem; /* 384px */`
    X96,
    /// `margin-top: 24rem; /* 384px */`
    Y96,
    /// `margin-left: 1px;`
    XPx,
    /// `margin-top: 1px;`
    YPx,
    /// `--tw-space-y-reverse: 1;`
    YReverse,
    /// `--tw-space-x-reverse: 1;`
    XReverse,
}
