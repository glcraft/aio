# Credentials Configuration Documentation

## Table of Contents

- [Credentials Configuration Documentation](#credentials-configuration-documentation)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [File Structure](#file-structure)
  - [Adding Credentials](#adding-credentials)
  - [Security](#security)

## Introduction

The credentials configuration file is used to store sensitive information, such as API keys and other secrets required for authenticating with various APIs. In the context of the `aio` project, this file stores the API keys used for interacting with AI engines like the OpenAI API. This is a separate file from `config.yaml` configuration file to easily share the configuration without compromising security.

By default, `aio` will try to read the credentials configuration file from `~/.config/aio/creds.yaml`. You can also specify the path to the credentials configuration file using the `--creds-path` argument. For example: `aio --creds-path ./creds.yaml`.

## File Structure

The credentials configuration file follows a simple YAML structure.

Here's an example of the full structure:

```yaml
openai:
    api_key: <openai_api_key>
```

As the project evolves, additional sections may be added to accommodate other engines or APIs.

## Adding Credentials

To add API credentials to the configuration file:

1. Open the credentials configuration file (`creds.yaml`) in a text editor.

2. Locate the relevant section for the API you want to add credentials for. In the current state of the project, this section will likely be `openai`.

3. Replace `<openai_api_key>` with your actual OpenAI API key. The API key is typically provided by the service provider when you sign up for API access.

4. Save the file after adding the necessary credentials.

5. Ensure that the permissions for the configuration file are set securely to prevent unauthorized access.

## Security

Security is paramount when handling API keys and secrets. Here are some best practices to keep in mind:

- **Restricted Access**: Limit access to the credentials configuration file. Only authorized individuals should have access to this file.

- **Secure Storage**: Store the credentials configuration file in a secure location, accessible only by authorized users.

- **Git Ignoring**: Ensure that the credentials configuration file is ignored by version control systems like Git. Never commit sensitive information to a public repository.

- **Environment Variables**: Whenever possible, use environment variables to store and access API keys. This provides an additional layer of security and keeps sensitive data out of your codebase.

- **Regular Updates**: Keep your credentials up to date. If you regenerate an API key, remember to update it in the configuration file as well.

Remember that the security of your credentials is your responsibility. Following these best practices helps safeguard your sensitive information.

---

*Note: This documentation provides guidelines for working with the credentials configuration file in the `aio` project. Customize it based on your specific use case and evolving project requirements.*