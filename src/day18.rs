use std::iter::Iterator;
use std::io::{BufReader, BufRead};
use std::fs::File;

use nom::{
    IResult,
    character::complete::{digit1, char, one_of, space0},
    combinator::{map},
    sequence::{delimited},
    branch::{alt},
};

#[derive(Clone, Debug)]
enum Node {
    Val(i64),
    Op(char, Box<Node>, Box<Node>),
    Parens(Box<Node>),
}

fn parse_num2(exp: &str, prec_fn: fn(char) -> i64) -> IResult<&str, Node> {
    alt((
        map(digit1, |d: &str| Node::Val(d.parse().unwrap())),
        delimited(char('('), map(|s| parse_expr(s, prec_fn), |n| Node::Parens(Box::new(n))), char(')')),
    ))(exp)
}

fn make_op(op: char, a: Node, b: Node) -> Node{
    Node::Op(op, Box::new(a), Box::new(b))
}

fn parse_step(exp: &str, left_val: Node, prec_fn: fn(char) -> i64) -> IResult<&str, Node> {
    let (exp, op) = delimited(space0, one_of("+*"), space0)(exp)?;
    let (exp, right_val) = parse_num2(exp, prec_fn)?;

    let node = match left_val {
        Node::Op(left_op, left_val, middle_val) if prec_fn(left_op) >= prec_fn(op) => {
            make_op(op, make_op(left_op, *left_val, *middle_val), right_val)
        },
        Node::Op(left_op, left_val, middle_val) => {
            make_op(left_op, *left_val, make_op(op, *middle_val, right_val))
        },
        left_val@Node::Val(_) => {
            make_op(op, left_val, right_val)
        }
        Node::Parens(left_val) => {
            make_op(op, *left_val, right_val)
        }
    };
    Ok((exp, node))
}

fn parse_expr(exp: &str, prec_fn: fn(char) -> i64) -> IResult<&str, Node> {
    let mut state = parse_num2(exp, prec_fn)?;
    loop {
        if let Ok(new_state) = parse_step(state.0, state.1.clone(), prec_fn) {
            state = new_state;
        } else {
            return Ok(state);
        }
    }
}

fn eval(node: &Node) -> i64 {
    match node {
        Node::Val(v) => *v,
        Node::Parens(n) => eval(n),
        Node::Op(op, a, b) if *op == '*' => eval(a) * eval(b),
        Node::Op(op, a, b) if *op == '+' => eval(a) + eval(b),
        Node::Op(op, _a, _b) => panic!("Unrecognized operator {}", op)
    }
}

pub fn problem18a(demo: bool) {
    let path = if demo { "inputs/input18demo.txt" } else { "inputs/input18.txt" };
    let prec_fn = |_| 0;
    let file = File::open(path).unwrap();
    let res: i64 = BufReader::new(file).lines().map(|l|
        eval(&parse_expr(l.unwrap().as_str(), prec_fn).unwrap().1)
    ).sum();
    println!("{}", res);
}

pub fn problem18b(demo: bool) {
    let path = if demo { "inputs/input18demo.txt" } else { "inputs/input18.txt" };
    let prec_fn = |op| if op == '+' { 1 } else { 0 };
    let file = File::open(path).unwrap();
    let res: i64 = BufReader::new(file).lines().map(|l|
        eval(&parse_expr(l.unwrap().as_str(), prec_fn).unwrap().1)
    ).sum();
    println!("{}", res);
}