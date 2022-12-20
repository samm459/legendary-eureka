use std::process;
use tokio::sync::Mutex;
use tokio;
use tokio::io;
use tokio::io::AsyncBufReadExt;

fn stdin_async_iterator_next_callback<'a, 'b>(
    scope: &'a mut v8::HandleScope,
    _args: v8::FunctionCallbackArguments,
    mut returns: v8::ReturnValue<'b>,
) {
    let stdin_line_promise_resolver = Mutex::new(v8::PromiseResolver::new(scope).unwrap());
    let promise_scope = Mutex::new(scope);

    tokio::spawn(async move {
        let mut input = io::BufReader::new(io::stdin());
        let mut line = String::new();
        let mut scope = promise_scope.lock().await;
        let mut stdin_async_iterator =  v8::Object::new(&mut scope);
    
        // Read a line from standard input
        input.read_line(&mut line).await.unwrap();

        stdin_async_iterator.set(
            &mut scope,
            v8::String::new(&mut scope, "done").unwrap().into(),
            v8::String::new(&mut scope, "true").unwrap().into(),
        );

        stdin_async_iterator.set(
            &mut scope,
            v8::String::new(&mut scope, "value").unwrap().into(),
            v8::String::new(&mut scope, &line).unwrap().into(),
        );

        stdin_line_promise_resolver.lock().await.resolve(&mut scope, stdin_async_iterator.to_object(&mut scope).unwrap().into());
    });

    let stdin_line_promise = stdin_line_promise_resolver.blocking_lock().get_promise(scope);
    returns.set(stdin_line_promise.into());
}

fn stdin_async_iterator_callback<'a, 'b>(
    scope: &'a mut v8::HandleScope,
    _args: v8::FunctionCallbackArguments,
    mut returns: v8::ReturnValue<'b>,
) {
    let stdin_async_iterator_template = v8::ObjectTemplate::new(scope);

    stdin_async_iterator_template.set(
        v8::String::new(scope, "next").unwrap().into(),
        v8::FunctionTemplate::new(scope, stdin_async_iterator_next_callback).into(),
    );

    returns.set(
        stdin_async_iterator_template
            .new_instance(scope)
            .unwrap()
            .into(),
    );
}

pub fn create_stdin_object<'a, 'b>(
    process: &mut v8::Local<v8::Object>,
    scope: &mut v8::ContextScope<v8::HandleScope<'a>>,
) {
    let stdin_sync_template = v8::ObjectTemplate::new(scope);

    // Define the Symbol.Iterator property
    stdin_sync_template.set(
        v8::Symbol::get_async_iterator(scope).into(),
        v8::FunctionTemplate::new(scope, stdin_async_iterator_callback).into(),
    );

    let stdin_sync_name = v8::String::new(scope, "stdinSync").unwrap();
    let stdin_sync = stdin_sync_template.new_instance(scope).unwrap();

    process.define_own_property(scope, stdin_sync_name.into(), stdin_sync.into(), v8::NONE);
}

pub fn create_process_object<'a, 'b>(
    context: &mut v8::Local<v8::Context>,
    scope: &mut v8::ContextScope<v8::HandleScope<'a>>,
) {
    let process_name = v8::String::new(scope, "Process").unwrap();
    let mut process = v8::Object::new(scope);

    let id_name = v8::String::new(scope, "id").unwrap();
    let id = v8::Number::new(scope, process::id() as f64);

    create_stdin_object(&mut process, scope);

    process.define_own_property(scope, id_name.into(), id.into(), v8::NONE);

    context
        .global(scope)
        .define_own_property(scope, process_name.into(), process.into(), v8::NONE)
        .unwrap();
}
