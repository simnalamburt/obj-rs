use obj::{load_obj, LoadErrorKind, Obj, ObjError};
use std::io::Cursor;

#[test]
fn issue_50_a() {
    let res: Result<Obj, _> = load_obj(Cursor::new(b"s 1000000000000000000"));
    let err = res.expect_err("Expect proper error");
    if let ObjError::Load(load) = err {
        assert_eq!(*load.kind(), LoadErrorKind::TooBigGroupNumber);
    } else {
        panic!("Unexpected error type");
    }
}

#[test]
fn issue_50_b() {
    let res: Result<Obj, _> = load_obj(Cursor::new(b"s 100000000000000000"));
    let err = res.expect_err("Expect proper error");
    if let ObjError::Load(load) = err {
        assert_eq!(*load.kind(), LoadErrorKind::TooBigGroupNumber);
    } else {
        panic!("Unexpected error type");
    }
}

#[test]
fn issue_50_c() {
    let res: Result<Obj, _> = load_obj(Cursor::new(b"s 100000000000"));
    let err = res.expect_err("Expect proper error");
    if let ObjError::Load(load) = err {
        assert_eq!(*load.kind(), LoadErrorKind::TooBigGroupNumber);
    } else {
        panic!("Unexpected error type");
    }
}

#[test]
fn issue_50_d() {
    let obj: Obj = load_obj(Cursor::new(b"s 67108864")).expect("Expect success");

    // Expect empty Obj
    assert_eq!(obj.name, None);
    assert_eq!(obj.vertices, vec![]);
    assert_eq!(obj.indices, vec![]);
}
