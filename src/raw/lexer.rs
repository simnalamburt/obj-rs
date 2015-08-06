use std::io::prelude::*;
use error::ObjResult;

pub fn lex<T, F>(input: T, mut callback: F) -> ObjResult<()>
    where T: BufRead, F: FnMut(&str, &[&str]) -> ObjResult<()>
{
    for line in input.lines() {
        let line = try!(line);
        let line = line.split('#').next().unwrap(); // Remove comments
        let mut words = line.split_whitespace();

        if let Some(stmt) = words.next() {
            let args: Vec<_> = words.collect();
            try!(callback(stmt, &args[..]))
        }
    }

    Ok(())
}

#[test]
fn test_lex() {
    let input = r#"
   statement0      arg0  arg1	arg2#argX   argX
statement1 arg0    arg1
# Comment
statement2 Hello, world!
"#;

    assert!(lex(&mut input.as_bytes(), |stmt, args| {
        match stmt {
            "statement0" => assert_eq!(args, ["arg0", "arg1", "arg2"]),
            "statement1" => assert_eq!(args, ["arg0", "arg1"]),
            "statement2" => assert_eq!(args, ["Hello,", "world!"]),
            _ => panic!("Unit test failed")
        }
        Ok(())
    }).is_ok());
}
