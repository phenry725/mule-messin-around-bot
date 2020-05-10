// extern crate rand;
//
// use rand::Rng;
use serenity::client::Client;

use serenity::{
    framework::{
        standard::macros::{command, group},
        standard::CommandResult,
        StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
};
use serenity::prelude::{Context, EventHandler};

#[group]
#[commands(ping, stupidspeak, reverse)]
struct General;

use std::env;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!")) // set the bots prefix to "!"
            .before(|_ctx, message, command_name| {
                println!(
                    "Got command '{}' by user '{}'",
                    command_name, message.author.id
                );
                true
            })
            .group(&GENERAL_GROUP),
    );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

#[command]
fn stupidspeak(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, &msg.content)?;

    Ok(())
}

#[command]
fn reverse(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, &msg.content.chars().rev().collect::<String>())?;

    Ok(())
}

// fn randomNumber(start: int, end: int) -> int {
//     return rng.gen_range(0, 10));
// }
