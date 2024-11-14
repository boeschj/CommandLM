use anyhow::Result;
use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use async_openai::{config::OpenAIConfig, Client};
use console::Style;
use dialoguer::Input;
use indicatif::{ProgressBar, ProgressStyle};

use crate::config::Config;
use crate::shell::ShellContext;

pub static OUTPUT_DELIMITER: &str = "§";

pub struct Assistant {
    client: Client<OpenAIConfig>,
    shell_context: ShellContext,
}

impl Assistant {
    pub fn new() -> Result<Self> {
        let config = Config::new();
        let api_key = config.get_api_key()?;

        let client = Client::with_config(OpenAIConfig::new().with_api_key(api_key));

        Ok(Self {
            client,
            shell_context: ShellContext::new(),
        })
    }

    pub async fn get_command_suggestion(
        &self,
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

        let system_prompt = self.get_command_suggestion_prompt();

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

        let response = self.client.chat().create(request).await?;

        if let Some(choice) = response.choices.first() {
            if let Some(content) = &choice.message.content {
                if let Some((explanation, command)) = content.split_once(&OUTPUT_DELIMITER) {
                    return Ok((explanation.trim().to_string(), command.trim().to_string()));
                }
                return Ok((content.trim().to_string(), String::new()));
            }
        }

        Ok((String::new(), String::new()))
    }

    pub async fn interactive_chat(&self) -> Result<()> {
        let os = self.shell_context.get_current_os();

        println!(
            "{}",
            Style::new()
                .blue()
                .apply_to("Starting interactive chat session (type 'exit' to quit)")
        );

        let system_prompt = format!(
            "You are a helpful command line assistant for {os}. \
                    You can help with shell commands, explain terminal concepts, \
                    and provide general assistance."
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

            let response = self.client.chat().create(request).await?;

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

    fn get_command_suggestion_prompt(&self) -> String {
        let shell_env = self.shell_context.get_current_shell();
        let os = self.shell_context.get_current_os();

        let role_and_env_assertion =
            format!("You are a command line assistant for {shell_env} on {os}.");

        let response_format =
            format!("Always response in this format: EXPLANATION{OUTPUT_DELIMITER}COMMAND");

        let format_description = format!(
            "
            where EXPLANATION is one sentence explaining what the command does, \
            and COMMAND is the exact command to run. 
            "
        );

        let prompt_example =
            format!("Example: 'Shows last 10 commands from history{OUTPUT_DELIMITER}history 10'",);

        let system_prompt = format!(
            "{role_and_env_assertion}\n\
             {response_format}\n\
             {format_description}\n\
             {prompt_example}"
        );

        return system_prompt;
    }
}
