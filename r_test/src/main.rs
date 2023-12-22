
struct Attribute {
    /// 类型
    r#type: u32,
    /// 数据
    data: f32,
}
fn main() {
    let arr = Attribute(32,23.0);
    println!("Hello, world!{:?}",arr);
}
