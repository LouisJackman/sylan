//! Sylan consists of items and expressions. Items are declarative whereas
//! expressions are executed and yield values. Such values can be the void value
//! for expressions executed solely for side-effects. "Statements" can be approximated by stacking
//! expressions one after the other and discarding their values.

use std::collections::{HashSet, LinkedList};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use common::multiphase::{Identifier, InterpolatedString, Shebang, SyDoc, SylanString};
use common::version::Version;

/// Shebangs and source versions are special, which is why they're outside of
/// the `FilePackage` in which all other items and expressions reside. Both
/// shebangs must be completely resolved before anything else can be parsed,
/// and the result of parsing version can completely change the lexing and
/// parsing of all subsequent tokens.
pub struct File {
    pub shebang: Option<Shebang>,
    pub version: Option<Version>,
    pub package: FilePackage,
}

/// The declarations that make up the static structure of a Sylan program. Items
/// can't be contained within expressions, with the exception of bindings.
#[derive(Clone)]
pub enum Item {
    Package(Package),
    Class(Class),
    Extension(Extension),
    Interface(Interface),
    SyDoc(SyDoc),
    ContextualIgnoral(ContextualIgnoral),
    Alias(Alias),
    Import(Import),
    Func(Func),

    // Unlike the previous variants, these can be arbitrarily nested within
    // expressions. This is to allow corecursion among other features.
    //
    // For loops also create bindings, but are not items because I can't
    // think of a use case for mutually recursive loop continuation bindings.
    Binding(Binding),
}

/// The expressions that allow Turing-complete computations, i.e. allowing
/// Sylan to do actual useful work.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    UnaryOperator(UnaryOperator, Box<Expression>),
    BinaryOperator(BinaryOperator, Box<Expression>, Box<Expression>),
    Switch(Switch),
    Select(Select),
    Context(Scope),
    Using(Using),
    If(If),
    IfLet(IfLet),
    Cond(Cond),
    For(For),
    Continue(Continue),
    Call(Call),
    Lookup(Lookup),
    Throw(Throw),
    While(Throw),
    WhileLet(WhileLet),
}

/// Every node in Sylan is either an item or an expression, even the special
/// shebang and version tokens (both of which are technically items).
pub enum Node {
    Item(Item),
    Expression(Expression),
}

/// Packages only have items at top-level, with the exception of the main package that can also have
/// executable code to simplify small scripts.
#[derive(Clone)]
pub struct Package {
    pub accessibility: Accessibility,
    pub name: Identifier,
    pub items: Vec<Item>,
}

pub struct MainPackage {
    pub package: Package,
    pub code: Code,
}

pub enum FilePackage {
    EntryPoint(MainPackage),

    // TODO: this needs to be removed; each package will be lexed and parsed individually
    // and linked in a later phase, probably in the simplification but more likely one of the
    // backends.
    Imported(Package),
}

#[derive(Clone)]
pub struct Import {
    pub lookup: Lookup,
}

