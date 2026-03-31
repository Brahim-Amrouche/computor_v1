use std::process::Command;

fn get_computor_output(equation: &str) -> String {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", equation])
        .output()
        .expect("Failed to execute process");

    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn test_every_real_number() {
    let out = get_computor_output("5 * X ^ 0 = 5 * X ^ 0");
    assert!(
        out.contains("Every real number is a solution."),
        "Failed Every Real Number. Output: {}",
        out
    );
}

#[test]
fn test_no_solution() {
    let out = get_computor_output("4 * X ^ 0 = 8 * X ^ 0");
    assert!(
        out.contains("There is no solution."),
        "Failed No Solution. Output: {}",
        out
    );
}

#[test]
fn test_first_degree() {
    let out = get_computor_output("5 * X ^ 0 = 4 * X ^ 0 + 7 * X ^ 1");
    assert!(
        out.contains("Polynomial degree: 1"),
        "Failed detecting degree 1. Output: {}",
        out
    );
    assert!(
        out.contains("The solution is:"),
        "Failed to output solution text. Output: {}",
        out
    );
    assert!(
        out.contains("0.142857"),
        "Failed First Degree math. Expected ~0.142857. Output: {}",
        out
    );
}

#[test]
fn test_second_degree_positive_discriminant() {
    let out = get_computor_output("5 * X ^ 0 + 13 * X ^ 1 + 3 * X ^ 2 = 1 * X ^ 0 + 1 * X ^ 1");
    assert!(
        out.contains("Polynomial degree: 2"),
        "Failed detecting degree 2. Output: {}",
        out
    );
    assert!(
        out.contains("Discriminant is strictly positive"),
        "Failed Discriminant rules. Output: {}",
        out
    );
}

#[test]
fn test_discriminant_zero() {
    let out = get_computor_output("6 * X ^ 0 + 11 * X ^ 1 + 5 * X ^ 2 = 1 * X ^ 0 + 1 * X ^ 1");
    assert!(
        out.contains("Polynomial degree: 2"),
        "Failed detecting degree 2. Output: {}",
        out
    );
    assert!(
        out.contains("Discriminant is exactly zero"),
        "Failed Discriminant rules. Output: {}",
        out
    );
    assert!(
        out.contains("-1\n") || out.contains("-1"),
        "Failed math for zero discriminant. Expected -1. Output: {}",
        out
    );
}

#[test]
fn test_negative_discriminant() {
    let out = get_computor_output("5 * X ^ 0 + 3 * X ^ 1 + 3 * X ^ 2 = 1 * X ^ 0 + 0 * X ^ 1");
    assert!(
        out.contains("Polynomial degree: 2"),
        "Failed detecting degree 2. Output: {}",
        out
    );
    assert!(
        out.contains("Discriminant is strictly negative"),
        "Failed Discriminant rules. Output: {}",
        out
    );
    assert!(
        out.contains("* i"),
        "Failed missing imaginary number identifier formatting. Output: {}",
        out
    );
}
