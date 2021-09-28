use obj::{load_obj, Obj};
use std::io::Cursor;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn issue_50_a() -> TestResult {
    let _: Obj = load_obj(Cursor::new(b"s 1000000000000000000"))?;
    Ok(())
}

#[test]
fn issue_50_b() -> TestResult {
    let _: Obj = load_obj(Cursor::new(b"s 100000000000000000"))?;
    Ok(())
}

#[test]
fn issue_50_c() -> TestResult {
    let _: Obj = load_obj(Cursor::new(b"s 100000000000"))?;
    Ok(())
}

#[test]
fn issue_50_d() -> TestResult {
    let _: Obj = load_obj(Cursor::new(b"s 67108864"))?;
    Ok(())
}
