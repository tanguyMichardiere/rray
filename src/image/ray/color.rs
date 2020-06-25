use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

#[allow(dead_code)]
pub const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

#[allow(dead_code)]
pub const WHITE: Color = Color {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};

#[allow(dead_code)]
pub const RED: Color = Color {
    red: 1.0,
    green: 0.0,
    blue: 0.0,
};

#[allow(dead_code)]
pub const GREEN: Color = Color {
    red: 0.0,
    green: 1.0,
    blue: 0.0,
};

#[allow(dead_code)]
pub const BLUE: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 1.0,
};

#[allow(dead_code)]
pub const YELLOW: Color = Color {
    red: 1.0,
    green: 1.0,
    blue: 0.0,
};

#[allow(dead_code)]
pub const MAGENTA: Color = Color {
    red: 1.0,
    green: 0.0,
    blue: 1.0,
};

#[allow(dead_code)]
pub const CYAN: Color = Color {
    red: 0.0,
    green: 1.0,
    blue: 1.0,
};

pub struct SuperColor {
    count: u8,
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }

    pub fn get_red(&self) -> u8 {
        (255.999 * self.red) as u8
    }

    pub fn get_green(&self) -> u8 {
        (255.999 * self.green) as u8
    }

    pub fn get_blue(&self) -> u8 {
        (255.999 * self.blue) as u8
    }
}

impl SuperColor {
    pub fn new() -> Self {
        SuperColor {
            count: 0,
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    pub fn as_color(&self) -> Color {
        Color::new(
            self.red / self.count as f64,
            self.green / self.count as f64,
            self.blue / self.count as f64,
        )
    }

    pub fn add(&mut self, color: Color) {
        self.count += 1;
        self.red += color.red;
        self.green += color.green;
        self.blue += color.blue;
    }
}
