pub mod gpt {
    use crate::{
        config,
        models::ai::{AiRequest, OpenAIResponse},
    };
    use reqwest::Client;

    pub async fn prompt(input: String) -> Result<OpenAIResponse, reqwest::Error> {
        let cfg = config::config();
        let client = Client::new();

        let ai_request = AiRequest::with_defaults(input);

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", cfg.ai_api_key),
            )
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&ai_request)
            .send()
            .await?;

        let parsed = response.json::<OpenAIResponse>().await?;

        Ok(parsed)
    }
}
