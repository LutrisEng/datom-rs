use datom::{backends::RedBlackTreeSetStorage, DynamicConnection};
use jni::{objects::JClass, sys::jlong, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_engineering_lutris_datom_Connection_00024JNI_create(
    _: JNIEnv,
    _: JClass,
) -> jlong {
    let connection = DynamicConnection::new(Box::new(RedBlackTreeSetStorage::new()));
    Box::into_raw(Box::new(connection)) as jlong
}

/// # Safety
/// This function interprets `connection_ptr` as a *mut [DynamicConnection].
#[no_mangle]
pub unsafe extern "system" fn Java_engineering_lutris_datom_Connection_00024JNI_destroy(
    _: JNIEnv,
    _: JClass,
    connection_ptr: jlong,
) {
    drop(Box::from_raw(connection_ptr as *mut DynamicConnection))
}

/// # Safety
/// This function interprets `connection_ptr` as a *mut [DynamicConnection].
#[no_mangle]
pub unsafe extern "system" fn Java_engineering_lutris_datom_Connection_00024JNI_latestT(
    _: JNIEnv,
    _: JClass,
    connection_ptr: jlong,
) -> jlong {
    let connection = connection_ptr as *mut DynamicConnection;
    (*connection).latest_t().expect("Couldn't get latest T!") as jlong
}
