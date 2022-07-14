use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "lib/cloud_init"]
pub struct UserData;

