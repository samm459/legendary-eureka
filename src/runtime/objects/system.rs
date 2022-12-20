use std::env;

pub fn create_system_object(
    context: &mut v8::Local<v8::Context>,
    scope: &mut v8::ContextScope<v8::HandleScope>,
) {
    let system_name = v8::String::new(scope, "System").unwrap();
    let system = v8::Object::new(scope);

    let platform_name = v8::String::new(scope, "platform").unwrap();
    let platform = v8::String::new(scope, env::consts::OS).unwrap();

    let architecture_name = v8::String::new(scope, "architecture").unwrap();
    let architecture = v8::String::new(scope, env::consts::ARCH).unwrap();

    let name_name = v8::String::new(scope, "name").unwrap();
    let name = v8::String::new(scope, gethostname::gethostname().to_str().unwrap()).unwrap();

    system.define_own_property(scope, platform_name.into(), platform.into(), v8::NONE);
    system.define_own_property(
        scope,
        architecture_name.into(),
        architecture.into(),
        v8::NONE,
    );
    system.define_own_property(scope, name_name.into(), name.into(), v8::NONE);

    context
        .global(scope)
        .define_own_property(scope, system_name.into(), system.into(), v8::NONE)
        .unwrap();
}