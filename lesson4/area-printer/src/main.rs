use std::{
    f32::consts::PI,
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

//Area trait, demands type R impls Display for print.
trait Area<R>
where
    R: Display,
{
    fn get_area(&self) -> R;
}

// Triangle struct, you can use any type field with impls basic arithmetic ops Traits
struct Triangle<T>
where
    T: Add + Div + Mul + Sub,
{
    a: T,
    b: T,
    c: T,
}

// Circle struct, you can use any type field with impls basic arithmetic ops Traits
struct Circle<T>
where
    T: Add + Div + Mul + Sub,
{
    radius: T,
}

// Square struct, you can use any type field with impls basic arithmetic ops Traits
struct Square<T>
where
    T: Add + Div + Mul + Sub,
{
    length: T,
    width: T,
}

// impl Area for Triangle with i32 and f32, return a f32 value, the get_area function use Heron's
// formula to calculate the area.
impl Area<f32> for Triangle<i32> {
    fn get_area(&self) -> f32 {
        let s: f32 = ((self.a + self.b + self.c) as f32) / 2.0;
        let area: f32 = s * (s - self.a as f32) * (s - self.b as f32) * (s - self.c as f32);
        area.sqrt()
    }
}

impl Area<f32> for Triangle<f32> {
    fn get_area(&self) -> f32 {
        let s: f32 = (self.a + self.b + self.c) / 2.0;
        let area: f32 = s * (s - self.a) * (s - self.b) * (s - self.c);
        area.sqrt()
    }
}
// impl Area for Square in both u32 and f32
impl Area<u32> for Square<u32> {
    fn get_area(&self) -> u32 {
        self.width * self.length
    }
}

impl Area<f32> for Square<f32> {
    fn get_area(&self) -> f32 {
        self.width * self.length
    }
}

//impl Area for Circle in both u32 and f32
impl Area<f32> for Circle<u32> {
    fn get_area(&self) -> f32 {
        let radius = self.radius as f32;
        radius * radius * PI
    }
}

impl Area<f32> for Circle<f32> {
    fn get_area(&self) -> f32 {
        self.radius * self.radius * PI
    }
}

fn main() {
    // Square with u32 width and length
    let square_u32 = Square {
        width: 20u32,
        length: 30,
    };
    //Square with f32 width and length
    let square_f32 = Square {
        width: 18.0f32,
        length: 26.5,
    };
    // Circle with u32 radius
    let circle_u32 = Circle { radius: 9u32 };
    //Circle with f32 radius
    let circle_f32 = Circle { radius: 12.00f32 };
    // triangle with i32 edges
    let triangle_i32 = Triangle {
        a: 3_i32,
        b: 4,
        c: 5,
    };
    //triangle with f32 edges
    let triangle_f32 = Triangle {
        a: 65.254_f32,
        b: 30.429,
        c: 72.00,
    };
    //print the area for each type
    print!("Square in u32 ");
    print_area(square_u32);

    print!("Square in f32 ");
    print_area(square_f32);

    print!("Circle in u32 ");
    print_area(circle_u32);

    print!("Circle in f32 ");
    print_area(circle_f32);

    print!("Triangle in i32 ");
    print_area(triangle_i32);

    print!("Triangle in f32 ");
    print_area(triangle_f32);
}

//the area printer function
fn print_area<U, R>(shape: U)
where
    U: Area<R>,
    R: Display,
{
    println!("areas is: {}", shape.get_area());
}
