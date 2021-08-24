fn main() {
    let one_byte: &[u8; 1] = &[0; 1];
    let five_bytes: &[u8] = &[0; 5];
    dbg!(&one_byte);
    dbg!(&five_bytes);
    dbg!(u8::MAX);
}
