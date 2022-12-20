use clearscreen;
use v8::CreateParams;
use std::{io, env};
use std::io::Write;
use std::fs;
use v8;

fn main() {
    // Initialize V8
    let platform = v8::new_default_platform(1, false);
    v8::V8::initialize_platform(platform.into());
    v8::V8::initialize();

    // Create a new V8 isolate
    let mut isolate = v8::Isolate::new(CreateParams::default());
    let scope = &mut v8::HandleScope::new(&mut isolate);
    let mut context = v8::Context::new(scope);
    let mut scope = v8::ContextScope::new(scope, context);

    // Add platinum runtime globals
    platinum::runtime::create_globals(&mut context, &mut scope);

    // If there is a file path argument, run the file
    if let Some(arg) = env::args().nth(1) {
        let contents = fs::read_to_string(arg).unwrap();
        let source = v8::String::new(&mut scope, &contents).unwrap();
        let script = v8::Script::compile(&mut scope, source, None).unwrap();

        script.run(&mut scope).unwrap();
        return;
    }

    // Otherwise, start a REPL
    clearscreen::clear().unwrap();
    let mut input = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        // Run the JavaScript code
        let source = v8::String::new(&mut scope, &input).unwrap();
        let script = v8::Script::compile(&mut scope, source, None).unwrap();
        let result = script.run(&mut scope).unwrap();

        // Print the result
        let result_str = result.to_string(&mut scope).unwrap();
        println!("{}", result_str.to_rust_string_lossy(&mut scope));
    }
}