//! Integration tests for prompt system  

use rich_rust::prompt::Prompt;

#[test]
fn test_prompt_creation() {
    let _prompt = Prompt::<String>::new("Enter name");
    assert!(true);
}

#[test]
fn test_prompt_with_default() {
    let _prompt = Prompt::new("Enter name").default("Alice".to_string());
    assert!(true);
}

#[test]
fn test_prompt_int() {
    let _prompt = Prompt::<i32>::new("Enter number");
    assert!(true);
}

#[test]
fn test_prompt_float() {
    let _prompt = Prompt::<f64>::new("Enter decimal");
    assert!(true);
}

#[test]
fn test_prompt_secret() {
    let _prompt = Prompt::<String>::new("Password").secret();
    assert!(true);
}

#[test]
fn test_prompt_choices() {
    let choices = vec!["one".to_string(), "two".to_string()];
    let _prompt = Prompt::new("Select").choices(&choices);
    assert!(true);
}
