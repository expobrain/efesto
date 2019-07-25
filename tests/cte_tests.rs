// MIT License
//
// Copyright (c) 2018 Hans-Martin Will
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

#[macro_use]
mod macros;

use efesto::ast::*;
use efesto::parse;
use efesto::symbols;

test_builder!(
    select_minimum_cte,
    "with my_cte as ( select 1 ) select 1",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![CommonTableExpression {
            identifier: symbols::Name::new("my_cte".to_string()),
            column_names: None,
            query: SelectStatement {
                common: vec![],
                expr: Box::new(SetExpression::Query(QuerySetExpression {
                    mode: SelectMode::All,
                    columns: ResultColumns::List(vec![ResultColumn::Expr(ExprResultColumn {
                        expr: Expression::Literal(Literal::Numeric("1".to_string())),
                        rename: None
                    })]),
                    from: vec![],
                    where_expr: None,
                    group_by: None,
                })),
                order_by: vec![],
                limit: None
            },
        }],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::List(vec![ResultColumn::Expr(ExprResultColumn {
                expr: Expression::Literal(Literal::Numeric("1".to_string())),
                rename: None
            })]),
            from: vec![],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);

test_builder!(
    select_from_cte,
    "with my_cte as ( select 1 as b ) select c from my_cte",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![CommonTableExpression {
            identifier: symbols::Name::new("my_cte".to_string()),
            column_names: None,
            query: SelectStatement {
                common: vec![],
                expr: Box::new(SetExpression::Query(QuerySetExpression {
                    mode: SelectMode::All,
                    columns: ResultColumns::List(vec![ResultColumn::Expr(ExprResultColumn {
                        expr: Expression::Literal(Literal::Numeric("1".to_string())),
                        rename: Some(symbols::Name::new("b".to_string()))
                    })]),
                    from: vec![],
                    where_expr: None,
                    group_by: None,
                })),
                order_by: vec![],
                limit: None
            },
        }],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::List(vec![ResultColumn::Expr(ExprResultColumn {
                expr: Expression::QualifiedIdentifier(QualifiedIdentifierExpression {
                    identifiers: vec![symbols::Name::new("c".to_string())]
                }),
                rename: None
            })]),
            from: vec![TableExpression::Named(NamedTableExpression {
                name: vec![symbols::Name::new("my_cte".to_string())],
                alias: None
            })],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);
