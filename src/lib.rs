#![feature(box_syntax)]
#![feature(box_patterns)]

#[macro_use]
extern crate assert_matches;

use crate::EqBuild::Operator;
use crate::Equation::Num;
use crate::ParseError::{EmptyEq, UnexpectedOperator, UnmatchedOperator};

#[derive(Debug, Clone)]
enum Equation {
    Num(i32),
    Add(Box<Equation>, Box<Equation>),
    Div(Box<Equation>, Box<Equation>),
    Sub(Box<Equation>, Box<Equation>),
    Mul(Box<Equation>, Box<Equation>),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum EqOperator {
    Mul,
    Sub,
    Div,
    Add,
}

#[derive(Debug)]
enum EqBuild {
    Equation(Equation),
    Operator(EqOperator),
}

#[derive(Debug)]
enum ParseError {
    UnmatchedOperator,
    UnexpectedOperator,
    EmptyEq,
}

fn parse_type(chars: &mut Vec<EqBuild>, to_match: &[EqOperator], combine: impl Fn(Equation, Equation, EqOperator) -> Equation) -> Result<(), ParseError> {
    let mut i = 1;
    while i < chars.len() - 1 {
        let operator = &chars[i];
        let mut change = false;
        if let &Operator(operator) = operator {
            if to_match.contains(&operator) {
                let rhs = match chars.remove(i + 1) {
                    EqBuild::Equation(eq) => eq,
                    _ => return Err(UnmatchedOperator)
                };

                let lhs = match chars.remove(i - 1) {
                    EqBuild::Equation(eq) => eq,
                    _ => return Err(UnmatchedOperator)
                };

                let res = combine(lhs, rhs, operator);

                chars[i - 1] = EqBuild::Equation(res);
                change = true;
            }
        }

        if !change {
            i += 1;
        }
    }

    Ok(())
}

impl Equation {
    // fn parse_chars(input: &[char]) -> Result<Equation, ParseError> {}

    pub fn first_compile(input: &str) -> Result<Vec<EqBuild>, ParseError> {
        use EqOperator::*;

        let mut res = Vec::new();


        let values = input.chars().filter(|&c| !c.is_whitespace());

        let mut value_to_push = None;

        for c in values {
            if let Some(value) = value_to_push {
                if !('0'..='9').contains(&c) {
                    res.push(EqBuild::Equation(Num(value)));
                    value_to_push = None;
                }
            }

            match c {
                '*' => res.push(EqBuild::Operator(Mul)),
                '+' => res.push(EqBuild::Operator(Add)),
                '-' => res.push(EqBuild::Operator(Sub)),
                '/' => res.push(EqBuild::Operator(Div)),
                '0'..='9' => {
                    let value = c as i32 - '0' as i32;
                    match value_to_push.as_mut() {
                        None => value_to_push = Some(value),
                        Some(v) => *v = *v * 10 + value
                    }
                }
                _ => return Err(UnexpectedOperator)
            }
        }

        if let Some(v) = value_to_push {
            res.push(EqBuild::Equation(Num(v)));
        }

        Ok(res)
    }
    pub fn parse(input: &str) -> Result<Equation, ParseError> {
        let mut chars = Self::first_compile(input)?;

        if chars.is_empty() {
            return Err(EmptyEq);
        }

        // 2 * 2 / 2 * 2

        parse_type(&mut chars, &[EqOperator::Mul, EqOperator::Div], |lhs, rhs, op| {
            match op {
                EqOperator::Mul => Equation::Mul(box lhs, box rhs),
                EqOperator::Div => Equation::Div(box lhs, box rhs),
                _ => panic!("logic error")
            }
        })?;

        parse_type(&mut chars, &[EqOperator::Add, EqOperator::Sub], |lhs, rhs, op| {
            match op {
                EqOperator::Add => Equation::Add(box lhs, box rhs),
                EqOperator::Sub => Equation::Sub(box lhs, box rhs),
                _ => panic!("logic error")
            }
        })?;

        debug_assert_eq!(chars.len(), 1);

        match chars.pop().unwrap() {
            EqBuild::Equation(eq) => Ok(eq),
            _ => panic!("logic error")
        }
    }

    fn solve(self) -> i32 {
        match self {
            Equation::Num(val) => val,
            Equation::Add(a, b) => a.solve() + b.solve(),
            Equation::Div(a, b) => a.solve() / b.solve(),
            Equation::Sub(a, b) => a.solve() - b.solve(),
            Equation::Mul(a, b) => a.solve() * b.solve()
        }
    }
}


/// Helper
struct Cas;

impl Cas {
    fn solve(input: &str) -> Result<i32, ParseError> {
        let equation = Equation::parse(input)?;
        Ok(equation.solve())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cas, Equation};

    use super::Equation::*;

    #[test]
    fn test_parse() {
        assert_matches!(Equation::parse("2+2").unwrap(),Add(box Num(2), box Num(2)));
        assert_matches!(Equation::parse("3*2+2").unwrap(),Add(box Mul(box Num(3), box Num(2)), box Num(2)));
        assert_matches!(Equation::parse("3*2-2").unwrap(),Sub(box Mul(box Num(3), box Num(2)), box Num(2)));
    }

    #[test]
    fn test_cas() {
        assert_eq!(Cas::solve("2+2").unwrap(), 4);
        assert_eq!(Cas::solve("2+2*3").unwrap(), 8);
        assert_eq!(Cas::solve("2 - 2*3 + 5").unwrap(), 1);
        assert_eq!(Cas::solve("8/2/2").unwrap(), 2);
        assert_eq!(Cas::solve("12/4/3").unwrap(), 1);
        assert_eq!(Cas::solve("2+3+5-3-2*5").unwrap(), -3);
    }

    #[test]
    fn test_equation_eq() {
        // (2 + 2) / 4
        let eq = Div(box Add(box Num(2), box Num(2)), box Num(4));
        assert_eq!(1, eq.clone().solve());

        let eq2 = Mul(box eq, box Add(box Num(3), box Num(7)));
        assert_eq!(10, eq2.clone().solve());

        let eq3 = Sub(box eq2, box Num(3));
        assert_eq!(7, eq3.solve())
    }
}
