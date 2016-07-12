// 1. Move semantics fo Boxed values and Structs

#[derive(Debug)]
pub struct Foo {
    x: i32,
    y: Box<i32>
}

pub fn move_semantics_of_struct_values_on_stack() -> i32 {
    let mut foo = Foo { x: 10, y: Box::new(10) };
    // foo owns instance of Foo, which is allocated on stack
    let a = foo.y;
    // ownership of boxed value is moved to a.
    // now foo is owning the partially moved value
    let b = foo.x;
    // use of foo.x is ok, becase foo stil holds the ownership of i32
    // use of foo.y is error, since foo transfered its ownership
    b
}

pub fn move_semantics_of_struct_values_on_heap() -> i32 {
    let foo = Box::new(Foo { x: 10, y: Box::new(10) });
    // foo owns the boxed value
    // Instance of Foo is allocated on heap.
    let a = foo.y;
    // unlike structs boxes won't allow partial ownership
    // so foo's ownership is moved to a
    // both foo.x and foo.y are illegal
    *a
}
