use std::cell::RefCell;
use std::rc::Rc;

use crate::unknown::interpreter::{Env, Interpreter};
use crate::unknown::lexer::Lexer;
use crate::unknown::parser::Parser;

use rusty_repl::{CleanPrompt, Color, DefaultPromptSegment, KeywordStyle, Repl, ReplConfig};

pub fn repl_loop() {
    // shared state (IMPORTANT)
    let interpreter = Rc::new(RefCell::new(Interpreter::new()));
    let env = Rc::new(RefCell::new(Env::new()));

    let interpreter_ref = interpreter.clone();
    let env_ref = env.clone();

    let run = move |input: String| -> bool {
        let input = input.trim();

        // exit
        match input {
            "exit" | "quit" | "q" | "e" => return true,
            "" => return false,
            _ => {}
        }

        // lexer → parser
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex_all();

        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        // execute safely
        for expr in ast {
            let result = interpreter_ref
                .borrow_mut()
                .eval(&expr, &mut env_ref.borrow_mut());

            match result {
                Ok(val) => println!("{}", val),
                Err(err) => println!("Error: {}", err),
            }
        }

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
