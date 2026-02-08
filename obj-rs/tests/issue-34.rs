use obj::{LoadError, LoadErrorKind, Obj, ObjError, load_obj};
use std::io::Cursor;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn issue_34() -> TestResult {
    let cursor = Cursor::new(b"p -18");
    let result: Result<Obj, _> = load_obj(cursor);

    match result {
        Ok(_) => assert!(false, "Shouldn't success"),
        Err(ObjError::Load(e @ LoadError { .. })) => {
            assert_eq!(*e.kind(), LoadErrorKind::IndexOutOfRange)
        }
        Err(_) => assert!(false, "Wrong error type"),
    }

    Ok(())
}
