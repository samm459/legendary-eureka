extern crate v8;

use clearscreen;
use gethostname;

use std::io;
use std::io::Write;
use std::env;
use std::process;

fn create_system_object(
    context: &mut v8::Local<v8::Context>, 
    scope: &mut v8::ContextScope<v8::HandleScope>
) {
    let system_name = v8::String::new(scope, "System").unwrap();
    let system = v8::Object::new(scope);

    let platform_name = v8::String::new(scope, "platform").unwrap();
    let platform = v8::String::new(scope, env::consts::OS).unwrap();

    let architecture_name = v8::String::new(scope, "architecture").unwrap();
    let architecture = v8::String::new(scope, env::consts::ARCH).unwrap();

    let name_name = v8::String::new(scope, "name").unwrap();
    let name = v8::String::new(scope, &format!("{:?}", gethostname::gethostname())).unwrap();
    
    system.define_own_property(scope, platform_name.into(), platform.into(), v8::NONE);
    system.define_own_property(scope, architecture_name.into(), architecture.into(), v8::NONE);
    system.define_own_property(scope, name_name.into(), name.into(), v8::NONE);

    context.global(scope).define_own_property(scope, system_name.into(), system.into(), v8::NONE).unwrap();
}

fn main() {
    // Initialize V8
    let platform = v8::new_default_platform(1, false);
    v8::V8::initialize_platform(platform.into());
    v8::V8::initialize();

    // Create a new V8 isolate
    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    let scope = &mut v8::HandleScope::new(&mut isolate);
    let mut context = v8::Context::new(scope);
    let mut scope = v8::ContextScope::new(scope, context);

    // Add platinum objects to JavaScript
    create_system_object(&mut context, &mut scope);
    
    // Clear the screen
    clearscreen::clear().unwrap();

    // Start a REPL
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
