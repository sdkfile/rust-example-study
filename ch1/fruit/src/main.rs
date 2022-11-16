fn main() {
    let fruit = vec!['🍎', '🍊', '🍌', '🍉', '🍇'];
    let buffer_overflow = fruit[5];
    assert_eq!(buffer_overflow, '🍇');
}
