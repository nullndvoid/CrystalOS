use alloc::format;
use alloc::string::String;
use crate::println;

const PI: f64 = 3.14159265358979323846264338327950288419716939937510;

pub fn run_func(func: String, x: f64) -> Result<f64, String> {
    match func.as_str() {
        "sqrt" => sqrt(x),
        "ln" => ln(x),
        "fact" => factorial(x),
        "sin" => sin(x),
        "cos" => cos(x),
        "tan" => tan(x),
        _ => Err(String::from(format!("unrecognised function name: {}", func))),
    }
}

fn sqrt(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        return Err(String::from("Cannot take the square root of a negative number"));
    }
    Ok(libm::sqrt(x))
}

fn ln(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        return Err(String::from("Cannot take the natural log of a negative number"));
    }
    Ok(libm::log(x))
}

fn factorial(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        return Err(String::from("Cannot take the factorial of a negative number"));
    }
    let x = x as u64;
    Ok((1..=x).fold(1, |a, b| a * b) as f64)
}

fn cos(mut x: f64) -> Result<f64, String> {
    while x > PI {
        x -= PI;
    }
    
    let res = 1.0 - trig_term(x, 2) + trig_term(x, 4) - trig_term(x, 6) + trig_term(x, 8) - trig_term(x, 10);
    if res >= -1.0 && res <= 1.0 {
        Ok(res)
    } else {
        panic!("something is very wrong with the cos function : {}", res);
    }
}

fn sin(mut x: f64) -> Result<f64, String> {
    while x > PI {
        x -= PI;
    }

    
    let res = x - trig_term(x, 3) + trig_term(x, 5) - trig_term(x, 7) + trig_term(x, 9) - trig_term(x, 11);
    if res >= -1.0 && res <= 1.0 {
        Ok(res) 
    } else {
        panic!("something is very wrong with the sin function: {}", res);
    }
}

fn tan(x: f64) -> Result<f64, String> {
    Ok(libm::tan(x))
}

pub fn exp(x: f64, y: f64) -> f64 {
    let mut res = 1.0;
    for _ in 0..(y as usize) {
        res *= x
    }
    res
}

fn trig_term(x: f64, y: usize) -> f64 {
    let mut ex = 1.0;
    for _ in 0..y {
        ex *= x;
    }
    let fact = (1..=y).fold(1, |a, b| a*b);

    ex as f64 / fact as f64
}