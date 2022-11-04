use jni::{objects::JClass, sys::jstring, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_engineering_lutris_datom_Datom_00024JNI_version(
    env: JNIEnv,
    _: JClass,
) -> jstring {
    let output = env
        .new_string(datom::version())
        .expect("Couldn't create java string!");

    output.into_raw()
}
