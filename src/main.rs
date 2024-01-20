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
    // let reversed_infix = result.reverse();

    // let mut output: Vec<&str> = Vec::new();
    // let mut operators: Vec<&str> = Vec::new();

    // let mut numeric_pattern = Regex::new(r"\d+").unwrap();

    // for i in reversed_infix.iter(){
    //     if numeric_pattern.is_match(i){
    //         output.push(i);
    //     } else {
            
    //     }
    // }
}
