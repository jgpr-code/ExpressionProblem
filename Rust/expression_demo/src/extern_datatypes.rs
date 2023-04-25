#[derive(Clone)]
pub struct Rect {
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn print_rect(&self) {
        println!("Rect {{ width: {}, height: {} }}", self.width, self.height);
    }
    pub fn sum_rect(&self) {
        println!("Sum = {}", self.width + self.height);
    }
}

#[derive(Clone)]
pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn print_circle(&self) {
        println!("Circle {{ radius: {} }}", self.radius);
    }
    pub fn sum_circle(&self) {
        println!("Sum = {}", self.radius);
    }
}
