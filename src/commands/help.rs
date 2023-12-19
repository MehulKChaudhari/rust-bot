use serenity::model::channel::Message;
use serenity::prelude::*;

const HELP_MESSAGE: &str = "
Hello there, **Welcome to Real Dev Squad!**

If you are new here, head to the `#lift-simulation` channel for your first task.

❓ Something wrong?
➡️ You can flag an admin with @admin

Need assistance or have specific questions? Feel free to ask our friendly community members or moderators. Enjoy your time here at Real Dev Squad!

I hope that resolves your issue! For more information about our bot's commands, type `!help` in any channel.

— HelpBot 🤖
";

pub async fn help_command(ctx: &Context, msg: &Message) {
    if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
        println!("Error sending message: {:?}", why);
    }
}
