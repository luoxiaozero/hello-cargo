use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn aa(self) {

    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn main() {
    let my = MyBox("ss");
    my.to_string();
    my.aa();
}