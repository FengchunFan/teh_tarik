use std::env;
use std::fs;
mod interpreter;

fn main() {
    // get commandline arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide an input file.");
        return;
    }

    if args.len() > 2 {
        println!("Too many commandline arguments.");
        return;
    }

    // read the entire file.
    let filename = &args[1];
    let result = fs::read_to_string(filename);
    let code = match result {
    Err(error) => {
        println!("**Error. File \"{}\": {}", filename, error);
        return;
    }

    Ok(code) => {
      code
    } 

    };

    let tokens = match lex(&code) {
    Err(error_message) => {
        println!("**Error**");
        println!("----------------------");
        println!("Lexer Error: {}", error_message);
        println!("----------------------");
        return;
    }

    Ok(tokens) => tokens,
    
    };

    let mut index: usize = 0;
    match parse_program(&tokens, &mut index) {

    Ok(code) => {
        println!("Program Parsed Successfully.");
        println!("--------------------------------------------");
        println!("{code}");
        println!("--------------------------------------------");
        interpreter::execute_ir(&code);
    }

    Err(message) => {
        println!("**Error**");
        println!("----------------------");
        if tokens.len() == 0 {
            println!("No code has been provided.");
        } else {
            println!("Parser Error: {message}");
            println!("----------------------");
        }
    }

    }
}

#[derive(Debug, Clone)]
enum Token {
  Func,
  Return,
  Int,
  Print,
  Read,
  While,
  If,
  Else,
  Break,
  Continue,
  LeftParen,
  RightParen,
  LeftCurly,
  RightCurly,
  LeftBracket,
  RightBracket,
  Comma,
  Semicolon,
  Plus,
  Subtract,
  Multiply,
  Divide,
  Modulus,
  Assign,
  Less,
  LessEqual,
  Greater,
  GreaterEqual,
  Equality,
  NotEqual,
  Num(i32),
  Ident(String),
  End,
}

struct Expression {
  code: String,
  name: String,
}

