use rust_embed::RustEmbed;


#[derive(RustEmbed, Default)]
#[folder = ".config/cloud_init"]
pub struct Config;
