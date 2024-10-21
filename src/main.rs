use std::io;
use std::str::FromStr;

// Функція для виконання звичайного калькулятора
fn classic_calculator() {
    let mut memory = 0.0;

    loop {
        println!("\n--- Звичайний калькулятор ---");
        println!("Результат в пам'яті: {}", memory);
        println!("Введіть операцію (+, -, *, /) або 'exit' для виходу:");

        let mut operation = String::new();
        io::stdin().read_line(&mut operation).expect("Помилка при введенні операції");
        let operation = operation.trim();

        if operation == "exit" {
            println!("Програма завершена.");
            break;
        }

        println!("Введіть число:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Помилка при введенні числа");
        let number: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Помилка: введено не число.");
                continue;
            }
        };

        memory = match operation {
            "+" => memory + number,
            "-" => memory - number,
            "*" => memory * number,
            "/" => {
                if number == 0.0 {
                    println!("Помилка: ділення на нуль.");
                    memory
                } else {
                    memory / number
                }
            }
            _ => {
                println!("Помилка: невідома операція.");
                memory
            }
        };

        println!("Результат: {}", memory);
    }
}

// Функція для виконання калькулятора з польською нотацією
fn evaluate_rpn(expression: &str) -> Result<f64, String> {
    let mut stack = Vec::new();

    for token in expression.split_whitespace() {
        if let Ok(num) = f64::from_str(token) {
            // Якщо це число, додаємо його до стека
            stack.push(num);
        } else {
            // Якщо це оператор, витягуємо два числа зі стека
            let b = stack.pop().ok_or("Недостатньо операндів у виразі")?;
            let a = stack.pop().ok_or("Недостатньо операндів у виразі")?;

            let result = match token {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => {
                    if b == 0.0 {
                        return Err("Помилка: ділення на нуль".to_string());
                    }
                    a / b
                },
                _ => return Err(format!("Невідомий оператор: {}", token)),
            };
            // Результат операції додаємо назад до стека
            stack.push(result);
        }
    }

    // Після обчислень у стеку повинно залишитися одне значення
    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Err("Помилка: некоректний вираз".to_string())
    }
}

fn rpn_calculator() {
    println!("Введіть вираз у зворотній польській нотації (наприклад, '5 1 2 + 4 * + 3 -'):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не вдалося прочитати рядок");

    match evaluate_rpn(input.trim()) {
        Ok(result) => println!("Результат: {}", result),
        Err(e) => println!("Помилка: {}", e),
    }
}

fn main() {
    loop {
        println!("\nОберіть режим:");
        println!("1: Звичайний калькулятор");
        println!("2: Калькулятор польської нотації");
        println!("3: Вихід");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Помилка при введенні");

        match choice.trim() {
            "1" => classic_calculator(),
            "2" => rpn_calculator(),
            "3" => {
                println!("Програма завершена.");
                break;
            }
            _ => println!("Невірний вибір, спробуйте ще раз."),
        }
    }
}
