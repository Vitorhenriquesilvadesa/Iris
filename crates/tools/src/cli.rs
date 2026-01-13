use compiler::driver::{CompilerDriver, CompilerOptions};

pub fn main() {
    let options = CompilerOptions::new("C:/Github/Rust/Iris/test/test.iris".into());
    let driver = CompilerDriver::new(options);

    match driver.run() {
        Ok(_) => {}
        Err(_) => {
            panic!();
            // std::process::exit(1)
        }
    }
}
