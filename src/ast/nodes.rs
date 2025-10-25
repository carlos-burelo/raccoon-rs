use super::types::Type;
use crate::tokens::{AccessModifier, BinaryOperator, Position, TokenType, UnaryOperator};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Program,
    VarDecl,
    FnDecl,
    ClassDecl,
    ConstructorDecl,
    PropertyAccessor,
    DecoratorDecl,
    InterfaceDecl,
    EnumDecl,
    TypeAliasDecl,
    ImportDecl,
    ExportDecl,
    Block,
    IfStmt,
    WhileStmt,
    ForStmt,
    ForInStmt,
    ReturnStmt,
    BreakStmt,
    ContinueStmt,
    ExprStmt,
    TryStmt,
    CatchClause,
    ThrowStmt,
    BinaryExpr,
    UnaryExpr,
    CallExpr,
    NewExpr,
    MemberExpr,
    MethodCallExpr,
    IndexExpr,
    AwaitExpr,
    ThisExpr,
    SuperExpr,
    TypeOfExpr,
    InstanceOfExpr,
    ArrowFnExpr,
    Identifier,
    Assignment,
    RangeExpr,
    ConditionalExpr,
    NullCoalescingExpr,
    OptionalChainingExpr,
    NullAssertionExpr,
    UnaryUpdateExpr,
    TemplateStr,
    TaggedTemplateExpr,
    IntLiteral,
    FloatLiteral,
    StrLiteral,
    BoolLiteral,
    NullLiteral,
    ListLiteral,
    ObjectLiteral,
    ListPattern,
    ObjectPattern,
    RestElement,
}

