use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserAiRequest {
    pub input: String,
}

#[derive(Deserialize, Serialize)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct OpenAIResponse {
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    pub message: OpenAIMessage,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub format_type: String,
}

#[derive(Deserialize, Serialize)]
pub struct AiRequest {
    pub model: String,
    pub messages: Vec<OpenAIMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub response_format: Option<ResponseFormat>,
}

impl AiRequest {
    pub fn with_defaults(input: String) -> Self {
        let system_prompt = r#"
            You are a perfect todo parser. Extract structured todos from natural language.
            Return ONLY valid JSON array of objects with these exact keys:

            - title: string
            - body: a descriptive and helpful description of the todo (max 100 words)

            Example: [{"title":"Call mom", "body": "Call mom about dinner" }]
        "#
        .trim();

        let mut messages = vec![OpenAIMessage {
            role: String::from("system"),
            content: String::from(system_prompt),
        }];

        messages.push(OpenAIMessage {
            role: String::from("user"),
            content: input,
        });

        Self {
            model: String::from("gpt-5-nano"),
            messages,
            temperature: Some(0.7),
            max_tokens: Some(2000),
            response_format: Some(ResponseFormat {
                format_type: String::from("json_object"),
            }),
        }
    }
}
