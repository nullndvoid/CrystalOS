use alloc::format;
use alloc::string::String;
use crate::println;

pub fn run_func(func: String, x: f64) -> Result<f64, String> {
    println!("function being run: {}({})", func, x);

    match func.as_str() {
        "sqrt" => sqrt(x),
        "ln" => ln(x),
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