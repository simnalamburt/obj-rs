#[experimental]
pub fn lex<T: Reader>(input: &mut T) {
    println!("{}", input.read_to_string().unwrap());
}

#[test]
fn test_lex() {
    let string = "Hello, world!".to_string();

    let mut reader = std::io::MemReader::new(string.into_bytes());
    lex(&mut reader);
}
