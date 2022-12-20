use std::fs;

pub fn create_file_object(
    context: &mut v8::Local<v8::Context>,
    scope: &mut v8::ContextScope<v8::HandleScope>,
) {
    let file_name = v8::String::new(scope, "File").unwrap();
    let file = v8::Object::new(scope);

    let read_sync_name = v8::String::new(scope, "readSync").unwrap();
    let read_sync = v8::FunctionTemplate::new(scope, read_sync_callback)
        .get_function(scope)
        .unwrap();

    file.define_own_property(scope, read_sync_name.into(), read_sync.into(), v8::NONE);

    context
        .global(scope)
        .define_own_property(scope, file_name.into(), file.into(), v8::NONE)
        .unwrap();
}

fn read_sync_callback<'a, 'b>(
    scope: &'a mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut returns: v8::ReturnValue<'b>,
) {
    let path = args.get(0).to_rust_string_lossy(scope);
    let contents = fs::read_to_string(path).unwrap();
    returns.set(v8::String::new(scope, &contents).unwrap().into());
}
