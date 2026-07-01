# CPF Validator (validador_de_cpf)

A simple and efficient Rust library for validating Brazilian CPF (Cadastro de Pessoas Físicas) numbers.

## Features

- **Automatic Formatting Handling:** It automatically ignores non-numeric characters (like `.`, `-`, or spaces). You can pass a formatted CPF (`"123.456.789-00"`) or just the numbers (`"12345678900"`).
- **Length Verification:** Ensures the CPF contains exactly 11 numeric digits.
- **Identical Digits Rejection:** Identifies and rejects CPFs made of a single repeated digit (e.g., `"111.111.111-11"`), which pass the mathematical checksum but are legally invalid.
- **Checksum Validation:** Precisely calculates and verifies both the first and the second check digits according to the official Brazilian government algorithm.

## How It Works

The core function `validar_cpf(cpf: &str) -> bool` performs the following steps:

1. **Sanitization:** It iterates through the input string, filters out any characters that are not ASCII digits, and converts the remaining characters into a vector of integers (`Vec<u32>`).
2. **Length Check:** It checks if the resulting vector has exactly 11 elements. If not, it immediately returns `false`.
3. **Repeated Digits Check:** It verifies if all digits in the array are identical. If they are, it returns `false`.
4. **First Check Digit:** It calculates the first verifying digit by multiplying the first 9 digits by a descending weight (from 10 to 2) and summing the results. The remainder of this sum divided by 11 is used to find the expected check digit. It then compares this with the 10th digit of the input.
5. **Second Check Digit:** It calculates the second verifying digit using the same logic, but including the first check digit and using a descending weight from 11 to 2. It compares this result with the 11th digit of the input.

## Usage Example

```rust
use validador_de_cpf::validar_cpf;

fn main() {
    let valid_cpf = "123.456.789-09";
    let invalid_cpf = "111.111.111-11";

    if validar_cpf(valid_cpf) {
        println!("The CPF {} is valid!", valid_cpf);
    } else {
        println!("The CPF {} is invalid.", valid_cpf);
    }

    assert_eq!(validar_cpf(invalid_cpf), false);
}
```
