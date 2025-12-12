pub mod ollama {
    use chrono::offset::Utc;
    use ollama_rs::{
        Ollama,
        generation::{
            completion::request::GenerationRequest,
            parameters::{FormatType, JsonStructure},
        },
        models::ModelOptions,
    };
    use std::fs;

    use crate::{config::config, errors::ServiceError, models::todos::InsertableTodo};

    pub async fn create_todos(input: String) -> Result<Vec<InsertableTodo>, ServiceError> {
        let cfg = config();

        let ollama = Ollama::new(&cfg.ollama_host, cfg.ollama_port.clone());
        let prompt = create_todo_prompt(input);

        // Try to log the file for backtracing
        fs::create_dir(".logs").ok();
        fs::write(format!(".logs/{:?}-logs.txt", Utc::now()), &prompt).ok();

        let format_type =
            FormatType::StructuredJson(Box::new(JsonStructure::new::<Vec<InsertableTodo>>()));

        let options = ModelOptions::default().temperature(0.6).num_predict(2048);

        let request = GenerationRequest::new("gemma2:2b".to_string(), prompt)
            .format(format_type)
            .options(options);

        let response = ollama
            .generate(request)
            .await
            .map_err(|_| ServiceError::BadRequest)?;

        let data: Vec<InsertableTodo> = serde_json::from_str(response.response.as_str())
            .map_err(|_| ServiceError::BadRequest)?;

        Ok(data)
    }

    fn create_todo_prompt(input: String) -> String {
        let system_prompt =
            r#"You are a perfect todo parser. Extract structured todos from natural language.
Please try to split the given source input into the smallest reasonable todo items as possible.
Return ONLY valid JSON array of objects with these exact keys:

 - title: string
 - body: a descriptive and helpful description of the todo (max 100 words)

JSON Schema:
[{"title":"Call mom", "body": "Call mom about dinner" }]

Please create the todos from this input:"#
                .trim();

        format!("{}\n{}", system_prompt, input)
    }
}