pub trait ASTNodeTrait {
    fn node_type(&self) -> NodeType;
    fn position(&self) -> Position;
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Stmt(Stmt),
    Expr(Expr),
    Pattern(DestructuringPattern),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Program(Program),
    VarDecl(VarDecl),
    FnDecl(FnDecl),
    ClassDecl(ClassDecl),
    InterfaceDecl(InterfaceDecl),
    EnumDecl(EnumDecl),
    TypeAliasDecl(TypeAliasDecl),
    ImportDecl(ImportDecl),
    ExportDecl(ExportDecl),
    Block(Block),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
    ForStmt(ForStmt),
    ForInStmt(ForInStmt),
    ReturnStmt(ReturnStmt),
    BreakStmt(BreakStmt),
    ContinueStmt(ContinueStmt),
    ExprStmt(ExprStmt),
    TryStmt(TryStmt),
    ThrowStmt(ThrowStmt),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Call(CallExpr),
    New(NewExpr),
    Member(MemberExpr),
    MethodCall(MethodCallExpr),
    Index(IndexExpr),
    Await(AwaitExpr),
    This(ThisExpr),
    Super(SuperExpr),
    TypeOf(TypeOfExpr),
    InstanceOf(InstanceOfExpr),
    ArrowFn(ArrowFnExpr),
    Identifier(Identifier),
    Assignment(Assignment),
    Range(RangeExpr),
    Conditional(ConditionalExpr),
    NullCoalescing(NullCoalescingExpr),
    OptionalChaining(OptionalChainingExpr),
    NullAssertion(NullAssertionExpr),
    UnaryUpdate(UnaryUpdateExpr),
    TemplateStr(TemplateStrExpr),
    TaggedTemplate(TaggedTemplateExpr),
    IntLiteral(IntLiteral),
    FloatLiteral(FloatLiteral),
    StrLiteral(StrLiteral),
    BoolLiteral(BoolLiteral),
    NullLiteral(NullLiteral),
    ListLiteral(ListLiteral),
    ObjectLiteral(ObjectLiteral),
    Spread(SpreadExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DestructuringPattern {
    List(ListPattern),
    Object(ObjectPattern),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub stmts: Vec<Stmt>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarPattern {
    Identifier(String),
    Destructuring(DestructuringPattern),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub pattern: VarPattern,
    pub type_annotation: Type,
    pub initializer: Option<Expr>,
    pub is_constant: bool,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnParam {
    pub pattern: VarPattern,
    pub param_type: Type,
    pub default_value: Option<Expr>,
    pub is_rest: bool,
    pub is_optional: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDecl {
    pub name: String,
    pub type_parameters: Vec<super::types::TypeParameter>,
    pub parameters: Vec<FnParam>,
    pub return_type: Option<Type>,
    pub body: Vec<Stmt>,
    pub is_async: bool,
    pub is_declare: bool,
    pub decorators: Vec<DecoratorDecl>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassProperty {
    pub name: String,
    pub property_type: Type,
    pub initializer: Option<Expr>,
    pub decorators: Vec<DecoratorDecl>,
    pub access_modifier: AccessModifier,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassMethod {
    pub name: String,
    pub parameters: Vec<FnParam>,
    pub return_type: Option<Type>,
    pub body: Vec<Stmt>,
    pub is_async: bool,
    pub decorators: Vec<DecoratorDecl>,
    pub access_modifier: AccessModifier,
    pub is_static: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDecl {
    pub name: String,
    pub type_parameters: Vec<super::types::TypeParameter>,
    pub superclass: Option<String>,
    pub properties: Vec<ClassProperty>,
    pub constructor: Option<ConstructorDecl>,
    pub methods: Vec<ClassMethod>,
    pub accessors: Vec<PropertyAccessor>,
    pub decorators: Vec<DecoratorDecl>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstructorDecl {
    pub parameters: Vec<FnParam>,
    pub body: Vec<Stmt>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyAccessor {
    pub name: String,
    pub kind: AccessorKind,
    pub parameters: Vec<FnParam>,
    pub return_type: Option<Type>,
    pub body: Vec<Stmt>,
    pub access_modifier: AccessModifier,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AccessorKind {
    Get,
    Set,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DecoratorDecl {
    pub name: String,
    pub args: Vec<Expr>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceDeclProperty {
    pub name: String,
    pub property_type: Type,
    pub optional: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceDecl {
    pub name: String,
    pub type_parameters: Vec<super::types::TypeParameter>,
    pub properties: Vec<InterfaceDeclProperty>,
    pub extends: Vec<String>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumMember {
    pub name: String,
    pub value: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDecl {
    pub name: String,
    pub members: Vec<EnumMember>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAliasDecl {
    pub name: String,
    pub alias_type: Type,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportSpecifier {
    pub imported: String,
    pub local: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportDecl {
    pub default_import: Option<String>,
    pub named_imports: Vec<ImportSpecifier>,
    pub namespace_import: Option<String>,
    pub module_specifier: String,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExportSpecifier {
    pub local: String,
    pub exported: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExportDecl {
    pub declaration: Option<Box<Stmt>>,
    pub specifiers: Vec<ExportSpecifier>,
    pub is_default: bool,
    pub module_specifier: Option<String>, // For export { ... } from "module"
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Stmt>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForStmt {
    pub initializer: Option<Box<Stmt>>,
    pub condition: Option<Expr>,
    pub increment: Option<Expr>,
    pub body: Box<Stmt>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForInStmt {
    pub variable: String,
    pub is_const: bool,
    pub type_annotation: Option<Type>,
    pub iterable: Expr,
    pub body: Box<Stmt>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub value: Option<Expr>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BreakStmt {
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContinueStmt {
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprStmt {
    pub expression: Expr,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TryStmt {
    pub try_block: Block,
    pub catch_clauses: Vec<CatchClause>,
    pub finally_block: Option<Block>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatchClause {
    pub error_var: String,
    pub error_type: Option<Type>,
    pub body: Block,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowStmt {
    pub value: Expr,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: BinaryOperator,
    pub right: Box<Expr>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub operand: Box<Expr>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
    pub named_args: HashMap<String, Expr>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NewExpr {
    pub class_name: String,
    pub type_args: Vec<Type>,
    pub args: Vec<Expr>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberExpr {
    pub object: Box<Expr>,
    pub property: String,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodCallExpr {
    pub object: Box<Expr>,
    pub method: String,
    pub args: Vec<Expr>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexExpr {
    pub object: Box<Expr>,
    pub index: Box<Expr>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AwaitExpr {
    pub expression: Box<Expr>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThisExpr {
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SuperExpr {
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeOfExpr {
    pub operand: Box<Expr>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InstanceOfExpr {
    pub operand: Box<Expr>,
    pub type_name: String,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrowFnExpr {
    pub parameters: Vec<FnParam>,
    pub return_type: Option<Type>,
    pub body: ArrowFnBody,
    pub is_async: bool,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArrowFnBody {
    Expr(Box<Expr>),
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub target: Box<Expr>,
    pub value: Box<Expr>,
    pub operator: TokenType,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RangeExpr {
    pub start: Box<Expr>,
    pub end: Box<Expr>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalExpr {
    pub condition: Box<Expr>,
    pub then_expr: Box<Expr>,
    pub else_expr: Box<Expr>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NullCoalescingExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OptionalChainingExpr {
    pub object: Box<Expr>,
    pub property: String,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NullAssertionExpr {
    pub operand: Box<Expr>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryUpdateExpr {
    pub operator: UpdateOperator,
    pub operand: Box<Expr>,
    pub is_prefix: bool,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateOperator {
    Increment,
    Decrement,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateStrExpr {
    pub parts: Vec<TemplateStrPart>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateStrPart {
    String(StrLiteral),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TaggedTemplateExpr {
    pub tag: Box<Expr>,
    pub template: TemplateStrExpr,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntLiteral {
    pub value: i64,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FloatLiteral {
    pub value: f64,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StrLiteral {
    pub value: String,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoolLiteral {
    pub value: bool,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NullLiteral {
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListLiteral {
    pub elements: Vec<Expr>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectLiteral {
    pub properties: HashMap<String, Expr>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListPattern {
    pub elements: Vec<Option<ListPatternElement>>,
    pub rest: Option<RestElement>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListPatternElement {
    Identifier(Identifier),
    List(Box<ListPattern>),
    Object(Box<ObjectPattern>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectPattern {
    pub properties: Vec<ObjectPatternProperty>,
    pub rest: Option<RestElement>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectPatternProperty {
    pub key: String,
    pub value: ObjectPatternValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectPatternValue {
    Identifier(Identifier),
    List(ListPattern),
    Object(ObjectPattern),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RestElement {
    pub argument: Identifier,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpreadExpr {
    pub argument: Box<Expr>,
    pub position: Position,
}
