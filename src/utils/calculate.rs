use std::cell::RefCell;
use std::f32::consts::E;
use std::process::id;

use super::fu_hc::{self, FuHc};
use super::number::Number;

pub struct Calculate {
    input_sentence: String,
    words: Vec<String>,
}
///初始化函数的实现
impl Calculate {
    pub fn new_from_string(s: String) -> Self {
        let mut s = s.clone().into_bytes();
        s.retain(|&x| -> bool {
            if x == '\t' as u8 || x == '\n' as u8 || x == '\r' as u8 {
                return false;
            }
            true
        });
        let s = String::from_utf8(s).unwrap();
        Calculate {
            input_sentence: s,
            words: Vec::<String>::new(),
        }
    }
    pub fn start(mut self) -> Result<Number, &'static str> {
        match self.parse() {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        match self.calculate(0) {
            Ok(o) => Ok(o.0),
            Err(e) => Err(e),
        }
    }
}
static ADD: u8 = '+' as u8;
static SUB: u8 = '-' as u8;
static MUL: u8 = '*' as u8;
static DIV: u8 = '/' as u8;
static ZERO: u8 = '0' as u8;
static NINE: u8 = '9' as u8;
static DOT: u8 = '.' as u8;
static N_LOG: u8 = 'e' as u8;
///私有函数的实现
impl Calculate {
    /**
     * 将sentence解析为一个个单独的word
     */
    fn parse(&mut self) -> Result<(), &'static str> {
        let mut temp = String::with_capacity(10);
        //0:当前字符串内放置数字
        //1:当前字符串内放置符号
        //2:等待初始化
        let mut mark = 2;
        let mut left_count = 0;
        let mut right_count = 0;
        for (idx, &val) in self.input_sentence.as_bytes().iter().enumerate() {
            //如果是空格
            if val == ' ' as u8 {
                if !temp.is_empty() {
                    self.words.push(temp.clone());
                    temp.clear();
                }
                mark = 2;
                continue;
            }

            //如果是数字(包括e)
            if Self::is_number(val) {
                //如果出现e
                if val == 'e' as u8 {
                    if !temp.is_empty() {
                        self.words.push(temp.clone());
                    }
                    self.words.push("e".to_string());
                    mark = 2;
                    temp.clear();
                    continue;
                }
                if mark == 2 {
                    mark = 0;
                    temp.push(val as char);
                } else if mark == 0 {
                    temp.push(val as char);
                } else {
                    self.words.push(temp.clone());
                    temp.clear();
                    temp.push(val as char);
                    mark = 0;
                }
            } else if val == '(' as u8 || val == ')' as u8 {
                if !temp.is_empty() {
                    self.words.push(temp.clone());
                }
                temp.clear();
                mark = 2;
                if val == '(' as u8 {
                    left_count += 1;
                    self.words.push("(".to_string());
                } else {
                    right_count += 1;
                    self.words.push(")".to_string());
                }
            } else {
                if mark == 2 {
                    mark = 1;
                    temp.push(val as char);
                } else if mark == 1 {
                    temp.push(val as char);
                } else {
                    mark = 1;
                    self.words.push(temp.clone());
                    temp.clear();
                    temp.push(val as char);
                }
            }
        }
        if !temp.is_empty() {
            self.words.push(temp.clone());
        }
        if right_count == left_count {
            Ok(())
        } else {
            Err("表达式括号不匹配")
        }
    }

    ///  It returns `true` if the byte is between the ASCII values of '0' and '9' or  '.'.
    ///
    ///  Otherwise, it returns `false`.
    fn is_number(a: u8) -> bool {
        if (a <= NINE && a >= ZERO) || a == DOT || a == N_LOG {
            return true;
        }
        false
    }
    ///开始解析token，当出现无法解析的数字或符号，返回错误
    ///
    /// 解析正常，则返回Ok(())

    fn calculate(&self, mut idx: usize) -> Result<(Number, usize), &'static str> {
        let n = self.words.len();
        let mut this_num_stk = Vec::with_capacity(n);
        let mut this_fh_hc_stk = Vec::with_capacity(n);

        while idx < n {
            let val: &String = self.words.get(idx).unwrap();
            idx += 1;

            //如果是数字(包括e)
            if Self::is_number(val.as_bytes()[0].clone()) {
                let num_res = Number::new_from_string(&val);
                this_num_stk.push(num_res?);
                continue;
            }
            if val == "(" {
                let res = self.calculate(idx)?;
                idx = res.1;
                this_num_stk.push(res.0);
                continue;
            }
            if val == ")" {
                return Ok((Self::clear(this_num_stk, this_fh_hc_stk)?, idx));
            }

            let now_fu_hc = FuHc::new_from_string(&val)?;

            if Self::can_push(&now_fu_hc, &this_fh_hc_stk) {
                this_fh_hc_stk.push(now_fu_hc);
            } else {
                idx -= 1;
                let pre_fu_hc = this_fh_hc_stk.pop();
                let next_num = this_num_stk.pop();
                let prev_num = this_num_stk.pop();
                let cla_res = FuHc::two_calculate(pre_fu_hc, prev_num, next_num);
                this_num_stk.push(cla_res?);
            }
        }
        Ok((Self::clear(this_num_stk, this_fh_hc_stk)?, idx))
    }
    ///bool:当前符号是否可以推入符号栈
    fn can_push(now_fu_hc: &FuHc, fu_hc_stk: &Vec<FuHc>) -> bool {
        let pre_fu_hc = fu_hc_stk.last();
        match pre_fu_hc {
            Some(s) => now_fu_hc.is_high_level(s),
            None => true,
        }
    }

    ///calculate函数解析完token后，开始清除栈
    /// return Number
    fn clear(mut num_stk: Vec<Number>, mut fh_hc_stk: Vec<FuHc>) -> Result<Number, &'static str> {
        while (!fh_hc_stk.is_empty()) {
            let pre_fu_hc = fh_hc_stk.pop();
            let next_num = num_stk.pop();
            let prev_num = num_stk.pop();

            let res = FuHc::two_calculate(pre_fu_hc, prev_num, next_num)?;
            num_stk.push(res);
        }
        Ok(num_stk.pop().unwrap())
    }
}
