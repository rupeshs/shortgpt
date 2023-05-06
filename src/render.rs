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
use crate::models::GptChatOutput;
use termimad::crossterm::style::Stylize;

pub fn display(gpt_chat_output: GptChatOutput, elapsed: f32) -> () {
    if gpt_chat_output.choices.len() > 0 {
        let usage = &gpt_chat_output.usage;
        let gpt_model = gpt_chat_output.model;
        let answer = &gpt_chat_output.choices[0].message.content;

        let status_msg = format!(
            "\n{} | Tokens -> sent: {}, received: {}, total: {} | {:.3}s",
            gpt_model, usage.prompt_tokens, usage.completion_tokens, usage.total_tokens, elapsed
        );

        termimad::print_inline(answer);
        println!("\n {}", status_msg.dark_grey());
    }
}
