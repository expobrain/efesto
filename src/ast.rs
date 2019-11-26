// MIT License
//
// Copyright (c) 2018 Hans-Martin Will
// Copyright (c) 2019 Daniele Esposti
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use dict_derive::IntoPyObject;

use super::symbols;

/// The error value; currently this is just a string
pub type Error = super::error::Error;

/// SQL statements that are supported by this implementation
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SqlStatement {
    /// A regular (DML) statement
    Statement(Statement),

    /// Query plan analysis
    ExplainQueryPlan(Statement),

    /// Attach an external file as source for query processing
    Attach(AttachStatement),

    /// Describe a particular schema object
    Describe(DescribeStatement),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Statement {
    Select(SelectStatement),
    Insert(InsertStatement),
    Delete(DeleteStatement),
    Update(UpdateStatement),
}

/// Representation of an insert statement
#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct InsertStatement {
    /// the name of the table into which we want to insert new values
    pub table_name: Vec<symbols::Name>,

    /// an optional list of columns that define a mapping between the provided values and the columns
    /// defined in the table
    pub columns: Option<Vec<symbols::Name>>,

    /// An expression that will yield the rows to insert
    pub source: SetExpression,
}

/// Representation of a common table expression, which provides a short-hand notation for
/// queries within the context of a single statement.
#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct CommonTableExpression {
    /// the name under which we will refer to these query results in the remainder of the query
    /// that is using this common table expression
    pub identifier: symbols::Name,

    /// column names that can define a re-ordering of the values returned by the enclosed query
    pub column_names: Option<Vec<symbols::Name>>,

    /// a query statement that defines the values for this common table expression
    pub query: SelectStatement,
}

/// Representation of a select statement.
#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct SelectStatement {
    /// 0 or more comon table expressions, that can be referenced by the main query expression
    pub common: Vec<CommonTableExpression>,

    /// the query expression
    pub expr: Box<SetExpression>,

    /// if non-empty, an sort-order that is applied to the rows returned as result
    pub order_by: Vec<Ordering>,

    /// an optional limit clause, which can restrict the rows returned to a window within the
    /// set of rows as generated by `expr` and `order_by`.
    pub limit: Option<Box<Limit>>,
}

/// Represenatation of a delete statement
#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct DeleteStatement {
    /// the name of the table from which rows should be deleted
    pub table_name: Vec<symbols::Name>,

    /// a predicate defining the rows to delete
    pub where_expr: Option<Expression>,
}

/// Representation of an update statement
#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct UpdateStatement {
    /// the qualified table name
    pub table_name: Vec<symbols::Name>,

    /// assignments providing new values for table columns
    pub assignments: Vec<Assignment>,

    /// a predicate restricting the set of rows to which the update should be applied
    pub where_expr: Option<Expression>,
}

/// Rerpresentation of an attach statement
#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct AttachStatement {
    /// the table name within the previous (or default) schema
    pub qualified_name: Vec<symbols::Name>,

    /// the file system path of the external file to be attached as table
    pub path: String,
}

impl AttachStatement {
    pub fn new(
        schema: Option<symbols::Name>,
        name: symbols::Name,
        path: String,
    ) -> AttachStatement {
        let mut qualified_name = Vec::new();

        if schema.is_some() {
            qualified_name.push(schema.unwrap())
        }

        qualified_name.push(name);

        AttachStatement {
            qualified_name,
            path,
        }
    }

    pub fn schema_name(&self) -> Option<&symbols::Name> {
        match self.qualified_name.len() {
            2 => Some(&self.qualified_name[0]),
            1 => None,
            _ => panic!(),
        }
    }

    pub fn table_name(&self) -> &symbols::Name {
        &self.qualified_name.last().unwrap()
    }
}

/// Representation of a describe statememnt
#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct DescribeStatement {
    /// the name of the object to describe
    pub qualified_name: Vec<symbols::Name>,
}

impl DescribeStatement {
    pub fn new(schema: Option<symbols::Name>, name: symbols::Name) -> DescribeStatement {
        let mut qualified_name = Vec::new();

        if schema.is_some() {
            qualified_name.push(schema.unwrap())
        }

        qualified_name.push(name);

        DescribeStatement { qualified_name }
    }

    pub fn schema_name(&self) -> Option<&symbols::Name> {
        match self.qualified_name.len() {
            2 => Some(&self.qualified_name[0]),
            1 => None,
            _ => panic!(),
        }
    }

    pub fn table_name(&self) -> &symbols::Name {
        &self.qualified_name.last().unwrap()
    }
}

