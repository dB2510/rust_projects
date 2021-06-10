fn main() {
    let x = 5;
    println!("x is equal to: {}", x);
    let x = 55; // Here this x shadows previous x by using let x
    println!("x is equal to: {}", x); 

    let x = "amazing";
    println!("x is equal to: {}", x);

    // This won't compile because we cannot mutate data type
    // let mut x = 86;
    // x = "best"; 

    const Y: i32 = 7;
    println!("y is equal to: {}", Y);
}
