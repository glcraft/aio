# CLI Arguments Documentation

## Usage

`aio <COMMAND> [OPTIONS] [INPUT] `

## Common arguments
- **`--config-path <PATH>`**: Path to the configuration file.
  
  **Default:** `~/.config/config.yml`

- **`--creds-path <PATH>`**: Path to the credentials file.
  
  **Default:** `~/.cache/creds.yml`

- **`-v, --verbose...`**: Verbose mode with different levels of logging.
  
  **Count:** 
    - 0: errors only.
    - 1: warnings.
    - 2: info.
    - 3: debug.
    - 4: trace
  
  **Default:** 0

- **`-f, --formatter <CHOICE>`:** Format the completion in the terminal
  
  **Choice:**
    - **`markdown`**: Markdown display
    - **`raw`**: Raw display
  
  **Default:** markdown

- **`-r, --run <CHOICE>`**: Run code block if the language is supported
  
  **Choice:**
    - **`no`**: Doesn't run anything
    - **`ask`**: Ask to run block of code
    - **`force`**: Run code without asking
  
  **Default:** markdown

## Subcommands and Arguments

### 1. `aio api`

Used to interact with the OpenAI API.

- **Arguments**:
  - **`--model`** (or `-m`): Specifies the model to use.
  - **`--prompt`** (or `-p`): Optional. Provides an conversational prompt for the remote model. The model configuration is defined in the configuration file.

#### Usage Examples

For the **api** subcommand:
```bash
$ ./program_name api --model gpt-3.5-turbo --prompt ask
```

### 2. `aio from-file`

Used to display a file's content like the AI completion does. It supports `--formatter` and `--run` arguments.

### 3. `aio local`

Used to operate locally with a specific model.

**Arguments**:
  - **`--model`** (or `-m`): Specifies the local model to use. The model configuration is defined in the configuration file.
  - **`--prompt`** (or `-p`): Optional. Provides an conversational prompt for the local model. The prompt configuration is defined in the configuration file.

### Usage Examples

For the **api** subcommand:
```bash
$ ./program_name api --model gpt-3.5-turbo --prompt ask
```

For the **from-file** subcommand:
```bash
$ ./program_name api FromFile
```

For the **local** subcommand:
```bash
$ ./program_name engine Local --model curie
```

### Notes
- Each subcommand can be used for specific use cases, ensuring flexibility based on user needs.
- Optional arguments like `--prompt` allow for advanced customization of requests.

### Usage Examples
```bash
# Use with configuration and debug log levels
$ ./aio --config-path path/to/config.yml --creds-path path/to/creds.yml -vvv ...
```