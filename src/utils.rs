use std::any::type_name;

#[allow(dead_code)]
pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
