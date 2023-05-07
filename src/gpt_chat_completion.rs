/*
    MIT License

    Copyright (c) 2023 Rupesh Sreeraman

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
*/
use crate::models::{GptChatOutput, GptInput, Message};
use crate::prompts::{get_long_prompt, get_short_prompt};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{blocking::Client, StatusCode};

const OPENAI_GPT_CHAT_API_URL: &str = "https://api.openai.com/v1/chat/completions";

fn get_authentication_bearer(api_key: &str) -> String {
    let auth_bearer = format!("Bearer {}", api_key);
    auth_bearer
}

fn get_gpt_input(model: &str, prompt: &str, temperature: f32) -> GptInput {
    let model = String::from(model);
    let messages = vec![Message {
        role: String::from("user"),
        content: String::from(prompt),
    }];
    let gpt_input = GptInput {
        model,
        temperature,
        messages,
    };
    gpt_input
}

fn get_headers(auth_bearer: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    let api_key = HeaderValue::from_str(&auth_bearer).unwrap();
    headers.insert(AUTHORIZATION, api_key);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers
}

pub fn ask(
    model: &str,
    api_key: &str,
    query: &str,
    is_long: bool,
    temperature: f32,
) -> Result<GptChatOutput, String> {
    let auth_bearer = get_authentication_bearer(api_key);
    let prompt;
    if is_long {
        prompt = get_long_prompt(query);
    } else {
        prompt = get_short_prompt(query);
    }

    let gpt_input = get_gpt_input(model, &prompt, temperature);
    let request_body = serde_json::to_string(&gpt_input).unwrap();

    let headers = get_headers(&auth_bearer);
    let client = Client::new();
    let response = client
        .post(OPENAI_GPT_CHAT_API_URL)
        .headers(headers)
        .body(request_body)
        .send();

    match response {
        Ok(response) => match response.status() {
            StatusCode::OK => {
                let gpt_output: GptChatOutput = response.json().unwrap();
                Ok(gpt_output)
            }
            status => {
                let description = status.canonical_reason().unwrap_or("<unknown status code>");
                let err = format!("Status code: {:?} - {}", status, description);
                Err(err)
            }
        },
        Err(error) => {
            let err = format!("{}", error);
            Err(err)
        }
    }
}
