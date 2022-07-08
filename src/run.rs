use rust_embed::RustEmbed;


#[derive(RustEmbed, Default)]
#[folder = ".run"]
pub struct Run;
