#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        if self.r == first {
            if self.g == second { 
                std::mem::swap(&mut self.r, &mut self.g);
            } else if self.b == second {
                std::mem::swap(&mut self.r, &mut self.b);
            } else if self.a == second {
                std::mem::swap(&mut self.r, &mut self.a);
            }
        } else if self.g == first {
            if self.r == second {
                std::mem::swap(&mut self.g, &mut self.r);
            } else if self.b == second {
                std::mem::swap(&mut self.g, &mut self.b);
            } else if self.a == second {
                std::mem::swap(&mut self.g, &mut self.a);
            }
        } else if self.b == first {
            if self.r == second {
                std::mem::swap(&mut self.b, &mut self.r);
            } else if self.g == second {
                std::mem::swap(&mut self.b, &mut self.g);
            } else if self.a == second {
                std::mem::swap(&mut self.b, &mut self.a);
            }
        } else if self.a == first {
            if self.r == second {
                std::mem::swap(&mut self.a, &mut self.r);
            } else if self.g == second {
                std::mem::swap(&mut self.a, &mut self.g);
            } else if self.b == second {
                std::mem::swap(&mut self.a, &mut self.b);
            }
        }

        self
    }
}

