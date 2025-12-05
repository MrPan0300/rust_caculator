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

struct ToKen {
    numbar: f64,
    operator: Operator
}


fn sort_string(value: String) {
    let char_vector:Vec<char> = value.chars().collect();
    dbg!(char_vector);
    let mut ToKens:Vec<ToKen> = Vec::new();
    let mut TokenStruct:ToKen;
    let mut operator:Operator;
    let mut num:String = String::new();
    for (i, value) in char_vector.iter().enumerate() {
        if i == 0 {
            if value == '-' {
                operator = Operator::Subtract;
            }else {
                operator = Operator::Add;
            }
        }
        match value {
            value: 'v' => 
        }
    
    }
}

