// The Rust Programming Language: A Crash Course and Building Our First Lexer
// CS152 Compiler Design using the Rust Programming Language.
// A Handwritten Compiler Using Rust.
// Creating a Lexer By Hand.

// used to get the commandline arguments from the commandline.
use std::env;
// used to interact with the file system
use std::fs;

fn main() {

    // Let us get commandline arguments and store them in a Vec<String>
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide an input file through the commandline arguments for the lexer.");
        return;
    }

    if args.len() > 2 {
        println!("Too many commandline arguments.");
        return;
    }

    // read the entire file contents, storing them inside 'code' as a string.
    let filename = &args[1];
    let code = match fs::read_to_string(filename) {
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

    Ok(data) => data,
    
    };

    // print out the lexer tokens parsed.

    println!("----------------------");
    println!("Finished Lexing the file {}", filename);
    println!("File Contents:");
    println!("{code}");
    println!("Here are the Results:");
    println!("----------------------");
    for t in &tokens {
      println!("{:?}", t);
    }

    // phase2 code in main
    let mut index: usize = 0;
    match parse_program(&tokens, &mut index) {
    
    Ok(()) => {
      println!("Program Parsed Successfully.")
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
    //end of phase2 code in main
}

// Creating an Enum within Rust.
// Documentation: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// Enums are a way of saying a value is one of a possible set of values.
// Unlike C, Rust enums can have values associated with that particular enum value.
// for example, a Num has a 'i32' value associated with it, 
// but Plus, Subtract, Multiply, etc. have no values associated with it.
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

// In Rust, you can model the function behavior using the type system.
// https://doc.rust-lang.org/std/result/
// Result < Vec<Token>, String>
// means that this function can either return:
// - A list of tokens as a Vec<Token>
// - Or an error message represented as a string
// If there is an error, it will return an error
// If successful, it will return Vec<Token>
// A Result is an enum like this:
// enum Result {
//     Ok(the_result),
//     Err(the_error),
// }


// This is a lexer that parses numbers and math operations
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

fn parse_program(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  assert!(tokens.len() >= 1 && matches!(tokens[tokens.len() - 1], Token::End));
  while !at_end(tokens, *index) {
    match parse_function(tokens, index) {
    Ok(()) => {}
    Err(e) => { return Err(e); }
    }
  }
  return Ok(());
}

fn at_end(tokens: &Vec<Token>, index: usize) -> bool {
  match tokens[index] {
    Token::End => { true }
    _ => { false }
  }
}

// parse function such as:
// func main(int a, int b) {
//    # ... statements here...
//    # ...
// }
// a loop is done to handle statements

fn parse_function(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {

  match tokens[*index] {
    Token::Func => { *index += 1; }
    _ => { return Err(String::from("functions must begin with func")); }
  }

  match tokens[*index] {
    Token::Ident(_) => { *index += 1; }
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
    match parse_declaration(tokens, index) {
      Ok(()) => {}
      Err(e) => {return Err(e);}
    }
    // While there is Comma
    while matches!(tokens[*index], Token::Comma) {
      // We pass forward from the Comma and check one additional declaration
      *index += 1;
      match parse_declaration(tokens, index) {
        Ok(()) => {}
        Err(e) => {return Err(e);}
      }
    }
    break;
  }

  match tokens[*index] {
    Token::RightParen => { *index += 1; }
    _ => { return Err(String::from("expected ')'"));}
  }

  match tokens[*index] {
    Token::LeftCurly => { *index += 1; }
    _ => { return Err(String::from("expected '{'"));}
  }

  while !matches!(tokens[*index], Token::RightCurly) {

    match parse_statement(tokens, index) {
      Ok(()) => {}
      Err(e) => {return Err(e);}
    }
  }

  match tokens[*index] {
    Token::RightCurly => { *index += 1; }
    _ => { return Err(String::from("expected '}'")); }
  }

  return Ok(());
}

//parsing a statement such as:
// int a;
// a = a + b;
// a = a % b;
// print(a)
// read(b)
// returns epsilon if '}'
fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
  Token::Int => parse_declaration_statement(tokens, index),
  Token::Ident(_) => parse_assignment_statement(tokens, index),
  Token::Return => parse_return_statement(tokens, index),
  Token::Print => parse_print_statement(tokens, index),
  Token::Read => parse_read_statement(tokens, index),
  Token::Break => parse_break_statement(tokens, index),
  Token::Continue => parse_continue_statement(tokens, index),
  Token::While => parse_while_statement(tokens, index),
  Token::If => parse_if_statement(tokens, index),
  _ => Err(String::from("invalid or empty statement"))
  }
}

