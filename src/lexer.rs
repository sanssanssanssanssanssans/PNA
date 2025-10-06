use crate::token::Tok;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexError {
    #[error("unexpected char `{0}`")]
    Unexpected(char),
}

pub fn lex(s: &str) -> Result<Vec<Tok>, LexError> {
    let b = s.as_bytes();
    let mut i = 0usize;
    let mut out = Vec::new();

    fn is_id_start(c: u8) -> bool {
        c.is_ascii_alphabetic() || c == b'_'
    }
    fn is_id_body(c: u8) -> bool {
        is_id_start(c) || c.is_ascii_digit() || c == b'.'
    }

    while i < b.len() {
        let c = b[i];

        if c.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if c == b'#' || (c == b'/' && i + 1 < b.len() && b[i + 1] == b'/') {
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if c == b'"' || c == b'\'' {
            let q = c;
            i += 1;
            let mut s = String::new();
            let mut esc = false;
            while i < b.len() {
                let d = b[i];
                i += 1;
                if esc {
                    s.push(match d {
                        b'n' => '\n',
                        x => x as char,
                    });
                    esc = false;
                    continue;
                }
                if d == b'\\' {
                    esc = true;
                    continue;
                }
                if d == q {
                    break;
                }
                s.push(d as char);
            }
            out.push(Tok::String(s));
            continue;
        }
        if c.is_ascii_digit() || (c == b'-' && i + 1 < b.len() && b[i + 1].is_ascii_digit()) {
            let mut j = i;
            if b[j] == b'-' {
                j += 1;
            }
            let mut dot = false;
            while j < b.len() && (b[j].is_ascii_digit() || (!dot && b[j] == b'.')) {
                if b[j] == b'.' {
                    dot = true;
                }
                j += 1;
            }
            let n = std::str::from_utf8(&b[i..j]).unwrap();
            i = j;
            let v: f64 = n.parse().unwrap_or(0.0);
            out.push(Tok::Number(v));
            continue;
        }
        if i + 1 < b.len() {
            match &b[i..i + 2] {
                b"->" => {
                    out.push(Tok::Arrow);
                    i += 2;
                    continue;
                }
                b"==" => {
                    out.push(Tok::EqEq);
                    i += 2;
                    continue;
                }
                b"!=" => {
                    out.push(Tok::NotEq);
                    i += 2;
                    continue;
                }
                b"<=" => {
                    out.push(Tok::Le);
                    i += 2;
                    continue;
                }
                b">=" => {
                    out.push(Tok::Ge);
                    i += 2;
                    continue;
                }
                b"&&" => {
                    out.push(Tok::AndAnd);
                    i += 2;
                    continue;
                }
                b"||" => {
                    out.push(Tok::OrOr);
                    i += 2;
                    continue;
                }
                _ => {}
            }
        }
        match c {
            b'{' => {
                out.push(Tok::LBrace);
                i += 1;
            }
            b'}' => {
                out.push(Tok::RBrace);
                i += 1;
            }
            b'(' => {
                out.push(Tok::LParen);
                i += 1;
            }
            b')' => {
                out.push(Tok::RParen);
                i += 1;
            }
            b':' => {
                out.push(Tok::Colon);
                i += 1;
            }
            b',' => {
                out.push(Tok::Comma);
                i += 1;
            }
            b'.' => {
                out.push(Tok::Dot);
                i += 1;
            }
            b'+' => {
                out.push(Tok::Plus);
                i += 1;
            }
            b'-' => {
                out.push(Tok::Minus);
                i += 1;
            }
            b'*' => {
                out.push(Tok::Star);
                i += 1;
            }
            b'/' => {
                out.push(Tok::Slash);
                i += 1;
            }
            b'%' => {
                out.push(Tok::Percent);
                i += 1;
            }
            b'<' => {
                out.push(Tok::Lt);
                i += 1;
            }
            b'>' => {
                out.push(Tok::Gt);
                i += 1;
            }
            b'!' => {
                out.push(Tok::Bang);
                i += 1;
            }
            _ => {
                if is_id_start(c) {
                    let mut j = i + 1;
                    while j < b.len() && is_id_body(b[j]) {
                        j += 1;
                    }
                    let id = std::str::from_utf8(&b[i..j]).unwrap().to_string();
                    i = j;
                    let kw = match id.as_str() {
                        "true" => Some(Tok::True),
                        "false" => Some(Tok::False),
                        "log" => Some(Tok::KwLog),
                        "cond" => Some(Tok::KwCond),
                        "else" => Some(Tok::KwElse),
                        "end" => Some(Tok::KwEnd),
                        "loop" => Some(Tok::KwLoop),
                        "while" => Some(Tok::KwWhile),
                        "ended" => Some(Tok::KwEnded),
                        "input" => Some(Tok::KwInput),
                        "break" => Some(Tok::KwBreak),
                        "continue" => Some(Tok::KwContinue),
                        "function" => Some(Tok::KwFunction),
                        "return" => Some(Tok::KwReturn),
                        "double" => Some(Tok::TyDouble),
                        "int" => Some(Tok::TyInt),
                        "string" => Some(Tok::TyString),
                        "void" => Some(Tok::TyVoid),
                        _ => None,
                    };
                    out.push(kw.unwrap_or(Tok::Ident(id)));
                } else {
                    return Err(LexError::Unexpected(c as char));
                }
            }
        }
    }

    Ok(out)
}
