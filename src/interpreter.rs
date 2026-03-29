#![allow(dead_code)]
use crate::parser::ParseTree;
use crate::tokenizer::PolynomialsToken;
use std::collections::BTreeMap;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    /// The mathematical terms of the polynomial.
    /// The `i32` key represents the exponent (power of X).
    /// The `f64` value represents the coefficient.
    ///
    /// Using BTreeMap automatically keeps the polynomial sorted by degree,
    /// making it extremely easy to print out the final "Reduced form".
    pub terms: BTreeMap<i32, f64>,
}

impl Polynomial {
    /// Creates a new, empty polynomial representing 0
    pub fn new() -> Self {
        Self {
            terms: BTreeMap::new(),
        }
    }

    /// Evaluates structural equality handling floating points and zeros.
    pub fn cleanup(&mut self) {
        // Remove mathematically insignificant elements to stay clean.
        self.terms.retain(|_, &mut coeff| coeff != 0.0);
    }

    /// Constructs a polynomial from a standalone constant (e.g., 5.6 -> 5.6 * X^0)
    pub fn from_constant(val: f64) -> Self {
        let mut terms = BTreeMap::new();
        if val != 0.0 {
            // Keep map perfectly clean if 0
            terms.insert(0, val);
        }
        Self { terms }
    }

    /// Constructs a single `X` polynomial element (1.0 * X^1)
    pub fn from_x() -> Self {
        let mut terms = BTreeMap::new();
        terms.insert(1, 1.0);
        Self { terms }
    }

    /// Elevates an entire polynomial to a given power (e.g. X^2)
    pub fn pow(self, power: i32) -> Self {
        if power == 0 {
            return Self::from_constant(1.0);
        }
        if power == 1 {
            return self;
        }

        let mut result = self.clone();
        for _ in 1..power {
            result = result * self.clone();
        }
        result
    }
}

impl std::fmt::Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.terms.is_empty() {
            return write!(f, "0 = 0");
        }

        let mut first = true;
        for (power, coeff) in &self.terms {
            if first {
                if *coeff < 0.0 {
                    write!(f, "-{} * X^{}", coeff.abs(), power)?;
                } else {
                    write!(f, "{} * X^{}", coeff, power)?;
                }
                first = false;
            } else {
                if *coeff < 0.0 {
                    write!(f, " - {} * X^{}", coeff.abs(), power)?;
                } else {
                    write!(f, " + {} * X^{}", coeff, power)?;
                }
            }
        }
        write!(f, " = 0")
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = self.terms.clone();
        for (power, coeff) in other.terms {
            let entry = result.entry(power).or_insert(0.0);
            *entry += coeff;
        }
        let mut poly = Polynomial { terms: result };
        poly.cleanup();
        poly
    }
}

impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = self.terms.clone();
        for (power, coeff) in other.terms {
            let entry = result.entry(power).or_insert(0.0);
            *entry -= coeff;
        }
        let mut poly = Polynomial { terms: result };
        poly.cleanup();
        poly
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = BTreeMap::new();
        // Cross multiplication of every term in `self` with every term in `other`
        for (p1, c1) in &self.terms {
            for (p2, c2) in &other.terms {
                let power = p1 + p2;
                let coeff = c1 * c2;
                let entry = result.entry(power).or_insert(0.0);
                *entry += coeff;
            }
        }
        let mut poly = Polynomial { terms: result };
        poly.cleanup();
        poly
    }
}

#[derive(Debug)]
pub enum InterpreterError {
    InvalidTree,
    InvalidToken(String),
    MissingOperand,
    InvalidExponent,
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterError::InvalidTree => write!(f, "Invalid parse tree"),
            InterpreterError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
            InterpreterError::MissingOperand => write!(f, "Missing operand for an operator"),
            InterpreterError::InvalidExponent => {
                write!(f, "Exponent must be a single integer constant")
            }
        }
    }
}
impl std::error::Error for InterpreterError {}