//this function handles the declaration of a function's parameters
fn parse_declaration(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::Int => {*index += 1;}
    _ => {return Err(String::from("Declaration statements must being with 'int' keyword"));}
  }

  //If it is an identifier, we can just pass forward
  //add code that handles declarations such as int [8] arr
  while !matches!(tokens[*index], Token::Ident(_)) {

    match tokens[*index] {
      Token::LeftBracket => {*index += 1;}
      _ => {return Err(String::from("expected '['"));}
    }

    match tokens[*index] {
      // If it is right bracket, we do not do anything and pass to next check point
      Token::RightBracket => {}
      // If it is number, we need to check if it is followed by an right bracket, which is same check point
      Token::Num(_) => {*index += 1;}
      _ => {return Err(String::from("expected ']' or number behind '['"));}
    }

    match tokens[*index] {
      Token::RightBracket => {*index += 1;}
      _ => {return Err(String::from("expected ']'"));}
    }

  }

  match tokens[*index] {
    Token::Ident(_) => {*index += 1;}
    _ => {return Err(String::from("Declarations must have an identifier"));}
  }

  return Ok(());
}

//this function handles declarations inside the body of the function
fn parse_declaration_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::Int => {*index += 1;}
    _ => {return Err(String::from("Declaration statements must being with 'int' keyword"));}
  }

  //add code that handles declarations such as int [8] arr
  while !matches!(tokens[*index], Token::Ident(_)) {

    match tokens[*index] {
      Token::LeftBracket => {*index += 1;}
      _ => {return Err(String::from("expected '['"));}
    }

    match tokens[*index] {
      Token::RightBracket => {}
      Token::Num(_) => {*index += 1;}
      _ => {return Err(String::from("expected ']' or number behind '['"));}
    }

    match tokens[*index] {
      Token::RightBracket => {*index += 1;}
      _ => {return Err(String::from("expected ']'"));}
    }

  }

  match tokens[*index] {
    Token::Ident(_) => {*index += 1;}
    _ => {return Err(String::from("Declarations must have an identifier"));}
  }

  match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statements must end with a semicolon"));}
  }

  return Ok(());
}

fn parse_var(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    // Start of var must be an identifier identifier
    Token::Ident(_) => {
      *index += 1;
      match tokens[*index] {
        // Under Identifier, if it follows a left bracket
        // Ident -> [ Expression ] ...
        Token::LeftBracket => {
          loop {
            match tokens[*index] {
              Token::LeftBracket => {*index += 1;}
              // Break and return if we dont find left bracket
              _ => { break; }
            }

            match parse_expression(tokens, index) {
              Ok(()) => {},
              Err(e) => {return Err(e);}
            }

            match tokens[*index] {
              Token::RightBracket => {*index += 1;}
              _ => { return Err(String::from("var missing left bracket ']'")); }
            }
          }
        }
        
        // If we see other characters, that is not part of this var
        // Identifier itself is a valid var
        _ => {
          // Do not do anything
        }
      }      
      return Ok(());
    }
    
    // Else, it idicates a missing identifier in var
    _ => {
      return Err(String::from("missing identifier in var"));
    }

  }
}

// Var = Expression
fn parse_assignment_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  //I might need to change this to match variable_expression instead of an ident(_)
  match parse_var(tokens, index) {
    Ok(()) => {},
    Err(e) => {return Err(e);}
  }

  match tokens[*index] {
    Token::Assign => {*index += 1;}
    _ => {return Err(String::from("Statement is missing the '=' operator"));}
  }

  match parse_expression(tokens, index) {
    Ok(()) => {},
    Err(e) => {return Err(e);}
  }

  match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statement is missing the ';' operator"));}
  }

  return Ok(());
}

