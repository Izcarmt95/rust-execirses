fn main() {
    let mut counter: u16 = 0;
    loop {
        println!("Hello, world!");
        counter += 1;

        if counter == 100 {
            break;
        }
    }
}
