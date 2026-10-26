// Practice 2: Passing to a function moves ownership.
// EXPECTED: does not compile. error[E0382]: borrow of moved value: `v2`
fn main() {
    let v = vec![10, 20, 30];
    let v2 = v; // moves ownership to v2

    display(v2.clone());
    // v2 is moved into display and is invalidated

    println!("In main {:?}", v2); // v2 is no longer usable here
}

fn display(v: Vec<i32>) {
    println!("inside display {:?}",v);
}