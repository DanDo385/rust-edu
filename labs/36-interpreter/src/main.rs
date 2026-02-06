// Project 33: Expression Interpreter
//
// Builds a simple interpreter for arithmetic expressions. Demonstrates
// tokenization (lexing), parsing to an AST, and recursive evaluation.
// This is the foundation for building programming languages!

fn main() {
    println!("=== Expression Interpreter ===\n");

    // ============================================================================
    // BASIC EXPRESSIONS
    // ============================================================================
    println!("=== Basic Arithmetic ===");

    let expressions = vec![
        "2 + 3",
        "10 - 4",
        "6 * 7",
        "20 / 5",
        "2 + 3 * 4",        // Tests operator precedence
        "(2 + 3) * 4",      // Tests parentheses
        "10 + 2 * 6",       // More precedence
        "100 * 2 + 12",
        "100 * (2 + 12)",
        "(5 - 3) * (7 + 2)",
    ];

    for expr in expressions {
        match interpret(expr) {
            Ok(result) => println!("{:20} = {}", expr, result),
            Err(e) => println!("{:20} ERROR: {}", expr, e),
        }
    }

    println!();

    // ============================================================================
    // COMPLEX EXPRESSIONS
    // ============================================================================
    println!("=== Complex Expressions ===");

    let complex = vec![
        "((15 / (7 - (1 + 1))) * 3) - (2 + (1 + 1))",
        "1 + 2 + 3 + 4 + 5",
        "10 - 5 - 2",
        "2 * 3 * 4",
        "100 / 10 / 2",
    ];

    for expr in complex {
        match interpret(expr) {
            Ok(result) => println!("{} = {}", expr, result),
            Err(e) => println!("{} ERROR: {}", expr, e),
        }
    }

    println!();

    // ============================================================================
    // ERROR HANDLING
    // ============================================================================
    println!("=== Error Handling ===");

    let invalid = vec![
        "2 +",           // Missing operand
        "* 3",           // Missing left operand
        "2 + + 3",       // Double operator
        "(2 + 3",        // Unclosed parenthesis
        "2 + 3)",        // Extra closing parenthesis
        "",              // Empty expression
    ];

    for expr in invalid {
        match interpret(expr) {
            Ok(result) => println!("{:20} = {} (should have failed!)", expr, result),
            Err(e) => println!("{:20} ERROR: {}", expr, e),
        }
    }

    println!();

    // ============================================================================
    // SHOW INTERNAL STEPS
    // ============================================================================
    println!("=== Internal Steps for: 2 + 3 * 4 ===");

    let input = "2 + 3 * 4";

    // Step 1: Tokenize
    println!("Input: {}", input);
    let tokens = tokenize(input);
    println!("Tokens: {:?}", tokens);

    // Step 2: Parse
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => {
            println!("AST: {:?}", ast);

            // Step 3: Evaluate
            match evaluate(&ast) {
                Ok(result) => println!("Result: {}", result),
                Err(e) => println!("Evaluation error: {}", e),
            }
        }
        Err(e) => println!("Parse error: {}", e),
    }

    println!();
    println!("=== Interpreter Demo Complete ===");
}

// ============================================================================
// HIGH-LEVEL INTERPRET FUNCTION
// ============================================================================

fn interpret(input: &str) -> Result<f64, String> {
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    evaluate(&ast)
}

// ============================================================================
// TOKENIZER (LEXICAL ANALYSIS)
// ============================================================================
// Converts input string into a stream of tokens

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Skip whitespace
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }

            // Operators and parentheses
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Star);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Slash);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }

            // Numbers
            '0'..='9' | '.' => {
                let mut number = String::new();

                while let Some(&ch) = chars.peek() {
                    if ch.is_numeric() || ch == '.' {
                        number.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if let Ok(value) = number.parse::<f64>() {
                    tokens.push(Token::Number(value));
                }
            }

            // Unknown character - skip it
            _ => {
                chars.next();
            }
        }
    }

    tokens
}

// ============================================================================
// ABSTRACT SYNTAX TREE (AST)
// ============================================================================
// Represents the structure of the expression

