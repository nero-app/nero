use rustwind::svg::Fill;
use sycamore::web::{
    tags::{path, svg},
    GlobalProps, SvgGlobalAttributes, View,
};

use crate::utils::ViewBuilder;

pub enum IconType {
    Bookmark,
    Search,
    Sort,
    Share,
    Play,
}

impl IconType {
    /// Returns the view box and the paths of the icon.
    fn attributtes(&self) -> (&'static str, &'static [&'static str]) {
        match self {
            IconType::Bookmark => (
                "0 0 14 19.02",
                &["m13,0L1,0C0.45,0 0,0.45 0,1v17.08c0,0.74 0.79,1.19 1.38,0.78l5.42,-3.64c0.12,-0.08 0.28,-0.08 0.4,0l5.41,3.64c0.6,0.4 1.39,-0.05 1.39,-0.79L14,1c0,-0.55 -0.45,-1 -1,-1ZM12,16.02l-4.42,-2.94c-0.35,-0.23 -0.81,-0.23 -1.16,0l-4.42,2.94L2,2.02h10v14Z"],
            ),
            IconType::Search => (
                "0 0 20 20",
                &["m19.71,18.3l-6.22,-6.3c0.94,-1.25 1.51,-2.81 1.51,-4.5C15,3.36 11.64,0 7.5,0S0,3.36 0,7.5s3.36,7.5 7.5,7.5c1.73,0 3.32,-0.59 4.59,-1.57l6.2,6.28c0.39,0.39 1.02,0.4 1.41,0 0.39,-0.39 0.4,-1.02 0,-1.41ZM2,7.5c0,-3.04 2.46,-5.5 5.5,-5.5s5.5,2.46 5.5,5.5 -2.46,5.5 -5.5,5.5 -5.5,-2.46 -5.5,-5.5Z"],
            ),
            IconType::Sort => (
                "0 0 20 14",
                &[
                    "M1,0L19,0A1,1 0,0 1,20 1L20,1A1,1 0,0 1,19 2L1,2A1,1 0,0 1,0 1L0,1A1,1 0,0 1,1 0z",
                    "M1,6L13,6A1,1 0,0 1,14 7L14,7A1,1 0,0 1,13 8L1,8A1,1 0,0 1,0 7L0,7A1,1 0,0 1,1 6z",
                    "M1,12L7,12A1,1 0,0 1,8 13L8,13A1,1 0,0 1,7 14L1,14A1,1 0,0 1,0 13L0,13A1,1 0,0 1,1 12z",
                ],
            ),
            IconType::Share => (
                "0 0 20 20",
                &["m16,12c-1.39,0 -2.61,0.71 -3.32,1.78l-5.04,-2.14c0.23,-0.5 0.36,-1.05 0.36,-1.64s-0.12,-1.11 -0.34,-1.61l5.04,-2.14c0.72,1.05 1.93,1.75 3.3,1.75 2.21,0 4,-1.79 4,-4s-1.79,-4 -4,-4 -4,1.79 -4,4c0,0.12 0.03,0.24 0.04,0.36l-5.5,2.34c-0.06,0.03 -0.1,0.07 -0.15,0.1 -0.67,-0.5 -1.49,-0.8 -2.38,-0.8C1.79,6 0,7.79 0,10s1.79,4 4,4c0.88,0 1.68,-0.29 2.34,-0.77 0.04,0.02 0.06,0.05 0.1,0.07l5.59,2.37c0,0.11 -0.03,0.21 -0.03,0.32 0,2.21 1.79,4 4,4s4,-1.79 4,-4 -1.79,-4 -4,-4ZM16,2c1.1,0 2,0.9 2,2s-0.9,2 -2,2 -2,-0.9 -2,-2 0.9,-2 2,-2ZM4,12c-1.1,0 -2,-0.9 -2,-2s0.9,-2 2,-2 2,0.9 2,2 -0.9,2 -2,2ZM16,18c-1.1,0 -2,-0.9 -2,-2s0.9,-2 2,-2 2,0.9 2,2 -0.9,2 -2,2Z"]
            ),
            IconType::Play => (
                "0 0 16 18.46",
                &["m15.56,8.46l-7.11,-4.18L1.36,0.13C0.76,-0.23 0,0.21 0,0.9v8.33S0,17.56 0,17.56c0,0.7 0.75,1.13 1.36,0.78l7.08,-4.15 7.12,-4.17c0.59,-0.35 0.59,-1.2 0,-1.55ZM13,9.21l-5.5,3.12 -5.5,3.12v-6.25s0,-6.25 0,-6.25l5.5,3.13 5.49,3.12h0Z"]
            ),
        }
    }
}

pub struct Icon {
    r#type: IconType,
    widht: &'static str,
    height: &'static str,
    fill: Option<Fill>,
}

impl Icon {
    pub fn new(r#type: IconType) -> Self {
        Self {
            r#type,
            widht: "20",
            height: "20",
            fill: None,
        }
    }

    pub fn width(mut self, width: &'static str) -> Self {
        self.widht = width;
        self
    }

    pub fn height(mut self, height: &'static str) -> Self {
        self.height = height;
        self
    }

    pub fn fill(mut self, fill: Fill) -> Self {
        self.fill = Some(fill);
        self
    }
}

impl From<Icon> for View {
    fn from(icon: Icon) -> Self {
        svg()
            .map(|this| {
                let (view_box, paths) = icon.r#type.attributtes();
                paths
                    .iter()
                    .fold(this.viewBox(view_box), |svg, &d| svg.children(path().d(d)))
            })
            .width(icon.widht)
            .height(icon.height)
            .class(icon.fill.unwrap_or(Fill::Black).as_class())
            .into()
    }
}
