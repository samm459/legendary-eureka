use v8;
use std::io::Write;
use std::io;

pub fn print_callback<'a, 'b>(
    scope: &'a mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _returns: v8::ReturnValue<'b>,
) {
    let num_args = args.length();
    for i in 0..num_args {
        let arg = args.get(i);
        let arg_string = arg.to_rust_string_lossy(scope);

        if i == 0 {
            print!("{}", arg_string);
        } else {
            print!(" {}", arg_string);
        }
    }

    io::stdout().flush().unwrap();
}    

pub fn create_print_function(
    context: &mut v8::Local<v8::Context>,
    scope: &mut v8::ContextScope<v8::HandleScope>,
) {
    let print_name = v8::String::new(scope, "print").unwrap().into();
    let print_template = v8::FunctionTemplate::new(scope, print_callback);
    let print_function = print_template.get_function(scope).unwrap();

    context
        .global(scope)
        .define_own_property(scope, print_name, print_function.into(), v8::NONE);
}

pub fn create_functions(
    context: &mut v8::Local<v8::Context>,
    scope: &mut v8::ContextScope<v8::HandleScope>,
) {
    create_print_function(context, scope)
}
