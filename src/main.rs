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

    https://platform.openai.com/docs/api-reference/completions/create
    https://help.openai.com/en/articles/5112595-best-practices-for-api-key-safety
*/

mod arg_parser;
mod completion_context;
mod gpt_chat_completion;
mod models;
mod render;

use colored::*;

#[cfg(target_os = "windows")]
fn windows_term_color() {
    control::set_virtual_terminal(true).unwrap();
}
fn main() {
    #[cfg(target_os = "windows")]
    windows_term_color();

    let matches = arg_parser::parse_arguments();
    let long_text = matches.get_flag("long");
    let query = matches.get_one::<String>("question").unwrap();
    let gpt_model = matches.get_one::<String>("model").unwrap();

    if long_text {
        println!("{}", "Running in long text mode!".cyan());
    }
    println!("{} : {}", "Question:".magenta(), query.bright_green());
    let result = std::env::var("OPENAI_API_KEY");
    match result {
        Ok(openai_api_key) => {
            completion_context::do_completion(gpt_model, &openai_api_key, query, long_text)
        }
        Err(_) => {
            let error_msg = "Error: Please set OPENAI_API_KEY environment variable!".red();
            println!("🛑 {}", error_msg);
        }
    }
}
