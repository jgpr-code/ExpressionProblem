// tagless final

// object algebra

// expression problem

// the data

// OO style

mod extern_datatypes;

use crate::extern_datatypes::*;

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

// enum SumTypes<L, R> {
//     Left(L),
//     Right(R),
// }

// Adding new shape
// Adding new operation for all shapes

trait LitSym<T> {
    fn lit(n: i32) -> T;
}

trait NegSym<T> {
    fn neg(n: T) -> T;
}

trait AddSym<T> {
    fn add(e1: T, e2: T) -> T;
}

fn tf1<T>() -> T
where
    T: LitSym<T> + NegSym<T> + AddSym<T>,
{
    T::add(T::lit(10), T::neg(T::lit(2)))
}

impl LitSym<i32> for i32 {
    fn lit(n: i32) -> i32 {
        n
    }
}

impl NegSym<i32> for i32 {
    fn neg(n: i32) -> i32 {
        -n
    }
}

impl AddSym<i32> for i32 {
    fn add(e1: i32, e2: i32) -> i32 {
        e1 + e2
    }
}

// adding new language construct mul

trait MulSym<T> {
    fn mul(e1: T, e2: T) -> T;
}

impl MulSym<i32> for i32 {
    fn mul(e1: i32, e2: i32) -> i32 {
        e1 * e2
    }
}

fn tfm1<T>() -> T
where
    T: LitSym<T> + NegSym<T> + MulSym<T>,
{
    T::mul(T::neg(T::lit(2)), T::neg(T::lit(3)))
}

// adding a new interpretation String

impl LitSym<String> for String {
    fn lit(n: i32) -> String {
        n.to_string()
    }
}

impl NegSym<String> for String {
    fn neg(n: String) -> String {
        format!("-{}", n)
    }
}

impl AddSym<String> for String {
    fn add(e1: String, e2: String) -> String {
        format!("{} + {}", e1, e2)
    }
}

impl MulSym<String> for String {
    fn mul(e1: String, e2: String) -> String {
        format!("{} * {}", e1, e2)
    }
}

fn main() {
    println!("{} = {}", tf1::<String>(), tf1::<i32>());
    println!("{} = {}", tfm1::<String>(), tfm1::<i32>());
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

// lit: Int
// add: Int -> Int -> Int
// mul: Int -> Int -> Int
// neg: Int -> Int

// Expr:
// eval: Expr -> Int
// print: Expr -> String

// lit: Int -> Expr
// add: Expr -> Expr -> Expr
// mul: Expr -> Expr -> Expr
// neg: Expr -> Expr
