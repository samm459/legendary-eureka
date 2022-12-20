pub mod constants;
pub mod functions;
pub mod objects;

pub fn create_globals(
    context: &mut v8::Local<v8::Context>,
    scope: &mut v8::ContextScope<v8::HandleScope>,
) {
    objects::create_objects(context, scope);
    functions::create_functions(context, scope);
    constants::create_constants(context, scope);
}