use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    // dont_panic();
    // out_of_bounds();
    // open_file();
    dbg!(read_username_from_file());
}

fn dont_panic() {
    panic!("crash and burn")
    // ↳ $ cargo run
    // thread 'main' panicked at 'crash and burn', src/main.rs:2:5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // [16:02] arkham ~/code/the-rust-programming-language/09_error_handling (master)
    // ↳ $ RUST_BACKTRACE=1 cargo run
    // thread 'main' panicked at 'crash and burn', src/main.rs:2:5
    // stack backtrace:
    //    0: rust_begin_unwind
    //              at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/std/src/panicking.rs:584:5
    //    1: core::panicking::panic_fmt
    //              at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/core/src/panicking.rs:142:14
    //    2: error_handling::main
    //              at ./src/main.rs:2:5
    //    3: core::ops::function::FnOnce::call_once
    //              at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/core/src/ops/function.rs:248:5
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
}

fn out_of_bounds() {
    let v = vec![1, 2, 3];
    // let v: Vec<u8> = Vec::with_capacity(10);
    v[99];
}

fn open_file() {
    let result = File::open("hello.txt");

    match result {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                dbg!("file not present");
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                }
            }
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    dbg!("file open ok");

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }

    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username);

    // fs::read_to_string("hello.txt")
}
