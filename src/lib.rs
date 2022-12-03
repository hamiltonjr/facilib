use std::io;
use std::io::prelude::*;

////////////////////// ROTINAS ÚTEIS //////////////////////
/*
 * Essa função mostra a mensagem na tela e captura uma das
 * opções passadas.
 * Exemplo:
 * mensagem: "Quer continuar?[s/n]"
 *     opção1: "sim"
 *     opção2: "não"
 * A função espera o usuário digitar uma dessas (não aceita
 * outra) e devolve true para  a  primeira  e  false para a
 * segunda.
 */
pub fn again(message: &str, op1: &str, op2: &str) -> bool {
    let mut choice: String = String::from("_");
    while choice != op1 && choice != op2 {
        print!("\n{}", message);
        choice = input();
    }
    choice == op1
}

/*
 * Essa função implementa uma composição de caracteres formando uma linha.
 * ch é o caractere, t é o número de vezes que o caractere é repetido para
 * compor o objeto retornado.
 * OBS: função privada usada em banner().
 */
fn repeat(ch: char, t: usize) -> String {
    let mut repeated = String::new();
    for _ in 0..t {
        repeated.push(ch);
    }
    repeated
}

/*
 * Essa função implementa um banner com 69 caracteres de largura.
 */
pub fn banner(message: &str) {
    const LENGTH: usize = 50;
    let line = repeat('-', LENGTH);
    let spaces = repeat(' ', (LENGTH - message.len()) / 2);
    println!("\n{line}\n{spaces}{message}{spaces}\n{line}\n");
}

/*
* Essa função implementa uma pausa para o final da
* execução do programa sequencial (ou outras situa-
* ções onde o conteúdo da tela precise ser lido).
*/
pub fn pause() {
    println!();
    println!("Tecle <ENTER> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}

/*
 * Essa função implementa a limpeza da tela.
 */
pub fn clear() {
    print!("{}[2J", 27 as char);
}

/*
 * Essa função verifica se o valor é par.
 */
pub fn is_even(x: u32) -> bool {
    x % 2 == 0
}

///////////////// ROTINAS DE ENTRADA/SAÍDA ////////////////
/*
* Essa função implementa a entrada de uma string.
*/
pub fn input() -> String {
    // entrada
    let mut s = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut s).unwrap();

    // retira <ENTER> da string
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    s
}

/*
 * Essa função implementa a entrada de valor inteiro de 32
 * bits. Se o valor digitado for não-numérico por exemplo,
 * o retorno é sempre 0.
 */
pub fn input_i32() -> i32 {
    // entrada
    let mut str = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut str).unwrap();

    // conversão
    let value: i32 = match str.trim().parse::<i32>() {
        Ok(value) => value, // valor correto
        Err(_) => 0,        // valor com qq tipo de erro
    };
    value
}

/*
 * Essa função implementa a entrada de valor inteiro de 64
 * bits. Se o valor digitado for não-numérico por exemplo,
 * o retorno é sempre 0.
 */
pub fn input_i64() -> i64 {
    // entrada
    let mut str = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut str).unwrap();

    // conversão
    let value: i64 = match str.trim().parse::<i64>() {
        Ok(value) => value, // valor correto
        Err(_) => 0,        // valor com qq tipo de erro
    };
    value
}

/*
 * Essa função implementa a entrada de valor inteiro positivo
 * de 32 bits. Se o valor digitado for não-numérico por exemplo,
 * o retorno é sempre 0.
 */
pub fn input_u32() -> u32 {
    // entrada
    let mut str = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut str).unwrap();

    // conversão
    let value: u32 = match str.trim().parse::<u32>() {
        Ok(value) => value, // valor correto
        Err(_) => 0,        // valor com qq tipo de erro
    };
    value
}

/*
 * Essa função implementa a entrada de valor inteiro positivo
 * de 64 its. Se o valor digitado for não-numérico por exemplo,
 * o retorno é sempre 0.
 */
pub fn input_u64() -> u64 {
    // entrada
    let mut str = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut str).unwrap();

    // conversão
    let value: u64 = match str.trim().parse::<u64>() {
        Ok(value) => value, // valor correto
        Err(_) => 0,        // valor com qq tipo de erro
    };
    value
}

/*
 * Essa função implementa a entrada de valor real de 32
 * bits. Se o valor digitado for não-numérico por exemplo,
 * o retorno é sempre 0.0.
 */
pub fn input_f32() -> f32 {
    // entrada
    let mut str = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut str).unwrap();

    // conversão
    let value: f32 = match str.trim().parse::<f32>() {
        Ok(value) => value, // valor correto
        Err(_) => 0.0,      // valor com qq tipo de erro
    };
    value
}

/*
 * Essa função implementa a entrada de valor real de 64
 * bits. Se o valor digitado for não-numérico por exemplo,
 * o retorno é sempre 0.0.
 */
pub fn input_f64() -> f64 {
    // entrada
    let mut str = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut str).unwrap();

    // conversão
    let value: f64 = match str.trim().parse::<f64>() {
        Ok(value) => value, // valor correto
        Err(_) => 0.0,      // valor com qq tipo de erro
    };
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat() {
        assert_eq!(repeat('-', 0), "");
        assert_eq!(repeat('-', 3), "---");
    }

}
