#![allow(non_snake_case)]

use core::num;
// use core::num::dec2flt::number;
use std::io;
use std::process;

// mod vars;
// mod types;
// mod strings;
// mod tuples;
// mod arrays;
// mod vectors;
// mod loops;
// mod functions;
// mod pointer_ref;
mod structs;

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct CircleBuilder {
    coordinate: f64,
    radius: f64,
}

impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder {coordinate: 0.0, radius: 0.0}
    }

    fn setCoordinate(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.coordinate = coordinate;
        self
    }
    // fn sety(&mut self, y: f64) -> &mut CircleBuilder {
    //     self.y = y;
    //     self
    // }
    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }
    fn finalize(&self) -> Circle {
        Circle {
            x: self.coordinate,
            y: self.coordinate,
            radius: self.radius
        }
    }
}

fn twice<F: Fn(i32) -> i32>(x: i32, f: F) -> i32 {
    f(x) + f(x)
}

fn compose<F, G>(x: i32, f: F, g: G) -> i32
    where F: Fn(i32) -> i32, G: Fn(i32) -> i32 {
        g(f(x))
}

fn main() {
    let c = CircleBuilder::new()
                                .setCoordinate(10.0)
                                // .sety(10.0)
                                .radius(5.0)
                                .finalize();

    println!("area: {}", c.area());

    let x: i32 = 5;
    let printer = || {println!("x is : {}", x)};
    printer();

    let square = |x: i32| { x * x };
    println!("twice of square: {}", twice(5, square));

    let y = compose(5,
        |n| {n + 42},
        |n| {n * 2 });

    let pair: (char, i32) = ('a', 17);
    
    let (some_char, some_int) = ('a', 17);
    // println!("{}", some_char);

    let u = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v = u
        .iter()
        .map(|u| u + 3)
        .fold(0, |x, y| x + y);
    
    let x = {
        let y = 1;
        let z = 2;
        y + z
    };
    println!("{}", x);

    let least = std::cmp::min(3,8);
    println!("{}", least);

    
    struct Vec2 {
        x: f64,
        y: f64,
    }
    let v1 = Vec2 {x: 1.0, y: 3.0};
    let v2 = Vec2 {y: 2.0, x: 4.0};

    let v3 = Vec2 {
        x: 14.0,
        ..v2
    };


    // loop {
    //     println!("please enter a first number: ");
    //     let a = read_user_input();
    //     println!("please enter a second number: ");
    //     let b = read_user_input();
    //     let result = sum(a, b);
    //     println!("{} + {} = {}", a, b, result);
    // }

    let my_first_initial = '*';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let cat = ("Tom", 3.5);
    let (name, age) = cat;
    println!("{} is {}", name, age);
    
    let numbers = (1, 2, 3);
    println!("The second number is {}", numbers.1);

    // vars::run();
    // strings::run();
    // tuples::run();
    // arrays::run();
    // vectors::run();
    // loops::run();
    // functions::run();
    // pointer_ref::run();
    structs::run();

}
struct ColorClassicStruct {
    name: String,
    hex: String,
}

struct ColorTupleStruct {

}

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        let green = ColorClassicStruct {
            name: String::from("green"),
            hex: String::from("#00FF00"),
        };
        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

}


fn sum(a: u32, b: u32) -> u32 {
    a + b
} 

fn read_user_input() -> u32 {
    // println!("please enter a first number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut digit:u32 = 0;

    match input.trim().parse() {
        Ok(val) => {
            digit = val;
        },
        Err(_err) => {
            println!("This is not a valid number");
            process::exit(1);
        }
    }
    digit
}