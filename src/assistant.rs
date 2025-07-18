use anyhow::Result;
use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use async_openai::{config::OpenAIConfig, Client};
use console::Style;
use dialoguer::Input;
use indicatif::{ProgressBar, ProgressStyle};

use crate::config::get_api_key;
use crate::shell::ShellContext;

pub static OUTPUT_DELIMITER: &str = "§";

pub fn create_client(shell_context: &ShellContext) -> Result<Client<OpenAIConfig>> {
    let api_key = get_api_key(&shell_context.shell_type)?;
    Ok(Client::with_config(OpenAIConfig::new().with_api_key(api_key)))
}

pub async fn get_command_suggestion(
    client: &Client<OpenAIConfig>,
    shell_context: &ShellContext,
    query: &str,
    context: Option<&str>,
) -> Result<(String, String)> {
    let mut context_str = String::new();

    // Add piped input context if provided
    if let Some(piped_context) = context {
        context_str.push_str("Input context:\n");
        context_str.push_str(piped_context);
        context_str.push_str("\n\n");
    };

    let system_prompt = get_command_suggestion_prompt(shell_context);

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-mini")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(system_prompt)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(format!("Context:\n{:#?}\n\nQuery: {}", context, query))
                .build()?
                .into(),
        ])
        .temperature(0.3)
        .max_tokens(150u16)
        .build()?;

    let response = client.chat().create(request).await?;

    if let Some(choice) = response.choices.first() {
        if let Some(content) = &choice.message.content {
            if let Some((explanation, command)) = content.split_once(OUTPUT_DELIMITER) {
                return Ok((explanation.trim().to_string(), command.trim().to_string()));
            }
            return Ok((content.trim().to_string(), String::new()));
        }
    }

    Ok((String::new(), String::new()))
}

pub async fn interactive_chat(
    client: &Client<OpenAIConfig>,
    shell_context: &ShellContext,
) -> Result<()> {
    println!(
        "{}",
        Style::new()
            .blue()
            .apply_to("Starting interactive chat session (type 'exit' to quit)")
    );

    let system_prompt = format!(
        "You are a helpful command line assistant for {}. \
                You can help with shell commands, explain terminal concepts, \
                and provide general assistance.",
        shell_context.os
    );

    let mut messages = vec![ChatCompletionRequestSystemMessageArgs::default()
        .content(system_prompt)
        .build()?
        .into()];

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈")
            .template("{spinner} Thinking...")?,
    );

    loop {
        let query: String = Input::new().with_prompt("You").interact()?;

        if query.eq_ignore_ascii_case("exit") || query.eq_ignore_ascii_case("quit") {
            break;
        }

        messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(query)
                .build()?
                .into(),
        );

        spinner.enable_steady_tick(std::time::Duration::from_millis(80));

        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4o-mini")
            .messages(messages.clone())
            .temperature(0.7)
            .build()?;

        let response = client.chat().create(request).await?;

        spinner.disable_steady_tick();

        if let Some(choice) = response.choices.first() {
            if let Some(content) = &choice.message.content {
                println!(
                    "\n{} {}",
                    Style::new().green().apply_to("Assistant:"),
                    content
                );

                messages.push(
                    ChatCompletionRequestAssistantMessageArgs::default()
                        .content(content.clone())
                        .build()?
                        .into(),
                );
            }
        }
    }

    Ok(())
}

fn get_command_suggestion_prompt(shell_context: &ShellContext) -> String {
    let role_and_env_assertion = format!(
        "You are a command line assistant for {} on {}.",
        shell_context.shell_type, shell_context.os
    );

    let response_format = format!(
        "Always respond in this format: EXPLANATION{OUTPUT_DELIMITER}COMMAND"
    );

    let format_description = "
        where EXPLANATION is one sentence explaining what the command does, \
        and COMMAND is the exact command to run.
        ";

    let prompt_example = format!(
        "Example: 'Shows last 10 commands from history{OUTPUT_DELIMITER}history 10'"
    );

    format!(
        "{}\n{}\n{}\n{}",
        role_and_env_assertion, response_format, format_description, prompt_example
    )
}
