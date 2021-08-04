#![feature(box_syntax)]

#[derive(Clone)]
enum Equation {
    Num(i32),
    Add(Box<Equation>, Box<Equation>),
    Div(Box<Equation>, Box<Equation>),
    Sub(Box<Equation>, Box<Equation>),
    Mul(Box<Equation>, Box<Equation>),
}

impl Equation {
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

#[cfg(test)]
mod tests {
    use super::Equation::*;

    #[test]
    fn test_eq() {
        // (2 + 2) / 4
        let eq = Div(box Add(box Num(2), box Num(2)), box Num(4));
        assert_eq!(1, eq.clone().solve());

        let eq2 = Mul(box eq, box Add(box Num(3), box Num(7)));
        assert_eq!(10, eq2.clone().solve());

        let eq3 = Sub(box eq2, box Num(3));
        assert_eq!(7, eq3.solve())
    }
}
