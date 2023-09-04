use anyhow::Result;
use langchain_rs::llms::OpenAI;
use langchain_rs::prompts::Prompt;
use langchain_rs::prompts::PromptArgs;
use langchain_rs::prompts::PromptTemplate;
use langchain_rs::prompts::TemplateFormat;
use langchain_rs::schema::LLM;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let template =
        "You're an helpful AI assistant developed by OpenAI. Your role is {role}!".to_string();
    let variables = vec!["role".to_string()];
    let mut args = PromptArgs::new();
    args.insert("role", "assistant");
    let prompt_template = PromptTemplate::new(template, variables, TemplateFormat::FString);
    let text = prompt_template.format(args)?;

    let openai = OpenAI::default();
    let sss = openai.generate(&text).await?;
    print!("{:#?}", sss);
    Ok(())
}
