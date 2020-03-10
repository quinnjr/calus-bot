//
//

//!

use lazy_static::lazy_static;

pub const DISCORD_API_VERSION: u8 = 6;
pub const DISCORD_GATEWAY_VERSION: u8 = 6;
pub const DISCORD_VOICE_GATEWAY_VERSION: u8 = 3;
pub const EMBED_MAX_LENGTH: u16 = 6000;
pub const LARGE_THRESHOLD: u8 = 250;
pub const MESSAGE_CODE_LIMIT: u16 = 2000;

lazy_static! {
  pub static ref USER_AGENT: &'static str = {
    use std::env;
    let version: &'static str = env!("CARGO_PKG_VERSION");
    let homepage: &'static str = env!("CARGO_PKG_HOMEPAGE");

    &["Cacophany Discord Library (", homepage, ", v", version, ")"]
      .join("")
  };

  pub static ref DISCORD_API_ENTRYPOINT: &'static str = {
    use dotenv;
    use std::env;

    dotenv::dotenv().expect("Failed to properly load the environment");

    match env::var("DISCORD_API_ENTRYPOINT") {
      Ok(val) => &val,
      Err(_) => {
        let mut buf = [0; 2];
        let num = char::from(DISCORD_API_VERSION)
          .encode_utf8(&mut buf);

        &["https://discordapp.com/api/", num].join("")
      }
    }
  };

  pub static ref DISCORD_GATEWAY_ENTRYPOINT: &'static str = {
    use dotenv;
    use std::env;

    dotenv::dotenv().expect("Failed to properly load the environment");

    match env::var("DISCORD_GATEWAY_ENTRYPOINT") {
      Ok(val) => &val,
      Err(_) => "https://discorapp.com/api/gateway"
    }
  };

  pub static ref DISCORD_GATEWAY_BOT_ENTRYPOINT: &'static str = {
    use dotenv;
    use std::env;

    dotenv::dotenv().expect("Failed to properly load the environment");

    match env::var("DISCORD_GATEWAY_ENTRYPOINT") {
      Ok(val) => &val,
      Err(_) => "https://discordapp.com/api/gateway/bot"
    }
  };

  pub static ref JOIN_MESSAGES: Vec<&'static str> = {
    vec![
      "$user just joined the server - glhf!",
      "$user just joined. Everyone, look busy!",
      "$user just joined. Can I get a heal?",
      "$user joined your party.",
      "$user joined. You must construct additional pylons.",
      "Ermagherd. $user is here.",
      "Welcome, $user. Stay awhile and listen.",
      "Welcome, $user. We were expecting you ( ͡° ͜ʖ ͡°)",
      "Welcome, $user. We hope you brought pizza.",
      "Welcome $user. Leave your weapons by the door.",
      "A wild $user appeared.",
      "Swoooosh. $user just landed.",
      "Brace yourselves. $user just joined the server.",
      "$user just joined... or did they?",
      "$user just arrived. Seems OP - please nerf.",
      "$user just slid into the server.",
      "A $user has spawned in the server.",
      "Big $user showed up!",
      "Where’s $user? In the server!",
      "$user hopped into the server. Kangaroo!!",
      "$user just showed up. Hold my beer.",
      "Challenger approaching - $user has appeared!",
      "It's a bird! It's a plane! Nevermind, it's just $user.",
      "It's $user! Praise the sun! \u{005C}\u{005B}T\u{0005D}\u{002F}",
      "Never gonna give $user up. Never gonna let $user down.",
      "$user has joined the battle bus.",
      "Cheers, love! $user's here!",
      "Hey! Listen! $user has joined!",
      "We've been expecting you $user",
      "It's dangerous to go alone, take $user!",
      "$user has joined the server! It's super effective!",
      "Cheers, love! $user is here!",
      "$user is here, as the prophecy foretold.",
      "$user has arrived. Party's over.",
      "Ready player $user",
      "$user is here to kick butt and chew bubblegum. And $user is all out of gum.",
      "Hello. Is it $user you're looking for?",
      "$user has joined. Stay a while and listen!",
      "Roses are red, violets are blue, $user joined this server with you"
    ]
  };
}
