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
    ParenExpr,   // parenthesized expression
    TupleExpr,   // tuple expression
    ListExpr,    // list expression
    DictExpr,    // dictionary expression
    SetExpr,     // set expression

    // Unary and binary operations
    UnaryExpr,   // unary operations (+x, -x, not x, ~x)
    BinaryExpr,  // binary operations (x + y, x and y, etc.)
    BoolOpExpr,  // boolean operations (and, or)
    CompareExpr, // comparisons (x < y, x == y, etc.)

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
    IfExpr,        // conditional expression (x if condition else y)
    YieldExpr,     // yield expression
    YieldFromExpr, // yield from expression
    AwaitExpr,     // await expression
    StarredExpr,   // *expression

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
    Indent,  // indentation
    Dedent,  // dedentation

    UnknownStat, // unknown statement
}
