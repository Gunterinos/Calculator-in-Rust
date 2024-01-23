use regex::Regex;

fn main() {
    println!("This calculator supports addition(+), subtraction(-), multiplication(*), and division(/)");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    //process the input the given by the user and also check if the input was given correctly
    let processed_input = process_input(&input);

    //take the notation of input from infix to postfix
    let postfix_notation = infix_to_postfix(&processed_input);

    //calculate the input
    let result = calculate_result(&postfix_notation);


    println!("The result to the inputted calculation is: {result}");
}

fn process_input(input: &str) -> Vec<&str> {
    //the pattern is made so we only take input numbers and the designated operators
    let pattern = Regex::new(r"(\d+|[-+*/])").unwrap();
    let result: Vec<&str> = pattern.find_iter(input).map(|m| m.as_str()).collect();

    //check if the operators are put in place correctly
    let pattern_operators = Regex::new(r"[-+*/]").unwrap();
    let mut trigger = false;

    for i in result.iter() {
        if pattern_operators.is_match(i) {
            if trigger {
                panic!("The operators weren't typed correctly in the calculator!");
            }
            trigger = true;
        } else {
            trigger = false;
        }
    }

    if trigger {
        panic!("The operators weren't typed correctly in the calculator!");
    }

    result
}

//this function takes the infix notation to postfix while maintaining the order of operations correctly
fn infix_to_postfix<'a>(input: &'a [&'a str]) -> Vec<&'a str> {
    let mut output: Vec<&str> = Vec::new();
    let mut operators: Vec<&str> = Vec::new();
    let numeric_pattern = Regex::new(r"\d+").unwrap();

    for i in input.iter().rev() {
        if numeric_pattern.is_match(i) {
            output.push(i);
        } else {
            while !operators.is_empty() && order(*i) < order(operators[operators.len() - 1]) {
                output.push(operators.pop().unwrap());
            }
            operators.push(*i);
        }
    }

    while !operators.is_empty() {
        output.push(operators.pop().unwrap());
    }

    output
}

//with the postfix notation we calculate the operation and return the final result
fn calculate_result(input: &[&str]) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    let numeric_pattern = Regex::new(r"\d+").unwrap();

    for i in input.iter() {
        if numeric_pattern.is_match(i) {
            stack.push(i.parse::<i32>().unwrap());
        } else {
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            let res = match *i {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => panic!("Unknown operator."),
            };
            stack.push(res);
        }
    }

    stack.pop().unwrap()
}

// Establishing precedence of operations
fn order(operator: &str) -> i32 {
    match operator {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => 0,
    }
}