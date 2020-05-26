use serenity::model::prelude::Message;
use serenity::client::Context;

pub(crate) async fn error_user(ctx: &mut Context, msg: &Message, id: u64){
    let _ = msg.channel_id.send_message(&ctx,
                                |f| f.embed( |e|
                                    e.title("Erreur !")
                                        .description(format!("L'utilisateur `{}` n'est pas présent sur le serveur !", id))

    )).await;
}