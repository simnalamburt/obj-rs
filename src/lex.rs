use std::io::IoError;

pub fn lex<T: Buffer>(input: &mut T, callback: |&str, &[&str]|) -> Option<IoError> {
    for maybe_line in input.lines() {
        match maybe_line {
            Ok(line) => {
                let line = line.as_slice();
                let line = line.split('#').next().unwrap();

                let mut words = line.words();
                match words.next() {
                    Some(stmt) => {
                        let args: Vec<&str> = words.collect();
                        callback(stmt, args.as_slice())
                    }
                    None => {}
                }
            }
            Err(e) => { return Some(e); }
        }
    }
    None
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
            _ => panic!()
        }
    });
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
            let args = args.as_slice();

            args.iter().map(|&input| from_str::<f32>(input).unwrap()).collect::<Vec<f32>>();
        })
    }

    #[bench]
    fn pass_iter(b: &mut test::Bencher) {
        b.iter(|| {
            let words = "1.00 2.00 3.00".words();

            words.map(|input| from_str::<f32>(input).unwrap()).collect::<Vec<f32>>();
        })
    }
}
