use serenity::{framework::standard::{Args, CommandResult, macros::command}, model::prelude::*, prelude::*, };

#[command]
#[description = "Reset la guild a une structure par défaut. Pratique après un raid !"]
#[required_permissions(ADMINISTRATOR)]
async fn guild_reset(_ctx: &mut Context, _msg: &Message, _arg: Args) -> CommandResult {
    /*
    let mut sure = msg.channel_id.send_message(&ctx, |f|{
        f.embed(|e|{
            e.title("Sûr ?")
                .description("Voulez-vous vraiment réinitialiser votre serveur ?\n\
                Ceci effacera tout votre travail ! Ou un raid.\n\
                Répondez-moi \"Oui\" !")
                .field("Info", "Si ce serveur a été créé avec un template, le serveur ne reviendra pas a l'état du template !", true)
        });
        f
    }).await.unwrap();

    sure.react(&ctx, "☑️").await;

    let test = sure.await_reaction(&ctx).await.unwrap();
    if test.is_added() {
        info!("Ajouté !");
        if test.as_inner_ref().user_id.0 == msg.author.id.0 && test.as_inner_ref().emoji.as_data() == "☑️"{
            info!("C'est bien le créa !");
            let _ = msg.channel_id.delete(&ctx).await;
        }
    }
*/
    Ok(())
}
