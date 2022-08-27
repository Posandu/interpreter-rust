use std::io;

fn main() {
    //
    // Simple programming language
    //
    // stack=[]
    //
    // p(value) -> push value to stack
    // o() -> pop value from stack
    // s() -> print stack
    // s(index) -> print stack[index]
    // o(index) -> pop value from stack[index]
    //

    // Code
    let mut code = String::new();

    struct Token {
        name: String,
        value: String,
    }

    println!(
        "{}",
        r#"
        UwU programming language -- Rust edition

        Help:

        p(value) -> push value to stack
        o() -> pop value from stack
        s() -> print stack

        s(index) -> print stack[index]
        o(index) -> pop value from stack[index]
    "#
    );

    println!("Enter code (type XX to run):");

    loop {
        let mut txt = String::new();
        io::stdin()
            .read_line(&mut txt)
            .expect("Failed to read line");

        txt = txt.trim().to_string();

        if txt == "XX" {
            println!("Running...");

            break;
        }

        txt = txt + "\n";

        code.push_str(&txt);
    }

    let mut tokens: Vec<Token> = vec![];
    let chars = code.chars().collect::<Vec<char>>();
    let mut i = 0;
    let len = chars.len();

    while i < len {
        let current = chars[i];

        if current == '(' {
            let mut value_str = String::new();

            i += 1;

            while i < len && chars[i] != ')' {
                value_str.push(chars[i]);
                i += 1;
            }

            tokens.push(Token {
                name: "InsideBrackets".to_string(),
                value: value_str,
            });

            i += 1;

            continue;
        }
        // Skip whitespace
        else if current.is_whitespace() {
            i += 1;

            continue;
        }
        // Identifiers
        else if current.is_alphabetic() {
            let mut name_str = String::new();

            name_str.push(current);

            i += 1;

            while i < len && chars[i].is_alphanumeric() {
                name_str.push(chars[i]);
                i += 1;
            }

            tokens.push(Token {
                name: "Identifier".to_string(),
                value: name_str,
            });

            continue;
        }

        i += 1;
    }

    //
    // The interpreter
    //
    let mut stack: Vec<String> = vec![];
    let mut i = 0;
    let len = tokens.len();

    while i < len {
        let token = &tokens[i];

        if token.name == "Identifier" {
            let next_token = &tokens[i + 1];

            // Push value to stack
            if token.value == "p" {
                let value = next_token.value.clone();
                stack.push(value);
            }
            // Pop value from stack
            else if token.value == "o" {
                if next_token.value.len() > 0 {
                    let index = next_token.value.parse::<usize>().unwrap();
                    stack.remove(index);
                } else {
                    stack.pop();
                }
            }
            // Print stack
            else if token.value == "s" {
                if next_token.value.len() > 0 {
                    let index = next_token.value.parse::<usize>().unwrap();
                    println!("{}", stack[index]);
                } else {
                    print!("Stack: ");

                    stack.iter().into_iter().for_each(|value| {
                        print!("{} ", value);
                    });

                    println!("");
                }
            }
        }

        i += 1;
    }

    println!("Done! Press any key to exit.");

    io::stdin().read_line(&mut String::new()).unwrap();
}
