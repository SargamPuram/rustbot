use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use teloxide::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    message: String,
}

    #[tokio::main]
    async fn main() {
        pretty_env_logger::init();

        let telegram_token =
            env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN not found in environment variables");
        let openai_api_key =
            env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in environment variables");

        let bot = Bot::new(telegram_token);

        teloxide::repl(bot, move |bot: Bot, message: Message| {
            let openai_api_key = openai_api_key.clone();
            async move {
                if let Some(text) = message.text() {
                    let response = get_openai_response(&openai_api_key, text).await;

                    if let Some(answer) = response.choices.first() {
                        let msg = format!("{}", answer.message);
                        bot.send_message(msg, "your answer").await?;
                    } else {
                        let word = String::from("Sorry");

                        bot.send_message(word, "try again").await?;
                    }
                }

                Ok(())
            }
        })
        .await;
    }

    async fn get_openai_response(api_key: &str, query: &str) -> OpenAIResponse {
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.openai.com/v1/engines/davinci-codex/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&json!({
                "prompt": query,
                "max_tokens": 100,
            }))
            .send()
            .await
            .expect("Failed to send request to OpenAI API")
            .json::<OpenAIResponse>()
            .await
            .expect("Failed to parse OpenAI API response");

        response
    }