/// Assignment used as part of an Update statement. One or more columns are updated with
/// the provided expression value.
#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct Assignment {
    pub columns: Vec<symbols::Name>,
    pub expr: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SelectMode {
    All,
    Distinct,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct ValuesSetExpression {
    pub values: Vec<Vec<Expression>>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct QuerySetExpression {
    pub mode: SelectMode,
    pub columns: ResultColumns,
    pub from: Vec<TableExpression>,
    pub where_expr: Option<Expression>,
    pub group_by: Option<GroupBy>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct OpSetExpression {
    pub op: SetOperator,
    pub left: Box<SetExpression>,
    pub right: Box<SetExpression>,
}

/// Representation of a SetExpression, a collection of rows, each having one or more columns.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SetExpression {
    /// Literal row values
    Values(ValuesSetExpression),

    /// Query result as `SetExpression`
    Query(QuerySetExpression),

    /// Binary operation on two `SetExpression` values
    Op(OpSetExpression),
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct NamedTableExpression {
    /// the qualified table name
    pub name: Vec<symbols::Name>,

    /// an alias to refer to the row set within this expression
    pub alias: Option<symbols::Name>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct SelectTableExpression {
    /// a nested select statement
    pub select: SelectStatement,

    /// an alias to refer to the row set within this expression
    pub alias: Option<symbols::Name>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct JoinTableExpression {
    /// the left table expression to join
    pub left: Box<TableExpression>,

    /// the right table expression to join
    pub right: Box<TableExpression>,

    /// the join operator
    pub op: JoinOperator,

    /// the join constraint, specifying what conditions need to apply for joining two rows
    pub constraint: JoinConstraint,
}

/// Representations of base queries
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TableExpression {
    /// The row set of a given table; possibly providing an alias
    Named(NamedTableExpression),

    /// A nested select statement
    Select(SelectTableExpression),

    /// The Join of two `TableExpression` values
    Join(JoinTableExpression),
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct ColumnsJoinConstraint {
    pub columns: Vec<symbols::Name>,
}

/// Representation of a join constraint
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum JoinConstraint {
    /// an expression describing the contraint
    Expr(Expression),

    /// join constraints provided via column value constraints
    Columns(ColumnsJoinConstraint),
}

/// Join operators
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum JoinOperator {
    /// Regular join
    Join(JoinType),

    /// Natural join
    Natural(JoinType),

    /// Cross join
    Cross,
}

/// Join types
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum JoinType {
    /// Inner join
    Inner,

    /// Left (outer) join
    Left,

    /// Right (outer) join
    Right,

    /// Full (outer) join
    Full,
}

/// Representation of result columns in a select statement
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ResultColumns {
    /// All columns ('*')
    All,

    /// Result column specification
    List(Vec<ResultColumn>),
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct ExprResultColumn {
    /// the expression to evaluate
    pub expr: Expression,

    /// an optional column name in the resulting row set
    pub rename: Option<symbols::Name>,
}

/// Representation of a single result column specification
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ResultColumn {
    /// All columns from a given named schema object
    AllFrom(symbols::Name),

    /// An expression
    Expr(ExprResultColumn),
}

/// Representation of grouping of result sets
#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct GroupBy {
    /// One or more expressions that define the buckets for grouping
    pub groupings: Vec<Expression>,

    /// an optional constraint to limit the groups to collect rows for
    pub having: Option<Expression>,
}

/// Possible binary operators on row sets
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SetOperator {
    /// Intersection operation
    Intersect,

    /// Set minus operation
    Except,

    /// Union of distinct values
    Union,

    /// Union including possible duplicates occuring on both sides
    UnionAll,
}

/// Possible unary operators for simple expressions
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UnaryOperator {
    /// Numeric negation
    Negate,

    /// Logical inversion
    Not,

    /// Null check
    IsNull,
}

/// Binary operators for simple expressions
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BinaryOperator {
    /// Numeric multiplication
    Multiply,

    /// Numeric division
    Divide,

    /// Numeric addition
    Add,

    /// Numeric subtraction
    Subtract,

    /// Concatenation of character sequences
    Concat,

    /// Logical and
    And,

    /// Logical or
    Or,
}

/// Comparison operators
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ComparisonOperator {
    /// Equality
    Equal,

    /// Inquality
    NotEqual,

    /// Less than
    LessThan,

    /// Less than or equal to
    LessEqual,

    /// Greater than
    GreaterThan,

    /// Greater than or equal to
    GreaterEqual,

    /// Like operator (string matching)
    Like,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct QualifiedIdentifierExpression {
    pub identifiers: Vec<symbols::Name>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct MakeTupleExpression {
    pub exprs: Vec<Expression>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct UnaryExpression {
    pub op: UnaryOperator,
    pub expr: Box<Expression>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct BinaryExpression {
    pub op: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct ComparisonExpression {
    pub op: ComparisonOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct InExpression {
    pub expr: Box<Expression>,
    pub set: SetSpecification,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct BetweenExpression {
    pub expr: Box<Expression>,
    pub lower: Box<Expression>,
    pub upper: Box<Expression>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct CaseExpression {
    pub expr: Option<Box<Expression>>,
    pub when_part: Vec<WhenClause>,
    pub else_part: Option<Box<Expression>>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct CoalesceExpression {
    pub exprs: Vec<Expression>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct ReplaceExpression {
    pub string: Box<Expression>,
    pub search_string: Box<Expression>,
    pub replace_string: Option<Box<Expression>>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct SubstringExpression {
    pub string: Box<Expression>,
    pub position: Box<Expression>,
    pub length: Option<Box<Expression>>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct ToDateExpression {
    pub string: Box<Expression>,
    pub format: Option<Box<Expression>>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct PowerExpression {
    pub base: Box<Expression>,
    pub exponent: Box<Expression>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct ConcatExpression {
    pub exprs: Vec<Expression>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct MaxExpression {
    pub mode: SelectMode,
    pub expr: Box<Expression>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct MinExpression {
    pub mode: SelectMode,
    pub expr: Box<Expression>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct SumExpression {
    pub mode: SelectMode,
    pub expr: Box<Expression>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct CastExpression {
    pub expr: Box<Expression>,
    pub data_type: DataType,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct RightExpression {
    pub string: Box<Expression>,
    pub length: Box<Expression>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct CountExpression {
    pub columns: ResultColumns,
    pub mode: SelectMode,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct UnknownExpression {
    pub name: Vec<symbols::Name>,
    pub exprs: Vec<Expression>,
}

/// Scalar expressions
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    /// a literal value
    Literal(Literal),

    /// a qualified name referring to an attribute of a bound relation
    QualifiedIdentifier(QualifiedIdentifierExpression),

    /// tuple construction
    MakeTuple(MakeTupleExpression),

    /// nested select statement
    Select(SelectStatement),

    /// unary operation
    Unary(UnaryExpression),

    /// Binary operation
    Binary(BinaryExpression),

    /// Comparison operation
    Comparison(ComparisonExpression),

    /// Set membership test
    In(InExpression),

    /// Range check
    Between(BetweenExpression),

    /// Case statement
    Case(CaseExpression),

    /// Coalesce function
    Coalesce(CoalesceExpression),

    /// Replace function
    Replace(ReplaceExpression),

    /// Substr[ing] function
    Substring(SubstringExpression),

    /// ToDate function
    ToDate(ToDateExpression),

    /// ToDate function
    Power(PowerExpression),

    /// Concat function
    Concat(ConcatExpression),

    /// Sum function
    Sum(SumExpression),

    /// Max/Min functions
    Max(MaxExpression),
    Min(MinExpression),

    /// Cast function
    Cast(CastExpression),

    /// Right function
    Right(RightExpression),

    /// Count function
    Count(CountExpression),

    /// Unknown Expression
    Unknown(UnknownExpression),
}

/// Specification of the containing set within a set membership expression
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SetSpecification {
    /// Rows returned by a select statement
    Select(SelectStatement),

    /// List of expressions
    List(Vec<Expression>),

    /// a qualified name specifying a collection
    Name(Vec<symbols::Name>),
}

/// Representation of a when clause used inside a case expression
#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct WhenClause {
    /// guard statement determining when this claause applies
    pub guard: Expression,

    /// the guarded expression to evaluate when this clause applies
    pub body: Expression,
}

/// Literal values
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Literal {
    /// String literal
    String(String),

    /// Numeric literal
    Numeric(String),

    /// the NULL value
    Null,

    /// the current time
    CurrentTime,

    /// the current date
    CurrentDate,

    /// the current timestamp
    CurrentTimestamp,

    /// DATE literal
    Date(String),

    /// TIME literal
    Time(String),

    /// TIMESTAMP literal
    Timestamp(String),
}

/// Sort ordering direction
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OrderingDirection {
    /// Sort in ascending order
    Ascending,

    /// Sort in descending order
    Descending,
}

/// Specification of a sort order
#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct Ordering {
    /// an expression evaluating to the sort key
    pub expr: Expression,

    /// an optional collation to use for string comparisons
    pub collation: Option<symbols::Name>,

    /// Sort ordering direction
    pub direction: OrderingDirection,
}

/// Limits for a limit clause
#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct Limit {
    /// number of rows to return
    pub number_rows: Expression,

    /// number of rows to skip
    pub offset_value: Option<Expression>,
}

/// Helper function to append an item to a vector
pub fn append<T>(list: Vec<T>, item: T) -> Vec<T> {
    let mut result = list;
    result.push(item);
    result
}

/// Supported data types
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DataType {
    /// boolean data type
    Boolean,

    /// char
    Char(Literal),

    /// date
    Date,

    /// decimal
    Decimal { p: Literal, s: Literal },

    /// double precision
    DoublePrecision,

    /// timestamp
    Timestamp,

    /// local timestamp
    LocalTimestamp,

    /// varchar
    Varchar(Literal),
}
