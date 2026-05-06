use iris_driver::{CompilerDriver, CompilerOptions};

pub fn main() {
    let options = CompilerOptions::new("C:/Github/Rust/Iris/test/test.iris".into());
    let driver = CompilerDriver::new(options);

    match driver.run() {
        true => {}
        false => std::process::exit(1),
    }
}
