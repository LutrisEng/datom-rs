// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use datom::Fact;
use jni::{
    objects::{JClass, JString},
    sys::{jlong, jstring},
    JNIEnv,
};

#[no_mangle]
pub extern "system" fn Java_engineering_lutris_datom_Fact_00024JNI_fromEdn(
    env: JNIEnv,
    _: JClass,
    edn: JString,
) -> jlong {
    let edn: String = env
        .get_string(edn)
        .expect("Couldn't get Java string!")
        .into();
    let parsed = datom::parse_edn(&edn).expect("Invalid edn");
    let fact = Fact::from_edn(parsed).expect("Invalid fact edn");
    Box::into_raw(Box::new(fact)) as jlong
}

/// # Safety
/// This function interprets `fact_ptr` as a *mut [Fact].
#[no_mangle]
pub unsafe extern "system" fn Java_engineering_lutris_datom_Fact_00024JNI_destroy(
    _: JNIEnv,
    _: JClass,
    fact_ptr: jlong,
) {
    drop(Box::from_raw(fact_ptr as *mut Fact))
}

/// # Safety
/// This function interprets `fact_ptr` as a *mut [Fact].
#[no_mangle]
pub unsafe extern "system" fn Java_engineering_lutris_datom_Fact_00024JNI_toEdn(
    env: JNIEnv,
    _: JClass,
    fact_ptr: jlong,
) -> jstring {
    let fact = fact_ptr as *mut Fact;
    env.new_string((*fact).to_edn())
        .expect("Failed to create string")
        .into_raw()
}