// Funcs are ultimately just lambdas used in a binding, the syntactic equivalent of combining `var`
// with a lambda expression. However, this is only realised during the simplification stage; at
// this stage they are still distinct.
//
// One notable difference is that omitting a return type on a lambda triggers type inference,
// whereas it always means the `Void` type for `func`. Also, `Func` expects the lambda signature to
// explicitly type every parameter, and will quickly fail in a later stage if any type annotations
// are missing. This is because `func` is intended to be used for top-level functions that define
// the shape of the program or API, in which types should always be explicitly annotated anyway.
#[derive(Clone)]
pub struct Func {
    pub name: Identifier,
    pub lambda: Lambda,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Accessibility {
    Public,
    Internal,
    Private,
}

#[derive(Clone)]
pub enum DeclarationItem {
    Binding(Binding),
    Type(Type),
    Package(Package),
}

#[derive(Clone)]
pub struct Declaration {
    pub accessibility: Accessibility,
    pub item: DeclarationItem,
}

/// Concrete classes that support implementing interfaces and embedding other
/// classes, but cannot extend other classes directly.
#[derive(Clone)]
pub struct Class {
    pub implements: LinkedList<Type>,
    pub methods: HashSet<Method>,
}

/// Interfaces that support extending other interfaces, providing empty methods
/// that implementors must implement, providing already-defined utility methods,
/// and even allowing already-defined methods to be specialised via overriding
/// in implementing classes.
#[derive(Clone)]
pub struct Interface {
    pub extends: LinkedList<Type>,
    pub methods: HashSet<Method>,
}

#[derive(Clone)]
pub enum TypeItem {
    Class(Class),
    Interface(Interface),
}

#[derive(Clone)]
pub enum Extension {
    Class(Class),
    Interface(Interface),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Type {
    pub name: Identifier,
    pub arguments: Vec<Argument<Type>>,
}

#[derive(Clone)]
pub struct DeclarationModifiers {
    accessibility: Accessibility,
}

#[derive(Clone)]
pub struct MethodModifiers {
    declaration: DeclarationModifiers,
    is_virtual: bool,
    overrides: bool,
    default: bool,
}

/// Methods and just bindings in a class, which can be potentially abstract (i.e. with no initial
/// value) in interfaces, can be overridable and virtual in interfaces, and must be tied to either
/// a class an interface. There is no meaningful distintion between a method and an attribute: a
/// `method` is just a binding in a class and it works like a traditional OOP method
/// when that binding contains a lambda.
///
/// They are just normal bindings but tied to their type. Like Python and unlike JS, their
/// reference to their type and instance are bound to the method itself.
///
/// Type annotations are only optional in a special case: direct literals. This means a very common
/// case, OOP-style methods, don't require spelling out type annotations twice as lambdas are
/// literals.
#[derive(Clone)]
pub struct Method {
    pub name: Identifier,
    pub modifiers: MethodModifiers,
    pub type_annotation: Option<Type>,
    pub non_abstract_value: Option<Expression>,
}

/// Value parameters are for values at runtime and have identifiers and
/// optional default values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValueParameter {
    pub pattern: Pattern,
    pub explicit_type_annotation: Option<Type>,
    pub default_value: Option<Expression>,
}

/// Type parameters are for types at compile-time and have optional upper
/// bounds, identifiers, and optional default values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeParameter {
    pub name: Identifier,
    pub upper_bounds: Vec<Type>,
    pub default_value: Option<Type>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Argument<T> {
    pub value: T,
    pub identifier: Option<Identifier>,
}

/// Value arguments are for values at runtime. They support being passed as
/// positional or keyword arguments; unlike other languages it is the choice of
/// the caller rather than the definer. If passed as a keyword argument, an
/// identifier is carried with it in the parse tree.
type ValueArgument = Argument<Expression>;

/// Type arguments are for values at runtime. They support being passed as
/// positional or keyword arguments; unlike other languages it is the choice of
/// the caller rather than the definer. If passed as a keyword argument, an
/// identifier is carried with it in the parse tree.
type TypeArguments = Argument<Type>;

// Sylan's "symbol tables" are just a collection of bindings in the current
// scope. Parent scopes can be looked up to find bindings in outer closures,
// which is how lexical scoping is implemented.

/// Bindings are for execution-time values. Statically deducible values go via
/// package and type definitions instead. (Note that "execution-time" can mean
/// both "runtime" and "running within a compile-time macro.)
#[derive(Clone, Debug, Eq)]
pub struct Binding {
    pub pattern: Pattern,
    pub value: Box<Expression>,
    pub explicit_type_annotation: Option<Type>,
}

impl PartialEq for Binding {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
    }
}

impl Hash for Binding {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pattern.hash(state)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Alias {
    pub new: Identifier,
    pub original: Lookup,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextualIgnoral {
    pub value: Expression,
}

/// Expressions are seperate from bindings.
type Expressions = Vec<Expression>;

/// Bindings within a code block are resolved before executing its
/// expressions, which is why they're items rather than expressions.
/// This is to allow techniques like mutual recursion and
/// self-referential functions and methods without forward declarations.
///
/// Note that the declarations aren't accessible until their declarations have
/// been executed, but don't cause compilation problems if accessed within a
/// delayed computation within the same scope.
///
/// In other words, these declarations are block scoped with a temporal dead
/// zone rather than using scope hoisting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Code {
    pub bindings: HashSet<Binding>,
    pub expressions: Expressions,
}

impl Code {
    pub fn new() -> Self {
        Self {
            bindings: HashSet::new(),
            expressions: vec![],
        }
    }
}

/// Scopes, unlike non-main packages, can contain executable code. Unlike all
/// packages, they can refer to parent scopes. They can declare variables with
/// bindings but cannot declare new types or subpackages like packages can.
///
/// All functions, methods, and lambdas have an attached scope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Scope {
    pub code: Code,
    pub parent: Option<Rc<Scope>>,
}

