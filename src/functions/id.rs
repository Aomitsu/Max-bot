pub(crate) async fn str_to_uid(string: Box<str>) -> String {
    string.replace("<@", "")
        .replace("!", "")
        .replace(">", "")
}