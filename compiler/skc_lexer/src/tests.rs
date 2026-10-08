use super::*;
use expect_test::{Expect, expect};

fn check_lexing(input: &str, expected: Expect) {
    let tokens: Vec<Token> = Cursor::tokenize(input).collect();
    expected.assert_debug_eq(&tokens);
}

#[test]
fn tokenize_empty_input() {
    let input = "";
    check_lexing(
        input,
        expect![[r#"
        []
    "#]],
    );
}

#[test]
fn tokenize_whitespace_only() {
    let input = "   \u{000C}\u{0020}\u{0009}";
    check_lexing(
        input,
        expect![[r#"
            [
                Token {
                    kind: Whitespace,
                    len: 6,
                },
            ]
        "#]],
    );
}
