use super::{expect_error, expect_valid_tokenization};
use crate::{error::Error, lexer::Token};

// IDENTIFIERS VALID TOKENIZATION
// ================================================================================================

#[test]
fn keywords_with_identifiers() {
    let source = "enf clk' = clk + 1";
    let tokens = vec![
        Token::Enf,
        Token::Ident("clk".to_string()),
        Token::Next,
        Token::Equal,
        Token::Ident("clk".to_string()),
        Token::Plus,
        Token::Num("1".to_string()),
    ];
    expect_valid_tokenization(source, tokens);
}

#[test]
fn keyword_and_identifier_without_space() {
    let source = "enfclk' = clkdef + 1";
    let tokens = vec![
        // enfclk' is considered as an identifier by logos
        Token::Ident("enfclk".to_string()),
        Token::Next,
        Token::Equal,
        // clkdef is considered as an identifier by logos
        Token::Ident("clkdef".to_string()),
        Token::Plus,
        Token::Num("1".to_string()),
    ];
    expect_valid_tokenization(source, tokens);
}

#[test]
fn number_and_identier_without_space() {
    let source = "enf 1clk' = clk + 1";
    let tokens = vec![
        Token::Enf,
        Token::Num("1".to_string()),
        Token::Ident("clk".to_string()),
        Token::Next,
        Token::Equal,
        Token::Ident("clk".to_string()),
        Token::Plus,
        Token::Num("1".to_string()),
    ];
    expect_valid_tokenization(source, tokens);
}

#[test]
fn valid_tokenization_next_token() {
    let source = "enf clk'' = clk + 1";
    let tokens = vec![
        Token::Enf,
        Token::Ident("clk".to_string()),
        Token::Next,
        // This is a parsing error, not a scanning error.
        Token::Next,
        Token::Equal,
        Token::Ident("clk".to_string()),
        Token::Plus,
        Token::Num("1".to_string()),
    ];
    expect_valid_tokenization(source, tokens);
}

#[test]
fn valid_tokenization_indexed_trace_access() {
    let source = "enf $main[0]' = $main[1] + $aux[0] + $aux[1]'";
    let tokens = vec![
        Token::Enf,
        Token::MainAccess,
        Token::Lsqb,
        Token::Num("0".to_string()),
        Token::Rsqb,
        Token::Next,
        Token::Equal,
        Token::MainAccess,
        Token::Lsqb,
        Token::Num("1".to_string()),
        Token::Rsqb,
        Token::Plus,
        Token::AuxAccess,
        Token::Lsqb,
        Token::Num("0".to_string()),
        Token::Rsqb,
        Token::Plus,
        Token::AuxAccess,
        Token::Lsqb,
        Token::Num("1".to_string()),
        Token::Rsqb,
        Token::Next,
    ];
    expect_valid_tokenization(source, tokens);
}

// SCAN ERRORS
// ================================================================================================

#[test]
fn error_identifier_with_invalid_characters() {
    let source = "enf clk@' = clk + 1";
    // "@" is not in the allowed characters.
    let expected = Error::ScanError(7..8);
    expect_error(source, expected);
}

#[test]
fn return_first_invalid_character_error() {
    let source = "enf clk@' = clk@ + 1";
    // "@" is not in the allowed characters.
    let expected = Error::ScanError(7..8);
    expect_error(source, expected);
}
