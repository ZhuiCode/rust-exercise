#[derive(Debug)]
struct Person{
    name:String,
    age:u8,
}
fn main() {
    println!("Hello, world!");
    let i = 3.1315926;
    let s = String::from("hello");
    let v = vec![1,2,3];
    let p = Person{name:"sunface".to_string(),age:18};
    println!("{:?}, {:?}, {:?}, {:?}", i, s, v, p);
}
