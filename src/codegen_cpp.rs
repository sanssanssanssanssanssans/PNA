use crate::ast::*;
use std::fmt::Write;

pub fn compile_to_cpp(p: &Program) -> Result<String, String> {
    let mut out = String::new();

    out.push_str("#include <bits/stdc++.h>\n");
    out.push_str("struct Value{std::variant<std::monostate,double,std::string,bool,std::map<std::string,Value>> v;Value():v(std::monostate{}){}Value(double d):v(d){}Value(const char*s):v(std::string(s)){}Value(const std::string&s):v(s){}Value(bool b):v(b){}Value(std::map<std::string,Value> o):v(std::move(o)){}template<class T,class=std::enable_if_t<std::is_integral_v<T>&& !std::is_same_v<T,bool>>>Value(T x):v(static_cast<double>(x)){} };");
    out.push_str("struct Env{std::unordered_map<std::string,Value> vars;Env* parent=nullptr;};");
    out.push_str("inline bool is_num(const Value& x){return std::holds_alternative<double>(x.v);}inline bool is_str(const Value& x){return std::holds_alternative<std::string>(x.v);}inline bool is_bool(const Value& x){return std::holds_alternative<bool>(x.v);}inline bool is_obj(const Value& x){return std::holds_alternative<std::map<std::string,Value>>(x.v);}");
    out.push_str("inline std::string as_str(const Value& x){if(is_str(x))return std::get<std::string>(x.v);if(is_num(x)){double d=std::get<double>(x.v);double rd=std::round(d);if(std::fabs(d-rd)<1e-9){std::ostringstream oss;oss.setf(std::ios::fixed,std::ios::floatfield);oss<<std::setprecision(0)<<rd;return oss.str();}std::ostringstream oss;oss.setf(std::ios::fixed,std::ios::floatfield);oss<<std::setprecision(12)<<d;auto s=oss.str();while(!s.empty()&&s.back()=='0')s.pop_back();if(!s.empty()&&s.back()=='.')s.pop_back();return s;}if(is_bool(x))return std::get<bool>(x.v)?\"true\":\"false\";if(is_obj(x)){std::string s=\"{\";bool first=true;for(auto&kv:std::get<std::map<std::string,Value>>(x.v)){if(!first)s+=\", \";first=false;s+=kv.first+\":\"+as_str(kv.second);}s+=\"}\";return s;}return \"\";}");
    out.push_str("inline double as_num(const Value& v){if(is_num(v))return std::get<double>(v.v);if(is_bool(v))return std::get<bool>(v.v)?1.0:0.0;if(is_str(v)){const auto&s=std::get<std::string>(v.v);char*end=nullptr;double d=std::strtod(s.c_str(),&end);if(end!=s.c_str()&&*end=='\\0')return d;return 0.0;}return 0.0;}");
    out.push_str("inline bool truthy(const Value& x){if(is_bool(x))return std::get<bool>(x.v);if(is_num(x))return std::get<double>(x.v)!=0;if(is_str(x))return !std::get<std::string>(x.v).empty();if(is_obj(x))return !std::get<std::map<std::string,Value>>(x.v).empty();return false;}");
    out.push_str("inline Value pna_get(Env&e,const std::string&k){auto it=e.vars.find(k);if(it!=e.vars.end())return it->second;if(e.parent)return pna_get(*e.parent,k);return Value();}inline void pna_set(Env&e,const std::string&k,const Value&v){e.vars[k]=v;}");
    out.push_str("inline Value pna_make_obj(){return Value(std::map<std::string,Value>{});}inline void pna_obj_set(Value&o,const std::string&k,const Value&v){if(!is_obj(o))o=std::map<std::string,Value>{};std::get<std::map<std::string,Value>>(o.v)[k]=v;}");
    out.push_str("inline Value pna_get_prop(const Value&o,const std::string&k){if(!is_obj(o))return Value();auto&m=std::get<std::map<std::string,Value>>(const_cast<Value&>(o).v);auto it=m.find(k);if(it!=m.end())return it->second;return Value();}inline void pna_set_prop(Env&e,const std::string&base,const std::string&key,const Value&v){Value b=pna_get(e,base);if(!is_obj(b))b=std::map<std::string,Value>{};auto m=std::get<std::map<std::string,Value>>(b.v);m[key]=v;b=Value(m);pna_set(e,base,b);}");
    out.push_str("inline Value pna_input(const char*prompt){if(prompt&&prompt[0]!='\\0'){std::cout<<prompt;std::cout.flush();}std::string s;if(!(std::cin>>s))s=\"\";char*end=nullptr;double d=std::strtod(s.c_str(),&end);if(end!=s.c_str()&&*end=='\\0')return Value(d);return Value(s);}inline void pna_log(const Value&v){std::cout<<as_str(v)<<'\\n';}");
    out.push_str("inline Value pna_add(const Value&a,const Value&b){if(is_str(a)||is_str(b))return Value(as_str(a)+as_str(b));return Value(as_num(a)+as_num(b));}inline Value pna_sub(const Value&a,const Value&b){return Value(as_num(a)-as_num(b));}inline Value pna_mul(const Value&a,const Value&b){if(is_str(a)&&is_num(b)){const std::string&s=std::get<std::string>(a.v);long long rll=(long long)std::llround(as_num(b));int r=(rll<0)?0:(int)rll;std::string out;out.reserve(s.size()*(size_t)std::max(r,0));for(char c:s){for(int i=0;i<r;++i)out.push_back(c);}return Value(out);}if(is_num(a)&&is_str(b))return pna_mul(b,a);return Value(as_num(a)*as_num(b));}");
    out.push_str("inline Value pna_div(const Value&a,const Value&b){double r=as_num(b);return Value(r==0.0?0.0:as_num(a)/r);}inline Value pna_mod(const Value&a,const Value&b){double x=as_num(a),y=as_num(b);if(y==0.0)return Value(0.0);double q=std::floor((x/y)+1e-12);double r=x-q*y;if(std::fabs(r)<1e-12)r=0.0;return Value(r);}inline Value pna_eq(const Value&a,const Value&b){if(is_num(a)&&is_num(b))return Value(as_num(a)==as_num(b));return Value(as_str(a)==as_str(b));}inline Value pna_neq(const Value&a,const Value&b){if(is_num(a)&&is_num(b))return Value(as_num(a)!=as_num(b));return Value(as_str(a)!=as_str(b));}inline Value pna_lt(const Value&a,const Value&b){return Value(as_num(a)<as_num(b));}inline Value pna_le(const Value&a,const Value&b){return Value(as_num(a)<=as_num(b));}inline Value pna_gt(const Value&a,const Value&b){return Value(as_num(a)>as_num(b));}inline Value pna_ge(const Value&a,const Value&b){return Value(as_num(a)>=as_num(b));}inline Value pna_and(const Value&a,const Value&b){return Value(truthy(a)&&truthy(b));}inline Value pna_or(const Value&a,const Value&b){return Value(truthy(a)||truthy(b));}inline Value pna_not(const Value&a){return Value(!truthy(a));}inline bool pna_truthy(const Value&v){return truthy(v);}");

    for it in &p.items {
        if let Item::Func { .. } = it {
            emit_func(&mut out, it)?;
        }
    }

    out.push_str("namespace pna_prog {\nint pna_main(){ Env env;\n");
    let mut cg = CgState::default();
    for it in &p.items {
        if let Item::Stmt(s) = it {
            emit_stmt(&mut out, s, "env", 1, &mut cg);
        }
    }
    out.push_str(
        "return 0; }\n} // namespace pna_prog\nint main(){ return pna_prog::pna_main(); }\n",
    );
    Ok(out)
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

fn emit_func(out: &mut String, it: &Item) -> Result<(), String> {
    let (name, params, body) = match it {
        Item::Func {
            name, params, body, ..
        } => (name, params, body),
        _ => return Ok(()),
    };
    write!(out, "static Value fn_{}(Env& env", esc(name)).unwrap();
    for p in params {
        write!(out, ", Value {}", p.name).unwrap();
    }
    out.push_str("){\n  Env __fenv; __fenv.parent=&env;\n");
    for p in params {
        writeln!(out, "  pna_set(__fenv,\"{}\",{});\n", esc(&p.name), p.name).unwrap();
    }
    let mut cg = CgState::default();
    for s in body {
        emit_stmt(out, s, "__fenv", 1, &mut cg);
    }
    out.push_str("  return Value();\n}\n");
    Ok(())
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
        Stmt::Return(opt) => match opt {
            Some(e) => {
                let r = emit_expr(e, env);
                out.push_str(&format!("{}return {};\n", indent(lvl), r));
            }
            None => {
                out.push_str(&format!("{}return Value();\n", indent(lvl)));
            }
        },
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
        Expr::Call { name, args } => {
            let xs: Vec<String> = args.iter().map(|e| emit_expr(e, env)).collect();
            if xs.is_empty() {
                format!("fn_{}({})", name, env)
            } else {
                format!("fn_{}({}, {})", name, env, xs.join(", "))
            }
        }
    }
}
