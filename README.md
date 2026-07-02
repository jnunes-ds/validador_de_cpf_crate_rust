# CPF and CNPJ Validator (validador_de_cpf)

A simple and efficient Rust library for validating and formatting Brazilian CPF (Cadastro de Pessoas Físicas) and CNPJ (Cadastro Nacional da Pessoa Jurídica) numbers.

## Features

- **Validation & Formatting:** Provides separate modules for validation (`validator`) and formatting (`formatter`).
- **CNPJ Support:** Support for validating and formatting CNPJ numbers.
- **Automatic Formatting Handling:** It automatically ignores non-numeric characters (like `.`, `-`, `/`, or spaces). You can pass a formatted string (`"123.456.789-00"`) or just the numbers (`"12345678900"`).
- **Length Verification:** Ensures the CPF contains exactly 11 numeric digits and CNPJ exactly 14 numeric digits.
- **Identical Digits Rejection:** Identifies and rejects sequences made of a single repeated digit (e.g., `"111.111.111-11"`), which pass the mathematical checksum but are legally invalid.
- **Checksum Validation:** Precisely calculates and verifies the check digits according to the official Brazilian government algorithm.

## How It Works

The core functions like `validator::cpf(cpf: &str) -> bool` and `validator::cnpj(cnpj: &str) -> bool` perform the following steps:

1. **Sanitization:** They iterate through the input string, filter out any characters that are not ASCII digits.
2. **Length Check:** They check if the resulting vector has the correct number of elements (11 for CPF, 14 for CNPJ). If not, they immediately return `false`.
3. **Repeated Digits Check:** They verify if all digits in the array are identical. If they are, they return `false`.
4. **Checksum Validation:** They calculate the two verifying digits based on the official descending weights and compare them with the provided digits.

## Usage Examples

### Validating CPF and CNPJ

```rust
use validador_de_cpf::validator;

fn main() {
    let valid_cpf = "123.456.789-09";

    if validator::cpf(valid_cpf) {
        println!("The CPF {} is valid!", valid_cpf);
    } else {
        println!("The CPF {} is invalid.", valid_cpf);
    }

    assert_eq!(validator::cpf(invalid_cpf), false);
  
    let valid_cnpj = "11.222.333/0001-81";
    if validator::cnpj(valid_cnpj) {
        println!("The CNPJ {} is valid!", valid_cnpj); // The CNPJ 11.222.333/0001-81 is valid!
    }
}
```

### Formatting CPF and CNPJ

```rust
use validador_de_cpf::formatter;

fn main() {
    let unformatted_cpf = "12345678909";
    let formatted_cpf = formatter::cpf(unformatted_cpf);
    assert_eq!(formatted_cpf, "123.456.789-09");

    let unformatted_cnpj = "11222333000181";
    let formatted_cnpj = formatter::cnpj(unformatted_cnpj);
    assert_eq!(formatted_cnpj, "11.222.333/0001-81");
}
```

## Deprecation Notice

Functions like `validar_cpf`, `validate_cpf` and `format_cpf` are now deprecated. Please migrate to the `validator` and `formatter` modules introduced in version 2.x.
