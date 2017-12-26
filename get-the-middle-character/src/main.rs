fn main() {
    let hello = "this is world";

    let s = &hello[0 as usize..4];

    println!("{}", s);
}
