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
                match expression_calculator(&mut memory) {
                    Ok(result) => {
                        memory = result;
                        println!("Результат: {}", result);
                    }
                    Err(e) => {
                        println!("Помилка: {}", e);
                    }
                }            }
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

fn expression_calculator(memory: &mut f64) -> Result<f64, String> {
    println!("Введіть математичний вираз (наприклад, (10-8)*3):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не вдалося прочитати рядок");

    let expr = input.trim();

    // Підтримка використання попереднього результату через 'm'
    let expr = expr.replace("m", &memory.to_string());

    evaluate_expression(&expr)
}

fn evaluate_expression(expr: &str) -> Result<f64, String> {
    // Видаляємо пробіли
    let expr = expr.replace(' ', "");

    // Спрощена функція для обчислення виразів
    parse_expression(&expr)
}

fn parse_expression(expr: &str) -> Result<f64, String> {
    parse_addition_subtraction(expr)
}

fn parse_addition_subtraction(expr: &str) -> Result<f64, String> {
    let mut current_result = parse_multiplication_division(get_first_term(expr))?;
    let mut remaining = get_remaining_expr(expr);

    while !remaining.is_empty() {
        let (op, next_term) = split_first_operation(remaining);
        match op {
            '+' => {
                let next_val = parse_multiplication_division(next_term)?;
                current_result += next_val;
            }
            '-' => {
                let next_val = parse_multiplication_division(next_term)?;
                current_result -= next_val;
            }
            _ => break
        }
        remaining = get_remaining_expr(remaining);
    }

    Ok(current_result)
}

fn parse_multiplication_division(expr: &str) -> Result<f64, String> {
    let mut current_result = parse_parentheses(get_first_term(expr))?;
    let mut remaining = get_remaining_expr(expr);

    while !remaining.is_empty() {
        let (op, next_term) = split_first_operation(remaining);
        match op {
            '*' => {
                let next_val = parse_parentheses(next_term)?;
                current_result *= next_val;
            }
            '/' => {
                let next_val = parse_parentheses(next_term)?;
                if next_val == 0.0 {
                    return Err("Ділення на нуль".to_string());
                }
                current_result /= next_val;
            }
            _ => break
        }
        remaining = get_remaining_expr(remaining);
    }

    Ok(current_result)
}

fn parse_parentheses(expr: &str) -> Result<f64, String> {
    if expr.starts_with('(') && expr.ends_with(')') {
        // Видаляємо зовнішні дужки
        let inner = &expr[1..expr.len()-1];
        parse_expression(inner)
    } else {
        // Пряме перетворення числа
        expr.parse().map_err(|_| format!("Невірне число: {}", expr))
    }
}

fn get_first_term(expr: &str) -> &str {
    // Знаходимо перший терм до першої операції або повертаємо весь вираз
    for (i, ch) in expr.chars().enumerate() {
        if ch == '+' || ch == '-' || ch == '*' || ch == '/' {
            return &expr[..i];
        }
    }
    expr
}

fn get_remaining_expr(expr: &str) -> &str {
    // Знаходимо вираз після першої операції
    for (i, ch) in expr.chars().enumerate() {
        if ch == '+' || ch == '-' || ch == '*' || ch == '/' {
            return &expr[i..];
        }
    }
    ""
}

fn split_first_operation(expr: &str) -> (char, &str) {
    // Повертає першу операцію та наступний терм
    for (i, ch) in expr.chars().enumerate() {
        if ch == '+' || ch == '-' || ch == '*' || ch == '/' {
            return (ch, &expr[i+1..]);
        }
    }
    ('+', expr)
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
