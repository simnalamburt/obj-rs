use obj::{load_obj, Obj};
use std::io::Cursor;

#[test]
fn issue_50() {
    let test_cases: [&[u8]; 4] = [
        b"s 1000000000000000000",
        b"s 100000000000000000",
        b"s 100000000000",
        b"s 67108864",
    ];

    for data in test_cases {
        let obj: Obj = load_obj(Cursor::new(data)).expect("Expect success");

        // Expect empty Obj
        assert_eq!(obj.name, None);
        assert_eq!(obj.vertices, vec![]);
        assert_eq!(obj.indices, vec![]);
    }
}
