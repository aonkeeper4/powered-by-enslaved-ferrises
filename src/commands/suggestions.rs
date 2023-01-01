use chrono::{DateTime, Utc};
use rand::prelude::*;
use serenity::builder::{CreateEmbed, CreateComponents};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::component::ButtonStyle;
use serenity::model::prelude::*;
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::prelude::*;
use serenity::utils::Color;
use std::sync::Arc;
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

    fn tags_string(&self) -> String {
        if self.tags.is_empty() {
            "No tags".to_string()
        } else {
            self.tags
                .iter()
                .map(|t| t.0.clone())
                .collect::<Vec<_>>()
                .join(", ")
        }
    }
}

fn create_menu_embed(suggestion: &Suggestion) -> CreateEmbed {
    let mut e = CreateEmbed::default();
    e.title(format!("Create Suggestion: {}", &suggestion.title))
        .description(&suggestion.desc)
        .footer(|f| {
            f.text(format!(
                "Tags: {} | Time created: {} | ID: {}",
                suggestion.tags_string(),
                suggestion.time_created.format("%d/%m/%Y @%H:%M:%S UTC"),
                suggestion.id,
            ))
        })
        .color(Color::DARK_TEAL);
    e
}

pub const INTERACTION_TIMEOUT: Duration = Duration::from_secs(60 * 3);

async fn get_interaction(ctx: &Context, msg: &mut Message, timeout: Duration, color: Color) -> CommandResult<Option<Arc<MessageComponentInteraction>>> {
    if let Some(interaction) = msg.await_component_interaction(ctx).timeout(timeout).await {
        Ok(Some(interaction))
    } else {
        // menu.reply(&ctx, "Timed out").await?;
        msg.edit(&ctx, |m| {
            m.embed(|e| {
                e.title("Timed out").description("Interaction timed out").color(color)
            })
            .set_components(CreateComponents::default())
        }).await?;
        Ok(None)
    }
}

#[command]
pub async fn create(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let channel_id = msg.channel_id;

    let mut suggestion = Suggestion::new();

    let mut menu_components = CreateComponents::default();
    menu_components.create_action_row(|a| {
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
            b.label("Edit Tags")
                .style(ButtonStyle::Primary)
                .custom_id("edit_tags")
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
    });

    let mut menu = channel_id
        .send_message(&ctx.http, |m| m.set_embed(create_menu_embed(&suggestion)).set_components(menu_components.clone()))
        .await?;

    loop {
        // TODO: possibly restructure?

        let Some(interaction) = get_interaction(ctx, &mut menu, INTERACTION_TIMEOUT, Color::DARK_TEAL).await? else {
            return Ok(());
        };
        let action = &interaction.data.custom_id;

        match action.as_str() {
            "edit_title" => {
                interaction
                    .create_interaction_response(&ctx, |r| {
                        r.kind(InteractionResponseType::UpdateMessage)
                            .interaction_response_data(|d| {
                                d.embed(|e| {
                                    e.title("Edit Title")
                                        .description(
                                            "Please type the new title of your suggestion",
                                        )
                                        .footer(|f| {
                                            f.text(format!(
                                                "Editing Suggestion ID {}",
                                                suggestion.id
                                            ))
                                        })
                                        .color(Color::DARK_TEAL)
                                })
                                .set_components(CreateComponents::default())
                            })
                    })
                    .await?;

                let Some(answer) = &msg.author.await_reply(ctx).timeout(INTERACTION_TIMEOUT).await else {
                    menu.edit(&ctx, |m| {
                        m.embed(|e| {
                            e.title("Timed out").description("Interaction timed out").color(Color::DARK_TEAL)
                        })
                    }).await?;
                    return Ok(());
                };
                let new_title = answer.content.trim().to_string();
                suggestion.title = new_title;
                answer.delete(ctx).await?;

                menu.edit(&ctx, |m| {
                    m.embed(|e| {
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
                }).await?;
            }
            "edit_desc" => {
                interaction
                    .create_interaction_response(&ctx, |r| {
                        r.kind(InteractionResponseType::UpdateMessage)
                            .interaction_response_data(|d| {
                                d.embed(|e| {
                                    e.title("Edit Description")
                                        .description(
                                            "Please type the new description of your suggestion",
                                        )
                                        .footer(|f| {
                                            f.text(format!(
                                                "Editing Suggestion ID {}",
                                                suggestion.id
                                            ))
                                        })
                                        .color(Color::DARK_TEAL)
                                })
                                .set_components(CreateComponents::default())
                            })
                    })
                    .await?;

                let Some(answer) = &msg.author.await_reply(ctx).timeout(INTERACTION_TIMEOUT).await else {
                    menu.edit(&ctx, |m| {
                        m.embed(|e| {
                            e.title("Timed out").description("Interaction timed out").color(Color::DARK_TEAL)
                        })
                        .set_components(CreateComponents::default())
                    }).await?;
                    return Ok(());
                };
                let new_desc = answer.content.trim().to_string();
                suggestion.desc = new_desc;
                answer.delete(ctx).await?;

                menu.edit(&ctx, |m| {
                    m.embed(|e| {
                        e.title("Success")
                            .description("Suggestion description was updated successfully!")
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
                }).await?;
            },
            "edit_tags" => todo!("edit tags"),
            "send" => todo!("send"),
            "cancel" => todo!("cancel"),
            _ => panic!("invalid action"),
        }

        let Some(interaction) = get_interaction(ctx, &mut menu, INTERACTION_TIMEOUT, Color::DARK_TEAL).await? else {
            return Ok(());
        };
        interaction
            .create_interaction_response(&ctx, |r| {
                r.kind(InteractionResponseType::UpdateMessage)
                    .interaction_response_data(|d| {
                        d.set_embed(create_menu_embed(&suggestion)).set_components(menu_components.clone())
                    })
            })
            .await?;
    }
    Ok(())
}
