use std::io::Read;

fn main() {
    let mut bytes: &[u8] = &[1; 5];
    let mut buf = [0; 3];
    bytes.read_exact(&mut buf).unwrap();
    dbg!(&buf);

    let bytes: Vec<u8> = vec![1; 5];
    let mut buf = [0; 3];
    // VecはReadを実装してないので一旦スライスに変換する
    (&bytes[..]).read_exact(&mut buf).unwrap();
}
