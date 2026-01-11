//! Integration tests for prompt system  

use fast_rich::prompt::Prompt;

#[test]
fn test_prompt_creation() {
    let _prompt = Prompt::<String>::new("Enter name");
    // Compilation verifies the prompt is valid
}

#[test]
fn test_prompt_with_default() {
    let _prompt = Prompt::new("Enter name").default("Alice".to_string());
    // Compilation verifies default works
}

#[test]
fn test_prompt_int() {
    let _prompt = Prompt::<i32>::new("Enter number");
    // Compilation verifies i32 prompts work
}

#[test]
fn test_prompt_float() {
    let _prompt = Prompt::<f64>::new("Enter decimal");
    // Compilation verifies f64 prompts work
}

#[test]
fn test_prompt_secret() {
    let _prompt = Prompt::<String>::new("Password").secret();
    // Compilation verifies secret prompts work
}

#[test]
fn test_prompt_choices() {
    let choices = vec!["one".to_string(), "two".to_string()];
    let _prompt = Prompt::new("Select").choices(&choices);
    // Compilation verifies choices work
}