fn parse_if_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::If => {*index += 1;}
    _ => {return Err(String::from("If statements must begin with a if keyword"));}
  }

  // In if statement, we do not take in paren
  // code that handles boolean expressions
  match parse_boolean_expression(tokens, index) {
    Ok(()) => {}
    Err(e) => {return Err(e);}
  }

  match tokens[*index] {
    Token::LeftCurly => {*index += 1;}
    _ => {return Err(String::from("Expected '{' under if statement"))}
  }

  // We need at least 1 statement inside the Curly
  //code that handles statements
  match parse_statement(tokens, index) {
    Ok (()) => {}
    Err(e) => {return Err(e);}
  }

  while !matches!(tokens[*index], Token::RightCurly) {
    match parse_statement(tokens, index) {
      Ok(()) => {}
      Err(e) => {return Err(e);}
    }
  }

  match tokens[*index] {
    Token::RightCurly => {*index += 1;}
    _ => {return Err(String::from("Expected '}' under if statemetn"))}
  }

  // While there is a Else keyword
  while matches!(tokens[*index], Token::Else) {
    *index += 1;
    // We check if there is {statement*}
    match tokens[*index] {
      Token::LeftCurly => {*index += 1;}
      _ => {return Err(String::from("Expected '{' under else statement"))}
    }

    // We need at least 1 statement inside the Curly
    //code that handles statements
    match parse_statement(tokens, index) {
      Ok (()) => {}
      Err(e) => {return Err(e);}
    }

    while !matches!(tokens[*index], Token::RightCurly) {
      match parse_statement(tokens, index) {
        Ok(()) => {}
        Err(e) => {return Err(e);}
      }
    }

    match tokens[*index] {
      Token::RightCurly => {*index += 1;}
      _ => {return Err(String::from("Expected '}' under else statement"))}
    }
  }


  return Ok(());
}

fn parse_while_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::While => {*index += 1;}
    _ => {return Err(String::from("While statements must begin with a while keyword"));}
  }

  // In while loop, we do not take in paren
  // code that handles boolean expressions
  match parse_boolean_expression(tokens, index) {
    Ok(()) => {}
    Err(e) => {return Err(e);}
  }

  match tokens[*index] {
    Token::LeftCurly => {*index += 1;}
    _ => {return Err(String::from("Expected '{' under while statement"))}
  }

  // We need at least 1 statement inside the while loop
  //code that handles statements
  match parse_statement(tokens, index) {
    Ok (()) => {}
    Err(e) => {return Err(e);}
  }

  while !matches!(tokens[*index], Token::RightCurly) {
    match parse_statement(tokens, index) {
      Ok(()) => {}
      Err(e) => {return Err(e);}
    }
  }

  match tokens[*index] {
    Token::RightCurly => {*index += 1;}
    _ => {return Err(String::from("Expected '}' under while statement"))}
  }

  return Ok(());

}

fn parse_boolean_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match parse_expression(tokens, index) {
    Ok(()) => {}
    Err(e) => {return Err(e);}
  }

  //code that will handle <, <=, ==, !=, >=, >
  match tokens[*index] {
    Token::Less => {
      *index += 1;
      match parse_expression(tokens, index) {
        Ok(()) => {},
        Err(e) => {return Err(e);}
      }
    }

    Token::LessEqual => {
      *index += 1;
      match parse_expression(tokens, index) {
        Ok(()) => {},
        Err(e) => {return Err(e);}
      }
    }

    Token::Equality => {
      *index += 1;
      match parse_expression(tokens, index) {
        Ok(()) => {},
        Err(e) => {return Err(e);}
      }
    }

    Token::NotEqual => {
      *index += 1;
      match parse_expression(tokens, index) {
        Ok(()) => {},
        Err(e) => {return Err(e);}
      }
    }

    Token::GreaterEqual => {
      *index += 1;
      match parse_expression(tokens, index) {
        Ok(()) => {},
        Err(e) => {return Err(e);}
      }
    }

    Token::Greater => {
      *index += 1;
      match parse_expression(tokens, index) {
        Ok(()) => {},
        Err(e) => {return Err(e);}
      }
    }

    _ => {return Err(String::from("In Boolean Expression, expecting relational operators after expression"));}

  };

  // match parse_expression(tokens, index) {
  //   Ok(()) => {}
  //   Err(e) => {return Err(e);}
  // }

  return Ok(());
}

fn parse_return_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::Return => {*index += 1;}
    _ => {return Err(String::from("Return statements must begin with a return keyword"));}
  }

  match parse_expression(tokens, index) {
    Ok(()) => {},
    Err(e) => {return Err(e);}
  }

  match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statement is missing the ';' operator"));}
  }

  return Ok(());
}

fn parse_print_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::Print=> {*index += 1;}
    _ => {return Err(String::from("Return statements must being with a return keyword"));}
  }

  match parse_expression(tokens, index) {
    Ok(()) => {},
    Err(e) => {return Err(e);}
  }

  match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statement is missing the ';' operator"));}
  }

  return Ok(());
}

