use jni::objects::JClass;

use crate::JVM_CLASS_CACHE;

pub fn find_class(name: &str) -> Option<JClass> {
    JVM_CLASS_CACHE.get().map_or(None, |c| {
        c.read()
            .get(name)
            .map(|c| JClass::from(c.as_obj().into_inner()))
    })
}
