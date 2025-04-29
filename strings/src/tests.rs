struct S<'a> {
    a: &'a str,
    b: &'a str,
    i: isize,
}

#[test]
fn test_compare() {
    let compareTests = vec![
        S { a: "", b: "", i: 0 },
        S {
            a: "a",
            b: "",
            i: 1,
        },
        S {
            a: "",
            b: "a",
            i: -1,
        },
        S {
            a: "abc",
            b: "abc",
            i: 0,
        },
        S {
            a: "ab",
            b: "abc",
            i: -1,
        },
        S {
            a: "abc",
            b: "ab",
            i: 1,
        },
        S {
            a: "x",
            b: "ab",
            i: 1,
        },
        S {
            a: "ab",
            b: "x",
            i: -1,
        },
        S {
            a: "x",
            b: "a",
            i: 1,
        },
        S {
            a: "b",
            b: "x",
            i: -1,
        },
        S {
            a: "abcdefgh",
            b: "abcdefgh",
            i: 0,
        },
        S {
            a: "abcdefghi",
            b: "abcdefghi",
            i: 0,
        },
        S {
            a: "abcdefghi",
            b: "abcdefghj",
            i: -1,
        },
    ];

    for tt in compareTests {
        let cmp = crate::Compare(tt.a, tt.b);
        assert!(cmp == tt.i, "Compare({}, {}) = {}", tt.a, tt.b, cmp)
    }
}
