fn main() {
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12c;

    println!("base10: {} {} {}", three, thirty, three_hundred);
    println!("base2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base16: {:x} {:x} {:x}", three, thirty, three_hundred);
}