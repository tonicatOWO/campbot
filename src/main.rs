use dotenv::dotenv;
use lazy_static::lazy_static;
use serenity::{
    async_trait,
    model::{
        channel::Reaction,
        gateway::Ready,
        id::{MessageId, RoleId},
        prelude::*,
    },
    prelude::*,
};
use std::env;

lazy_static! {
    static ref TARGET_MESSAGE_ID: MessageId = {
        let id_str = env::var("TARGET_MESSAGE_ID").expect("MESSAGE_ID not found");
        MessageId::new(id_str.parse().expect("Invalid MESSAGE_ID"))
    };
    static ref TARGET_ROLE_ID: RoleId = {
        let id_str = env::var("TARGET_ROLE_ID").expect("ROLE_ID not found");
        RoleId::new(id_str.parse().expect("Invalid ROLE_ID"))
    };
}

const TARGET_EMOJI: &str = "✅";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} 已上線！", ready.user.name);
    }

    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        if add_reaction.message_id != *TARGET_MESSAGE_ID {
            return;
        }

        let emoji_matches = match &add_reaction.emoji {
            ReactionType::Unicode(emoji) => emoji == TARGET_EMOJI,
            _ => false,
        };

        if !emoji_matches {
            return;
        }

        if let Some(user_id) = add_reaction.user_id {
            if let Ok(user) = user_id.to_user(&ctx.http).await {
                if user.bot {
                    return;
                }
            }

            if let Some(guild_id) = add_reaction.guild_id {
                match guild_id.member(&ctx.http, user_id).await {
                    Ok(member) => match member.add_role(&ctx.http, *TARGET_ROLE_ID).await {
                        Ok(_) => println!("成功給用戶 {} 添加角色 (✅ reaction)", user_id),
                        Err(why) => println!("添加角色失敗: {:?}", why),
                    },
                    Err(why) => println!("無法獲取成員資訊: {:?}", why),
                }
            }
        }
    }

    async fn reaction_remove(&self, ctx: Context, removed_reaction: Reaction) {
        if removed_reaction.message_id != *TARGET_MESSAGE_ID {
            return;
        }

        let emoji_matches = match &removed_reaction.emoji {
            ReactionType::Unicode(emoji) => emoji == TARGET_EMOJI,
            _ => false,
        };

        if !emoji_matches {
            return;
        }

        if let Some(user_id) = removed_reaction.user_id {
            if let Ok(user) = user_id.to_user(&ctx.http).await {
                if user.bot {
                    return;
                }
            }

            if let Some(guild_id) = removed_reaction.guild_id {
                match guild_id.member(&ctx.http, user_id).await {
                    Ok(member) => match member.remove_role(&ctx.http, *TARGET_ROLE_ID).await {
                        Ok(_) => println!("成功移除用戶 {} 的角色 (移除 ✅ reaction)", user_id),
                        Err(why) => println!("移除角色失敗: {:?}", why),
                    },
                    Err(why) => println!("無法獲取成員資訊: {:?}", why),
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("未找到 DISCORD_TOKEN 環境變數");

    let intents = GatewayIntents::GUILD_MESSAGE_REACTIONS | GatewayIntents::GUILDS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("創建 client 失敗");

    if let Err(why) = client.start().await {
        println!("Client 錯誤: {:?}", why);
    }
}
