use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Not;
use std::ops::Sub;
use std::str::FromStr;

#[derive(Debug,Clone)]
pub enum RBasicValue {
    String(String),
    Number(i32),
    Bool(bool),
}

// -----------------------------------------------
// Implementations of unary operators
impl Neg for RBasicValue {
    type Output = Result<RBasicValue, String>;

    fn neg(self) -> Self::Output {
        match self {
            RBasicValue::Number(ref number) => Ok(RBasicValue::Number(-*number)),
            _ => Err("Cannot negate non-numeric values!".to_string()),
        }
    }
}

impl Not for RBasicValue {
    type Output = Result<RBasicValue, String>;

    fn not(self) -> Self::Output {
        match self {
            RBasicValue::Bool(ref boolean) => Ok(RBasicValue::Bool(!boolean)),
            _ => Err("Cannot apply unary not to non-Boolean values.".to_string()),
        }
    }
}

// -----------------------------------------------
// Implementations of binary operators
impl Add for RBasicValue {
    type Output = Result<RBasicValue, String>;

    fn add(self, other: RBasicValue) -> Self::Output {
        match (self, other) {
            (RBasicValue::Number(number1), RBasicValue::Number(number2)) => {
                Ok(RBasicValue::Number(number1 + number2))
            }
            (RBasicValue::String(string1), RBasicValue::String(string2)) => {
                Ok(RBasicValue::String(format!("{}{}", string1, string2)))
            }
            (RBasicValue::Number(number1), RBasicValue::String(string2)) => {
                let number2 = i32::from_str(string2.as_str());

                if number2.is_ok() {
                    Ok(RBasicValue::Number(number1 + number2.unwrap()))
                } else {
                    Err(format!("Cannot add integer {} and string {}", number1, string2))
                }
            }
            (RBasicValue::String(string1), RBasicValue::Number(number2)) => {
                let number1 = i32::from_str(string1.as_str());

                if number1.is_ok() {
                    Ok(RBasicValue::Number(number1.unwrap() + number2))
                } else {
                    Err(format!("Cannot add string {} and integer {}", string1, number2))
                }
            }
            _ => Err("Can only add integers or concatenate strings.".to_string()),
        }
    }
}

impl Div for RBasicValue {
    type Output = Result<RBasicValue, String>;

    fn div(self, other: RBasicValue) -> Self::Output {
        match (self, other) {
            (RBasicValue::Number(number1), RBasicValue::Number(number2)) => {
                Ok(RBasicValue::Number(number1 / number2))
            }
            (RBasicValue::Number(number1), RBasicValue::String(string2)) => {
                let number2 = i32::from_str(string2.as_str());

                if number2.is_ok() {
                    Ok(RBasicValue::Number(number1 / number2.unwrap()))
                } else {
                    Err(format!("Cannot divide integer {} and string {}", number1, string2))
                }
            }
            (RBasicValue::String(string1), RBasicValue::Number(number2)) => {
                let number1 = i32::from_str(string1.as_str());

                if number1.is_ok() {
                    Ok(RBasicValue::Number(number1.unwrap() / number2))
                } else {
                    Err(format!("Cannot divide string {} and integer {}", string1, number2))
                }
            }
            _ => Err("Can only divide integers.".to_string()),
        }
    }
}

impl Mul for RBasicValue {
    type Output = Result<RBasicValue, String>;

    fn mul(self, other: RBasicValue) -> Self::Output {
        match (self, other) {
            (RBasicValue::Number(number1), RBasicValue::Number(number2)) => {
                Ok(RBasicValue::Number(number1 * number2))
            }
            (RBasicValue::Number(number1), RBasicValue::String(string2)) => {
                let number2 = i32::from_str(string2.as_str());

                if number2.is_ok() {
                    Ok(RBasicValue::Number(number1 * number2.unwrap()))
                } else {
                    Err(format!("Cannot multiply integer {} and string {}", number1, string2))
                }
            }
            (RBasicValue::String(string1), RBasicValue::Number(number2)) => {
                let number1 = i32::from_str(string1.as_str());

                if number1.is_ok() {
                    Ok(RBasicValue::Number(number1.unwrap() * number2))
                } else {
                    Err(format!("Cannot multiply string {} and integer {}", string1, number2))
                }
            }
            _ => Err("Can only multiply integers.".to_string()),
        }
    }
}

impl Sub for RBasicValue {
    type Output = Result<RBasicValue, String>;