pub struct Interpreter;

impl Interpreter {
    pub fn evaluate(tree: &ParseTree<PolynomialsToken>) -> Result<Polynomial, InterpreterError> {
        let root_idx = tree.root.ok_or(InterpreterError::InvalidTree)?;
        let root_node = &tree.tree[root_idx];
        if root_node.value != PolynomialsToken::Equals {
            return Err(InterpreterError::InvalidToken(
                "Expected '=' at the root of the equation".to_string(),
            ));
        }

        let lhs_idx = root_node.lhs.ok_or(InterpreterError::MissingOperand)?;
        let rhs_idx = root_node.rhs.ok_or(InterpreterError::MissingOperand)?;

        let lhs_poly = Self::eval_node(tree, lhs_idx)?;
        let rhs_poly = Self::eval_node(tree, rhs_idx)?;

        // Bring RHS to LHS: LHS - RHS = 0
        Ok(lhs_poly - rhs_poly)
    }

    fn eval_node(
        tree: &ParseTree<PolynomialsToken>,
        idx: usize,
    ) -> Result<Polynomial, InterpreterError> {
        let node = &tree.tree[idx];
        match &node.value {
            PolynomialsToken::Expression(s) => {
                if s == "X" {
                    Ok(Polynomial::from_x())
                } else if let Ok(val) = s.parse::<f64>() {
                    Ok(Polynomial::from_constant(val))
                } else {
                    Err(InterpreterError::InvalidToken(format!(
                        "Unknown expression: {}",
                        s
                    )))
                }
            }
            PolynomialsToken::Addition => {
                if let Some(lhs_idx) = node.lhs {
                    let lhs = Self::eval_node(tree, lhs_idx)?;
                    let rhs =
                        Self::eval_node(tree, node.rhs.ok_or(InterpreterError::MissingOperand)?)?;
                    Ok(lhs + rhs)
                } else {
                    // Unary plus
                    let rhs =
                        Self::eval_node(tree, node.rhs.ok_or(InterpreterError::MissingOperand)?)?;
                    Ok(rhs)
                }
            }
            PolynomialsToken::Substraction => {
                if let Some(lhs_idx) = node.lhs {
                    let lhs = Self::eval_node(tree, lhs_idx)?;
                    let rhs =
                        Self::eval_node(tree, node.rhs.ok_or(InterpreterError::MissingOperand)?)?;
                    Ok(lhs - rhs)
                } else {
                    // Unary minus
                    let rhs =
                        Self::eval_node(tree, node.rhs.ok_or(InterpreterError::MissingOperand)?)?;
                    Ok(Polynomial::new() - rhs)
                }
            }
            PolynomialsToken::Multiplication => {
                let lhs = Self::eval_node(tree, node.lhs.ok_or(InterpreterError::MissingOperand)?)?;
                let rhs = Self::eval_node(tree, node.rhs.ok_or(InterpreterError::MissingOperand)?)?;
                Ok(lhs * rhs)
            }
            PolynomialsToken::Exponential => {
                let lhs = Self::eval_node(tree, node.lhs.ok_or(InterpreterError::MissingOperand)?)?;
                let rhs = Self::eval_node(tree, node.rhs.ok_or(InterpreterError::MissingOperand)?)?;

                if rhs.terms.len() > 1
                    || (!rhs.terms.is_empty() && rhs.terms.keys().next() != Some(&0))
                {
                    return Err(InterpreterError::InvalidExponent);
                }

                let power_f64 = *rhs.terms.get(&0).unwrap_or(&0.0);
                if power_f64.fract() != 0.0 {
                    return Err(InterpreterError::InvalidExponent);
                }

                Ok(lhs.pow(power_f64 as i32))
            }
            _ => Err(InterpreterError::InvalidToken(format!(
                "Unexpected token: {:?}",
                node.value
            ))),
        }
    }
}
