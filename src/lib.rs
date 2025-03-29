use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq, Debug)]
pub struct MyType(pub i32);

impl Hash for MyType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // New hash: reverse the value
        (!self.0).hash(state);
    }
}
