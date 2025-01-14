use super::{expect_valid_tokenization, Token};

#[test]
fn constants_scalar() {
    let source = "
    const A = 1
    const B = 2";

    let tokens = vec![
        Token::Const,
        Token::Ident("A".to_string()),
        Token::Equal,
        Token::Num("1".to_string()),
        Token::Const,
        Token::Ident("B".to_string()),
        Token::Equal,
        Token::Num("2".to_string()),
    ];
    expect_valid_tokenization(source, tokens);
}

#[test]
fn constants_vector() {
    let source = "
    const A = [1, 2, 3, 4]
    const B = [5, 6, 7, 8]";

    let tokens = vec![
        Token::Const,
        Token::Ident("A".to_string()),
        Token::Equal,
        Token::Lsqb,
        Token::Num("1".to_string()),
        Token::Comma,
        Token::Num("2".to_string()),
        Token::Comma,
        Token::Num("3".to_string()),
        Token::Comma,
        Token::Num("4".to_string()),
        Token::Rsqb,
        Token::Const,
        Token::Ident("B".to_string()),
        Token::Equal,
        Token::Lsqb,
        Token::Num("5".to_string()),
        Token::Comma,
        Token::Num("6".to_string()),
        Token::Comma,
        Token::Num("7".to_string()),
        Token::Comma,
        Token::Num("8".to_string()),
        Token::Rsqb,
    ];
    expect_valid_tokenization(source, tokens);
}

#[test]
fn constants_matrix() {
    let source = "
        const A = [[1, 2], [3, 4]]
        const B = [[5, 6], [7, 8]]";

    let tokens = vec![
        Token::Const,
        Token::Ident("A".to_string()),
        Token::Equal,
        Token::Lsqb,
        Token::Lsqb,
        Token::Num("1".to_string()),
        Token::Comma,
        Token::Num("2".to_string()),
        Token::Rsqb,
        Token::Comma,
        Token::Lsqb,
        Token::Num("3".to_string()),
        Token::Comma,
        Token::Num("4".to_string()),
        Token::Rsqb,
        Token::Rsqb,
        Token::Const,
        Token::Ident("B".to_string()),
        Token::Equal,
        Token::Lsqb,
        Token::Lsqb,
        Token::Num("5".to_string()),
        Token::Comma,
        Token::Num("6".to_string()),
        Token::Rsqb,
        Token::Comma,
        Token::Lsqb,
        Token::Num("7".to_string()),
        Token::Comma,
        Token::Num("8".to_string()),
        Token::Rsqb,
        Token::Rsqb,
    ];
    expect_valid_tokenization(source, tokens);
}

#[test]
fn constants_access_inside_boundary_expr() {
    // This is invalid since the constants are not declared but this error will be thrown at the
    // IR level.
    let source = "
    boundary_constraints:
        enf clk.first = A + B[0]
        enf clk.last = C[0][1]
    ";

    let tokens = vec![
        Token::BoundaryConstraints,
        Token::Colon,
        Token::Enf,
        Token::Ident("clk".to_string()),
        Token::Dot,
        Token::First,
        Token::Equal,
        Token::Ident("A".to_string()),
        Token::Plus,
        Token::Ident("B".to_string()),
        Token::Lsqb,
        Token::Num("0".to_string()),
        Token::Rsqb,
        Token::Enf,
        Token::Ident("clk".to_string()),
        Token::Dot,
        Token::Last,
        Token::Equal,
        Token::Ident("C".to_string()),
        Token::Lsqb,
        Token::Num("0".to_string()),
        Token::Rsqb,
        Token::Lsqb,
        Token::Num("1".to_string()),
        Token::Rsqb,
    ];
    expect_valid_tokenization(source, tokens);
}

#[test]
fn constants_access_inside_integrity_expr() {
    let source = "
        const A = 1
        const B = [1, 0]
        const C = [[1, 0], [0, 1]]
        integrity_constraints:
            enf clk * 2^A = B[0] + C[0][1]
    ";
    let tokens = vec![
        Token::Const,
        Token::Ident("A".to_string()),
        Token::Equal,
        Token::Num("1".to_string()),
        Token::Const,
        Token::Ident("B".to_string()),
        Token::Equal,
        Token::Lsqb,
        Token::Num("1".to_string()),
        Token::Comma,
        Token::Num("0".to_string()),
        Token::Rsqb,
        Token::Const,
        Token::Ident("C".to_string()),
        Token::Equal,
        Token::Lsqb,
        Token::Lsqb,
        Token::Num("1".to_string()),
        Token::Comma,
        Token::Num("0".to_string()),
        Token::Rsqb,
        Token::Comma,
        Token::Lsqb,
        Token::Num("0".to_string()),
        Token::Comma,
        Token::Num("1".to_string()),
        Token::Rsqb,
        Token::Rsqb,
        Token::IntegrityConstraints,
        Token::Colon,
        Token::Enf,
        Token::Ident("clk".to_string()),
        Token::Mul,
        Token::Num("2".to_string()),
        Token::Exp,
        Token::Ident("A".to_string()),
        Token::Equal,
        Token::Ident("B".to_string()),
        Token::Lsqb,
        Token::Num("0".to_string()),
        Token::Rsqb,
        Token::Plus,
        Token::Ident("C".to_string()),
        Token::Lsqb,
        Token::Num("0".to_string()),
        Token::Rsqb,
        Token::Lsqb,
        Token::Num("1".to_string()),
        Token::Rsqb,
    ];
    expect_valid_tokenization(source, tokens);
}

#[test]
fn constants_access_inside_integrity_expr_invalid() {
    // This is invalid since the constants are not declared and the constant names should be
    // capitalized but these errors will be thrown at the IR level and parsing level respectively.
    let source = "
        integrity_constraints:
            enf clk * 2^a = b[0] + c[0][1]
    ";
    let tokens = vec![
        Token::IntegrityConstraints,
        Token::Colon,
        Token::Enf,
        Token::Ident("clk".to_string()),
        Token::Mul,
        Token::Num("2".to_string()),
        Token::Exp,
        Token::Ident("a".to_string()),
        Token::Equal,
        Token::Ident("b".to_string()),
        Token::Lsqb,
        Token::Num("0".to_string()),
        Token::Rsqb,
        Token::Plus,
        Token::Ident("c".to_string()),
        Token::Lsqb,
        Token::Num("0".to_string()),
        Token::Rsqb,
        Token::Lsqb,
        Token::Num("1".to_string()),
        Token::Rsqb,
    ];
    expect_valid_tokenization(source, tokens);
}
