use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Not;
use std::ops::Sub;
use std::ops::Rem;

use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum RBasicValue {
    String(String),
    Number(f64),
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
                let number2 = f64::from_str(string2.as_str());

                if let Result::Ok(number2_value) = number2 {
                    Ok(RBasicValue::Number(number1 + number2_value))
                } else {
                    Err(format!(
                        "Cannot add number {} and string {}",
                        number1, string2
                    ))
                }
            }
            (RBasicValue::String(string1), RBasicValue::Number(number2)) => {
                let number1 = f64::from_str(string1.as_str());

                if let Result::Ok(number1_value) = number1 {
                    Ok(RBasicValue::Number(number1_value + number2))
                } else {
                    Err(format!(
                        "Cannot add string {} and number {}",
                        string1, number2
                    ))
                }
            }
            _ => Err("Can only add numbers or concatenate strings.".to_string()),
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
                let number2 = f64::from_str(string2.as_str());

                if let Result::Ok(number2_value) = number2 {
                    Ok(RBasicValue::Number(number1 / number2_value))
                } else {
                    Err(format!(
                        "Cannot divide number {} and string {}",
                        number1, string2
                    ))
                }
            }
            (RBasicValue::String(string1), RBasicValue::Number(number2)) => {
                let number1 = f64::from_str(string1.as_str());

                if let Result::Ok(number1_value) = number1 {
                    Ok(RBasicValue::Number(number1_value / number2))
                } else {
                    Err(format!(
                        "Cannot divide string {} and number {}",
                        string1, number2
                    ))
                }
            }
            _ => Err("Can only divide numbers.".to_string()),
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
                let number2 = f64::from_str(string2.as_str());

                if let Result::Ok(number2_value) = number2 {
                    Ok(RBasicValue::Number(number1 * number2_value))
                } else {
                    Err(format!(
                        "Cannot multiply number {} and string {}",
                        number1, string2
                    ))
                }
            }
            (RBasicValue::String(string1), RBasicValue::Number(number2)) => {
                let number1 = f64::from_str(string1.as_str());

                if let Result::Ok(number1_value) = number1 {
                    Ok(RBasicValue::Number(number1_value * number2))
                } else {
                    Err(format!(
                        "Cannot multiply string {} and number {}",
                        string1, number2
                    ))
                }
            }
            _ => Err("Can only multiply numbers.".to_string()),
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
                let number2 = f64::from_str(string2.as_str());

                if let Result::Ok(number2_value) = number2 {
                    Ok(RBasicValue::Number(number1 - number2_value))
                } else {
                    Err(format!(
                        "Cannot subtract number {} from string {}",
                        number1, string2
                    ))
                }
            }
            (RBasicValue::String(string1), RBasicValue::Number(number2)) => {
                let number1 = f64::from_str(string1.as_str());

                if let Result::Ok(number1_value) = number1 {
                    Ok(RBasicValue::Number(number1_value - number2))
                } else {
                    Err(format!(
                        "Cannot subtract string {} from number {}",
                        string1, number2
                    ))
                }
            }
            _ => Err("Can only subtract numbers.".to_string()),
        }
    }
}

impl Rem for RBasicValue {
    type Output = Result<RBasicValue, String>;

    fn rem(self, other: RBasicValue) -> Self::Output {
        match (self, other) {
            (RBasicValue::Number(number1), RBasicValue::Number(number2)) => {
                Ok(RBasicValue::Number(number1 % number2))
            }
            (RBasicValue::Number(number1), RBasicValue::String(string2)) => {
                let number2 = f64::from_str(string2.as_str());

                if let Result::Ok(number2_value) = number2 {
                    Ok(RBasicValue::Number(number1 % number2_value))
                } else {
                    Err(format!(
                        "Cannot mod number {} from string {}",
                        number1, string2
                    ))
                }
            }
            (RBasicValue::String(string1), RBasicValue::Number(number2)) => {
                let number1 = f64::from_str(string1.as_str());

                if let Result::Ok(number1_value) = number1 {
                    Ok(RBasicValue::Number(number1_value % number2))
                } else {
                    Err(format!(
                        "Cannot mod string {} from number {}",
                        string1, number2
                    ))
                }
            }
            _ => Err("Can only mod numbers.".to_string()),
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
                let number2 = f64::from_str(string2.as_str());

                if let Result::Ok(number2_value) = number2 {
                    Ok(number1 == number2_value)
                } else {
                    Err(format!(
                        "Cannot compare number {} from string {}",
                        number1, string2
                    ))
                }
            }
            (&RBasicValue::String(ref string1), &RBasicValue::Number(number2)) => {
                let number1 = f64::from_str(string1.as_str());

                if let Result::Ok(number1_value) = number1 {
                    Ok(number1_value == number2)
                } else {
                    Err(format!(
                        "Cannot compare string {} and number {}",
                        string1, number2
                    ))
                }
            }
            _ => Err(format!(
                "Cannot compare values of different types {:?} and {:?}",
                *self, *other
            )),
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
            (&RBasicValue::Bool(bool1), &RBasicValue::Bool(bool2)) => Ok(!bool1 && bool2),
            (&RBasicValue::Number(number1), &RBasicValue::String(ref string2)) => {
                let number2 = f64::from_str(string2.as_str());

                if let Result::Ok(number2_value) = number2 {
                    Ok(number1 < number2_value)
                } else {
                    Err(format!(
                        "Cannot compare number {} from string {}",
                        number1, string2
                    ))
                }
            }
            (&RBasicValue::String(ref string1), &RBasicValue::Number(number2)) => {
                let number1 = f64::from_str(string1.as_str());

                if let Result::Ok(number1_value) = number1 {
                    Ok(number1_value < number2)
                } else {
                    Err(format!(
                        "Cannot compare string {} and number {}",
                        string1, number2
                    ))
                }
            }
            _ => Err(format!(
                "Cannot compare values of different types {:?} and {:?}",
                *self, *other
            )),
        }
    }

    pub fn gt(&self, other: &RBasicValue) -> Result<bool, String> {
        match (self, other) {
            (&RBasicValue::Number(number1), &RBasicValue::Number(number2)) => Ok(number1 > number2),
            (&RBasicValue::String(ref string1), &RBasicValue::String(ref string2)) => {
                Ok(string1 > string2)
            }
            (&RBasicValue::Bool(bool1), &RBasicValue::Bool(bool2)) => Ok(bool1 && !bool2),
            (&RBasicValue::Number(number1), &RBasicValue::String(ref string2)) => {
                let number2 = f64::from_str(string2.as_str());

                if let Result::Ok(number2_value) = number2 {
                    Ok(number1 > number2_value)
                } else {
                    Err(format!(
                        "Cannot compare number {} from string {}",
                        number1, string2
                    ))
                }
            }
            (&RBasicValue::String(ref string1), &RBasicValue::Number(number2)) => {
                let number1 = f64::from_str(string1.as_str());

                if let Result::Ok(number1_value) = number1 { 
                    Ok(number1_value > number2)
                } else {
                    Err(format!(
                        "Cannot compare string {} and number {}",
                        string1, number2
                    ))
                }
            }
            _ => Err(format!(
                "Cannot compare values of different types {:?} and {:?}",
                *self, *other
            )),
        }
    }

    pub fn lteq(&self, other: &RBasicValue) -> Result<bool, String> {
        self.gt(other).map(|value| !value)
    }

    pub fn gteq(&self, other: &RBasicValue) -> Result<bool, String> {
        self.lt(other).map(|value| !value)
    }
}