// This is a lexer that parses numbers/identifiers and math operations
fn lex(code: &str) -> Result<Vec<Token>, String> {
  let bytes = code.as_bytes();
  let mut tokens: Vec<Token> = vec![];

  let mut i = 0;
  while i < bytes.len() {
    let c = bytes[i] as char;

    match c {

    // Digits
    // If characters among digits, return error message
    '0'..='9' => {
      let start = i;
      i += 1;
      while i < bytes.len() {
        let digit = bytes[i] as char;
        if digit >= '0' && digit <= '9' {
          i += 1;
        } else if digit == ' ' || digit == '\n' {
          // If reached here, means all digits until space or newline
          break;
        } else if digit == '(' || digit == ')' || digit == '{' || digit == '}' || digit == '[' || digit == ']'{
          // If reached here, similar logic as above, all digits are legal until meeting a type of bracket
          break;
        } else if digit == ',' || digit == ';'{
          // If reached here, similar logic as above, all digits are legal until meeting a type of comma
          break;
        } else {
          // If current character is a alphabet or any other unrecognized character
          // Return error message
          let end = i+1;
          let string_token = &code[start..end]; 
          return Err(format!("Detect invalid identifier {}", string_token));
        }
      }
      let end = i;
      let string_token = &code[start..end];
      let number_value = string_token.parse::<i32>().unwrap();
      let token = Token::Num(number_value);
      tokens.push(token);
    }
   
    // Characters
    // If character-made string does not match any keywords
    // It is considered a Identifier
    'a'..='z' | 'A'..='Z' => {
      let start = i;
      i += 1;
      while i < bytes.len() {
        let curr = bytes[i] as char;
        // Variables begin with an upper or lower case letters A-Z followed by a sequence of underscores or numbers.
        // reference: https://stackoverflow.com/questions/29873569/check-whether-a-char-is-a-letter-or-a-number
        if curr.is_alphanumeric() || curr == '_'{
          i += 1;
        } else if curr == ' ' || curr == '\n' {
          // If reached here, means all characters are legal until space or newline
          break;
        } else if curr == '(' || curr == ')' || curr == '{' || curr == '}' || curr == '[' || curr == ']'{
          // If reached here, similar logic as above, all characters are legal until meeting a type of bracket
          break;
        } else if curr == ',' || curr == ';'{
          // If reached here, similar logic as above, all characters are legal until meeting a type of comma
          break;
        } else {
          // If current character is a unrecognized, return error message
          let end = i+1;
          let string_token = &code[start..end]; 
          return Err(format!("Detect invalid identifier {}", string_token));
        }
      }
      let end = i;
      let string_token = &code[start..end];
      // Check if this parsed string is a predefined keyword
      match string_token {
        "func" => {
          tokens.push(Token::Func);
        }
        "return" => {
          tokens.push(Token::Return);
        }
        "int" => {
          tokens.push(Token::Int);
        }
        "print" => {
          tokens.push(Token::Print);
        }
        "read" => {
          tokens.push(Token::Read);
        }
        "while" => {
          tokens.push(Token::While);
        }
        "if" => {
          tokens.push(Token::If);
        }
        "else" => {
          tokens.push(Token::Else);
        }
        "break" => {
          tokens.push(Token::Break);
        }
        "continue" => {
          tokens.push(Token::Continue);
        }
        // Else, it is a identifier
        _ => {
          // change &str -> String
          let token = Token::Ident(string_token.to_string());
          tokens.push(token);
        }
      }
    }

    // Simple symbols
    // No need further clarification
    '+' => {
      tokens.push(Token::Plus);
      i += 1;
    }
    '-' => {
      tokens.push(Token::Subtract);
      i += 1;
    }
    '*' => {
      tokens.push(Token::Multiply);
      i += 1;
    }
    '/' => {
      tokens.push(Token::Divide);
      i += 1;
    }
    '(' => {
      tokens.push(Token::LeftParen);
      i += 1;
    }
    ')' => {
      tokens.push(Token::RightParen);
      i += 1;
    }
    '{' => {
      tokens.push(Token::LeftCurly);
      i += 1;
    }
    '}' => {
      tokens.push(Token::RightCurly);
      i += 1;
    }
    '[' => {
      tokens.push(Token::LeftBracket);
      i += 1;
    }
    ']' => {
      tokens.push(Token::RightBracket);
      i += 1;
    }
    ',' => {
      tokens.push(Token::Comma);
      i += 1;
    }
    ';' => {
      tokens.push(Token::Semicolon);
      i += 1;
    }
    '%' => {
      tokens.push(Token::Modulus);
      i += 1;
    }

    // Special symbols
    // Need to check the next character behind the current character
    '>' => {
      i += 1;
      // If this is the end of the string
      if i >= bytes.len(){
        tokens.push(Token::Greater);      
      } else {
        // Insert token based on what is next character
        let curr = bytes[i] as char;
        match curr {
          '=' => {
            tokens.push(Token::GreaterEqual);
            i += 1;
          }
          _ => {
            tokens.push(Token::Greater);
          }
        }
      }
    }
    '<' => {
      i += 1;
      // If this is the end of the string
      if i >= bytes.len(){
        tokens.push(Token::Less);
      } else {
        // Insert token based on what is next character
        let curr = bytes[i] as char;
        match curr {
          '=' => {
            tokens.push(Token::LessEqual);
            i += 1;
          }
          _ => {
            tokens.push(Token::Less);
          }
        }
      }
    }
    '=' => {
      i += 1;
      // If this is the end of the string
      if i >= bytes.len(){
        tokens.push(Token::Assign);
      } else {
        // Insert token based on what is next character
        let curr = bytes[i] as char;
        match curr {
          '=' => {
            tokens.push(Token::Equality);
            i += 1;
          }
          _ => {
            tokens.push(Token::Assign);
          }
        }
      }
    }
    '!' => {
      i += 1;
      // If this is the end of the string
      if i >= bytes.len(){
        // '!' itself is not recognized
        return Err(format!("Unrecognized symbol '!'"));
      } else {
        // Insert token based on what is next character
        let curr = bytes[i] as char;
        match curr {
          '=' => {
            tokens.push(Token::NotEqual);
            i += 1;
          }
          _ => {
            return Err(format!("Unrecognized symbol '!'"));
          }
        }
      }
    }

    // Comment
    // We will ignore all characters following '#' until newline (\n)
    '#' => {
      i += 1;
      while i < bytes.len(){
        let curr = bytes[i] as char;
        if curr != '\n'{
          i += 1;
        } else {
          i += 1;
          break;
        }
      }
    }

    // Space or newline
    ' ' | '\n' => {
      i += 1;
    }
    
    // If other characters encountered
    // Return error message
    _ => {
      return Err(format!("Unrecognized symbol '{}'", c));
    }

    }
  }

  tokens.push(Token::End);
  return Ok(tokens);
}

