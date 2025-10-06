#[derive(Debug, Clone)]
pub enum Ty {
    Double,
    Int,
    String,
    Void,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    #[allow(dead_code)]
    pub ty: Ty,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Num(f64),
    Str(String),
    Bool(bool),
    Ident(String),
    Member(Box<Expr>, String),
    Unary {
        op: String,
        rhs: Box<Expr>,
    },
    Binary {
        op: String,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Target {
    Var(String),
    Prop { base: String, key: String },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    ObjBlock {
        name: String,
        fields: Vec<(String, Expr)>,
    },
    PropAssign {
        base: String,
        key: String,
        expr: Expr,
    },
    VarAssign {
        name: String,
        expr: Expr,
    },
    Log(Expr),
    Cond {
        cond: Expr,
        then_blk: Vec<Stmt>,
        else_blk: Option<Vec<Stmt>>,
    },
    Loop {
        cond: Expr,
        body: Vec<Stmt>,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
        ended: Option<Vec<Stmt>>,
    },
    Input {
        prompt: String,
        dst: Target,
    },
    Break,
    Continue,
    Return(Option<Expr>),
}

#[derive(Debug, Clone)]
pub enum Item {
    Stmt(Stmt),
    Func {
        name: String,
        params: Vec<Param>,
        #[allow(dead_code)]
        ret: Ty,
        body: Vec<Stmt>,
    },
}

#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Item>,
}
