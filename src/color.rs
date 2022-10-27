use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    const GAMMA: f64 = 2.2;

    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }

    fn from_gray(gray: f64) -> Self {
        Color::new(gray, gray, gray)
    }

    fn from_rgb(red: f64, green: f64, blue: f64) -> Self {
        Color::new(
            f64::powf(red / 255.0, Color::GAMMA),
            f64::powf(green / 255.0, Color::GAMMA),
            f64::powf(blue / 255.0, Color::GAMMA),
        )
    }

    pub fn brightness(&self) -> f64 {
        self.red + self.green + self.blue
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            red: self * rhs.red,
            green: self * rhs.green,
            blue: self * rhs.blue,
        }
    }
}

impl ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Color {
        Color {
            red: self.red / rhs,
            green: self.green / rhs,
            blue: self.blue / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_color() {
        let c = Color::new(0.1, 0.2, 0.3);
        assert_eq!(c.red, 0.1);
        assert_eq!(c.green, 0.2);
        assert_eq!(c.blue, 0.3);
    }

    #[test]
    fn add_assign() {
        let mut c = Color::new(0.1, 0.2, 0.3);
        c += Color::new(0.1, 0.2, 0.3);
        assert_eq!(c.red, 0.2);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 0.6);
    }
}