impl Scope {
    pub fn new_root() -> Rc<Self> {
        let code = Code::new();
        let parent = None;
        Rc::new(Self { code, parent })
    }

    pub fn within(parent: &Rc<Scope>) -> Rc<Self> {
        let code = Code::new();
        let parent = Some(parent.clone());
        Rc::new(Self { code, parent })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LambdaSignature {
    pub type_parameters: Vec<TypeParameter>,
    pub value_parameters: Vec<ValueParameter>,
    pub explicit_return_type_annotation: Option<Type>,
    pub ignorable: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lambda {
    pub signature: LambdaSignature,
    pub scope: Scope,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Literal {
    Boolean(bool),
    Char(char),
    InterpolatedString(InterpolatedString),
    Number(i64, u64),
    String(SylanString),
    Lambda(Lambda),
}

/// A lookup is an expression, but its information should be completely resolvable in the parsing
/// and semantic analysis. It allows looking items up in static program structure, e.g. types and
/// packages.
pub type Lookup = Vec<Identifier>;

/// Sylan allows defining operations but only binary ones. Unary ones are fixed in the language
/// and cannot be used as names for identifiers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnaryOperator {
    InvocableHandle,
    Not,
    ContextualBind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BinaryOperator(Identifier);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Switch {
    pub expression: Box<Expression>,
    pub cases: Vec<Case>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Timeout {
    pub nanoseconds: Box<Expression>,
    pub body: Scope,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Select {
    pub message_type: Type,
    pub cases: Vec<Case>,
    pub timeout: Option<Timeout>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LambdaArgument {
    Normal(Argument<Expression>),
    Entry(Box<Expression>, Box<Expression>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Call {
    pub target: Box<Expression>,
    pub arguments: Vec<LambdaArgument>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Using(Box<Expression>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct If {
    pub condition: Box<Expression>,
    pub then: Scope,
    pub else_clause: Option<Scope>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IfLet {
    pub binding: Binding,
    pub then: Scope,
    pub else_clause: Option<Scope>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CondCase {
    pub conditions: LinkedList<Expression>,
    pub then: Scope,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cond(pub Vec<CondCase>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaseMatch {
    pub pattern: Pattern,
    pub guard: Option<Expression>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Case {
    pub matches: LinkedList<CaseMatch>,
    pub body: Scope,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct For {
    pub bindings: Vec<Binding>,
    pub scope: Scope,
    pub label: Option<Identifier>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct While {
    pub condition: Box<Expression>,
    pub scope: Scope,
    pub label: Option<Identifier>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WhileLet {
    pub binding: Binding,
    pub scope: Scope,
    pub label: Option<Identifier>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Continue {
    pub bindings: Vec<Argument<Expression>>,
    pub label: Option<Identifier>,
}

/// Throwing an expression does not yield a value as it destroys its current
/// process. However, it is an expression and can therefore be used anywhere an
/// expression can be used. It can throw any expression that yields a type which
/// implements the Exception interface. In "returns" the bottom type which
/// allows it to be used anywhere.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Throw(pub Box<Expression>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatternGetter {
    pub identifier: Identifier,
    pub pattern: Pattern,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositePattern {
    pub composite_type: Type,
    pub getters: Vec<PatternGetter>,
    pub ignore_rest: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PatternItem {
    Literal(Literal),
    Identifier(Identifier),
    Composite(CompositePattern),
    Ignored,
}

#[derive(Clone, Debug, Eq)]
pub struct Pattern {
    pub item: PatternItem,
    pub bound_match: Option<Identifier>,
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.bound_match == other.bound_match
    }
}

impl Hash for Pattern {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bound_match.hash(state)
    }
}
