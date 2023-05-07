# shortgpt 

shortgpt is a AI assistant console app for Windows/Linux/MacOS we can run it as command . It answers questions with less than 50 words,use GPT 3/4 from the terminal.

![shortgot](https://raw.githubusercontent.com/rupeshs/shortgpt/main/images/shortgpt.png)

## Features 
 - Ask shortgpt for instant and concise answers
 - Add shortgpt to PATH environment and use it as command
 - You can use any one of the GPT model (gpt-4, gpt-4-0314, gpt-4-32k, gpt-4-32k-0314, gpt-3.5-turbo, gpt-3.5-turbo-0301)
 - Default model is gpt-3.5-turbo
 - Cost effective
 - Use -l to enable long text mode

## Screenshots
![shortgot linux](https://raw.githubusercontent.com/rupeshs/shortgpt/main/images/shortgpt_demo.PNG)

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

## Set OPENAI_API_KEY 
To set OpenAI API key run the following command.
### Windows
 To set OPENAI_API_KEY environment variable for the current `cmd` shell
 `set OPENAI_API_KEY=<yourkey>`

 To set OPENAI_API_KEY environment variable permanently

`setx OPENAI_API_KEY “<yourkey>”`

## Linux / MacOS

`export OPENAI_API_KEY=yourkey`

Or set it permanently on .bashrc or .zshrc file
```
echo "export OPENAI_API_KEY='yourkey'" >> ~/.zshrc
source ~/.zshrc
echo $OPENAI_API_KEY
```

## Development

To run in the development mode run the following command :

`cargo run -- "hello"`