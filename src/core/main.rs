use std::cell::RefCell;
use std::rc::Rc;

use crate::unknown::interpreter::{Env, Interpreter};
use crate::unknown::lexer::Lexer;
use crate::unknown::parser::Parser;

use rusty_repl::{CleanPrompt, Color, DefaultPromptSegment, KeywordStyle, Repl, ReplConfig};

pub fn run(source: String) {
    let interpreter = Rc::new(RefCell::new(Interpreter::new()));
    let env = Rc::new(RefCell::new(Env::new()));

    execute(&source, interpreter, env);
}

fn execute(source: &str, interpreter: Rc<RefCell<Interpreter>>, env: Rc<RefCell<Env>>) {
    // lexer
    let mut lexer = Lexer::new(source);
    let tokens = lexer.lex_all();

    // parser
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    // eval
    for expr in ast {
        let result = interpreter.borrow_mut().eval(&expr, &mut env.borrow_mut());

        match result {
            Ok(val) => println!("{}", val),
            Err(err) => println!("Error: {}", err),
        }
    }
}

pub fn repl_loop() {
    // shared state
    let interpreter = Rc::new(RefCell::new(Interpreter::new()));
    let env = Rc::new(RefCell::new(Env::new()));

    let interpreter_ref = interpreter.clone();
    let env_ref = env.clone();

    let run = move |input: String| -> bool {
        let input = input.trim();

        match input {
            "exit" | "quit" | "q" | "e" => return true,
            "" => return false,
            _ => {}
        }

        execute(input, interpreter_ref.clone(), env_ref.clone());

        false
    };

    let ks = KeywordStyle::new(vec!["let", "mut", "exit", "quit"], Color::Red);

    let prompt = CleanPrompt::from(
        DefaultPromptSegment::Basic("Unknown ❯ ".to_string()),
        DefaultPromptSegment::Empty,
    );

    let cfg = ReplConfig::new("UnknownLang REPL")
        .with_kw_style(ks)
        .with_prompt(prompt);

    let repl = Repl::from(cfg);

    let _ = repl.run(run);
}
