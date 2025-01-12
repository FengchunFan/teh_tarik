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
        println!("{}", error_message);
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
          return Err(format!("Detect unknown character behind digits at index {}", i));
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
          return Err(format!("Detect unknown character behind variable at index {}", i));
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
          ' ' => {
            tokens.push(Token::Greater);
            i += 1;
          }
          _ => {
            return Err(format!("Unrecognized symbol '>{}'", curr));
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
          ' ' => {
            tokens.push(Token::Less);
            i += 1;
          }
          _ => {
            return Err(format!("Unrecognized symbol '<{}'", curr));
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
          ' ' => {
            tokens.push(Token::Assign);
            i += 1;
          }
          _ => {
            return Err(format!("Unrecognized symbol '={}'", curr));
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
          ' ' => {
            return Err(format!("Unrecognized symbol '!'"));
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

// writing tests!
// testing shows robustness in software, and is good for spotting regressions
// to run a test, type "cargo test" in the terminal.
// Rust will then run all the functions annotated with the "#[test]" keyword.
#[cfg(test)]
mod tests {
    use crate::Token;
    use crate::lex;

    #[test]
    fn lexer_test() {
        // test that lexer works on correct cases
        let toks = lex("1 + 2 + 3").unwrap();
        assert!(toks.len() == 6);
        assert!(matches!(toks[0], Token::Num(1)));
        assert!(matches!(toks[1], Token::Plus));
        assert!(matches!(toks[2], Token::Num(2)));
        assert!(matches!(toks[3], Token::Plus));
        assert!(matches!(toks[4], Token::Num(3)));
        assert!(matches!(toks[5], Token::End));

        let toks = lex("3 + 215 +").unwrap();
        assert!(toks.len() == 5);
        assert!(matches!(toks[0], Token::Num(3)));
        assert!(matches!(toks[1], Token::Plus));
        assert!(matches!(toks[2], Token::Num(215)));
        assert!(matches!(toks[3], Token::Plus));
        assert!(matches!(toks[4], Token::End));

        // test that the lexer catches invalid tokens
        assert!(matches!(lex("^^^"), Err(_)));
    }

}
