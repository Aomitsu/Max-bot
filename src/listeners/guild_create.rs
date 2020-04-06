use log::info;
use serenity::{model::prelude::*, prelude::*};

pub fn guild_create(ctx: Context, guild: Guild, _is_new: bool) {
    if _is_new == true {
        info!(
            "New guild named {}, with {} members",
            guild.clone().name,
            guild.clone().member_count
        );
        let _ = ctx.http.get_user(guild.owner_id.0).unwrap().direct_message(ctx.http, |m| {
            m.embed(|e| {
                e.title("Un grand merci !")
                    .description(format!("Merci beaucoup de m'avoir ajouté sur `{}` ! Cela me fait extrêmement plaisir !\n\
                    N'hésitez surtout pas a rejoindre notre support si vous avez une question.", guild.clone().name));

                e
            })
        });
    }
}
