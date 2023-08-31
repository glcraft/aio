# aio - Streamlined AI Terminal Interactions

Welcome to the README for the `aio` command line tool – your gateway to seamless communication with AI engines via the terminal. This tool streamlines interactions with AI APIs, including the OpenAI API, and conveniently formats the results using integrated markdown formatting. Whether you're seeking information, generating content, or experimenting with AI, `aio` has you covered.

## Table of Contents

- [aio - Streamlined AI Terminal Interactions](#aio---streamlined-ai-terminal-interactions)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Installation](#installation)
    - [Install from Github releases](#install-from-github-releases)
    - [Install from source](#install-from-source)
  - [Usage](#usage)
  - [Arguments](#arguments)
  - [Configuration](#configuration)
  - [Credentials](#credentials)
  - [Examples](#examples)
  - [Contributing](#contributing)

## Introduction

The `aio` command line tool is designed to simplify your interactions with AI engines by providing an intuitive interface directly within your terminal. Harness the power of large language models such as the OpenAI API without leaving your command line environment.

## Installation

`aio` is currently not available in any package manager (nor crates.io).

### Install from Github releases

To install `aio`, follow these steps:

1. Download the [latest release](https://github.com/glcraft/aio/releases/latest) based on you operating system and architecture.

2. Extract the downloaded archive.

3. (optional) Copy the `aio` executable to your `PATH` (for example, `/usr/local/bin`).
    ```sh
    # sudo may be required
    sudo cp aio /usr/local/bin
    ```

### Install from source

To install `aio`, follow these steps:

1. **Prerequisites**: Make sure you have Rust installed on your system. If not, you can [install Rust](https://www.rust-lang.org/tools/install).

2. **Clone Repository**: Clone the `aio` repository to your local machine:

   ```sh
   git clone https://github.com/yourusername/aio.git
   ```

3. **Build and Install**: Navigate to the `aio` directory and build the tool:

   ```sh
   cd aio
   cargo install --path .
   ```

## Usage

Using `aio` is straightforward. In your terminal, simply invoke the tool with appropriate arguments to communicate with AI engines and receive formatted responses.

```sh
aio --engine openai:ask --input "Write an informative article about space exploration."
```

## Arguments

The `aio` command line tool supports the following arguments:

- `--config_path`: Path to the configuration file. Default is `~/.config/aio/config.yaml`.

- `--creds_path`: Path to the credentials file. Default is `~/.config/aio/creds.yaml`.

- `--engine`: Name of the AI engine to use. You can optionally append a custom prompt name from the [configuration file](#configuration) (e.g., `openai:command`).

- `--input`: User text prompt that will be used for interaction with the AI engine.

Note : `aio` can't read input from stdin for now but will in the future.

## Configuration

To fine-tune your AI interactions, you can create and modify a configuration file in YAML format. This file allows you to define prompts, messages, and parameters for different AI engines. Refer to the [Configuration Details](./docs/CONFIG.md) file for more information.

## Credentials

Secure your API credentials by storing them in a credentials file. This ensures a safe and convenient way to authenticate with AI engines. Credentials can be set up using the `creds.yaml` file. Refer to the [Credentials Details](./docs/CREDS.md) file for more information.

## Examples

Here are a few examples to get you started:

1. Generate a creative story using the OpenAI engine:
   ```sh
   aio --engine openai --input "Once upon a time in a distant galaxy..."
   ```

2. Ask for a command to extract a compressed archive:
   ```sh
   aio --engine openai:command --input "Extract a compressed archive `./archive.tar.gz` to the current directory."
   ```

## Contributing

We welcome contributions from the community to enhance the `aio` project. If you're interested in making improvements, fixing issues, or adding new features, feel free to [contribute](https://github.com/yourusername/aio/blob/main/CONTRIBUTING.md).

By contributing to `aio`, you become part of an open-source community working to improve AI interactions in the terminal.

For more detailed guidelines, please refer to our [Contribution Guide](./docs/CONTRIBUTING.md).

We appreciate your contributions and look forward to collaborating with you!