    fn sub(self, other: RBasicValue) -> Self::Output {
        match (self, other) {
            (RBasicValue::Number(number1), RBasicValue::Number(number2)) => {
                Ok(RBasicValue::Number(number1 - number2))
            }
            (RBasicValue::Number(number1), RBasicValue::String(string2)) => {
                let number2 = i32::from_str(string2.as_str());

                if number2.is_ok() {
                    Ok(RBasicValue::Number(number1 - number2.unwrap()))
                } else {
                    Err(format!("Cannot subtract integer {} from string {}",
                                number1,
                                string2))
                }
            }
            (RBasicValue::String(string1), RBasicValue::Number(number2)) => {
                let number1 = i32::from_str(string1.as_str());

                if number1.is_ok() {
                    Ok(RBasicValue::Number(number1.unwrap() - number2))
                } else {
                    Err(format!("Cannot subtract string {} from integer {}",
                                string1,
                                number2))
                }
            }
            _ => Err("Can only subtract integers.".to_string()),
        }
    }
}

// -----------------------------------------------
// Implementations of binary comparison operators
impl RBasicValue {
    pub fn eq(&self, other: &RBasicValue) -> Result<bool, String> {
        match (self, other) {
            (&RBasicValue::Number(number1), &RBasicValue::Number(number2)) => {
                Ok(number1 == number2)
            }
            (&RBasicValue::String(ref string1), &RBasicValue::String(ref string2)) => {
                Ok(string1 == string2)
            }
            (&RBasicValue::Bool(bool1), &RBasicValue::Bool(bool2)) => Ok(bool1 == bool2),
            (&RBasicValue::Number(number1), &RBasicValue::String(ref string2)) => {
                let number2 = i32::from_str(string2.as_str());

                if number2.is_ok() {
                    Ok(number1 == number2.unwrap())
                } else {
                    Err(format!("Cannot compare integer {} from string {}", number1, string2))
                }
            }
            (&RBasicValue::String(ref string1), &RBasicValue::Number(number2)) => {
                let number1 = i32::from_str(string1.as_str());

                if number1.is_ok() {
                    Ok(number1.unwrap() == number2)
                } else {
                    Err(format!("Cannot compare string {} and integer {}", string1, number2))
                }
            }
            _ => {
                Err(format!("Cannot compare values of different types {:?} and {:?}",
                            *self,
                            *other))
            }
        }
    }

    pub fn neq(&self, other: &RBasicValue) -> Result<bool, String> {
        self.eq(other).map(|value| !value)
    }

    pub fn lt(&self, other: &RBasicValue) -> Result<bool, String> {
        match (self, other) {
            (&RBasicValue::Number(number1), &RBasicValue::Number(number2)) => Ok(number1 < number2),
            (&RBasicValue::String(ref string1), &RBasicValue::String(ref string2)) => {
                Ok(string1 < string2)
            }
            (&RBasicValue::Bool(bool1), &RBasicValue::Bool(bool2)) => Ok(bool1 == bool2),
            (&RBasicValue::Number(number1), &RBasicValue::String(ref string2)) => {
                let number2 = i32::from_str(string2.as_str());

                if number2.is_ok() {
                    Ok(number1 < number2.unwrap())
                } else {
                    Err(format!("Cannot compare integer {} from string {}", number1, string2))
                }
            }
            (&RBasicValue::String(ref string1), &RBasicValue::Number(number2)) => {
                let number1 = i32::from_str(string1.as_str());

                if number1.is_ok() {
                    Ok(number1.unwrap() < number2)
                } else {
                    Err(format!("Cannot compare string {} and integer {}", string1, number2))
                }
            }
            _ => {
                Err(format!("Cannot compare values of different types {:?} and {:?}",
                            *self,
                            *other))
            }
        }
    }

    pub fn gt(&self, other: &RBasicValue) -> Result<bool, String> {
        match (self, other) {
            (&RBasicValue::Number(number1), &RBasicValue::Number(number2)) => Ok(number1 > number2),
            (&RBasicValue::String(ref string1), &RBasicValue::String(ref string2)) => {
                Ok(string1 > string2)
            }
            (&RBasicValue::Bool(bool1), &RBasicValue::Bool(bool2)) => Ok(bool1 > bool2),
            (&RBasicValue::Number(number1), &RBasicValue::String(ref string2)) => {
                let number2 = i32::from_str(string2.as_str());

                if number2.is_ok() {
                    Ok(number1 > number2.unwrap())
                } else {
                    Err(format!("Cannot compare integer {} from string {}", number1, string2))
                }
            }
            (&RBasicValue::String(ref string1), &RBasicValue::Number(number2)) => {
                let number1 = i32::from_str(string1.as_str());

                if number1.is_ok() {
                    Ok(number1.unwrap() > number2)
                } else {
                    Err(format!("Cannot compare string {} and integer {}", string1, number2))
                }
            }
            _ => {
                Err(format!("Cannot compare values of different types {:?} and {:?}",
                            *self,
                            *other))
            }
        }
    }

    pub fn lteq(&self, other: &RBasicValue) -> Result<bool, String> {
        self.gt(other).map(|value| !value)
    }

    pub fn gteq(&self, other: &RBasicValue) -> Result<bool, String> {
        self.lt(other).map(|value| !value)
    }
}
