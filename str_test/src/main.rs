fn main() {
    let v = String::from("🗻∈🌏");
    println!("{:?}",v.get(0..4));
    assert_eq!(Some("🗻"), v.get(0..4));

    // indices not on UTF-8 sequence boundaries
    assert!(v.get(1..).is_none());
    assert!(v.get(..8).is_none());

    // out of bounds
    assert!(v.get(..42).is_none());
}
