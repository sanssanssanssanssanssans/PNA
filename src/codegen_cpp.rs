use crate::ast::*;
use std::error::Error;

const TPL_CODEGEN: &str = include_str!("../templates/codegen");
const TPL_RUNTIME: &str = include_str!("../templates/runtime.cpp");

pub fn emit_cpp(p: &Program) -> Result<String, Box<dyn Error>> {
    let mut body = String::new();
    let mut cg = CgState::default();
    for it in &p.items {
        emit_stmt(&mut body, it, "env", 1, &mut cg);
    }
    let cpp = TPL_CODEGEN
        .replace("{{RUNTIME}}", TPL_RUNTIME)
        .replace("{{BODY}}", &body);
    Ok(cpp)
}

fn esc(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '"' => "\\\"".chars().collect(),
            '\n' => "\\n".chars().collect(),
            x => vec![x],
        })
        .collect()
}

#[derive(Default)]
struct CgState {
    while_id: usize,
    while_stack: Vec<usize>,
}

fn indent(n: usize) -> String {
    "  ".repeat(n)
}

fn emit_stmt(out: &mut String, s: &Stmt, env: &str, lvl: usize, cg: &mut CgState) {
    match s {
        Stmt::ObjBlock { name, fields } => {
            out.push_str(&format!("{}{{ Value __o=pna_make_obj();\n", indent(lvl)));
            for (k, e) in fields {
                let expr = emit_expr(e, env);
                out.push_str(&format!(
                    "{}pna_obj_set(__o,\"{}\",{});\n",
                    indent(lvl + 1),
                    esc(k),
                    expr
                ));
            }
            out.push_str(&format!(
                "{}pna_set({},\"{}\",__o); }}\n",
                indent(lvl + 1),
                env,
                esc(name)
            ));
        }
        Stmt::PropAssign { base, key, expr } => {
            let e = emit_expr(expr, env);
            out.push_str(&format!(
                "{}pna_set_prop({},\"{}\",\"{}\",{});\n",
                indent(lvl),
                env,
                esc(base),
                esc(key),
                e
            ));
        }
        Stmt::VarAssign { name, expr } => {
            let e = emit_expr(expr, env);
            out.push_str(&format!(
                "{}pna_set({},\"{}\",{});\n",
                indent(lvl),
                env,
                esc(name),
                e
            ));
        }
        Stmt::Log(e) => {
            let x = emit_expr(e, env);
            out.push_str(&format!("{}pna_log({});\n", indent(lvl), x));
        }
        Stmt::Cond {
            cond,
            then_blk,
            else_blk,
        } => {
            let c = emit_expr(cond, env);
            out.push_str(&format!("{}if(pna_truthy({})){{\n", indent(lvl), c));
            for st in then_blk {
                emit_stmt(out, st, env, lvl + 1, cg);
            }
            out.push_str(&format!("{}}}", indent(lvl)));
            if let Some(eb) = else_blk {
                out.push_str(" else {\n");
                for st in eb {
                    emit_stmt(out, st, env, lvl + 1, cg);
                }
                out.push_str(&format!("{}}}\n", indent(lvl)));
            } else {
                out.push('\n');
            }
        }
        Stmt::Loop { cond, body } => {
            let c = emit_expr(cond, env);
            out.push_str(&format!("{}while(pna_truthy({})){{\n", indent(lvl), c));
            for st in body {
                emit_stmt(out, st, env, lvl + 1, cg);
            }
            out.push_str(&format!("{}}}\n", indent(lvl)));
        }
        Stmt::While { cond, body, ended } => {
            let c = emit_expr(cond, env);
            let id = cg.while_id;
            cg.while_id += 1;
            let broke = format!("__broke_{}", id);
            out.push_str(&format!(
                "{}bool {}=false, __ran_{}=false; while(pna_truthy({})){{ __ran_{}=true;\n",
                indent(lvl),
                broke,
                id,
                c,
                id
            ));
            cg.while_stack.push(id);
            for st in body {
                emit_stmt(out, st, env, lvl + 1, cg);
            }
            cg.while_stack.pop();
            out.push_str(&format!("{}}} if(!{}){{\n", indent(lvl), broke));
            if let Some(eb) = ended {
                for st in eb {
                    emit_stmt(out, st, env, lvl + 1, cg);
                }
            }
            out.push_str(&format!("{}}}\n", indent(lvl)));
        }
        Stmt::Input { prompt, dst } => {
            out.push_str(&format!(
                "{}{{ Value __in=pna_input(\"{}\");\n",
                indent(lvl),
                esc(prompt)
            ));
            match dst {
                Target::Var(v) => out.push_str(&format!(
                    "{}pna_set({},\"{}\",__in);\n",
                    indent(lvl + 1),
                    env,
                    esc(v)
                )),
                Target::Prop { base, key } => out.push_str(&format!(
                    "{}pna_set_prop({},\"{}\",\"{}\",__in);\n",
                    indent(lvl + 1),
                    env,
                    esc(base),
                    esc(key)
                )),
            }
            out.push_str(&format!("{}}}\n", indent(lvl)));
        }
        Stmt::Break => {
            if let Some(&wid) = cg.while_stack.last() {
                out.push_str(&format!("{}__broke_{}=true;\n", indent(lvl), wid));
            }
            out.push_str(&format!("{}break;\n", indent(lvl)));
        }
        Stmt::Continue => {
            out.push_str(&format!("{}continue;\n", indent(lvl)));
        }
    }
}

fn emit_expr(e: &Expr, env: &str) -> String {
    match e {
        Expr::Num(n) => {
            let n = *n;
            if n.is_finite() && n.fract() == 0.0 {
                format!("Value({}.0)", n as i64)
            } else {
                format!("Value({})", n)
            }
        }
        Expr::Str(s) => format!("Value(\"{}\")", esc(s)),
        Expr::Bool(b) => format!("Value({})", b),
        Expr::Ident(id) => format!("pna_get({},\"{}\")", env, esc(id)),
        Expr::Member(base, key) => {
            let b = emit_expr(base, env);
            format!("pna_get_prop({},\"{}\")", b, esc(key))
        }
        Expr::Unary { op, rhs } => {
            let r = emit_expr(rhs, env);
            match op.as_str() {
                "!" => format!("pna_not({})", r),
                _ => r,
            }
        }
        Expr::Binary { op, lhs, rhs } => {
            let a = emit_expr(lhs, env);
            let b = emit_expr(rhs, env);
            match op.as_str() {
                "+" => format!("pna_add({}, {})", a, b),
                "-" => format!("pna_sub({}, {})", a, b),
                "*" => format!("pna_mul({}, {})", a, b),
                "/" => format!("pna_div({}, {})", a, b),
                "%" => format!("pna_mod({}, {})", a, b),
                "==" => format!("pna_eq({}, {})", a, b),
                "!=" => format!("pna_neq({}, {})", a, b),
                "<" => format!("pna_lt({}, {})", a, b),
                "<=" => format!("pna_le({}, {})", a, b),
                ">" => format!("pna_gt({}, {})", a, b),
                ">=" => format!("pna_ge({}, {})", a, b),
                "&&" => format!("pna_and({}, {})", a, b),
                "||" => format!("pna_or({}, {})", a, b),
                _ => a,
            }
        }
    }
}
