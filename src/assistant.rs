use anyhow::Result;
use async_openai::{Client, config::OpenAIConfig};
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs,
    ChatCompletionRequestAssistantMessageArgs,
    CreateChatCompletionRequestArgs,
};
use console::Style;
use dialoguer::Input;
use indicatif::{ProgressBar, ProgressStyle};

use crate::config::Config;
use crate::shell::ShellContext;

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

    pub async fn get_command_suggestion(&self, query: &str) -> Result<(String, String)> {
        let recent_commands = self.shell_context.get_recent_commands();
        
        let mut context = String::new();
        if !recent_commands.is_empty() {
            context.push_str("Recent commands:\n");
            for cmd in recent_commands {
                context.push_str(&format!("- {}\n", cmd));
            }
        }

        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4o-mini")
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content("You are a command line assistant for zsh on MacOS. \
                        Always respond in this format: EXPLANATION|COMMAND \
                        where EXPLANATION is one sentence explaining what the command does, \
                        and COMMAND is the exact command to run. \
                        Example: 'Shows last 10 commands from history|history 10'")
                    .build()?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(format!("Context:\n{}\n\nQuery: {}", context, query))
                    .build()?
                    .into(),
            ])
            .temperature(0.3)
            .max_tokens(150u16)
            .build()?;

        let response = self.client.chat().create(request).await?;
        
        if let Some(choice) = response.choices.first() {
            if let Some(content) = &choice.message.content {
                if let Some((explanation, command)) = content.split_once('|') {
                    return Ok((explanation.trim().to_string(), command.trim().to_string()));
                }
                return Ok((content.trim().to_string(), String::new()));
            }
        }
        
        Ok((String::new(), String::new()))
    }

    pub async fn interactive_chat(&self) -> Result<()> {
        println!("{}", Style::new().blue().apply_to(
            "Starting interactive chat session (type 'exit' to quit)"
        ));

        let mut messages = vec![
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful command line assistant for MacOS. \
                    You can help with shell commands, explain terminal concepts, \
                    and provide general assistance.")
                .build()?
                .into()
        ];

        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈")
                .template("{spinner} Thinking...")?
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
                    .into()
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
                    println!("\n{} {}", 
                        Style::new().green().apply_to("Assistant:"),
                        content
                    );
                    
                    messages.push(
                        ChatCompletionRequestAssistantMessageArgs::default()
                            .content(content.clone())
                            .build()?
                            .into()
                    );
                }
            }
        }

        Ok(())
    }
}