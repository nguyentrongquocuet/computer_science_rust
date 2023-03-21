pub mod binary;

fn main() {
    println!("Hello, world!");
    let result = binary::core::int2bin(19);

    println!("{result}")
}
