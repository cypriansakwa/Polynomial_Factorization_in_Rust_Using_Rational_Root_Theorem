use std::collections::HashSet;
// Function to get the divisors of a number
fn divisors(n: i32) -> Vec<i32> {
    let mut divs = Vec::new();
    for i in 1..=n.abs() {
        if n % i == 0 {
            divs.push(i);
            divs.push(-i);
        }
    }
    divs
}

// Function to evaluate a polynomial at a given value
fn evaluate_polynomial(coeffs: &[i32], x: i32) -> i32 {
    coeffs.iter().enumerate().fold(0, |acc, (i, &coeff)| {
        acc + coeff * x.pow((coeffs.len() - 1 - i) as u32)
    })
}

// Function to factorize the polynomial using Rational Root Theorem
fn factor_polynomial(coeffs: &[i32]) -> Vec<i32> {
    let leading_coeff = coeffs[0];
    let constant_term = coeffs[coeffs.len() - 1];

    let possible_roots = {
        let p_divs = divisors(constant_term);
        let q_divs = divisors(leading_coeff);

        let mut roots = HashSet::new();
        for p in p_divs {
            for q in &q_divs {
                roots.insert(p / q);
            }
        }
        roots
    };

    // Find the actual roots
    let mut factors = Vec::new();
    for root in possible_roots {
        if evaluate_polynomial(coeffs, root) == 0 {
            factors.push(root);
        }
    }

    factors
}

fn main() {
    let polynomial = vec![1, -6, 11, -6]; // x^3 - 6x^2 + 11x - 6

    println!("Polynomial: {:?}", polynomial);

    let factors = factor_polynomial(&polynomial);

    if factors.is_empty() {
        println!("No rational roots found.");
    } else {
        println!("Factors: {:?}", factors);
    }
}

