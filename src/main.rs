//main.rs

extern crate notes;

use notes::move_semantics_of_structs_and_boxed_values as moves;


fn main() {
    moves::move_semantics_of_struct_values_on_heap();
    println!("hello");
}
