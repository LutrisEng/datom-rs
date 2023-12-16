(function() {var type_impls = {
"jni":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3C%26AutoArray%3C'a,+T%3E%3E-for-*mut+T\" class=\"impl\"><a class=\"src rightside\" href=\"src/jni/wrapper/objects/auto_array.rs.html#133-137\">source</a><a href=\"#impl-From%3C%26AutoArray%3C'a,+T%3E%3E-for-*mut+T\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T: <a class=\"trait\" href=\"jni/objects/trait.TypeArray.html\" title=\"trait jni::objects::TypeArray\">TypeArray</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"struct\" href=\"jni/objects/struct.AutoArray.html\" title=\"struct jni::objects::AutoArray\">AutoArray</a>&lt;'a, T&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.pointer.html\">*mut T</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/jni/wrapper/objects/auto_array.rs.html#134-136\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(other: &amp;'a <a class=\"struct\" href=\"jni/objects/struct.AutoArray.html\" title=\"struct jni::objects::AutoArray\">AutoArray</a>&lt;'_, T&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.pointer.html\">*mut T</a></h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<&'a AutoArray<'a, T>>","jni::sys::va_list","jni::sys::jobject","jni::sys::jclass","jni::sys::jthrowable","jni::sys::jstring","jni::sys::jarray","jni::sys::jbooleanArray","jni::sys::jbyteArray","jni::sys::jcharArray","jni::sys::jshortArray","jni::sys::jintArray","jni::sys::jlongArray","jni::sys::jfloatArray","jni::sys::jdoubleArray","jni::sys::jobjectArray","jni::sys::jweak","jni::sys::jfieldID","jni::sys::jmethodID"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3C%26AutoPrimitiveArray%3C'a,+'_%3E%3E-for-*mut+c_void\" class=\"impl\"><a class=\"src rightside\" href=\"src/jni/wrapper/objects/auto_primitive_array.rs.html#86-90\">source</a><a href=\"#impl-From%3C%26AutoPrimitiveArray%3C'a,+'_%3E%3E-for-*mut+c_void\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"struct\" href=\"jni/objects/struct.AutoPrimitiveArray.html\" title=\"struct jni::objects::AutoPrimitiveArray\">AutoPrimitiveArray</a>&lt;'a, '_&gt;&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.pointer.html\">*mut </a><a class=\"type\" href=\"https://doc.rust-lang.org/nightly/std/os/raw/type.c_void.html\" title=\"type std::os::raw::c_void\">c_void</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/jni/wrapper/objects/auto_primitive_array.rs.html#87-89\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(other: &amp;'a <a class=\"struct\" href=\"jni/objects/struct.AutoPrimitiveArray.html\" title=\"struct jni::objects::AutoPrimitiveArray\">AutoPrimitiveArray</a>&lt;'_, '_&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.pointer.html\">*mut </a><a class=\"type\" href=\"https://doc.rust-lang.org/nightly/std/os/raw/type.c_void.html\" title=\"type std::os::raw::c_void\">c_void</a></h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<&'a AutoPrimitiveArray<'a, '_>>","jni::sys::va_list"]],
"jni_sys":[],
"neon_runtime":[]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()