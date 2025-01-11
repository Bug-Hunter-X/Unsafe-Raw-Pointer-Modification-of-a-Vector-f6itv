fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 42; // Safe modification using vector indexing
    println!("v = {:?}", v);
}