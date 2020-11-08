use cxx::CxxString;

#[cxx::bridge(namespace = "mcrestool::lib")]
mod ffi {
    extern "Rust" {
        fn hello(arg: &CxxString);
    }
}

fn hello(arg: &CxxString) {
    println!("Hello from Rust, {}!", arg);
}
