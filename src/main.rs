use regex::Regex;

fn main() {
    println!("This calculator supports addition(+), subtraction(-), multiplication(*), and division.(/)");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    //diving the string into numbers and operators
    let pattern = Regex::new(r"(\d+|[-+*/])").unwrap();
    let result: Vec<&str> = pattern.find_iter(&input).map(|m| m.as_str()).collect();

    //verify if the operators are correctly in place
    let pattern_operators = Regex::new(r"[-+*/]").unwrap();
    let mut trigger = false;

    for i in result.iter(){
        if pattern_operators.is_match(i){
            if trigger == true{
                panic!("The operators weren't typed correctly in the calculator!");
            }
            trigger = true;
        } else {
            trigger = false;
        }
    }

    //this checks if there is an operator on the last position
    if trigger == true{
        panic!("The operators weren't typed correctly in the calculator!");
    }

    //infix to prefix to parse through the calculation and have the correct order of operations
    let mut output: Vec<&str> = Vec::new();
    let mut operators: Vec<&str> = Vec::new();

    let numeric_pattern = Regex::new(r"\d+").unwrap();

    //here I put the infix to postfix while maintaining the order of operations
    for i in result.iter().rev(){
        if numeric_pattern.is_match(i){
            output.push(i);
        } else {
            while !operators.is_empty() 
                && (order(*i) < order(operators[operators.len() - 1]))
                {
                    output.push(operators.pop().unwrap());
                }
            operators.push(*i);
        }
    }

    //I pop the remaining operators from the stack to the output
    while !operators.is_empty(){
        output.push(operators.pop().unwrap());
    }

    let mut stack: Vec<_> = vec![];
    
    for i in output.iter() {
        if numeric_pattern.is_match(i){
            stack.push(i.parse::<i32>().unwrap());
        } else {
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            let res = match *i{
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => panic!("Unknown operator."),
            };
            stack.push(res);
        }
    }

    println!("{}", stack.pop().unwrap());
}

// establishing precedence of operations
fn order(operator: &str) -> i32{
    match operator{
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => 0,
    }
}