pub(crate) struct Number {
    data: Data,
}
impl Clone for Data {
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }

    fn clone(&self) -> Self {
        match self {
            Self::F(arg0) => Self::F(arg0.clone()),
            Self::I(arg0) => Self::I(arg0.clone()),
        }
    }
}
impl Number {
    pub fn new_from_string(s: &String) -> Result<Self, &'static str> {
        let mut res;
        if s.as_bytes()[0] == 'e' as u8 {
            return Ok(Number {
                data: Data::F(std::f64::consts::E),
            });
        };
        if s.contains(".") {
            let data = s.parse::<f64>();
            match data {
                Ok(o) => res = Number { data: Data::F(o) },
                Err(_) => return Err("error in parse number to f64"),
            }
        } else {
            let data = s.parse::<i64>();
            match data {
                Ok(o) => res = Number { data: Data::I(o) },
                Err(_) => return Err("error in parse number to i64"),
            }
        }
        Ok(res)
    }
    pub fn get_num(&self) -> Data {
        self.data.clone()
    }
}
///各种运算的实现
impl Number {
    pub fn add(&self, other: Self) -> Self {
        let n1 = match self.data {
            Data::F(f) => f,
            Data::I(i) => i as f64,
        };
        let n2 = match other.data {
            Data::F(f) => f,
            Data::I(i) => i as f64,
        };
        if (n1 + n2).fract().abs() < 1e-9 {
            Number {
                data: Data::I((n1 + n2) as i64),
            }
        } else {
            Number {
                data: Data::F((n1 + n2) as f64),
            }
        }
    }

    pub fn sub(&self, other: Self) -> Self {
        let n1 = match self.data {
            Data::F(f) => f,
            Data::I(i) => i as f64,
        };
        let n2 = match other.data {
            Data::F(f) => f,
            Data::I(i) => i as f64,
        };
        if (n1 - n2).fract().abs() < 1e-9 {
            Number {
                data: Data::I((n1 - n2) as i64),
            }
        } else {
            Number {
                data: Data::F((n1 - n2) as f64),
            }
        }
    }
    pub fn div(&self, other: Self) -> Self {
        let n1 = match self.data {
            Data::F(f) => f,
            Data::I(i) => i as f64,
        };
        let n2 = match other.data {
            Data::F(f) => f,
            Data::I(i) => i as f64,
        };
        if (n1 / n2).fract().abs() < 1e-9 {
            Number {
                data: Data::I((n1 / n2) as i64),
            }
        } else {
            Number {
                data: Data::F((n1 / n2) as f64),
            }
        }
    }
    pub fn mul(&self, other: Self) -> Self {
        let n1 = match self.data {
            Data::F(f) => f,
            Data::I(i) => i as f64,
        };
        let n2 = match other.data {
            Data::F(f) => f,
            Data::I(i) => i as f64,
        };
        if (n1 * n2).fract().abs() < 1e-9 {
            Number {
                data: Data::I((n1 * n2) as i64),
            }
        } else {
            Number {
                data: Data::F((n1 * n2) as f64),
            }
        }
    }
    pub fn is_zero(&self) -> bool {
        match self.data {
            Data::F(f) => f.abs() < 1e-9,
            Data::I(i) => i == 0,
        }
    }

    pub fn pow(&self, other: Self) -> Self {
        let n1 = match self.data {
            Data::F(f) => f,
            Data::I(i) => i as f64,
        };
        let n2 = match other.data {
            Data::F(f) => f,
            Data::I(i) => i as f64,
        };
        if (n1.powf(n2)).fract().abs() < 1e-9 {
            Number {
                data: Data::I(n1.powf(n2) as i64),
            }
        } else {
            Number {
                data: Data::F(n1.powf(n2) as f64),
            }
        }
    }
    pub fn log(&self, other: Self) -> Self {
        let n1 = match self.data {
            Data::F(f) => f,
            Data::I(i) => i as f64,
        };
        let n2 = match other.data {
            Data::F(f) => f,
            Data::I(i) => i as f64,
        };
        if (n2.log(n1)).fract().abs() < 1e-9 {
            Number {
                data: Data::I(n2.log(n1) as i64),
            }
        } else {
            Number {
                data: Data::F(n2.log(n1) as f64),
            }
        }
    }
}
pub enum Data {
    F(f64),
    I(i64),
}
