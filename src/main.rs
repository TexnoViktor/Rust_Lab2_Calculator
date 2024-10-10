use std::io;
use std::collections::VecDeque;

fn main() {
    let mut memory: f64 = 0.0;

    loop {
        println!("Оберіть режим:");
        println!("1 - Класичний калькулятор");
        println!("2 - Польська нотація (RPN)");
        println!("Введіть 'exit' для виходу");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Не вдалося прочитати рядок");

        if input.trim() == "exit" {
            break;
        }

        match input.trim() {
            "1" => {
                classical_calculator(&mut memory);
            }
            "2" => {
                println!("Введіть вираз у польській нотації:");
                let mut rpn_input = String::new();
                io::stdin().read_line(&mut rpn_input).expect("Не вдалося прочитати вираз");

                match rpn_calculator(&rpn_input.trim()) {
                    Ok(result) => {
                        memory = result;
                        println!("Результат (RPN): {}", result);
                    }
                    Err(e) => {
                        println!("Помилка: {}", e);
                    }
                }
            }
            _ => {
                println!("Невірний вибір. Спробуйте знову.");
            }
        }
    }
}

fn classical_calculator(memory: &mut f64) {
    println!("Введіть операцію (+, -, *, /) або 'm' для використання запам'ятованого результату:");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Не вдалося прочитати рядок");

    println!("Введіть перше число:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Не вдалося прочитати число");

    let num1: f64 = match num1.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Помилка: введіть коректне число.");
            return;
        }
    };

    println!("Введіть друге число:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Не вдалося прочитати число");

    let num2: f64 = match num2.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Помилка: введіть коректне число.");
            return;
        }
    };

    match operation.trim() {
        "+" => *memory = num1 + num2,
        "-" => *memory = num1 - num2,
        "*" => *memory = num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Помилка: ділення на нуль.");
                return;
            }
            *memory = num1 / num2;
        },
        _ => {
            println!("Невідома операція.");
            return;
        }
    }
    println!("Результат: {}", *memory);
}

fn rpn_calculator(expr: &str) -> Result<f64, &str> {
    let mut stack = VecDeque::new();

    for token in expr.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop_back().ok_or("Недостатньо операндів")?;
                let a = stack.pop_back().ok_or("Недостатньо операндів")?;
                let result = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => {
                        if b == 0.0 {
                            return Err("Ділення на нуль");
                        }
                        a / b
                    },
                    _ => unreachable!(),
                };
                stack.push_back(result);
            }
            _ => {
                let num: f64 = token.parse().map_err(|_| "Невірний ввод")?;
                stack.push_back(num);
            }
        }
    }

    stack.pop_back().ok_or("Невірний вираз")
}
