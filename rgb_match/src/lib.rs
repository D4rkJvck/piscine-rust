#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Self {
        
        if first == self.r {
            if second == self.g {
                self.r = second;
                self.g = first;
            } else if second == self.b {
                self.r = second;
                self.b = first;
            } else if second == self.a {
                self.r = second;
                self.a = first;
            } else {
                return self
            }
        } else if first == self.g {
            if second == self.r {
                self.g = second;
                self.r = first;
            } else if second == self.b {
                self.g = second;
                self.b = first;
            } else if second == self.a {
                self.g = second;
                self.a = first;
            } else {
                return self
            }
        } else if first == self.b {
            if second == self.r {
                self.b = second;
                self.r = first;
            } else if second == self.g {
                self.b = second;
                self.g = first;
            } else if second == self.a {
                self.b = second;
                self.a = first;
            } else {
                return self
            }
        } else if first == self.a {
            if second == self.r {
                self.a = second;
                self.r = first;
            } else if second == self.g {
                self.a = second;
                self.g = first;
            } else if second == self.b {
                self.a = second;
                self.b = first;
            } else {
                return self
            }
        }

        self
    }
}
