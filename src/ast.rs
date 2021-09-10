use crate::Number;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Identifier<'a>(pub &'a str);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ModuleIdent<'a>(pub &'a str);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnaryOp {
    Plus,
    Minus,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Alt,
    And,
    Or,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UpdateOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Alt,
    Modify,
    Assign,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Comparator {
    Eq,
    Neq,
    Gt,
    Ge,
    Lt,
    Le,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Index<'a> {
    /// '[' ']'
    Explode,
    /// '.' <ident> | '.' <string>
    Index(&'a str),
    /// '[' query ']'
    Query(Box<Query<'a>>),
    /// '[' (<query>)? ':' (<query>)? ']' except '[' ':' ']'
    Slice(Option<Box<Query<'a>>>, Option<Box<Query<'a>>>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StringFragment<'a> {
    String(&'a str),
    Char(char),
    Query(Query<'a>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ObjectBindPatternEntry<'a> {
    /// (<ident> | <variable> | <keyword> | <string> | '(' <query> ')') ':' pattern
    KeyValue(Box<Query<'a>>, Box<BindPattern<'a>>),
    /// <variable>
    KeyOnly(Identifier<'a>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BindPattern<'a> {
    /// <variable>
    Variable(Identifier<'a>),
    /// '[' <patten> (',' <pattern>)* ']'
    Array(Vec<BindPattern<'a>>),
    /// '{' <object pattern elem> (',' <object pattern elem>)* '}'
    Object(Vec<ObjectBindPatternEntry<'a>>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FuncDef<'a> {
    pub name: Identifier<'a>,
    pub args: Vec<Identifier<'a>>,
    pub body: Box<Query<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Term<'a> {
    /// 'null'
    Null,
    /// 'true'
    True,
    /// 'false'
    False,
    /// <number>
    Number(Number),
    /// <string>
    String(Vec<StringFragment<'a>>),

    /// '.'
    Identity,
    /// '..'
    Recurse,
    /// '.' '[' ( | <query> | (<query>)? ':' (<query>)? ) ']'
    /// '.' (<ident> | <string>)
    /// <term> '[' ( | <query> | (<query>)? ':' (<query>)? ) ']'
    /// <term> '.' '[' ( | <query> | (<query>)? ':' (<query>)? ) ']'
    /// <term> '?'
    /// <term> '.' (<ident> | <string>)
    Index(Box<Term<'a>>, Vec<Index<'a>>),

    /// (<ident> | <moduleident>) ( '(' query (';' query)* ')' )? | (<var> | <modulevar>)
    FunctionCall(Identifier<'a>, Vec<Query<'a>>),
    /// '@' <ident-allowing-num-prefix> (<string>)?
    Format(Identifier<'a>),
    /// '(' <query> ')'
    Query(Box<Query<'a>>),
    /// ('+' | '-') <term>
    Unary(UnaryOp, Box<Term<'a>>),
    /// '{' (<ident> | <variable> | <keyword> | <string> | '(' <query> ')') (':' <term> ('|' <term>)*)? (',' ....)* '}'
    Object(Vec<(Query<'a>, Option<Query<'a>>)>),
    /// '[' (<query>)? ']'
    Array(Option<Box<Query<'a>>>),
    /// 'break' <variable>
    Break(Identifier<'a>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Query<'a> {
    /// <term>
    Term(Box<Term<'a>>),
    /// 'def' <ident> ':' <query> ';' <query>
    WithFunc(FuncDef<'a>, Box<Query<'a>>),
    /// <query> ('|' <query>)+
    Pipe(Box<Query<'a>>, Box<Query<'a>>),
    /// <query> (',' <query>)+
    Concat(Box<Query<'a>>, Box<Query<'a>>),
    /// <term> 'as' <pattern> ('?//' <pattern>)* '|' <query>
    Bind(Box<Term<'a>>, Vec<BindPattern<'a>>, Box<Query<'a>>),
    /// 'reduce' <term> 'as' <pattern> '(' <query> ';' <query> ')'
    Reduce(
        Box<Term<'a>>,
        BindPattern<'a>,
        Box<Query<'a>>,
        Box<Query<'a>>,
    ),
    /// 'foreach' <term> 'as' <pattern> '(' <query> ';' <query> (';' <query>)? ')'
    ForEach(
        Box<Term<'a>>,
        BindPattern<'a>,
        Box<Query<'a>>,
        Box<Query<'a>>,
        Option<Box<Query<'a>>>,
    ),
    /// 'if' <query> 'then' <query> ('elif' <query> 'then' <query>)* ('else' <query>)? 'end'
    If {
        cond: Box<Query<'a>>,
        positive: Box<Query<'a>>,
        negative: Option<Box<Query<'a>>>,
    },
    /// 'try' <query> ('catch' <query>)?
    Try(Box<Query<'a>>, Option<Box<Query<'a>>>),
    /// 'label' <variable> '|' <query>
    Label(Identifier<'a>, Box<Query<'a>>),
    /// <query> '?'
    Optional(Box<Query<'a>>),

    /// <query> ('//' | '+' | '-' | '*' | '/' | '%' | 'and' | 'or') <query>
    Operate(Box<Query<'a>>, BinaryOp, Box<Query<'a>>),
    /// <query> ('=' | '|=' | '//=' | '+=' | '-=' | '*=' | '/=' | '%=') <query>
    Update(Box<Query<'a>>, UpdateOp, Box<Query<'a>>),
    /// <query> <comparator> <query>
    Compare(Box<Query<'a>>, Comparator, Box<Query<'a>>),
}
