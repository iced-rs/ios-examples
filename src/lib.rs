extern crate iced;

// If you have a local path to iced that's up a directory, uncomment this line
//#[path = "../../../iced/examples/todos.rs"]
mod todos;

#[no_mangle]
pub extern "C" fn run_app() {
    color_backtrace::install();
    todos::main();
}
