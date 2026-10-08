use std::process::ExitCode;

use skc::driver::ShakvaDriver;

fn main() -> ExitCode {
    ShakvaDriver::default().run()
}