#[derive(Debug, Clone)]
enum Expr {
    Number(f64),
    BinOp {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// ============================================================================
// PARSER (SYNTAX ANALYSIS)
// ============================================================================
// Converts tokens into an AST using recursive descent parsing
//
// Grammar (with precedence):
//   expression  = term ((PLUS | MINUS) term)*
//   term        = factor ((STAR | SLASH) factor)*
//   factor      = NUMBER | LEFT_PAREN expression RIGHT_PAREN

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    /// Main entry point - parse entire expression
    fn parse(&mut self) -> Result<Expr, String> {
        if self.tokens.is_empty() {
            return Err("Empty expression".to_string());
        }

        let expr = self.expression()?;

        // Make sure we consumed all tokens
        if self.position < self.tokens.len() {
            return Err(format!(
                "Unexpected token: {:?}",
                self.tokens[self.position]
            ));
        }

        Ok(expr)
    }

    /// Parse expression: term ((+ | -) term)*
    /// Lower precedence (evaluated last)
    fn expression(&mut self) -> Result<Expr, String> {
        let mut left = self.term()?;

        while self.position < self.tokens.len() {
            let op = match &self.tokens[self.position] {
                Token::Plus => Operator::Add,
                Token::Minus => Operator::Subtract,
                _ => break,
            };

            self.position += 1;
            let right = self.term()?;

            left = Expr::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse term: factor ((* | /) factor)*
    /// Higher precedence (evaluated first)
    fn term(&mut self) -> Result<Expr, String> {
        let mut left = self.factor()?;

        while self.position < self.tokens.len() {
            let op = match &self.tokens[self.position] {
                Token::Star => Operator::Multiply,
                Token::Slash => Operator::Divide,
                _ => break,
            };

            self.position += 1;
            let right = self.factor()?;

            left = Expr::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse factor: NUMBER | ( expression )
    /// Highest precedence
    fn factor(&mut self) -> Result<Expr, String> {
        if self.position >= self.tokens.len() {
            return Err("Unexpected end of expression".to_string());
        }

        match &self.tokens[self.position] {
            Token::Number(n) => {
                let value = *n;
                self.position += 1;
                Ok(Expr::Number(value))
            }

            Token::LeftParen => {
                self.position += 1; // Consume (

                let expr = self.expression()?;

                if self.position >= self.tokens.len() {
                    return Err("Unclosed parenthesis".to_string());
                }

                match &self.tokens[self.position] {
                    Token::RightParen => {
                        self.position += 1; // Consume )
                        Ok(expr)
                    }
                    _ => Err("Expected closing parenthesis".to_string()),
                }
            }

            _ => Err(format!(
                "Unexpected token: {:?}",
                self.tokens[self.position]
            )),
        }
    }
}

// ============================================================================
// EVALUATOR (INTERPRETER)
// ============================================================================
// Recursively evaluates the AST to compute the result

fn evaluate(expr: &Expr) -> Result<f64, String> {
    match expr {
        Expr::Number(n) => Ok(*n),

        Expr::BinOp { left, op, right } => {
            let left_val = evaluate(left)?;
            let right_val = evaluate(right)?;

            match op {
                Operator::Add => Ok(left_val + right_val),
                Operator::Subtract => Ok(left_val - right_val),
                Operator::Multiply => Ok(left_val * right_val),
                Operator::Divide => {
                    if right_val == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(left_val / right_val)
                    }
                }
            }
        }
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. ENUM REPRESENTATION
//    - Enums are tagged unions (discriminant + largest variant)
//    - Expr::Number: 1 byte tag + 8 bytes f64 = ~16 bytes
//    - Expr::BinOp: 1 byte tag + 2 Box pointers + 1 byte op = ~24 bytes
//    - Very memory efficient (no vtables like classes)
//
// 2. BOX ALLOCATION
//    - Box<Expr> allocates on the heap
//    - Pointer is 8 bytes (64-bit system)
//    - Allows recursive types without infinite size
//    - Automatically freed when dropped
//
// 3. RECURSIVE CALLS
//    - Each function call uses stack space (~32-64 bytes)
//    - Very deep expressions can cause stack overflow
//    - Rust's default stack is 2MB (can handle ~10,000 levels)
//    - Could use iterative evaluation for unlimited depth
//
// 4. PATTERN MATCHING
//    - Compiles to efficient jump tables or if-chains
//    - Exhaustiveness checked at compile time
//    - No runtime overhead vs hand-written if-else
//    - Compiler optimizes out enum tags when possible
//
// 5. STRING PARSING
//    - chars() iterator is UTF-8 aware (no buffer overflows)
//    - peekable() adds minimal overhead (~1 pointer)
//    - String allocations for numbers (could optimize)
//
// 6. ERROR HANDLING
//    - Result<T, E> is zero-cost (same as returning T on success)
//    - ? operator compiles to efficient early returns
//    - No exceptions or stack unwinding overhead

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Interpreters have three phases: tokenize, parse, evaluate
// 2. Tokenizer converts text to tokens (lexical analysis)
// 3. Parser converts tokens to AST (syntax analysis)
// 4. AST represents program structure as a tree
// 5. Evaluator recursively computes the result
// 6. Enums are perfect for AST nodes (sum types)
// 7. Box enables recursive data structures
// 8. Pattern matching ensures exhaustive handling
// 9. Operator precedence is encoded in parser structure
// 10. This is the foundation for building real languages!

// ============================================================================
// PARSING TECHNIQUES
// ============================================================================
// We used RECURSIVE DESCENT parsing:
//   - Simple to implement and understand
//   - Hand-written, no tools needed
//   - Good for simple grammars
//   - Can handle operator precedence
//
// Other parsing techniques:
//   - PARSER COMBINATORS (nom, combine):
//     - Compose small parsers into larger ones
//     - Type-safe, functional style
//     - Great for complex formats
//
//   - PARSER GENERATORS (LALRPOP, pest):
//     - Write grammar file, tool generates parser
//     - Handles complex grammars (LR, LALR, PEG)
//     - Better error messages
//
//   - PRATT PARSING:
//     - Elegant operator precedence handling
//     - Single function instead of multiple
//     - Used in many production parsers

// ============================================================================
// OPERATOR PRECEDENCE
// ============================================================================
// Our grammar encodes precedence:
//   - expression (+ -) has LOWER precedence
//   - term (* /) has HIGHER precedence
//   - factor (numbers, parens) has HIGHEST precedence
//
// This means:
//   2 + 3 * 4  →  2 + (3 * 4)  →  14  (correct!)
//   Not:          (2 + 3) * 4  →  20  (wrong)
//
// The parser builds the AST to reflect this:
//   Add(2, Multiply(3, 4))  ✅
//   Not: Multiply(Add(2, 3), 4)  ❌

// ============================================================================
// EXTENDING THIS INTERPRETER
// ============================================================================
// To add more features:
//
// 1. VARIABLES:
//    - Add Token::Identifier
//    - Add Expr::Variable(String)
//    - Pass HashMap<String, f64> to evaluator
//
// 2. ASSIGNMENT:
//    - Add Token::Equals
//    - Add Expr::Assign(String, Box<Expr>)
//    - Modify evaluator to update HashMap
//
// 3. FUNCTIONS:
//    - Add Token::Identifier(String)
//    - Add Expr::Call(String, Vec<Expr>)
//    - Store function definitions in environment
//
// 4. CONTROL FLOW:
//    - Add if/else, loops
//    - Change evaluator to return statements, not just values
//    - Add Expr::If, Expr::While, etc.
//
// 5. TYPE SYSTEM:
//    - Add type checking pass before evaluation
//    - Prevents runtime type errors
//    - More complex but safer

// ============================================================================
// REAL-WORLD INTERPRETERS
// ============================================================================
// TREE-WALKING INTERPRETERS (like ours):
//   - Ruby (MRI), early Python
//   - Simple to implement
//   - Slow (10-100x slower than compiled)
//
// BYTECODE INTERPRETERS:
//   - Python (CPython), Lua, Java (JVM)
//   - Compile to bytecode first, then interpret
//   - Faster (5-20x slower than compiled)
//
// JIT COMPILERS:
//   - JavaScript (V8), LuaJIT, PyPy
//   - Compile hot code to machine code at runtime
//   - Can match or exceed C performance
//
// AOT COMPILERS:
//   - Rust, C, C++, Go
//   - Compile to machine code ahead of time
//   - Fastest but no runtime flexibility

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Not using Box for recursive enums (infinite size error)
// ❌ Forgetting to handle all enum variants (non-exhaustive pattern)
// ❌ Wrong operator precedence (parsing 2+3*4 as (2+3)*4)
// ❌ Not checking for division by zero
// ❌ Not consuming all tokens (leaving garbage at end)
// ❌ Off-by-one errors in token position
// ❌ Unclosed parentheses not detected
// ❌ Stack overflow with very deep expressions

// ============================================================================
// PERFORMANCE CONSIDERATIONS
// ============================================================================
// Our interpreter is simple but not optimized:
//   - Tokenize: O(n) where n = input length
//   - Parse: O(n) where n = number of tokens
//   - Evaluate: O(n) where n = AST nodes
//   - Total: O(n) which is pretty good!
//
// Optimizations:
//   - Cache parsed ASTs (don't re-parse)
//   - Constant folding (compute 2+3 at parse time)
//   - Bytecode compilation (faster interpretation)
//   - JIT compilation (compile to machine code)
//   - Tail call optimization (for recursive functions)
