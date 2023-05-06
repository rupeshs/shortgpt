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

use crate::gpt_chat_completion;
use crate::render;
use colored::*;
use spinoff::{spinners, Color, Spinner};
use std::time::Instant;

pub fn do_completion(gpt_model: &str, openai_api_key: &str, query: &str, long_text: bool) {
    let mut spinner = Spinner::new(spinners::Line, "Thinking...", Color::Green);
    let result = gpt_chat_completion::ask(gpt_model, &openai_api_key, query, long_text);
    let start_time = Instant::now();
    match result {
        Ok(gpt_chat_output) => {
            spinner.update_text("");
            spinner.stop();
            let end_time = Instant::now();
            let duration = end_time.duration_since(start_time);
            render::display(gpt_chat_output, duration.as_secs_f32());
        }

        Err(error) => {
            spinner.update_text("");
            spinner.stop();
            println!("🛑 ERROR : {}", error.red())
        }
    }
}