// Helper function to check if given function name is in function table
fn find_func_name(func_table: &Vec<String>, func_name: &String) -> bool {
    for func_name_in_table in func_table{
        if func_name_in_table.eq(func_name){
            return true;
        }
    }
    return false;
}

// Helper function to check if main function is in the function table
fn has_main(func_table: &Vec<String>) -> bool {
    for func_name_in_table in func_table{
        if func_name_in_table.eq("main"){
            return true;
        }
    }
    return false;
}

// Helper function to check if given symbol is in symbol table
fn find_symbol(symbol_table: &Vec<(String, String)>, symbol: &String) -> bool {
    for (symbol_in_table, _) in symbol_table{
        if symbol_in_table.eq(symbol){
            return true;
        }
    }
    return false;
}

// Helper function to return the type of given symbol
fn find_type(symbol_table: &Vec<(String, String)>, symbol: &String) -> String {
    for (symbol_in_table, symbol_type) in symbol_table{
        if symbol_in_table.eq(symbol){
            return symbol_type.clone();
        }
    }
    // Won't reach here
    panic!("Error: Symbol '{}' not found!", symbol);
}

// parse programs with multiple functions
// loop over everything, outputting generated code.
fn parse_program(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
    assert!(tokens.len() >= 1 && matches!(tokens[tokens.len() - 1], Token::End));

    let mut code = String::new();
    // Function name table
    let mut func_table: Vec<String> = vec![];
    while !at_end(tokens, *index) {
      match parse_function(tokens, index, &mut func_table) {
      Ok(function_code) => {
        code += &function_code;
      }
      Err(e) => { return Err(e); }
      }
    }
    // After the program is done
    // Check if we have the main function
    if !has_main(&func_table){
      return Err(String::from("Main function not detected in the program"));
    }
    return Ok(code);
}

fn at_end(tokens: &Vec<Token>, index: usize) -> bool {
  match tokens[index] {
  Token::End => { true }
  _ => { false }
  }
}

static mut VAR_NUM: i64 = 0;

fn create_temp() -> String {
    unsafe {
        VAR_NUM += 1;
        format!("_temp{}", VAR_NUM)
    }
}


// parse function such as:
// func main(int a, int b) {
//    # ... statements here...
//    # ...
// }
// a loop is done to handle statements.

