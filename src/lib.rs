pub mod validator {
    /// Validates a Brazilian CPF (Cadastro de Pessoas Físicas) number.
    ///
    /// This function performs sanitization by ignoring non-digit characters, checks for
    /// invalid sequences (like all identical digits), and implements the official
    /// two-digit checksum algorithm.
    ///
    /// # Arguments
    ///
    /// * `cpf` - A string slice that holds the CPF (can be formatted or only digits).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use validador_de_cpf::validator;
    ///
    /// let valid = validator::cpf("123.456.789-09");
    /// assert!(valid);
    ///
    /// let invalid = validator::cpf("111.111.111-11");
    /// assert!(!invalid);
    /// ```
    pub fn cpf(cpf: &str) -> bool {
        let cpf: Vec<u32> = cpf
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        if cpf.len() != 11 {
            return false;
        }

        if cpf.iter().all(|&x| x == cpf[0]) {
            return false;
        }


        let mut sum = 0;
        for i in 0..9 {
            sum += cpf[i] * (10 - i as u32);
        }

        let mut digit1 = 11 - (sum % 11);
        if digit1 > 9 {
            digit1 = 0;
        }


        if cpf[9] != digit1 {
            return false;
        }

        sum = 0;
        for i in 0..10 {
            sum += cpf[i] * (11 - i as u32);
        }

        let mut digit2 = 11 - (sum % 11);
        if digit2 > 9 {
            digit2 = 0;
        }
        cpf[10] == digit2

    }

    pub fn cnpj(cnpj: &str) -> bool {
        let cnpj: Vec<u32> = cnpj
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        if cnpj.len() != 14 {
            return false;
        }

        if cnpj.iter().all(|&x| x == cnpj[0]) {
            return false;
        }

        let calculate_digit = |digits: &[u32], weights: &[u32]| -> u32 {
            let sum: u32 = digits
                .iter()
                .zip(weights.iter())
                .map(|(d, w)| d * w)
                .sum();
            let remainder = sum % 11;
            if remainder < 2 {
                0
            } else {
                11 - remainder
            }
        };

        let weights1 = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
        let digit1 = calculate_digit(&cnpj[0..12], &weights1);
        if cnpj[12] != digit1 {
            return false;
        }

        let weights2 = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
        let digit2 = calculate_digit(&cnpj[0..13], &weights2);
        cnpj[13] == digit2
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_invalid_cpf_with_symbols() {
            assert!(!cpf("543.645.626-54"));
        }

        #[test]
        fn test_invalid_cpf_without_symbols() {
            assert!(!cpf("54364562654"));
        }

        #[test]
        fn test_valid_cpf_with_symbols() {
            assert!(cpf("035.711.250-45"));
        }

        #[test]
        fn test_valid_cpf_without_symbols() {
            assert!(cpf("03571125045"));
        }

        #[test]
        fn test_invalid_cnpj_with_symbols() { assert!(!cnpj("12.345.678/0001-99")); }

        #[test]
        fn test_invalid_cnpj_without_symbols() { assert!(!cnpj("12345678000199")); }

        #[test]
        fn test_valid_cnpj_with_symbols() { assert!(cnpj("11.222.333/0001-81")); }

        #[test]
        fn test_valid_cnpj_without_symbols() { assert!(cnpj("11222333000181")); }

        #[test]
        fn test_cnpj_identical_digits() { assert!(!cnpj("11.111.111/1111-11")); }

    }
}

pub mod formatter {
    /// Formats a string into a CPF pattern (000.000.000-00).
    ///
    /// If the input string does not contain exactly 11 digits after sanitization,
    /// it returns the original string.
    ///
    /// # Arguments
    ///
    /// * `cpf` - A string slice that holds the CPF digits.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use validador_de_cpf::formatter;
    ///
    /// let formatted = formatter::cpf("12345678909");
    /// assert_eq!(formatted, "123.456.789-09");
    ///```
    ///
    pub fn cpf<'a>(cpf: &'a str) -> &'a str {
        let digits: String = cpf.chars().filter(|c| c.is_ascii_digit()).collect();

        if digits.len() != 11 {
            return cpf;
        }

        format!(
            "{}.{}.{}-{}",
            &digits[0..3],
            &digits[3..6],
            &digits[6..9],
            &digits[9..11]
        )
            .leak()

    }

    pub fn cnpj<'a>(cnpj: &'a str) -> &'a str {
        let digits: String = cnpj.chars().filter(|c| c.is_ascii_digit()).collect();

        if digits.len() != 14 {
            return cnpj
        }

        //11.222.333/0001-81
        format!(
            "{}.{}.{}/{}-{}",
            &digits[0..2],
            &digits[2..5],
            &digits[5..8],
            &digits[8..12],
            &digits[12..14]
        ).leak()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_format_cpf_with_symbols() {
            assert_eq!(cpf("035.711.250-45"), "035.711.250-45");
        }

        #[test]
        fn test_format_cpf_without_symbols() {
            assert_eq!(cpf("03571125045"), "035.711.250-45");
        }

        #[test]
        fn test_format_cpf_with_invalid_chars() {
            assert_eq!(cpf("fsdr325432432"), "fsdr325432432");
        }

        #[test]
        fn test_format_cnpj_with_symbols() {
            assert_eq!(cnpj("11.222.333/0001-81"), "11.222.333/0001-81");
        }

        #[test]
        fn test_format_cnpj_without_symbols() {
            assert_eq!(cnpj("11222333000181"), "11.222.333/0001-81");
        }

        #[test]
        fn test_format_cnpj_with_invalid_chars() {
            assert_eq!(cnpj("abc11222333000181xyz"), "11.222.333/0001-81");
        }

    }
}

