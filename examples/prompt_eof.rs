use fast_rich::prompt::{IntPrompt, Prompt, PromptError};
fn main() {
    // Run with stdin closed: `cargo run --example prompt_eof < /dev/null`
    println!("OLD: IntPrompt::ask() with closed stdin -> hot infinite loop, never returns");
    let p = IntPrompt::new("num");
    match p.try_ask() {
        Ok(v) => println!("NOW: try_ask() returned Ok({v})"),
        Err(e @ PromptError::Eof) => println!("NOW: try_ask() returned Err(Eof) immediately: {e}"),
        Err(e) => println!("NOW: try_ask() returned Err({e})"),
    }

    // With a default, EOF falls back to it instead of erroring.
    let p = Prompt::<String>::new("name").default("anonymous".to_string());
    match p.try_ask() {
        Ok(v) => println!("NOW: with default, EOF returned Ok({v:?})"),
        Err(e) => println!("NOW: with default, EOF returned Err({e})"),
    }
}
