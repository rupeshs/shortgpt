# shortgpt 

shortgpt is a console application.

## Features 
 - Ask shortgpt for instant and concise answers
 - You can use any one of the GPT model (gpt-4, gpt-4-0314, gpt-4-32k, gpt-4-32k-0314, gpt-3.5-turbo, gpt-3.5-turbo-0301)
 - Default model is gpt-3.5-turbo
 - Cost effective
 - Use -l to enable long text mode

## Usage and Options
```
Usage: shortgpt.exe [OPTIONS] <question>

Arguments:
  <question>

Options:
  -m, --model <model>              GPT model (gpt-4, gpt-4-0314, gpt-4-32k, gpt-4-32k-0314, gpt-3.5-turbo, gpt-3.5-turbo-0301) [default: gpt-3.5-turbo]
  -t, --temperature <temperature>  Sampling temperature to use, between 0 and 2 [default: 0.7]
  -l, --long                       Turn on long text mode (50 words output limit off)
  -h, --help                       Print help
  -V, --version                    Print version
```
