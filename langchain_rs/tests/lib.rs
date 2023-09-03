use langchain_rs::prompts::Prompt;
use langchain_rs::prompts::PromptArgs;
use langchain_rs::prompts::PromptTemplate;
use langchain_rs::prompts::TemplateFormat;

#[test]
fn should_work() {
    let template = PromptTemplate::new(
        "Hello {{name}}!".to_string(),
        vec!["name".to_string()],
        TemplateFormat::Jinja2,
    );

    let input_variables = PromptArgs::new();
    let result = template.format(input_variables);
    assert!(result.is_err());

    let mut input_variables = PromptArgs::new();
    input_variables.insert("name", "world");
    let result = template.format(input_variables);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello world!");
}