fn parse_function(tokens: &Vec<Token>, index: &mut usize, func_table: &mut Vec<String>) -> Result<String, String> {
    
    match tokens[*index] {
    Token::Func => { *index += 1; }
    _ => { return Err(String::from("functions must begin with func")); }
    }

    let mut function_code: String;
    //let mut symbol_table: Vec<String> = vec![];
    let mut symbol_table: Vec<(String, String)> = vec![];

    match &tokens[*index] {
    Token::Ident(ident) => {
        *index += 1;
        // if identifier has been previously declared, return error
        if find_func_name(&func_table, ident){
            return Err(format!("Found duplicating function name {ident}"));
        }
        func_table.push(ident.clone());
        function_code = format!("%func {ident}");
    }
    _  => { return Err(String::from("functions must have a function identifier"));}
    }


    match tokens[*index] {
    Token::LeftParen => { *index += 1; }
    _ => { return Err(String::from("expected '('"));}
    }

    // If there is a declaration in the function
    // Then we go into the loop
    while !matches!(tokens[*index], Token::RightParen) {
      // We need to first match an declaration
      match parse_declaration(tokens, index, &mut symbol_table) {
        Ok(statement) => {
          function_code += "(";
          function_code += &statement;
        }
        Err(e) => {return Err(e);}
      }
      // While there is Comma
      while matches!(tokens[*index], Token::Comma) {
        // We pass forward from the Comma and check one additional declaration
        function_code += ", ";
        *index += 1;
        match parse_declaration(tokens, index, &mut symbol_table) {
          Ok(statement) => {
            function_code += &statement;
          }
          Err(e) => {return Err(e);}
        }
      }

      // When there are no more parameters
      function_code += ")";
      break;
    }

    match tokens[*index] {
    Token::RightParen => { 
      *index += 1; 
      function_code += "\n";
    }
    _ => { return Err(String::from("expected ')'"));}
    }

    match tokens[*index] {
    Token::LeftCurly => { *index += 1; }
    _ => { return Err(String::from("expected '{'"));}
    }

    while !matches!(tokens[*index], Token::RightCurly) {
        match parse_statement(tokens, index, &mut symbol_table, func_table) {
        Ok(statement_code) => {
          // Each statement should contain a newline itself
          function_code += &statement_code;
        }
        Err(e) => {return Err(e);}
        }
    }


    match tokens[*index] {
    Token::RightCurly => { *index += 1; }
    _ => { return Err(String::from("expected '}'"));}
    }

    function_code += "%endfunc\n";
    return Ok(function_code);
}

// parsing a statement such as:
// int a;
// a = a + b;
// a = a % b;
// print(a)
// read(a)
// returns epsilon if '}'
fn parse_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<(String, String)>, func_table: &mut Vec<String>) -> Result<String, String> {
    match tokens[*index] {
    Token::Int => parse_declaration_statement(tokens, index, symbol_table),
    Token::Ident(_) => parse_assignment_statement(tokens, index, symbol_table, func_table),
    Token::Return => parse_return_statement(tokens, index),
    Token::Print => parse_print_statement(tokens, index),
    Token::Read => parse_read_statement(tokens, index),
    // Token::Break => parse_break_statement(tokens, index),
    // Token::Continue => parse_continue_statement(tokens, index),
    _ => Err(String::from("invalid statement"))
    }
}

// In this phase, we do not pass in array as parameter
fn parse_declaration(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<(String, String)>) -> Result<String, String> {

    let statement: String;

    match tokens[*index] {
    Token::Int => {*index += 1;}
    _ => {return Err(String::from("Declaration statements must being with 'int' keyword"));}
    }

    match &tokens[*index] {
    Token::Ident(ident) => {
        // if identifier has been previously declared, return error
        if find_symbol(&symbol_table, ident){
            return Err(format!("Found duplicating declaration inside function {ident}"));
        }
        symbol_table.push((ident.clone(), "int".to_string()));
        *index += 1;
        statement = format!("%int {ident}");
    }
    _ => {return Err(String::from("Declarations must have an identifier"));}
    }

    return Ok(statement);
}

fn parse_declaration_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<(String, String)>) -> Result<String, String> {

    // Since there are multiple ways statement might be assigned
    let mut statement = String::new();
    //let statement: String;

    match tokens[*index] {
    Token::Int => {*index += 1;}
    _ => {return Err(String::from("Declaration statements must being with 'int' keyword"));}
    }

    // Boolean to check if we are dealing with variable
    let mut is_variable = true;
    // Variable to store array size if applicable
    // let mut number = 0;

    // If it is an identifier, we can just pass forward
    // add code that handles declarations such as int [8] arr
    while !matches!(tokens[*index], Token::Ident(_)) {
      is_variable = false;
      match tokens[*index] {
        Token::LeftBracket => {*index += 1;}
        _ => {return Err(String::from("expected '['"));}
      }

      match tokens[*index] {
        // If it is right bracket, we are missing the array size
        Token::RightBracket => {
          return Err(String::from("missing array size"));
        }
        // If it is number, we need to check if it is followed by an right bracket, which is same check point
        Token::Num(num) => {
          // Array size must not be less or equal to 0
          if num <= 0 {
            return Err(String::from("array size is less or equal to 0"));
          } else {
            *index += 1;
            // number = num;
            statement = format!("%int[] array, {num}\n");
          }
        }
        _ => {return Err(String::from("expected ']' or number behind '['"));}
      }

      match tokens[*index] {
        Token::RightBracket => {*index += 1;}
        _ => {return Err(String::from("expected ']'"));}
      }

    }

    match &tokens[*index] {
    Token::Ident(ident) => {
        *index += 1;
        // if identifier has been previously declared, return error
        if find_symbol(&symbol_table, ident){
            return Err(format!("Found duplicating declaration inside statement {ident}"));
        }
        if is_variable {
          statement = format!("%int {ident}\n");
          symbol_table.push((ident.clone(), "int".to_string()));
        } else {
          symbol_table.push((ident.clone(), "array".to_string()));
        }
    }
    _ => {return Err(String::from("Declarations must have an identifier"));}
    }

    match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statements must end with a semicolon"));}
    }

    return Ok(statement);
}