/// Validates a Brazilian CPF (Cadastro de Pessoas Físicas) number.
///
/// This function performs sanitization by ignoring non-digit characters, checks for
/// invalid sequences (like all identical digits), and implements the official
/// two-digit checksum algorithm.
///
/// # Arguments
///
/// * `cpf` - A string slice that holds the CPF (can be formatted or only digits).
///
/// # Examples
///
/// ```rust
/// use validador_de_cpf::validate_cpf;
///
/// let valid = validate_cpf("123.456.789-09");
/// assert!(valid);
///
/// let invalid = validate_cpf("111.111.111-11");
/// assert!(!invalid);
/// ```
#[deprecated(since = "2.0.0", note = "Please use `validator::cpf` instead.")]
pub fn validate_cpf(cpf: &str) -> bool {
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

/// Formats a string into a CPF pattern (000.000.000-00).
///
/// If the input string does not contain exactly 11 digits after sanitization,
/// it returns the original string.
///
/// # Arguments
///
/// * `cpf` - A string slice that holds the CPF digits.
///
/// # Examples
///
/// ```rust
/// use validador_de_cpf::format_cpf;
///
/// let formatted = format_cpf("12345678909");
/// assert_eq!(formatted, "123.456.789-09");
///```
///
#[deprecated(since = "2.0.0", note = "Please use `formatter::cpf` instead.")]
pub fn format_cpf<'a>(cpf: &'a str) -> &'a str {
    let digits: String = cpf.chars().filter(|c| c.is_ascii_digit()).collect();

    if digits.len() != 11 {
        return cpf;
    }

    format!(
        "{}.{}.{}-{}",
        &digits[0..3],
        &digits[3..6],
        &digits[6..9],
        &digits[9..11]
    )
    .leak()

}

/// Validates a Brazilian CPF (Cadastro de Pessoas Físicas) number.
///
/// This function performs sanitization by ignoring non-digit characters, checks for
/// invalid sequences (like all identical digits), and implements the official
/// two-digit checksum algorithm.
///
/// # Arguments
///
/// * `cpf` - A string slice that holds the CPF (can be formatted or only digits).
///
/// # Examples
///
/// ```rust
/// use validador_de_cpf::validate_cpf;
///
/// let valid = validate_cpf("123.456.789-09");
/// assert!(valid);
///
/// let invalid = validate_cpf("111.111.111-11");
/// assert!(!invalid);
/// ```
#[deprecated(since = "1.0.0", note = "please use `validate_cpf` instead")]
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
#[allow(deprecated)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_cpf_with_symbols() {
        assert!(!validate_cpf("543.645.626-54"));
    }

    #[test]
    fn test_invalid_cpf_without_symbols() {
        assert!(!validate_cpf("54364562654"));
    }

    #[test]
    fn test_valid_cpf_with_symbols() {
        assert!(validate_cpf("035.711.250-45"));
    }

    #[test]
    fn test_valid_cpf_without_symbols() {
        assert!(validate_cpf("03571125045"));
    }

    #[test]
    fn test_format_cpf_with_symbols() {
        assert_eq!(format_cpf("035.711.250-45"), "035.711.250-45");
    }

    #[test]
    fn test_format_cpf_without_symbols() {
        assert_eq!(format_cpf("03571125045"), "035.711.250-45");
    }

    #[test]
    fn test_format_cpf_with_invalid_chars() {
        assert_eq!(format_cpf("fsdr325432432"), "fsdr325432432");
    }
}
