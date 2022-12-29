use chrono::{DateTime, Utc};
use rand::prelude::*;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::component::ButtonStyle;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;
use std::time::Duration;

struct SuggestionTag(pub String);

struct Suggestion {
    pub title: String,
    pub desc: String,
    pub tags: Vec<SuggestionTag>,
    pub id: u32,
    pub time_created: DateTime<Utc>,
}

impl Suggestion {
    fn new() -> Self {
        Self {
            title: String::new(),
            desc: String::new(),
            tags: vec![],
            id: thread_rng().next_u32(),
            time_created: chrono::offset::Utc::now(),
        }
    }
}

#[command]
pub async fn create(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let channel_id = msg.channel_id;

    let mut suggestion = Suggestion::new();

    loop {
        let menu = channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("Create Suggestion: {}", suggestion.title))
                    .description("With a description")
                    .footer(|f| {
                        f.text(format!(
                            "Time created: {} | ID: {}",
                            suggestion.time_created.format("%d/%m/%Y @%H:%M:%S UTC"),
                            suggestion.id
                        ))
                    })
                    .color(Color::DARK_TEAL)
            })
            .components(|c| {
                c.create_action_row(|a| {
                    a.create_button(|b| {
                        b.label("Edit Title")
                            .style(ButtonStyle::Primary)
                            .custom_id("edit_title")
                    })
                    .create_button(|b| {
                        b.label("Edit Description")
                            .style(ButtonStyle::Primary)
                            .custom_id("edit_desc")
                    })
                    .create_button(|b| {
                        b.label("Confirm")
                            .style(ButtonStyle::Success)
                            .custom_id("send")
                    })
                    .create_button(|b| {
                        b.label("Cancel")
                            .style(ButtonStyle::Danger)
                            .custom_id("cancel")
                    })
                })
            })
        }).await?;

        let Some(interaction) = menu.await_component_interaction(ctx).timeout(Duration::from_secs(60 * 3)).await else {
            menu.reply(&ctx, "Timed out").await.expect("Unable to reply to message");
            return Ok(());
        };

        let action = &interaction.data.custom_id;
        match action.as_str() {
            "edit_title" => todo!("edit title"),
            "edit_desc" => todo!("edit desc"),
            "send" => todo!("send"),
            "cancel" => todo!("cancel"),
            _ => unreachable!(),
        }
    }
    Ok(())
}
