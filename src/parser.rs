use crate::ast::*;
use crate::token::Tok;

#[derive(Debug)]
pub struct Parser {
    toks: Vec<Tok>,
    i: usize,
}

fn is_eof(t: &Tok) -> bool {
    matches!(t, Tok::Eof)
}
fn peek(p: &Parser) -> &Tok {
    p.toks.get(p.i).unwrap_or(&Tok::Eof)
}
fn bump(p: &mut Parser) -> Tok {
    let t = peek(p).clone();
    if !is_eof(&t) {
        p.i += 1;
    }
    t
}

fn expect(p: &mut Parser, want: Tok) -> Tok {
    let t = bump(p);
    if std::mem::discriminant(&t) != std::mem::discriminant(&want) {
        panic!("expected {:?}, got {:?}", want, t);
    }
    t
}

pub fn parse(toks: Vec<Tok>) -> Result<Program, String> {
    let mut p = Parser { toks, i: 0 };
    let mut items = Vec::new();
    while !is_eof(peek(&p)) {
        items.push(parse_stmt(&mut p)?);
    }
    Ok(Program { items })
}

fn parse_stmt(p: &mut Parser) -> Result<Stmt, String> {
    match peek(p) {
        Tok::Ident(_) => {
            let name = if let Tok::Ident(s) = bump(p) {
                s
            } else {
                unreachable!()
            };
            match peek(p) {
                Tok::Colon => {
                    bump(p);
                    if let Tok::LBrace = peek(p) {
                        bump(p);
                        let mut fields = Vec::new();
                        while !matches!(peek(p), Tok::RBrace | Tok::Eof) {
                            let key = if let Tok::Ident(s) = bump(p) {
                                s
                            } else {
                                return Err("field name".into());
                            };
                            expect(p, Tok::Colon);
                            let e = parse_expr(p)?;
                            if let Tok::Comma = peek(p) {
                                bump(p);
                            }
                            fields.push((key, e));
                        }
                        expect(p, Tok::RBrace);
                        Ok(Stmt::ObjBlock { name, fields })
                    } else {
                        let e = parse_expr(p)?;
                        Ok(Stmt::VarAssign { name, expr: e })
                    }
                }
                Tok::Dot => {
                    bump(p);
                    let key = if let Tok::Ident(s) = bump(p) {
                        s
                    } else {
                        return Err("prop".into());
                    };
                    expect(p, Tok::Colon);
                    let e = parse_expr(p)?;
                    Ok(Stmt::PropAssign {
                        base: name,
                        key,
                        expr: e,
                    })
                }
                _ => Err("invalid statement".into()),
            }
        }
        Tok::KwLog => {
            bump(p);
            let e = parse_expr(p)?;
            Ok(Stmt::Log(e))
        }
        Tok::KwCond => parse_cond(p),
        Tok::KwLoop => parse_loop(p),
        Tok::KwWhile => parse_while(p),
        Tok::KwInput => parse_input(p),
        Tok::KwBreak => {
            bump(p);
            Ok(Stmt::Break)
        }
        Tok::KwContinue => {
            bump(p);
            Ok(Stmt::Continue)
        }
        Tok::Eof => Err("eof".into()),
        _ => Err(format!("unexpected token in stmt: {:?}", peek(p))),
    }
}

fn parse_cond(p: &mut Parser) -> Result<Stmt, String> {
    expect(p, Tok::KwCond);
    expect(p, Tok::LParen);
    let cond = parse_expr(p)?;
    expect(p, Tok::RParen);
    expect(p, Tok::Arrow);
    expect(p, Tok::LBrace);
    let then_blk = parse_block_until(p, Tok::RBrace)?;
    let mut else_blk = None;
    if let Tok::KwElse = peek(p) {
        bump(p);
        expect(p, Tok::Arrow);
        expect(p, Tok::LBrace);
        else_blk = Some(parse_block_until(p, Tok::RBrace)?);
    }
    expect(p, Tok::KwEnd);
    Ok(Stmt::Cond {
        cond,
        then_blk,
        else_blk,
    })
}

