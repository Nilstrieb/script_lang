//!
//! The AST module contains all structs and enums for the abstract syntax tree generated by the parser

use crate::errors::Span;
use crate::value::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    pub sym: Symbol,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program(pub Vec<Stmt>);

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Declaration(Declaration),
    Assignment(Assignment),
    FnDecl(FnDecl),
    If(IfStmt),
    Loop(Block, Span),
    While(WhileStmt),
    Break(Span),
    Return(Option<Expr>, Span),
    Block(Block),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub span: Span,
    pub name: Ident,
    pub init: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub span: Span,
    pub lhs: Expr,
    pub rhs: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDecl {
    pub span: Span,
    pub name: Ident,
    pub params: Vec<Ident>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub span: Span,
    pub cond: Expr,
    pub body: Block,
    pub else_part: Option<Box<ElsePart>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElsePart {
    Else(Block, Span),
    ElseIf(IfStmt, Span),
}

impl ElsePart {
    pub fn span(&self) -> Span {
        match self {
            ElsePart::Else(_, span) => *span,
            ElsePart::ElseIf(_, span) => *span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStmt {
    pub span: Span,
    pub cond: Expr,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Ident(Ident),
    Literal(Literal),
    UnaryOp(Box<UnaryOp>),
    BinaryOp(Box<BinaryOp>),
    Call(Box<Call>),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Literal(lit) => lit.span(),
            Expr::UnaryOp(unary) => unary.span,
            Expr::BinaryOp(binary) => binary.span,
            Expr::Ident(Ident { span, .. }) => *span,
            Expr::Call(call) => call.span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String, Span),
    Number(f64, Span),
    Array(Vec<Expr>, Span),
    Object(Span),
    Boolean(bool, Span),
    Null(Span),
}

impl Literal {
    pub fn span(&self) -> Span {
        match self {
            Literal::String(_, span) => *span,
            Literal::Number(_, span) => *span,
            Literal::Array(_, span) => *span,
            Literal::Object(span) => *span,
            Literal::Boolean(_, span) => *span,
            Literal::Null(span) => *span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOp {
    pub span: Span,
    pub expr: Expr,
    pub kind: UnaryOpKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOpKind {
    Not,
    Neg,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOp {
    pub span: Span,
    pub lhs: Expr,
    pub rhs: Expr,
    pub kind: BinaryOpKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOpKind {
    And,
    Or,
    Equal,
    GreaterEqual,
    Greater,
    LessEqual,
    Less,
    NotEqual,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub callee: Expr,
    pub span: Span,
    pub kind: CallKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CallKind {
    Field(Ident),
    Fn(Vec<Expr>),
}
