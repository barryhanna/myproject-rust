fn main() {
    let x: u8 = 2;
    let mut y: u8 = 8;
    let scores = (1, 'c', true);
    let arr = [23, 34, 56];
    let sli = &arr[0..1];
    y += 1;
    println!("Hello, world! {} {}", x, y);
    println!("{}", x << 1);
    println!("{}", scores.0);
    println!("{}", scores.2);
    println!("{:?}", arr);
    println!("{:?}", sli);
}
