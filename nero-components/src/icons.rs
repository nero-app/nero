use sycamore::web::{
    tags::{path, svg},
    GlobalAttributes, GlobalProps, View,
};

/// Possible icon types.
pub enum IconType {
    Search,
    Sort,
    Play,
    Share,
}

/// Represents an icon with configurable properties for width, height, color and icon type.
pub struct Icon {
    width: u16,
    height: u16,
    hex_color: Option<u32>,
    icon: IconType,
}

impl Icon {
    /// Creates a new icon with the specified `IconType`.
    ///
    /// By default, the icon is created with a 20x20 size.
    pub fn new(icon: IconType) -> Self {
        Self {
            width: 20,
            height: 20,
            hex_color: None,
            icon,
        }
    }

    /// Sets the width of the icon.
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the icon.
    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    /// Sets the hex color of the icon.
    ///
    /// # Example
    /// ```
    /// use nero_components::{Icon, IconType};
    ///
    /// let icon = Icon::new(IconType::Search)
    ///     .hex_color(0xFF0000);
    /// ```
    pub fn hex_color(mut self, hex_color: u32) -> Self {
        self.hex_color = Some(hex_color);
        self
    }
}

impl From<Icon> for View {
    fn from(value: Icon) -> Self {
        match value.icon {
            IconType::Search => svg()
                .attr("viewBox", "0 0 20 20")
                .children(
                    path().attr("d", "m19.71,18.3l-6.22,-6.3c0.94,-1.25 1.51,-2.81 1.51,-4.5C15,3.36 11.64,0 7.5,0S0,3.36 0,7.5s3.36,7.5 7.5,7.5c1.73,0 3.32,-0.59 4.59,-1.57l6.2,6.28c0.39,0.39 1.02,0.4 1.41,0 0.39,-0.39 0.4,-1.02 0,-1.41ZM2,7.5c0,-3.04 2.46,-5.5 5.5,-5.5s5.5,2.46 5.5,5.5 -2.46,5.5 -5.5,5.5 -5.5,-2.46 -5.5,-5.5Z")
                ),
            IconType::Sort => svg()
                .attr("viewBox", "0 0 20 14")
                .children(
                    (path().attr("d", "M1,0L19,0A1,1 0,0 1,20 1L20,1A1,1 0,0 1,19 2L1,2A1,1 0,0 1,0 1L0,1A1,1 0,0 1,1 0z"),
                    path().attr("d", "M1,6L13,6A1,1 0,0 1,14 7L14,7A1,1 0,0 1,13 8L1,8A1,1 0,0 1,0 7L0,7A1,1 0,0 1,1 6z"),
                    path().attr("d", "M1,12L7,12A1,1 0,0 1,8 13L8,13A1,1 0,0 1,7 14L1,14A1,1 0,0 1,0 13L0,13A1,1 0,0 1,1 12z"))
                ),
            IconType::Play => svg()
                .attr("viewBox", "0 0 16 18.46")
                .children(
                    path().attr("d", "m15.56,8.46l-7.11,-4.18L1.36,0.13C0.76,-0.23 0,0.21 0,0.9v8.33S0,17.56 0,17.56c0,0.7 0.75,1.13 1.36,0.78l7.08,-4.15 7.12,-4.17c0.59,-0.35 0.59,-1.2 0,-1.55ZM13,9.21l-5.5,3.12 -5.5,3.12v-6.25s0,-6.25 0,-6.25l5.5,3.13 5.49,3.12h0Z")
                ),
            IconType::Share => svg()
                .attr("viewBox", "0 0 20 20")
                .children(
                    path().attr("d", "m16,12c-1.39,0 -2.61,0.71 -3.32,1.78l-5.04,-2.14c0.23,-0.5 0.36,-1.05 0.36,-1.64s-0.12,-1.11 -0.34,-1.61l5.04,-2.14c0.72,1.05 1.93,1.75 3.3,1.75 2.21,0 4,-1.79 4,-4s-1.79,-4 -4,-4 -4,1.79 -4,4c0,0.12 0.03,0.24 0.04,0.36l-5.5,2.34c-0.06,0.03 -0.1,0.07 -0.15,0.1 -0.67,-0.5 -1.49,-0.8 -2.38,-0.8C1.79,6 0,7.79 0,10s1.79,4 4,4c0.88,0 1.68,-0.29 2.34,-0.77 0.04,0.02 0.06,0.05 0.1,0.07l5.59,2.37c0,0.11 -0.03,0.21 -0.03,0.32 0,2.21 1.79,4 4,4s4,-1.79 4,-4 -1.79,-4 -4,-4ZM16,2c1.1,0 2,0.9 2,2s-0.9,2 -2,2 -2,-0.9 -2,-2 0.9,-2 2,-2ZM4,12c-1.1,0 -2,-0.9 -2,-2s0.9,-2 2,-2 2,0.9 2,2 -0.9,2 -2,2ZM16,18c-1.1,0 -2,-0.9 -2,-2s0.9,-2 2,-2 2,0.9 2,2 -0.9,2 -2,2Z")
                ),
        }
        .attr("fill", format!("#{:06x}", value.hex_color.unwrap_or(0x000000)))
        // FIXME: 
        // .attr("width", self.width)
        // .attr("height", self.height)
        .attr("width", "20")
        .attr("height", "20")
        .into()
    }
}
