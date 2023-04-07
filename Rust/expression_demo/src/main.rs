// tagless final

// object algebra

// expression problem

// the data

#[derive(Clone)]
struct Rect {
    width: f32,
    height: f32,
}

impl Rect {
    fn print_rect(&self) {
        println!("Rect {{ width: {}, height: {} }}", self.width, self.height);
    }
    fn sum_rect(&self) {
        println!("Sum = {}", self.width + self.height);
    }
}

#[derive(Clone)]
struct Circle {
    radius: f32,
}

impl Circle {
    fn print_circle(&self) {
        println!("Circle {{ radius: {} }}", self.radius);
    }
    fn sum_circle(&self) {
        println!("Sum = {}", self.radius);
    }
}

// OO style

trait ShapeTrait {
    fn trait_print(&self);
    fn trait_sum(&self);
}

impl ShapeTrait for Rect {
    fn trait_print(&self) {
        print!("oo: ");
        self.print_rect();
    }
    fn trait_sum(&self) {
        print!("oo: ");
        self.sum_rect();
    }
}

impl ShapeTrait for Circle {
    fn trait_print(&self) {
        print!("oo: ");
        self.print_circle();
    }
    fn trait_sum(&self) {
        print!("oo: ");
        self.sum_circle();
    }
}

// FP style

enum Shape {
    RECT(Rect),
    CIRCLE(Circle),
}

fn print_shape(shape: &Shape) {
    print!("fp: ");
    match shape {
        Shape::RECT(rect) => rect.print_rect(),
        Shape::CIRCLE(circle) => circle.print_circle(),
    }
}

fn sum_shape(shape: &Shape) {
    print!("fp: ");
    match shape {
        Shape::RECT(rect) => rect.sum_rect(),
        Shape::CIRCLE(circle) => circle.sum_circle(),
    }
}

// Adding new shape
// Adding new operation for all shapes

fn main() {
    let the_rect = Rect {
        width: 3.123,
        height: 1.23,
    };
    let the_circle = Circle { radius: 2.34 };

    let rect_fp = Shape::RECT(the_rect.clone());
    print_shape(&rect_fp);
    sum_shape(&rect_fp);

    let circle_fp = Shape::CIRCLE(the_circle.clone());
    print_shape(&circle_fp);
    sum_shape(&circle_fp);

    let dynrect: Box<dyn ShapeTrait> = Box::new(the_rect);
    dynrect.trait_print();
    dynrect.trait_sum();

    let dyncircle = Box::new(the_circle);
    dyncircle.trait_print();
    dyncircle.trait_sum();
}
