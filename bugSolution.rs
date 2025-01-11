fn main() {
    let mut v = vec![1, 2, 3];
    for i in 0..v.len() {
        v[i] = v[i] * 2; // Modify in place
    }
    println!("{:?}", v);
    // Or using iterators for more complex scenarios
    let v2 = vec![1, 2, 3];
    let v3: Vec<_> = v2.iter().map(|&x| x * 2).collect();
    println!("{:?}", v3);
}