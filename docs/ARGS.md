# CLI Arguments Documentation

## General Structure
- **Program:** Communicate with large language models and AI APIs.

## Arguments
- **`--config-path`**: Path to the configuration file.
  - **Default:** `{config_dir}/config.yml` (path depends on the system-defined location).
- **`--creds-path`**: Path to the credentials file.
  - **Default:** `{cache_dir}/creds.yml` (path depends on the system-defined location).
- **`-v, --verbose`**: Verbose mode with different levels of logging.
  - **Count:** 
    - 0 : errors only.
    - 1 : warnings.
    - 2 : info.
    - 3 : debug.

#### Subcommands
- **`engine`**: Specifies the engine used, can be followed by a custom command name from the configuration file (e.g., `openai:command`).

### Usage Examples
```bash
# Use with configuration and log levels
$ ./program_name --config-path path/to/config.yml --creds-path path/to/creds.yml -vvv

# Specify a subcommand with the engine
$ ./program_name engine openai:command
```

### Notes
- The paths for the configuration and credentials files are computed based on the user's environment and can be customized.
- Verbose mode allows controlling the level of log details displayed during the program's execution.

---

### Subcommands and Arguments Documentation

#### 1. **OpenAIAPI**
   Used to interact with the OpenAI API.

   - **Arguments**:
     - **`--model`** (or `-m`): Specifies the model to use.
     - **`--prompt`** (or `-p`): Optional. Provides an initial prompt for the model.

#### 2. **FromFile**
   This subcommand does not have specific arguments. It likely serves to load data from a file.

#### 3. **Local**
   Used to operate locally with a specific model.

   - **Arguments**:
     - **`--model`** (or `-m`): Specifies the local model to use.
     - **`--prompt`** (or `-p`): Optional. Provides an initial prompt for the local model.

### Usage Examples

For the **OpenAIAPI** subcommand:
```bash
$ ./program_name engine OpenAIAPI --model davinci --prompt "Hello"
```

For the **FromFile** subcommand:
```bash
$ ./program_name engine FromFile
```

For the **Local** subcommand:
```bash
$ ./program_name engine Local --model curie
```

### Notes
- Each subcommand can be used for specific use cases, ensuring flexibility based on user needs.
- Optional arguments like `--prompt` allow for advanced customization of requests.