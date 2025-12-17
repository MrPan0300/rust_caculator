use std::io;

fn main() {
    let mut value:String = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    println!("the value is:{}", value);
    sort_string(value);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
struct ToKen {
    numbar: f64,
    operator: Operator
}


fn sort_string(value: String) {
    let char_vector:Vec<char> = value.chars().collect();
    let mut ToKens:Vec<ToKen> = Vec::new();
    let mut operators:Operator = Operator::Add;
    let mut num:String = String::new();
    for (i, value) in char_vector.iter().enumerate() {
        if i == 0 {
            if value == &'-' {
                operators = Operator::Subtract;
            }
        }
        match value {
            '+' => {
                let token = ToKen{
                    numbar: num.parse().expect("error string to float"),
                    operator: operators,
                };
                ToKens.push(token);
                num = String::new();
                operators = Operator::Add;        
            },
            '-' => {
                if i != 0 {
                let token = ToKen{
                        numbar: num.parse().expect("error string to float"),
                        operator: operators,
                    };
                ToKens.push(token);
                num = String::new();
                operators = Operator::Subtract;
                }
            },
            '*' => {
                let token = ToKen{
                    numbar: num.parse().expect("error string to float"),
                    operator: operators,
                };
                ToKens.push(token);
                num = String::new();
                operators = Operator::Multiply;
            },
            '/' => {
                let token = ToKen{
                    numbar: num.parse().expect("error string to float"),
                    operator: operators,
                };
                ToKens.push(token);
                num = String::new();
                operators = Operator::Divide;
            }
            _ => num.push_str(&value.to_string()),
        }
        
        if i == char_vector.len() - 1 {
            println!("here");
            println!("{num}");
            let token = ToKen {
                numbar: num.parse().expect("error string to float"),
                operator: operators,
            };
            ToKens.push(token);
        }
        println!("{i}");
    }
    dbg!(ToKens);
}
