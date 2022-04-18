use crate::property::env_property::{get_line_token, get_line_userid};
use reqwest;
use std::process;

pub async fn show_profile(){
    let user_id = get_line_userid();
    let bearer_token = get_line_token();

    let res = match reqwest::Client::new()
        .get(format!("{}{}", "https://api.line.me/v2/bot/profile/".to_string(), user_id))
        .header("authorization", format!("{}{}", "Bearer ".to_string(), bearer_token))
        .send()
        .await {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
    let res_text = match res.text().await {
        Ok(tex) => tex,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
    dbg!(res_text);
}