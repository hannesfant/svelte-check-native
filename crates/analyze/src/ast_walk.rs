//! Shared AST-walking helpers consumed by multiple analyze + emit passes.
//!
//! Per `notes/PARITY_TESTING_PLAN.md` P3, hand-rolled `match Statement::*`
//! recursion duplicates a generated visitor we don't have. The first piece
//! to pull out of the duplication is `collect_function_body_stmts`, which
//! had byte-identical copies in `analyze::props` and
//! `emit::dispatcher_typing_rewrite` — every R14/R15 review round had to
//! land its descent fix in two places. Keeping the recursion in one
//! module means a new oxc enum variant only adds one match arm, not seven.

use oxc_ast::ast::{Expression, Statement};

/// Walk an expression looking for nested function/arrow bodies — including
/// those passed as call arguments (`setTimeout(() => { … })`) and reachable
/// through every other expression form (object-literal values, array
/// elements, ternary branches, sequence expressions, TS-cast wrappers,
/// etc.). All recovered statements get flattened into `out` so each
/// dispatcher walker can iterate uniformly. Mirrors upstream's
/// `ts.forEachChild` descent pattern.
///
/// Stops at `ClassExpression` / `JSXElement` / `JSXFragment` — they have
/// their own scoping rules that the dispatcher walkers don't currently
/// model.
pub fn collect_function_body_stmts<'a, 'b>(
    expr: &'a Expression<'b>,
    out: &mut Vec<&'a Statement<'b>>,
) {
    match expr {
        Expression::ArrowFunctionExpression(arrow) => {
            for s in &arrow.body.statements {
                out.push(s);
            }
        }
        Expression::FunctionExpression(fe) => {
            if let Some(body) = &fe.body {
                for s in &body.statements {
                    out.push(s);
                }
            }
        }
        Expression::ClassExpression(_) => {}
        Expression::JSXElement(_) | Expression::JSXFragment(_) => {}
        Expression::ParenthesizedExpression(p) => {
            collect_function_body_stmts(&p.expression, out);
        }
        Expression::CallExpression(call) => {
            collect_function_body_stmts(&call.callee, out);
            for arg in &call.arguments {
                if let Some(arg_expr) = arg.as_expression() {
                    collect_function_body_stmts(arg_expr, out);
                }
            }
        }
        Expression::NewExpression(call) => {
            collect_function_body_stmts(&call.callee, out);
            for arg in &call.arguments {
                if let Some(arg_expr) = arg.as_expression() {
                    collect_function_body_stmts(arg_expr, out);
                }
            }
        }
        Expression::ConditionalExpression(c) => {
            collect_function_body_stmts(&c.test, out);
            collect_function_body_stmts(&c.consequent, out);
            collect_function_body_stmts(&c.alternate, out);
        }
        Expression::LogicalExpression(b) => {
            collect_function_body_stmts(&b.left, out);
            collect_function_body_stmts(&b.right, out);
        }
        Expression::BinaryExpression(b) => {
            collect_function_body_stmts(&b.left, out);
            collect_function_body_stmts(&b.right, out);
        }
        Expression::UnaryExpression(u) => {
            collect_function_body_stmts(&u.argument, out);
        }
        Expression::AwaitExpression(a) => {
            collect_function_body_stmts(&a.argument, out);
        }
        Expression::YieldExpression(y) => {
            if let Some(arg) = &y.argument {
                collect_function_body_stmts(arg, out);
            }
        }
        Expression::SequenceExpression(s) => {
            for e in &s.expressions {
                collect_function_body_stmts(e, out);
            }
        }
        Expression::AssignmentExpression(a) => {
            collect_function_body_stmts(&a.right, out);
        }
        Expression::ObjectExpression(o) => {
            for prop in &o.properties {
                match prop {
                    oxc_ast::ast::ObjectPropertyKind::ObjectProperty(p) => {
                        if p.computed
                            && let Some(key_expr) = p.key.as_expression()
                        {
                            collect_function_body_stmts(key_expr, out);
                        }
                        collect_function_body_stmts(&p.value, out);
                    }
                    oxc_ast::ast::ObjectPropertyKind::SpreadProperty(s) => {
                        collect_function_body_stmts(&s.argument, out);
                    }
                }
            }
        }
        Expression::ArrayExpression(a) => {
            for elem in &a.elements {
                if let Some(e) = elem.as_expression() {
                    collect_function_body_stmts(e, out);
                } else if let oxc_ast::ast::ArrayExpressionElement::SpreadElement(s) = elem {
                    collect_function_body_stmts(&s.argument, out);
                }
            }
        }
        Expression::TemplateLiteral(t) => {
            for e in &t.expressions {
                collect_function_body_stmts(e, out);
            }
        }
        Expression::TaggedTemplateExpression(t) => {
            collect_function_body_stmts(&t.tag, out);
            for e in &t.quasi.expressions {
                collect_function_body_stmts(e, out);
            }
        }
        Expression::StaticMemberExpression(me) => {
            collect_function_body_stmts(&me.object, out);
        }
        Expression::ComputedMemberExpression(me) => {
            collect_function_body_stmts(&me.object, out);
            collect_function_body_stmts(&me.expression, out);
        }
        Expression::PrivateFieldExpression(me) => {
            collect_function_body_stmts(&me.object, out);
        }
        Expression::ImportExpression(i) => {
            collect_function_body_stmts(&i.source, out);
        }
        Expression::TSAsExpression(t) => {
            collect_function_body_stmts(&t.expression, out);
        }
        Expression::TSSatisfiesExpression(t) => {
            collect_function_body_stmts(&t.expression, out);
        }
        Expression::TSTypeAssertion(t) => {
            collect_function_body_stmts(&t.expression, out);
        }
        Expression::TSNonNullExpression(t) => {
            collect_function_body_stmts(&t.expression, out);
        }
        Expression::TSInstantiationExpression(t) => {
            collect_function_body_stmts(&t.expression, out);
        }
        // Literals, identifiers, this/super/meta, update/private-in
        // — no nested function bodies to discover.
        _ => {}
    }
}
