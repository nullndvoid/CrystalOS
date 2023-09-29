use alloc::format;
use alloc::string::String;
use crate::println;

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

fn sin(x: f64) -> Result<f64, String> {
    Ok(libm::sin(x))
}

fn cos(x: f64) -> Result<f64, String> {
    Ok(libm::cos(x))
}

fn tan(x: f64) -> Result<f64, String> {
    Ok(libm::tan(x))
}