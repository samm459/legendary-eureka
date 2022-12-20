pub mod file;
pub mod module;
pub mod network;
pub mod process;
pub mod system;

pub fn create_objects(
    context: &mut v8::Local<v8::Context>,
    scope: &mut v8::ContextScope<v8::HandleScope>,
) {
    system::create_system_object(context, scope);
    process::create_process_object(context, scope);
    file::create_file_object(context, scope);
}