use std::io;
use std::io::prelude::*;

//////////////////////// UTILITIES ////////////////////////
/*
 * This function shows a message in the screen and catches
 * an user option.
 * Example:
 * mesage: "Continue?[yes/no]"
 *     op1: "yes"
 *     op2: "no"
 * The function wait user type one of this options (and never
 * other option) and returns true for op1 and false for op2.
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
 * This function implements a banner with width of 60 characters.
 */
pub fn banner(message: &str) {
    const LENGTH: usize = 60;
    let nspaces = (LENGTH - message.trim().chars().count()) / 2;
    println!("\n{}\n{}{}{}\n{}\n",
        "-".repeat(LENGTH), " ".repeat(nspaces), message, 
        " ".repeat(nspaces), "-".repeat(LENGTH) 
    );
}

/*
 * This function implements a pause waiting the user
 * type <ENTER> (for example).
 */
pub fn pause() {
    println!();
    println!("Tecle <ENTER> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}

/*
 * This function implements a clear for the screen.
 */
pub fn clear() {
    print!("{}[2J", 27 as char);
}

/*
 * Boolean function returns if the number is even.
 */
pub fn is_even(x: u32) -> bool {
    x % 2 == 0
}

////////////////////// I/O UTILITIES //////////////////////
/*
* This function implements string input.
*/
pub fn input() -> String {
    // entrada
    let mut s = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut s).unwrap();

    // retira <ENTER> da string (no Windows ou Linux)
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    s
}

/*
 * This function implements input of an 32-bits integer
 * Case the value is non-numeric the return is 0.
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
 * This function implements input of an 64-bits integer
 * Case the value is non-numeric the return is 0.
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
 * This function implements input of an 32-bits non-signed
 * integer. Case the value is non-numeric the return is 0.
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
 * This function implements input of an 64-bits non-signed
 * integer. Case the value is non-numeric the return is 0.
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
 * This function implements input of an 32-bits non-signed
 * floating-point number. Case the value is non-numeric the 
 * return is 0.
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
 * This function implements input of an 64-bits non-signed
 * floating-point number. Case the value is non-numeric the 
 * return is 0.
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
