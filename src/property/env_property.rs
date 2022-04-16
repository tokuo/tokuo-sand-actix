use std::{env, process};

pub fn get_line_token() -> String {
    match env::var("LINE_BOT_CHANNEL_TOKEN") {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, "LINE_BOT_CHANNEL_TOKEN");
            process::exit(1);
        },
    }
}

pub fn get_line_userid() -> String {
    match env::var("LINE_BOT_USERID") {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, "LINE_BOT_USERID");
            process::exit(1);
        },
    }
}