fn parse_read_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::Read => {*index += 1;}
    _ => {return Err(String::from("Return statements must being with a return keyword"));}
  }

  match parse_expression(tokens, index) {
    Ok(()) => {},
    Err(e) => {return Err(e);}
  }
  match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statement is missing the ';' operator"));}
  }

  return Ok(());
}

fn parse_break_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::Break=> {*index += 1;}
    _ => {return Err(String::from("Break statements must begin with a break keyword"));}
  }

  match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statement is missing the ';' operator"));}
  }

  return Ok(());
}

fn parse_continue_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::Continue=> {*index += 1;}
    _ => {return Err(String::from("Continue statements must begin with a continue keyword"));}
  }

  match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statement is missing the ';' operator"));}
  }

  return Ok(());
}

//parsing complex expressions such as: "a + b - (c * d) / (f + g - 8);"
fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match parse_multiply_expression(tokens, index) {
    Ok(()) => {},
    Err(e) => {return Err(e);}
  }
  loop {
    match tokens[*index] {
    
      Token::Plus => {
        *index += 1;
        match parse_multiply_expression(tokens, index) {
          Ok(()) => {},
          Err(e) => {return Err(e);}
        }
      }

      Token::Subtract => {
        *index += 1;
        match parse_multiply_expression(tokens, index) {
          Ok(()) => {},
          Err(e) => {return Err(e);}
        }
      }

      _ => {
        break;
      }

    };
  }

  return Ok(());
}

fn parse_multiply_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match parse_term(tokens, index) {
    Ok(()) => {},
    Err(e) => {return Err(e);}
  }
  loop {
    match tokens[*index] {
      Token::Multiply => {
        *index += 1;
        match parse_term(tokens, index) {
          Ok(()) => {},
          Err(e) => {return Err(e);}
        }
      }

      Token::Divide => {
        *index += 1;
        match parse_term(tokens, index) {
          Ok(()) => {},
          Err(e) => {return Err(e);}
        }
      }

      Token::Modulus => {
        *index += 1;
        match parse_term(tokens, index) {
          Ok(()) => {},
          Err(e) => {return Err(e);}
        }
      }

      _ => {
        break;
      }

    };

  }

  return Ok(());
}

// a term is either a Nummber or an Identifier
fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    // If it is number return immediately
    Token::Num(_) => {
      *index += 1;
      return Ok(());
    }

    // If it is identifier
    Token::Ident(_) => {
      *index += 1;
      match tokens[*index] {
        // Under Identifier, if it follows a left bracket
        // Ident -> [ Expression ] ...
        Token::LeftBracket => {
          loop {
            match tokens[*index] {
              Token::LeftBracket => {*index += 1;}
              // Break and return if we dont find left bracket
              _ => { break; }
            }

            match parse_expression(tokens, index) {
              Ok(()) => {},
              Err(e) => {return Err(e);}
            }

            match tokens[*index] {
              Token::RightBracket => {*index += 1;}
              _ => { return Err(String::from("term missing right bracket ']'")); }
            }
          }
        }
        // Under Identifier, if it follows a left Parenthesis
        // Identifier (Expression(, Expression)*)
        Token::LeftParen => {
          // We have start with a left parenthesis and a expression
          match tokens[*index] {
            Token::LeftParen => {*index += 1;}
            _ => {return Err(String::from("term missing left parenthesis ')'"));}
          }
          match parse_expression(tokens, index) {
            Ok(()) => {},
            Err(e) => {return Err(e);}
          }
          
          // If there are more expressions between parenthesis
          // It must start with a comma
          // Else, it will be checked and throw an error in the next check point
          while matches!(tokens[*index], Token::Comma) {
            *index += 1;
            // Then we can parse another expressino
            match parse_expression(tokens, index) {
              Ok(()) => {},
              Err(e) => {return Err(e);}
            }
          }

          match tokens[*index] {
            Token::RightParen => {*index += 1;}
            _ => { return Err(String::from("term missing right parenthesis ')'")); }
          }
        }
        // If we see other characters, that is not part of this term
        // Identifier itself is a valid term
        _ => {
          // Do not do anything
        }
      }      
      return Ok(());
    }

    // ( Expression )
    Token::LeftParen => {
      *index += 1;
      match parse_expression(tokens, index) {
        Ok(()) => {},
        Err(e) => {return Err(e);}
      }

      match tokens[*index] {
        Token::RightParen => {*index += 1;}
        _ => { return Err(String::from("missing right parenthesis ')'")); }
      }
      return Ok(());
    }
    
    // Else, it idicates a missing term
    _ => {
      return Err(String::from("missing expression term."));
    }

  }
}