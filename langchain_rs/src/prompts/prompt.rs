use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Result;

pub enum TemplateFormat {
    FString,
    Jinja2,
}

pub struct PromptTemplate {
    template: String,
    variables: Vec<String>,
    format: TemplateFormat,
}

pub type PromptArgs<'a> = HashMap<&'a str, &'a str>;

pub trait Prompt: Send + Sync {
    fn template(&self) -> String;
    fn variables(&self) -> Vec<String>;
    fn format(&self, input_variables: HashMap<&str, &str>) -> Result<String>;
}

impl PromptTemplate {
    pub fn new(template: String, variables: Vec<String>, format: TemplateFormat) -> Arc<Self> {
        Arc::new(Self {
            template,
            variables,
            format,
        })
    }
}

impl Prompt for PromptTemplate {
    fn template(&self) -> String {
        self.template.clone()
    }

    fn variables(&self) -> Vec<String> {
        self.variables.clone()
    }

    fn format(&self, input_variables: HashMap<&str, &str>) -> Result<String> {
        let mut prompt = self.template();

        // check if all variables are in the input variables
        for key in self.variables() {
            if !input_variables.contains_key(key.as_str()) {
                return Err(anyhow!(
                    "variable: '{}' is not in the input variables: {:?}",
                    key,
                    input_variables
                ));
            }
        }

        for (key, value) in input_variables {
            let key = match self.format {
                TemplateFormat::FString => format!("{{{}}}", key),
                TemplateFormat::Jinja2 => format!("{{{{{}}}}}", key),
            };
            prompt = prompt.replace(&key, value);
        }

        Ok(prompt)
    }
}

// Creates a hashmap of arguments for a prompt
// #[macro_export]
// macro_rules! prompt_args {
//     () => (PromptArgs::new());
//     ( $($key:expr => $value:expr,)+ ) => (prompt_args!($($key => $value),+));
//     ( $($key:expr => $value:expr),* ) => {
//         {
//             let mut args= PromptArgs::new();
//             $(
//                 args.insert($key, $value).unwrap();
//             )*
//             args
//         }
//     };
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_format_jinja2_template() {
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

    #[test]
    fn should_format_fstring_template() {
        let template = PromptTemplate::new(
            "Hello {name}!".to_string(),
            vec!["name".to_string()],
            TemplateFormat::FString,
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

    // #[test]
    // fn should_prompt_macro_work() {
    //     let args: HashMap<&str, &str> = prompt_args! {};
    //     assert!(args.is_empty());

    //     let args = prompt_args! {
    //         "name" => "world"
    //     };
    //     assert_eq!(args.len(), 1);
    //     assert_eq!(args.get("name").unwrap(), &"world");

    //     let args = prompt_args! {
    //         "name" => "world",
    //         "age" => "18"
    //     };
    //     assert_eq!(args.len(), 2);
    //     assert_eq!(args.get("name").unwrap(), &"world");
    //     assert_eq!(args.get("age").unwrap(), &"18");
    // }
}
