#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum PySyntaxKind {
    None,

    // Root
    Module,
    Suite, // a block of statements (indented)

    // Statements
    ExprStmt,       // expression statement
    AssignStmt,     // assignment statement
    AnnAssignStmt,  // annotated assignment
    AugAssignStmt,  // augmented assignment (+=, -=, etc.)
    RaiseStmt,      // raise statement
    AssertStmt,     // assert statement
    DeleteStmt,     // del statement
    PassStmt,       // pass statement
    BreakStmt,      // break statement
    ContinueStmt,   // continue statement
    ReturnStmt,     // return statement
    YieldStmt,      // yield statement
    GlobalStmt,     // global statement
    NonlocalStmt,   // nonlocal statement
    ImportStmt,     // import statement
    ImportFromStmt, // from ... import statement

    // Compound statements
    IfStmt,        // if statement
    ElifClause,    // elif clause
    ElseClause,    // else clause
    WhileStmt,     // while statement
    ForStmt,       // for statement
    AsyncForStmt,  // async for statement
    WithStmt,      // with statement
    AsyncWithStmt, // async with statement
    TryStmt,       // try statement
    ExceptClause,  // except clause
    FinallyClause, // finally clause
    MatchStmt,     // match statement (Python 3.10+)
    CaseClause,    // case clause

    // Function and class definitions
    FuncDef,      // function definition
    AsyncFuncDef, // async function definition
    ClassDef,     // class definition

    // Type annotations
    TypeAlias,      // type alias
    TypeAnnotation, // type annotation

    // Expressions
    NameExpr,    // identifier/name
    LiteralExpr, // literal values (int, float, string, etc.)
    FStringExpr, // f-string expression with embedded expressions
    TStringExpr, // t-string expression (template string, Python 3.14+)
    ParenExpr,   // parenthesized expression
    TupleExpr,   // tuple expression
    ListExpr,    // list expression
    DictExpr,    // dictionary expression
    SetExpr,     // set expression

    // Unary and binary operations
    UnaryExpr,       // unary operations (+x, -x, not x, ~x)
    BinaryExpr,      // binary operations (x + y, x and y, etc.)
    BoolOpExpr,      // boolean operations (and, or)
    CompareExpr,     // comparisons (x < y, x == y, etc.)
    AssignExpr,      // assignment expression (walrus operator :=)
    ConditionalExpr, // conditional expression (x if condition else y)

    // Function and method calls
    CallExpr,       // function call
    MethodCallExpr, // method call

    // Subscripting and attribute access
    SubscriptExpr, // x[y]
    AttributeExpr, // x.y
    SliceExpr,     // x[start:stop:step]

    // Lambda and comprehensions
    LambdaExpr,    // lambda expression
    ListCompExpr,  // list comprehension
    DictCompExpr,  // dictionary comprehension
    SetCompExpr,   // set comprehension
    GeneratorExpr, // generator expression

    // Conditional and special expressions
    IfExpr,            // conditional expression (x if condition else y)
    YieldExpr,         // yield expression
    YieldFromExpr,     // yield from expression
    AwaitExpr,         // await expression
    StarredExpr,       // *expression
    DoubleStarredExpr, // **expression

    // Python 3.9+ features
    DictMergeExpr,  // dictionary merge with | operator
    DictUpdateExpr, // dictionary update with |= operator

    // Python 3.10+ features
    MatchExpr,       // match expression (hypothetical)
    GuardClause,     // guard clause in pattern matching (if condition)
    WildcardPattern, // _ wildcard pattern
    ValuePattern,    // literal value pattern
    BindPattern,     // variable binding pattern
    ClassPattern,    // class pattern matching
    SequencePattern, // sequence pattern (list/tuple)
    MappingPattern,  // mapping pattern (dict)
    OrPattern,       // | pattern (pattern1 | pattern2)
    UnionType,       // union type with | (int | str)

    // Python 3.11+ features
    TryStarStmt,      // try* statement for exception groups
    ExceptStarClause, // except* clause
    ExceptGroupStmt,  // except* statement for exception groups
    TaskGroup,        // async task group
    TypeParam,        // generic type parameter [T]
    TypeConstraint,   // type constraint [T: int | str]
    SelfType,         // Self type annotation

    // Python 3.12+ features
    TypeStatement,   // type alias statement (type X = Y)
    TypeAliasStmt,   // type alias statement
    GenericFuncDef,  // function with type parameters
    GenericClassDef, // class with type parameters
    GenericClass,    // class with type parameters
    GenericFunction, // function with type parameters
    TypeParameters,  // type parameter list

    // Python 3.13+ features
    OverrideDecorator, // @override decorator
    FrozenDataclass,   // frozen dataclass enhancement
    SlottedDataclass,  // slotted dataclass enhancement

    // Python 3.14+ features (experimental)
    NullCoalescing,     // ?? null coalescing operator (hypothetical)
    MatchExpression,    // match as expression (hypothetical)
    AsyncComprehension, // async comprehension enhancements
    AsyncCompStmt,      // async comprehensive statement
    TimeoutContext,     // async timeout context
    EnhancedPattern,    // enhanced pattern matching
    Decorated,          // decorated statement

    // Other nodes
    Parameter,  // function parameter
    Parameters, // parameter list
    Arguments,  // argument list
    Keyword,    // keyword argument
    Alias,      // import alias (as clause)
    Decorator,  // decorator
    Decorators, // list of decorators
    Docstring,  // docstring (first string literal in module/class/function)

    // Comments and whitespace
    Comment, // comment
    Newline, // newline

    UnknownStat, // unknown statement
}
