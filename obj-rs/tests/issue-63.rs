use obj::{load_obj};
use std::io::Cursor;

fn do_test<V: obj::FromRawVertex<u8> + std::fmt::Debug>(test_case: &str) {
    let err = load_obj::<V, _, _>(Cursor::new(test_case))
        .expect_err("Should error out due to index out of bounds");
    if let obj::ObjError::Load(err) = err {
        let expected_error = obj::LoadError::new(
            obj::LoadErrorKind::IndexOutOfRange,
            "Unable to convert the index from usize",
        );
        assert_eq!(err.to_string(), expected_error.to_string());
    } else {
        panic!("Expected a LoadError");
    }
}

#[test]
fn issue_63() {
    let mut test_case: String = "o LargeObj\n".into();
    for i in 0..1000 {
        test_case.push_str(format!("v {i}.0 {i}.0 {i}.0\n").as_str());
    }
    test_case.push_str("vt 0.0 0.0\nvn 0.0 0.0 1.0\n");
    for i in 0..(1000-2) {
        let i = i + 1;
        let j = i + 1;
        let k = i + 2;
        test_case.push_str(format!("f {i}/1/1 {j}/1/1 {k}/1/1\n").as_str())
    }

    load_obj::<obj::TexturedVertex, _, u16>(Cursor::new(&test_case))
        .expect("this should load properly");

    do_test::<obj::Position>(&test_case);
    do_test::<obj::Vertex>(&test_case);
    do_test::<obj::TexturedVertex>(&test_case);
}
