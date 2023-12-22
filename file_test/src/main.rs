use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "lines.txt";

    // 创建文件
    let mut output = File::create(path)?;
    // 写入三行内容
    write!(output, "Rust\n💖\nFun")?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    // 迭代文件中的每一行内容，line 是字符串
    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}