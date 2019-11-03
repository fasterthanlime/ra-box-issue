use rawsock::open_best_library;
use std::error::Error;

fn boxed_string_result() -> Result<Box<String>, Box<dyn Error>> {
    unimplemented!()
}

fn string_result() -> Result<String, Box<dyn Error>> {
    unimplemented!()
}

fn main() -> Result<(), Box<dyn Error>> {
    let _lib = open_best_library();
    let _lib = open_best_library().unwrap();
    let _lib = open_best_library()?;

    let _str = boxed_string_result();
    let _str = boxed_string_result().unwrap();
    let _str = boxed_string_result()?;

    let _bstr = string_result();
    let _bstr = string_result().unwrap();
    let _bstr = string_result()?;

    Ok(())
}
