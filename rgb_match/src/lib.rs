#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Self {
        match (first, second) {
            (r, g) if r == self.r && g == self.g => {
                self.r = g;
                self.g = r;
            }
            (r, b) if r == self.r && b == self.b => {
                self.r = b;
                self.b = r;
            }
            (r, a) if r == self.r && a == self.a => {
                self.r = a;
                self.a = r;
            }
            (g, r) if g == self.g && r == self.r => {
                self.g = r;
                self.r = g;
            }
            (g, b) if g == self.g && b == self.b => {
                self.g = b;
                self.b = g;
            }
            (g, a) if g == self.g && a == self.a => {
                self.g = a;
                self.a = g;
            }
            (b, r) if b == self.b && r == self.r => {
                self.b = r;
                self.r = b;
            }
            (b, g) if b == self.b && g == self.g => {
                self.b = g;
                self.g = b;
            }
            (b, a) if b == self.b && a == self.a => {
                self.b = a;
                self.a = b;
            }
            (a, r) if a == self.a && r == self.r => {
                self.a = r;
                self.r = a;
            }
            (a, g) if a == self.a && g == self.g => {
                self.a = g;
                self.g = a;
            }
            (a, b) if a == self.a && b == self.b => {
                self.a = b;
                self.b = a;
            }
            _ => return self, // No swap needed
        }

        self
    }
}
