pub trait Expr<T> {
    fn eval(&self) -> T;
}

pub struct Integer(i32);

impl Expr<i32> for Integer {
    fn eval(&self) -> i32 {
        self.0
    }
}

pub struct Add(Box<dyn Expr<i32>>, Box<dyn Expr<i32>>);

impl Expr<i32> for Add {
    fn eval(&self) -> i32 {
        self.0.eval() + self.1.eval()
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Expr;

    use super::{Add, Integer};

    #[test]
    fn should_evaluate_integer() {
        let unit = Integer(10);
        assert_eq!(unit.eval(), 10);
    }

    #[test]
    fn should_evaluate_add() {
        let left: Box<dyn Expr<i32>> = Box::new(Integer(2));
        let right: Box<dyn Expr<i32>> = Box::new(Integer(2));
        let unit = Add(left, right);
        assert_eq!(unit.eval(), 4);
    }

    #[test]
    fn should_evaluate_nested_add() {
        let left: Box<dyn Expr<i32>> = Box::new(Integer(2));
        let right: Box<dyn Expr<i32>> = Box::new(Add(Box::new(Integer(3)), Box::new(Integer(4))));
        let unit: Box<dyn Expr<i32>> = Box::new(Add(left, right));
        assert_eq!(unit.eval(), 9);
    }
}
