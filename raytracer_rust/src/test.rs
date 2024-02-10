fn main() {
    
    let x : u8 = 200;
    let y : u8 = 200;

    let sum = x.overflowing_add(y).0;
    println!("{}", sum);
}