fn parse_var(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
  // initialize empty expression
  let mut var_statement = Expression {
      name: String::new(),
      code: String::new(),
  };
  let dest: String;

  match &tokens[*index] {
    // Start of var must be an identifier
    Token::Ident(ident) => {
      *index += 1;
      // Get a copy of the identifier
      dest = ident.clone();
      match tokens[*index] {
        // Under Identifier, if it follows a left bracket
        // Ident -> [ Expression ] ...
        Token::LeftBracket => {
          *index += 1;

          match parse_expression(tokens, index) {
            Ok(expression) => {
                let src = expression.name;
                // Append the previous expression code first
                var_statement.code = expression.code;
                var_statement.name = format!("[array + {}]", src);
            },
            Err(e) => {return Err(e);}
          }

          match tokens[*index] {
            Token::RightBracket => {*index += 1;}
            _ => { return Err(String::from("var missing right bracket ']'")); }
          }
        }
        
        // If we see other characters, that is not part of this var
        // Identifier itself is a valid var
        _ => {
          // Take the identifier as name
          // Leave the code field empty
          var_statement.name = dest;
        }
      }      
      return Ok(var_statement);
    }
    
    // Else, it idicates a missing identifier in var
    _ => {
      return Err(String::from("missing identifier in var"));
    }

  }
}

fn parse_assignment_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<(String, String)>, func_table: &mut Vec<String>) -> Result<String, String> {
    let mut statement = String::new();

    let dest: String;
    // Get the dest (left part)
    match parse_var(tokens, index) {
      Ok(var_statement) => {
        // This is because expression is involved in parse_var
        statement += &var_statement.code;
        dest = var_statement.name;
        // Check if variable has been declared
        // Extract symbol name
        let dest_name: String;
        if dest.contains('[') {
            // In our program, by the template, the array will all be under symbol name: array
            // Else, the interpreter cannot recognize
            dest_name = "array".to_string();
        } else {
            dest_name = dest.clone();
        };
        if !find_symbol(&symbol_table, &dest_name){
          return Err(format!("Variable {dest} called before declaration"));
        }
      },
      Err(e) => {return Err(e);}
    }

    match tokens[*index] {
    Token::Assign => {*index += 1;}
    _ => {return Err(String::from("Statement is missing the '=' operator"));}
    }

    match parse_expression(tokens, index) {
    Ok(expression) => {
        let src = expression.name;
        statement += &expression.code;

        // Check if both dest and src are in the symbol table
        if find_symbol(&symbol_table, &dest) && find_symbol(&symbol_table, &src){
          // Fetch their type from the symbol table
          let dest_type = find_type(symbol_table, &dest);
          let src_type = find_type(symbol_table, &src);

          if dest_type != src_type {
            return Err(format!("Type mismatch between {} and {}", dest, src));
          }         
        }

        // Continue only when type matches
        // Check if the expression is a function call
        if src.contains("(") && src.contains(")") {
          // Extract function name (substring before the first '(')
          let function_name = src.split('(').next().unwrap().to_string(); 
          // Check if function has been declared
          if !find_func_name(&func_table, &function_name){
            return Err(format!("Function {function_name} called before declaration"));
          }
          statement += &format!("%call {dest}, {src}\n");
        } else {
          statement += &format!("%mov {dest}, {src}\n");
        }
    },
    Err(e) => {return Err(e);}
    }

    match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statement is missing the ';' semicolon"));}
    }

    return Ok(statement);
}