fn parse_loop(p: &mut Parser) -> Result<Stmt, String> {
    expect(p, Tok::KwLoop);
    expect(p, Tok::LParen);
    let cond = parse_expr(p)?;
    expect(p, Tok::RParen);
    expect(p, Tok::Arrow);
    expect(p, Tok::LBrace);
    let body = parse_block_until(p, Tok::RBrace)?;
    expect(p, Tok::KwEnd);
    Ok(Stmt::Loop { cond, body })
}

fn parse_while(p: &mut Parser) -> Result<Stmt, String> {
    expect(p, Tok::KwWhile);
    expect(p, Tok::LParen);
    let cond = parse_expr(p)?;
    expect(p, Tok::RParen);
    expect(p, Tok::Arrow);
    expect(p, Tok::LBrace);
    let body = parse_block_until(p, Tok::RBrace)?;
    let mut ended = None;
    if let Tok::KwEnded = peek(p) {
        bump(p);
        expect(p, Tok::LBrace);
        ended = Some(parse_block_until(p, Tok::RBrace)?);
    }
    expect(p, Tok::KwEnd);
    Ok(Stmt::While { cond, body, ended })
}

fn parse_input(p: &mut Parser) -> Result<Stmt, String> {
    expect(p, Tok::KwInput);
    let prompt = if let Tok::String(s) = bump(p) {
        s
    } else {
        return Err("input needs string".into());
    };
    expect(p, Tok::Arrow);
    let target = match bump(p) {
        Tok::Ident(v) => {
            if let Tok::Dot = peek(p) {
                bump(p);
                if let Tok::Ident(k) = bump(p) {
                    Target::Prop { base: v, key: k }
                } else {
                    return Err("prop name".into());
                }
            } else {
                Target::Var(v)
            }
        }
        _ => return Err("input target".into()),
    };
    Ok(Stmt::Input {
        prompt,
        dst: target,
    })
}

fn parse_block_until(p: &mut Parser, until: Tok) -> Result<Vec<Stmt>, String> {
    let mut v = Vec::new();
    while std::mem::discriminant(peek(p)) != std::mem::discriminant(&until) {
        v.push(parse_stmt(p)?);
    }
    expect(p, until);
    Ok(v)
}

/*** Expressions (Pratt) ***/
fn bp_infix(op: &Tok) -> Option<(u8, u8)> {
    use Tok::*;
    Some(match op {
        OrOr => (1, 2),
        AndAnd => (3, 4),
        EqEq | NotEq | Lt | Le | Gt | Ge => (5, 6),
        Plus | Minus => (9, 10),
        Star | Slash | Percent => (11, 12),
        _ => return None,
    })
}

fn parse_expr(p: &mut Parser) -> Result<Expr, String> {
    parse_bp(p, 0)
}

fn parse_bp(p: &mut Parser, min_bp: u8) -> Result<Expr, String> {
    use Tok::*;
    let mut lhs = match bump(p) {
        Number(n) => Expr::Num(n),
        String(s) => Expr::Str(s),
        True => Expr::Bool(true),
        False => Expr::Bool(false),
        Ident(id) => {
            // ident + member chain (함수 호출 제거)
            let mut base = Expr::Ident(id);
            while let Dot = peek(p) {
                bump(p);
                if let Ident(k) = bump(p) {
                    base = Expr::Member(Box::new(base), k);
                } else {
                    return Err("member".into());
                }
            }
            base
        }
        Bang => {
            let rhs = parse_bp(p, 13)?;
            Expr::Unary {
                op: "!".into(),
                rhs: Box::new(rhs),
            }
        }
        LParen => {
            let e = parse_bp(p, 0)?;
            expect(p, RParen);
            e
        }
        t => return Err(format!("bad prefix token: {:?}", t)),
    };

    loop {
        let op = peek(p).clone();
        if let Some((l_bp, r_bp)) = bp_infix(&op) {
            if l_bp < min_bp {
                break;
            }
            bump(p);
            let rhs = parse_bp(p, r_bp)?;
            let s = match op {
                OrOr => "||",
                AndAnd => "&&",
                EqEq => "==",
                NotEq => "!=",
                Lt => "<",
                Le => "<=",
                Gt => ">",
                Ge => ">=",
                Plus => "+",
                Minus => "-",
                Star => "*",
                Slash => "/",
                Percent => "%",
                _ => unreachable!(),
            }
            .to_string();
            lhs = Expr::Binary {
                op: s,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
            continue;
        }
        break;
    }
    Ok(lhs)
}
