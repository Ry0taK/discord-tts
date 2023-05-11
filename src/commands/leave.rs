use crate::commands::simple_resp_helper;

use serenity::{
    builder::CreateApplicationCommand, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

pub fn register(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    cmd.name("leave")
        .description("Leave from VC")
        .dm_permission(false)
}

pub async fn run(ctx: &Context, interaction: ApplicationCommandInteraction) {
    let guild_id = interaction.guild_id.unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird is not initialized.");

    let Ok(()) = manager.leave(guild_id).await else {
        simple_resp_helper(&interaction, ctx, "Not in a voice channel", true).await;
        return;
    };

    simple_resp_helper(&interaction, ctx, "Leaved!", false).await;
}
