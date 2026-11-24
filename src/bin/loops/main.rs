fn main() {
    let mut counter = 0u32;

    let loop_result: u32 = loop {
        counter += 1;
        if counter >= 10 {
            break counter;
        }
    };

    println!("{}", loop_result);
}
