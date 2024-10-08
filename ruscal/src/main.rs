use std::{collections::HashMap, io::Read};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace0, multispace1},
    combinator::{opt, recognize},
    error::ParseError,
    multi::{fold_many0, many0},
    number::complete::recognize_float,
    sequence::{delimited, pair, preceded, terminated},
    Finish, IResult, Parser,
};

fn main() {
    let mut buf = String::new();
    if !std::io::stdin().read_to_string(&mut buf).is_ok() {
        panic!("Failed to read from stdin");
    }
    let parsed_statements = match statements_finish(&buf) {
        Ok(parsed_statements) => parsed_statements,
        Err(e) => {
            eprintln!("Parse error: {e:?}");
            return;
        }
    };

    let mut variables = HashMap::new();

    eval_stmts(&parsed_statements, &mut variables);
}

type Variables = HashMap<String, f64>;

fn eval_stmts(stmts: &[Statement], variables: &mut Variables) {
    for statement in stmts {
        match statement {
            Statement::Expression(expr) => {
                println!("eval: {:?}", eval(expr, variables))
            }
            Statement::VarDef(name, expr) => {
                let value = eval(expr, variables);
                variables.insert(name.to_string(), value);
            }
            Statement::VarAssign(name, expr) => {
                if !variables.contains_key(*name) {
                    panic!("Variable is not defined");
                }
                let value = eval(expr, variables);
                variables.insert(name.to_string(), value);
            }
            Statement::For {
                loop_var,
                start,
                end,
                stmts,
            } => {
                let start = eval(start, variables) as isize;
                let end = eval(end, variables) as isize;
                for i in start..end {
                    variables.insert(loop_var.to_string(), i as f64);
                    eval_stmts(stmts, variables);
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Expression<'src> {
    Ident(&'src str),
    NumLiteral(f64),
    FnInvoke(&'src str, Vec<Expression<'src>>),
    Add(Box<Expression<'src>>, Box<Expression<'src>>),
    Sub(Box<Expression<'src>>, Box<Expression<'src>>),
    Mul(Box<Expression<'src>>, Box<Expression<'src>>),
    Div(Box<Expression<'src>>, Box<Expression<'src>>),
    If(
        Box<Expression<'src>>,
        Box<Expression<'src>>,
        Option<Box<Expression<'src>>>,
    ),
}

#[derive(Debug, PartialEq, Clone)]
enum Statement<'src> {
    Expression(Expression<'src>),
    VarDef(&'src str, Expression<'src>),
    VarAssign(&'src str, Expression<'src>),
    For {
        loop_var: &'src str,
        start: Expression<'src>,
        end: Expression<'src>,
        stmts: Statements<'src>,
    },
}

type Statements<'a> = Vec<Statement<'a>>;

fn unary_fn(f: fn(f64) -> f64) -> impl Fn(&[Expression], &Variables) -> f64 {
    move |args, variables| {
        f(eval(
            args.into_iter().next().expect("function missing argument"),
            variables,
        ))
    }
}

fn binary_fn(f: fn(f64, f64) -> f64) -> impl Fn(&[Expression], &Variables) -> f64 {
    move |args, variables| {
        let mut args = args.into_iter();
        let lhs = eval(
            args.next().expect("function missing the first argument"),
            variables,
        );
        let rhs = eval(
            args.next().expect("function missing the second argument"),
            variables,
        );
        f(lhs, rhs)
    }
}

fn eval(expr: &Expression, vars: &Variables) -> f64 {
    use Expression::*;
    match expr {
        Ident("pi") => std::f64::consts::PI,
        Ident(id) => *vars.get(*id).expect("Variable not found"),
        NumLiteral(n) => *n,
        FnInvoke("sqrt", args) => unary_fn(f64::sqrt)(args, vars),
        FnInvoke("sin", args) => unary_fn(f64::sin)(args, vars),
        FnInvoke("cos", args) => unary_fn(f64::cos)(args, vars),
        FnInvoke("tan", args) => unary_fn(f64::tan)(args, vars),
        FnInvoke("asin", args) => unary_fn(f64::asin)(args, vars),
        FnInvoke("acos", args) => unary_fn(f64::acos)(args, vars),
        FnInvoke("atan", args) => unary_fn(f64::atan)(args, vars),
        FnInvoke("atan2", args) => binary_fn(f64::atan2)(args, vars),
        FnInvoke("pow", args) => binary_fn(f64::powf)(args, vars),
        FnInvoke("exp", args) => unary_fn(f64::exp)(args, vars),
        FnInvoke("log", args) => binary_fn(f64::log)(args, vars),
        FnInvoke("log10", args) => unary_fn(f64::log10)(args, vars),
        FnInvoke(name, _) => {
            panic!("Unknown function {name:?}")
        }
        Add(lhs, rhs) => eval(lhs, vars) + eval(rhs, vars),
        Sub(lhs, rhs) => eval(lhs, vars) - eval(rhs, vars),
        Mul(lhs, rhs) => eval(lhs, vars) * eval(rhs, vars),
        Div(lhs, rhs) => eval(lhs, vars) / eval(rhs, vars),
        If(cond, t_case, f_case) => {
            if eval(cond, vars) != 0. {
                eval(t_case, vars)
            } else if let Some(f_case) = f_case {
                eval(f_case, vars)
            } else {
                0.
            }
        }
    }
}

fn space_delimited<'src, O, E>(
    f: impl Parser<&'src str, O, E>,
) -> impl FnMut(&'src str) -> IResult<&'src str, O, E>
where
    E: ParseError<&'src str>,
{
    delimited(multispace0, f, multispace0)
}

fn factor(i: &str) -> IResult<&str, Expression> {
    alt((number, func_call, ident, parens))(i)
}

fn func_call(i: &str) -> IResult<&str, Expression> {
    let (r, ident) = space_delimited(identifier)(i)?;
    let (r, args) = space_delimited(delimited(
        tag("("),
        many0(delimited(multispace0, expr, space_delimited(opt(tag(","))))),
        tag(")"),
    ))(r)?;
    Ok((r, Expression::FnInvoke(ident, args)))
}

fn ident(input: &str) -> IResult<&str, Expression> {
    let (r, res) = space_delimited(identifier)(input)?;
    Ok((r, Expression::Ident(res)))
}

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
}

fn number(input: &str) -> IResult<&str, Expression> {
    let (r, v) = space_delimited(recognize_float)(input)?;
    Ok((
        r,
        Expression::NumLiteral(v.parse().map_err(|_| {
            nom::Err::Error(nom::error::Error {
                input,
                code: nom::error::ErrorKind::Digit,
            })
        })?),
    ))
}

fn parens(i: &str) -> IResult<&str, Expression> {
    space_delimited(delimited(tag("("), expr, tag(")")))(i)
}

fn term(i: &str) -> IResult<&str, Expression> {
    let (i, init) = factor(i)?;

    fold_many0(
        pair(space_delimited(alt((char('*'), char('/')))), factor),
        move || init.clone(),
        |acc, (op, val): (char, Expression)| match op {
            '*' => Expression::Mul(Box::new(acc), Box::new(val)),
            '/' => Expression::Div(Box::new(acc), Box::new(val)),
            _ => panic!("Multiplicative expression should have '*' or '/' operator"),
        },
    )(i)
}

fn num_expr(i: &str) -> IResult<&str, Expression> {
    let (i, init) = term(i)?;

    fold_many0(
        pair(space_delimited(alt((char('+'), char('-')))), term),
        move || init.clone(),
        |acc, (op, val): (char, Expression)| match op {
            '+' => Expression::Add(Box::new(acc), Box::new(val)),
            '-' => Expression::Sub(Box::new(acc), Box::new(val)),
            _ => {
                panic!("Additive expression should have '+' or '-' operator")
            }
        },
    )(i)
}

fn open_brace(i: &str) -> IResult<&str, ()> {
    let (i, _) = space_delimited(char('{'))(i)?;
    Ok((i, ()))
}

fn close_brace(i: &str) -> IResult<&str, ()> {
    let (i, _) = space_delimited(char('}'))(i)?;
    Ok((i, ()))
}

fn if_expr(i: &str) -> IResult<&str, Expression> {
    let (i, _) = space_delimited(tag("if"))(i)?;
    let (i, cond) = expr(i)?;
    let (i, t_case) = delimited(open_brace, expr, close_brace)(i)?;
    let (i, f_case) = opt(preceded(
        space_delimited(tag("else")),
        delimited(open_brace, expr, close_brace),
    ))(i)?;

    Ok((
        i,
        Expression::If(Box::new(cond), Box::new(t_case), f_case.map(Box::new)),
    ))
}

fn expr(i: &str) -> IResult<&str, Expression> {
    alt((if_expr, num_expr))(i)
}

fn var_def(i: &str) -> IResult<&str, Statement> {
    let (i, _) = delimited(multispace0, tag("var"), multispace1)(i)?;
    let (i, name) = space_delimited(identifier)(i)?;
    let (i, _) = space_delimited(char('='))(i)?;
    let (i, expr) = space_delimited(expr)(i)?;
    Ok((i, Statement::VarDef(name, expr)))
}

fn var_assign(i: &str) -> IResult<&str, Statement> {
    let (i, name) = space_delimited(identifier)(i)?;
    let (i, _) = space_delimited(char('='))(i)?;
    let (i, expr) = space_delimited(expr)(i)?;
    Ok((i, Statement::VarAssign(name, expr)))
}

fn expr_statement(i: &str) -> IResult<&str, Statement> {
    let (i, res) = expr(i)?;
    Ok((i, Statement::Expression(res)))
}

fn for_statement(i: &str) -> IResult<&str, Statement> {
    let (i, _) = space_delimited(tag("for"))(i)?;
    let (i, loop_var) = space_delimited(identifier)(i)?;
    let (i, _) = space_delimited(tag("in"))(i)?;
    let (i, start) = space_delimited(expr)(i)?;
    let (i, _) = space_delimited(tag("to"))(i)?;
    let (i, end) = space_delimited(expr)(i)?;
    let (i, stmts) = delimited(open_brace, statements, close_brace)(i)?;
    Ok((
        i,
        Statement::For {
            loop_var,
            start,
            end,
            stmts,
        },
    ))
}

fn statement(i: &str) -> IResult<&str, Statement> {
    alt((
        for_statement,
        terminated(alt((var_def, var_assign, expr_statement)), char(';')),
    ))(i)
}

fn statements(i: &str) -> IResult<&str, Statements> {
    let (i, stmts) = many0(statement)(i)?;
    let (i, _) = opt(char(';'))(i)?;
    Ok((i, stmts))
}

fn statements_finish(i: &str) -> Result<Statements, nom::error::Error<&str>> {
    let (_, res) = statements(i).finish()?;
    Ok(res)
}
