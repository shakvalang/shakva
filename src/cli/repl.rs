use std::io::{Write, stdin, stdout};

pub(super) fn repl() {
    let input = stdin();
    let mut output = stdout();
    let mut buf = String::new();

    loop {
        print!(">> ");
        output.flush().expect("unable to flush stdout");
    }
}