fn parse_return_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
    let expression: Expression;
    match tokens[*index] {
    Token::Return => {*index += 1;}
    _ => {return Err(String::from("Return statements must being with a return keyword"));}
    }

    match parse_expression(tokens, index) {
    Ok(expr) => {
      expression = expr;
    },
    Err(e) => {return Err(e);}
    }

    match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statement is missing the ';' semicolon"));}
    }

    let mut statement = expression.code;
    statement += &format!("%ret {}\n", expression.name);
    return Ok(statement);
}

fn parse_print_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
    let expression: Expression;
    match tokens[*index] {
    Token::Print=> {*index += 1;}
    _ => {return Err(String::from("Print statements must being with a return keyword"));}
    }

    match parse_expression(tokens, index) {
    Ok(expr) => {
        expression = expr;
    },
    Err(e) => {return Err(e);}
    }

    match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statement is missing the ';' semicolon"));}
    }

    let mut statement = expression.code;
    let src1 = expression.name;
    let dest = create_temp();
    statement += &format!("%int {dest}\n");
    statement += &format!("%mov {}, {}\n", dest, src1);
    statement += &format!("%out {dest}\n");
    return Ok(statement);
}

fn parse_read_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
    let expression: Expression;
    match tokens[*index] {
    Token::Read => {*index += 1;}
    _ => {return Err(String::from("Read statements must being with a 'read' keyword"));}
    }

    match parse_expression(tokens, index) {
    Ok(expr) => {
      expression = expr;
    },
    Err(e) => {return Err(e);}
    }
    match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statement is missing the ';' semicolon"));}
    }

    let mut statement = expression.code;
    statement += &format!("%input {}\n", expression.name);
    return Ok(statement);
}

// parsing complex expressions such as: "a + b - (c * d) / (f + g - 8);
fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    let mut expression: Expression;
    match parse_multiply_expression(tokens, index) {
    Ok(expr) => {
        expression = expr;
    },
    Err(e) => {return Err(e);}
    }
    loop {
       match tokens[*index] {

       Token::Plus => {
           *index += 1;
           match parse_multiply_expression(tokens, index) {
           Ok(expr2) => {
               let src1 = expression.name;
               let src2 = expr2.name;
               let dest = create_temp();
               expression.code += &expr2.code;
               expression.code += &format!("%int {dest}\n");
               expression.code += &format!("%add {dest}, {src1}, {src2}\n");
               expression.name = dest;
           },
           Err(e) => {return Err(e);}

           }
       }

       Token::Subtract => {
           *index += 1;
           match parse_multiply_expression(tokens, index) {
           Ok(expr2) => {
               let src1 = expression.name;
               let src2 = expr2.name;
               let dest = create_temp();
               expression.code += &expr2.code;
               expression.code += &format!("%int {dest}\n");
               expression.code += &format!("%sub {dest}, {src1}, {src2}\n");
               expression.name = dest;
           },
           Err(e) => {return Err(e);}

           }
       }

       _ => { 
           break;
       }

       };
    }

    return Ok(expression);
}

