use std::{fmt::Error, str::FromStr};

use super::number::Number;

pub struct FuHc {
    level: usize,
    operator: Operator,
}
impl FuHc {
    ///构造
    pub fn new_from_string(c: &String) -> Result<Self, &'static str> {
        let a = c.as_bytes()[0] as char;

        let (level, operator) = match a {
            '+' => (1, Operator::Add),
            '-' => (1, Operator::Sub),
            '/' => (2, Operator::Div),
            '\\' => (2, Operator::Div),
            '*' => (2, Operator::Mul),
            '^' => (3, Operator::Pow),
            _ => (0, Operator::Nil),
        };
        if level != 0 {
            return Ok(FuHc { level, operator });
        }
        let s = c.as_str();
        let (level, operator) = match s {
            "log" => (3, Operator::Log),

            _ => (0, Operator::Nil),
        };
        if level != 0 {
            return Ok(FuHc { level, operator });
        }
        Err("There are unimplemented operators in the expression")
    }
}
impl FuHc {
    pub fn is_eq_level(&self, other: &Self) -> bool {
        self.level == self.level
    }

    pub fn is_high_level(&self, other: &Self) -> bool {
        self.level > other.level
    }
    pub fn is_low_level(&self, other: &Self) -> bool {
        self.level < other.level
    }
}
impl FuHc {
    ///双目运算
    /// pre_fu_hc:运算符
    /// prev_num:运算数1
    /// next_num:运算符2
    /// return:运算结果:Result<Number,str>
    pub fn two_calculate(
        pre_fu_hc: Option<FuHc>,
        prev_num: Option<Number>,
        next_num: Option<Number>,
    ) -> Result<Number, &'static str> {
        let fu_hc;
        let (num1, num2);
        let res = Err("表达式错误");
        match pre_fu_hc {
            Some(s) => fu_hc = s,
            None => return res,
        };
        match prev_num {
            Some(s) => num1 = s,
            None => return res,
        };
        match next_num {
            Some(s) => num2 = s,
            None => return res,
        };
        Ok(fu_hc.compute(num1, num2)?)
    }
    fn compute(&self, num1: Number, num2: Number) -> Result<Number, &'static str> {
        match self.operator {
            Operator::Add => Ok(num1.add(num2)),
            Operator::Sub => {
                if (num2.is_zero()) {
                    Err("The divisor cannot be zero in division")
                } else {
                    Ok(num1.sub(num2))
                }
            }
            Operator::Div => Ok(num1.div(num2)),
            Operator::Mul => Ok(num1.mul(num2)),
            Operator::Nil => Err("The operator is not implemented"),
            Operator::Pow => Ok(num1.pow(num2)),
            Operator::Log => Ok(num1.log(num2)),
        }
    }
}
enum Operator {
    Add,
    Sub,
    Div,
    Mul,
    Nil,
    Pow,
    Log,
}
