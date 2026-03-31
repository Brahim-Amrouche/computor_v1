use crate::interpreter::Polynomial;

#[derive(Debug)]
pub enum ExecutorError {
    DegreeTooHigh(i32),
    NegativeDegree(i32),
}

impl std::fmt::Display for ExecutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutorError::DegreeTooHigh(d) => write!(
                f,
                "Polynomial degree: {}\nThe polynomial degree is strictly greater than 2, I can't solve it.",
                d
            ),
            ExecutorError::NegativeDegree(d) => write!(
                f,
                "Polynomial degree: {}\nNegative degrees are not supported.",
                d
            ),
        }
    }
}

impl std::error::Error for ExecutorError {}

pub struct Executor;

impl Executor {
    pub fn execute(poly: &Polynomial) -> Result<(), ExecutorError> {
        let max_degree = poly.terms.keys().copied().max().unwrap_or(0);
        let min_degree = poly.terms.keys().copied().min().unwrap_or(0);

        if min_degree < 0 {
            return Err(ExecutorError::NegativeDegree(min_degree));
        }

        if max_degree > 2 {
            return Err(ExecutorError::DegreeTooHigh(max_degree));
        }

        println!("Polynomial degree: {}", max_degree);

        let a = *poly.terms.get(&2).unwrap_or(&0.0);
        let b = *poly.terms.get(&1).unwrap_or(&0.0);
        let c = *poly.terms.get(&0).unwrap_or(&0.0);

        if max_degree == 0 {
            if c == 0.0 {
                println!("Every real number is a solution.");
            } else {
                println!("There is no solution.");
            }
            return Ok(());
        }

        if max_degree == 1 {
            println!("The solution is:");
            let mut sol = -c / b;
            if sol == -0.0 {
                sol = 0.0;
            }
            println!("{}", sol);
            return Ok(());
        }

        if max_degree == 2 {
            let delta = b * b - 4.0 * a * c;

            if delta > 0.0 {
                println!("Discriminant is strictly positive, the two solutions are:");
                let sqrt_delta = delta.sqrt();
                let sol1 = (-b - sqrt_delta) / (2.0 * a);
                let sol2 = (-b + sqrt_delta) / (2.0 * a);
                println!("{}", sol1);
                println!("{}", sol2);
            } else if delta == 0.0 {
                println!("Discriminant is exactly zero, the real solution is:");
                let sol = -b / (2.0 * a);
                if sol == -0.0 {
                    println!("0");
                } else {
                    println!("{}", sol);
                }
            } else {
                println!("Discriminant is strictly negative, the two complex solutions are:");
                let sqrt_neg_delta = (-delta).sqrt();

                let mut real_part = -b / (2.0 * a);
                if real_part == -0.0 {
                    real_part = 0.0;
                }

                let imag_part = sqrt_neg_delta / (2.0 * a);
                let abs_imag = imag_part.abs();

                if real_part == 0.0 {
                    println!("-{} * i", abs_imag);
                    println!("{} * i", abs_imag);
                } else {
                    println!("{} - {} * i", real_part, abs_imag);
                    println!("{} + {} * i", real_part, abs_imag);
                }
            }
        }

        Ok(())
    }
}
