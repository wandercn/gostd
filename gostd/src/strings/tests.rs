struct List<'a> {
    a: &'a str,
    b: &'a str,
    i: isize,
}

#[test]
fn test_compare() {
    let compareTests = vec![
        List { a: "", b: "", i: 0 },
        List {
            a: "a",
            b: "",
            i: 1,
        },
        List {
            a: "",
            b: "a",
            i: -1,
        },
        List {
            a: "abc",
            b: "abc",
            i: 0,
        },
        List {
            a: "ab",
            b: "abc",
            i: -1,
        },
        List {
            a: "abc",
            b: "ab",
            i: 1,
        },
        List {
            a: "x",
            b: "ab",
            i: 1,
        },
        List {
            a: "ab",
            b: "x",
            i: -1,
        },
        List {
            a: "x",
            b: "a",
            i: 1,
        },
        List {
            a: "b",
            b: "x",
            i: -1,
        },
        List {
            a: "abcdefgh",
            b: "abcdefgh",
            i: 0,
        },
        List {
            a: "abcdefghi",
            b: "abcdefghi",
            i: 0,
        },
        List {
            a: "abcdefghi",
            b: "abcdefghj",
            i: -1,
        },
    ];

    for tt in compareTests {
        let cmp = crate::strings::Compare(tt.a, tt.b);
        assert!(cmp == tt.i, "Compare({}, {}) = {}", tt.a, tt.b, cmp)
    }
}
