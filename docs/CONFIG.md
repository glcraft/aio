# AIO Configuration File README

Welcome to the configuration file README for our project! This README provides an overview of the configuration file structure and its purpose. The configuration file is written in YAML and is designed to fine-tune the behavior of the engines for specific prompts. 

## Table of Contents

- [AIO Configuration File README](#aio-configuration-file-readme)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Usage](#usage)
    - [Prompts](#prompts)
      - [Example](#example)
  - [Sample Prompts](#sample-prompts)

## Introduction

The configuration file allows you to define prompts, local models and their respective. Prompts are common to 

This document will guide you through setting up your prompts and using the configuration file effectively.

By default, `aio` will try to read the configuration file from `~/.config/aio/config.yaml`. You can also specify the path to the configuration file using the `--config-path` argument. For example: `aio --config-path ./config.yaml`.

## Usage

### Prompts 

To use the configuration file effectively, follow these steps:

In the configuration file, you can define different prompts under the `prompts` section.

1. **Name the prompt**: This is the name to refer in the `--prompt` argument.

2. **Messages**: The whole prompt consists of several messages of three types:
   - "system" messages provide context and instructions to the model, while 
   - "user" messages define the user's input or query.
   - "assistant" messages are used to mimic the AI response.

    Use the variable `$input` to represent the input from the command line.

3. **Parameters**: Parameters can adjust AI generation. Here is the list of parameters : 
   - [`max_tokens`](https://platform.openai.com/docs/api-reference/chat/create#chat-create-max_tokens): The maximum number of tokens that can be generated in the chat completion.
   - [`temperature`](https://platform.openai.com/docs/api-reference/chat/create#chat-create-temperature): The temperature setting for the generated text. Higher temperatures result in more creative, but potentially incoherent, text.
   - [`top_p`](https://platform.openai.com/docs/api-reference/chat/create#chat-create-top_p): The maximum probability that the ASSISTANT should generate. The ASSISTANT will return the most likely answer, but with a probability below this threshold. This allows the ASSISTANT to return even the most unlikely of all possible answers, if the model is very certain.
   - [`presence_penalty`](https://platform.openai.com/docs/api-reference/chat/create#chat-create-presence_penalty): The presence penalty modifies the likelihood of selected tokens based on their presence in the input.
   - [`frequency_penalty`](https://platform.openai.com/docs/api-reference/chat/create#chat-create-frequency_penalty): The frequency penalty modifies the likelihood of selected tokens based on their frequency in the input.
   - [`best_of`](https://platform.openai.com/docs/api-reference/chat/create#chat-create-best_of): The number of responses to generate.
   - [`stop`](https://platform.openai.com/docs/api-reference/chat/create#chat-create-stop): The text used to stop the ASSISTANT from generating more text.

    **OpenAI API specific parameters**
    - [`n`](https://platform.openai.com/docs/api-reference/chat/create#chat-create-n): The number of responses to generate.

    **Note**: each parameter above is optional.

    Refer to the Documentation of the [*chat create* OpenAI API](https://platform.openai.com/docs/api-reference/chat/create) for more information about the parameters.

#### Example

```yaml
prompts:
  - name: command
    model: gpt-3.5-turbo # optional
    messages: 
      # System message for context
      - role: system
      content: In markdown, write the command...
      # User message
      - role: user
      content: $input
    parameters:
      # Parameters to control model behavior. Each parameter is optional
      temperature: 0
      top-p: 1.0 
      frequency-penalty: 0.2
      presence-penalty: 0 
      max-tokens: 200
```



## Sample Prompts

Here are examples of prompt definitions within [the sample configuration file](../config.yml) you can find in the repository:

1. **Command Prompt**:
   - System Message: Provides instructions for formatting a command.
   - User Message: Represents user input for the command.
   - Parameters: Parameters controlling the response characteristics.

2. **Ask Prompt**:
   - System Message: Provides a brief introduction to ChatGPT.
   - User Message: Represents user input for the query.
   - Parameters: Parameters influencing the model's response.

---

*Note: This README is a general guide for understanding and using the configuration file. Feel free to customize it according to your desire.*