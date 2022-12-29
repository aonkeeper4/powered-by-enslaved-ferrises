use chrono::{DateTime, Utc};
use rand::prelude::*;
use serenity::builder::CreateMessage;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::component::{ButtonStyle, InputTextStyle};
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
            title: "New Suggestion".into(),
            desc: "Description".into(),
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

    let menu = channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("Create Suggestion: {}", &suggestion.title))
                    .description(&suggestion.desc)
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
        })
        .await?;

    loop {
        // TODO: possibly restructure?

        let Some(interaction) = menu.await_component_interaction(ctx).timeout(Duration::from_secs(60 * 3)).await else {
            menu.reply(&ctx, "Timed out").await?;
            return Ok(());
        };
        let action = &interaction.data.custom_id;

        match action.as_str() {
            "edit_title" => {
                println!("editing title");
                interaction
                    .create_interaction_response(&ctx, |r| {
                        r.kind(InteractionResponseType::UpdateMessage)
                            .interaction_response_data(|d| {
                                d.embed(|e| {
                                    e.title("Edit Title")
                                        .description(
                                            "Please input the new title of your suggestion",
                                        )
                                        .footer(|f| {
                                            f.text(format!(
                                                "Editing Suggestion ID {}",
                                                suggestion.id
                                            ))
                                        })
                                        .color(Color::DARK_TEAL)
                                })
                                .components(|c| {
                                    c.create_action_row(|a| {
                                        a.create_input_text(|i| {
                                            i.label("New title:")
                                                .custom_id("new_title")
                                                .style(InputTextStyle::Short)
                                        })
                                    })
                                })
                            })
                    })
                    .await?;

                let Some(interaction) = menu.await_component_interaction(ctx).timeout(Duration::from_secs(60 * 3)).await else {
                    menu.reply(&ctx, "Timed out").await?;
                    return Ok(());
                };
                let new_title = &interaction.data.values[0];
                suggestion.title = new_title.trim().to_string();

                interaction
                    .create_interaction_response(&ctx, |r| {
                        r.kind(InteractionResponseType::UpdateMessage)
                            .interaction_response_data(|d| {
                                d.embed(|e| {
                                    e.title("Success")
                                        .description("Suggestion title was updated successfully!")
                                        .footer(|f| {
                                            f.text(format!(
                                                "Editing Suggestion ID {}",
                                                suggestion.id
                                            ))
                                        })
                                        .color(Color::DARK_TEAL)
                                })
                                .components(|c| {
                                    c.create_action_row(|a| {
                                        a.create_button(|b| {
                                            b.label("Continue")
                                                .style(ButtonStyle::Success)
                                                .custom_id("continue")
                                        })
                                    })
                                })
                            })
                    })
                    .await?;
            }
            "edit_desc" => todo!("edit desc"),
            "send" => todo!("send"),
            "cancel" => todo!("cancel"),
            _ => panic!("invalid action"),
        }

        let Some(interaction) = menu.await_component_interaction(ctx).timeout(Duration::from_secs(60 * 3)).await else {
            menu.reply(&ctx, "Timed out").await?;
            return Ok(());
        };
        interaction
            .create_interaction_response(&ctx, |r| {
                r.kind(InteractionResponseType::UpdateMessage)
                    .interaction_response_data(|d| {
                        d.embed(|e| {
                            e.title(format!("Create Suggestion: {}", &suggestion.title))
                                .description(&suggestion.desc)
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
                    })
            })
            .await?;
    }
    Ok(())
}