fn parse_multiply_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    let mut expression: Expression;
    match parse_term(tokens, index) {
    Ok(expr) => {
        expression = expr;
    },
    Err(e) => {return Err(e);}
    }
    loop {
       match tokens[*index] {
       Token::Multiply => {
          *index += 1;
          match parse_term(tokens, index) {
          Ok(expr2) => {
              let src1 = expression.name;
              let src2 = expr2.name;
              let dest = create_temp();
              expression.code += &expr2.code;
              expression.code += &format!("%int {dest}\n");
              expression.code += &format!("%mult {dest}, {src1}, {src2}\n");
              expression.name = dest;
          },
          Err(e) => {return Err(e);}
          }
       }

       Token::Divide => {
          *index += 1;
          match parse_term(tokens, index) {
          Ok(expr2) => {
              let src1 = expression.name;
              let src2 = expr2.name;
              let dest = create_temp();
              expression.code += &expr2.code;
              expression.code += &format!("%int {dest}\n");
              expression.code += &format!("%div {dest}, {src1}, {src2}\n");
              expression.name = dest;
          },
          Err(e) => {return Err(e);}
          }
       }

       Token::Modulus => {
          *index += 1;
          match parse_term(tokens, index) {
          Ok(expr2) => {
              let src1 = expression.name;
              let src2 = expr2.name;
              let dest = create_temp();
              expression.code += &expr2.code;
              expression.code += &format!("%int {dest}\n");
              expression.code += &format!("%mod {dest}, {src1}, {src2}\n");
              expression.name = dest;
          },
          Err(e) => {return Err(e);}
          }
       }
  
       _ => {
           break;
       }

       };

    }

    return Ok(expression);
}

// a term is either a Number or an Identifier.
fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    match &tokens[*index] {

    // If just a number, return immediately
    Token::Num(number) => {
        *index += 1;
        let expression = Expression {
            code : String::from(""),
            name : number.to_string()
        };
        return Ok(expression);
    }

    // If it is identifier
    Token::Ident(identifier) => {
      *index += 1;
      let mut expression = Expression {
        code: String::new(),
        name: identifier.clone(),
      };
      match tokens[*index] {
        // Under Identifier, if it follows a left bracket
        // Ident -> [ Expression ] ...
        Token::LeftBracket => {
          *index += 1;
            
          // Using temp here to avoid %add _temp4, [array + 0], [array + 0]
          match parse_expression(tokens, index) {
            Ok(inner_expr) => {
              let src1 = format!("[{} + {}]", identifier, inner_expr.name);
              let dest = create_temp();
              expression.code += &format!("%int {dest}\n");
              expression.code += &format!("%mov {}, {}\n", dest, src1);
              expression.name = dest;
            },
            Err(e) => {return Err(e);}
          }

           match tokens[*index] {
            Token::RightBracket => {*index += 1;}
            _ => { return Err(String::from("term missing right bracket ']'")); }
          }
          
        }
        // Under Identifier, if it follows a left Parenthesis
        // Identifier (Expression(, Expression)*)
        Token::LeftParen => {
          // We have start with a left parenthesis and a expression
          *index += 1;
          // Collect function arguments.
          let mut args = Vec::new();

          // Push the collected argument
          match parse_expression(tokens, index) {
            Ok(inner_expr) => {
              expression.code += &inner_expr.code;
              args.push(inner_expr.name);
            },
            Err(e) => {return Err(e);}
          }
          
          // If there are more expressions between parenthesis
          // It must start with a comma
          // Else, it will be checked and throw an error in the next check point
          while matches!(tokens[*index], Token::Comma) {
            *index += 1;
            // Then we can parse another expressino
            match parse_expression(tokens, index) {
              Ok(inner_expr) => {
                expression.code += &inner_expr.code;
                args.push(inner_expr.name);
              },
              Err(e) => {return Err(e);}
            }
          }

          match tokens[*index] {
            Token::RightParen => {*index += 1;}
            _ => { return Err(String::from("term missing right parenthesis ')'")); }
          }

          // Format the expression name
          expression.name = format!("{}({})", identifier, args.join(","));
        }
        // If we see other characters, that is not part of this term
        // Identifier itself is a valid term
        _ => {
          // Do not do anything
        }
      }      
      return Ok(expression);
    }

    // ( Expression )
    Token::LeftParen => {
        *index += 1;
        let expression: Expression;
        match parse_expression(tokens, index) {
        Ok(e) => {expression = e;},
        Err(e) => {return Err(e);}
        }

        match tokens[*index] {
        Token::RightParen => {*index += 1;}
        _ => { return Err(String::from("missing right parenthesis ')'")); }
        }
        return Ok(expression);
    }
    
    // Missing term
    _ => {
        return Err(String::from("missing expression term."));
    }

    }
}