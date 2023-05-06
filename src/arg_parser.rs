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
use clap::{Arg, ArgAction, Command};

pub fn parse_arguments() -> clap::ArgMatches {
    let version = env!("CARGO_PKG_VERSION");
    let matches = Command::new("shortgpt")
        .version(version)
        .author("Rupesh Sreeraman. <exmplayer.dev@gmail.com>")
        .about("Short and Sweet: Ask shortGPT for Instant and Concise Answers!")
        .arg(Arg::new("question").required(true))
        .arg(
            Arg::new("long")
                .short('l')
                .long("long")
                .help("Turn on long text mode (lengthy output from GPT)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("model")
                .short('m')
                .long("model")
                .help("GPT model (gpt-4, gpt-4-0314, gpt-4-32k, gpt-4-32k-0314, gpt-3.5-turbo, gpt-3.5-turbo-0301)")
                .default_value("gpt-3.5-turbo")
        )
        .get_matches();

    matches
}
