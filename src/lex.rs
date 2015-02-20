use std::io::prelude::*;
use error::ObjResult;

pub fn lex<T, F>(input: T, mut callback: F) -> ObjResult<()>
    where T: BufRead, F: FnMut(&str, &[&str]) -> ObjResult<()>
{
    for line in input.lines() {
        let line = try!(line);
        let line = line.split('#').next().unwrap();

        let mut words = line.words();
        match words.next() {
            Some(stmt) => {
                let args: Vec<&str> = words.collect();
                try!(callback(stmt, &args[..]))
            }
            None => ()
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

    lex(&mut input.as_bytes(), |stmt, args| {
        match stmt {
            "statement0" => assert_eq!(args, ["arg0", "arg1", "arg2"]),
            "statement1" => assert_eq!(args, ["arg0", "arg1"]),
            "statement2" => assert_eq!(args, ["Hello,", "world!"]),
            _ => error!(UnexpectedStatement, "Text failed")
        }
        Ok(())
    }).unwrap();
}

#[cfg(test)]
mod bench {
    //! There is a slight overhead (~30ns) in `lex()` function because it passes arguments as a
    //! slice not an iterator.

    extern crate test;

    #[bench]
    fn pass_slice(b: &mut test::Bencher) {
        b.iter(|| {
            let words = "1.00 2.00 3.00".words();
            let args: Vec<&str> = words.collect();
            let args = &args[];

            args.iter().map(|&input| input.parse().unwrap()).collect::<Vec<f32>>();
        })
    }

    #[bench]
    fn pass_iter(b: &mut test::Bencher) {
        b.iter(|| {
            let words = "1.00 2.00 3.00".words();

            words.map(|input| input.parse().unwrap()).collect::<Vec<f32>>();
        })
    }
}
