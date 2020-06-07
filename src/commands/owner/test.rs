use serenity::{
    framework::standard::{Args, CommandResult, macros::command},
    model::prelude::*,
    prelude::*,
};
use rustbreak::Database;
use serde::{Serialize, Deserialize};

#[command]
#[owners_only]
async fn test(_ctx: &Context, _msg: &Message, _arg: Args) -> CommandResult {

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        test: String,
        test2: String,
    }

    let db = Database::open("my_contacts").unwrap();

    /*let test = Test{
        test: "Abcd".to_string()
    };*/

    db.insert("Lapfox", "lapfoxtrax.com").unwrap();
    //db.insert("Rust", test).unwrap();

    let rust : String = db.retrieve("Rust").unwrap();
    println!("You can find Rust at: {}", rust);
    db.flush().unwrap();

    Ok(())
}
