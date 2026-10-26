// Practice 1: Assignment moves ownership.
// EXPECTED: does not compile. error[E0382]: borrow of moved value: `v`
fn main() {
    let v = vec![101, 250, 330, 400];
    // vector v owns the object in heap

    // only a single variable owns the heap memory at any given time
    let v2 = v;
    // two pointers to the same content is not allowed in Rust

    println!("{:?}", v2); // v was moved into v2 above
}
