pub fn validar_cpf(cpf: &str) -> bool {
    // Converts the input string (`&str`) into a character iterator using `chars()`,
    // filters only the characters that are numeric digits via `is_ascii_digit()`,
    // converts each validated character to a base 10 integer (`to_digit(10)`),
    // and finally collects everything into a new vector of numbers of type `Vec<u32>`.
    let cpf: Vec<u32> = cpf
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    // Checks the vector length: a valid CPF must contain exactly 11 numeric digits after filtering.
    // If it doesn't, the function immediately returns false.
    if cpf.len() != 11 {
        return false;
    }

    // CPFs with all identical digits (e.g., "111.111.111-11", "000.000.000-00") might pass the mathematical
    // validation, but they are considered invalid.
    // Here we verify if all elements in the vector are equal to the first element.
    if cpf.iter().all(|&x| x == cpf[0]) {
        return false;
    }

    // Starts the calculation for the first check digit.
    // Multiplies each of the first 9 digits by descending weights, starting from 10 down to 2.
    // The results of these multiplications are added to the `sum` variable.
    let mut sum = 0;
    for i in 0..9 {
        sum += cpf[i] * (10 - i as u32);
    }
    
    // The first check digit is the result of subtracting from 11 the remainder of the sum divided by 11.
    // If this result is 10 or 11 (i.e., greater than 9), the check digit becomes 0.
    let mut digit1 = 11 - (sum % 11);
    if digit1 > 9 {
        digit1 = 0;
    }

    // Compares the calculated first check digit with the 10th digit (index 9) of the provided CPF.
    // If they are not equal, the CPF is invalid.
    if cpf[9] != digit1 {
        return false;
    }

    // Resets the `sum` variable to start calculating the second check digit.
    // Now, it multiplies each of the first 10 digits (including the newly validated first check digit)
    // by descending weights, starting from 11 down to 2, summing the results.
    sum = 0;
    for i in 0..10 {
        sum += cpf[i] * (11 - i as u32);
    }
    
    // The second check digit is calculated similarly to the first:
    // subtracts from 11 the remainder of this new sum divided by 11.
    // Again, if the result is greater than 9, the digit becomes 0.
    let mut digit2 = 11 - (sum % 11);
    if digit2 > 9 {
        digit2 = 0;
    }

    // The final validation: checks if the calculated second check digit is exactly equal
    // to the 11th digit (index 10) of the provided CPF.
    // Returns `true` if they are equal (valid CPF) or `false` otherwise.
    cpf[10] == digit2

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_cpf_with_symbols() {
        assert!(!validar_cpf("543.645.626-54"));
    }

    #[test]
    fn test_invalid_cpf_without_symbols() {
        assert!(!validar_cpf("54364562654"));
    }

    #[test]
    fn test_valid_cpf_with_symbols() {
        assert!(validar_cpf("035.711.250-45"));
    }

    #[test]
    fn test_valid_cpf_without_symbols() {
        assert!(validar_cpf("03571125045"));
    }

}
