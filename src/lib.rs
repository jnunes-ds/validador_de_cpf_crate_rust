pub fn validar_cpf(cpf: &str) -> bool {
    // Converte a string de entrada (`&str`) em um iterador de caracteres iterando com `chars()`,
    // filtra apenas os caracteres que são dígitos numéricos através do `is_ascii_digit()`,
    // converte cada caractere validado para um número inteiro base 10 (`to_digit(10)`)
    // e finalmente coleta tudo em um novo vetor de números do tipo `Vec<u32>`.
    let cpf: Vec<u32> = cpf
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    // Verifica o tamanho do vetor: um CPF válido deve conter exatamente 11 dígitos numéricos após a filtragem.
    // Se não tiver, a função retorna falso imediatamente.
    if cpf.len() != 11 {
        return false;
    }

    // CPFs com todos os dígitos iguais (ex: "111.111.111-11", "000.000.000-00") podem passar na validação
    // matemática, mas são considerados inválidos.
    // Aqui verificamos se todos os elementos do vetor são iguais ao primeiro elemento.
    if cpf.iter().all(|&x| x == cpf[0]) {
        return false;
    }

    // Inicia o cálculo do primeiro dígito verificador.
    // Multiplica-se cada um dos 9 primeiros dígitos por pesos decrescentes, começando de 10 até 2.
    // Os resultados dessas multiplicações são somados na variável `sum`.
    let mut sum = 0;
    for i in 0..9 {
        sum += cpf[i] * (10 - i as u32);
    }
    
    // O primeiro dígito verificador é o resultado da subtração de 11 pelo resto da divisão da soma por 11.
    // Se esse resultado for 10 ou 11 (ou seja, maior que 9), o dígito verificador é convertido para 0.
    let mut digit1 = 11 - (sum % 11);
    if digit1 > 9 {
        digit1 = 0;
    }

    // Compara o primeiro dígito verificador calculado com o 10º dígito (índice 9) do CPF fornecido.
    // Se não forem iguais, o CPF não é válido.
    if cpf[9] != digit1 {
        return false;
    }

    // Reinicia a variável `sum` para iniciar o cálculo do segundo dígito verificador.
    // Agora, multiplica-se cada um dos 10 primeiros dígitos (incluindo o primeiro dígito verificador recém-validado)
    // por pesos decrescentes, começando de 11 até 2, somando os resultados.
    sum = 0;
    for i in 0..10 {
        sum += cpf[i] * (11 - i as u32);
    }
    
    // O segundo dígito verificador é calculado de forma semelhante ao primeiro:
    // subtrai-se de 11 o resto da divisão dessa nova soma por 11.
    // Novamente, se o resultado for maior que 9, o dígito passa a ser 0.
    let mut digit2 = 11 - (sum % 11);
    if digit2 > 9 {
        digit2 = 0;
    }

    // A validação final: verifica se o segundo dígito verificador calculado é exatamente igual
    // ao 11º dígito (índice 10) do CPF fornecido.
    // Retorna `true` se forem iguais (CPF válido) ou `false` caso contrário.
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
