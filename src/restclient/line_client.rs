use crate::property::env_property::{get_line_token, get_line_userid};
use reqwest;

pub async fn show_profile(){
    let user_id = get_line_userid();
    let bearer_token = get_line_token();

    let res = match reqwest::Client::new()
        .get(format!("{}{}", "https://api.line.me/v2/bot/profile/".to_string(), user_id))
        .header("authorization", format!("{}{}", "Bearer ".to_string(), bearer_token))
        .send()
        .await {
        Ok(res) => res.text().await.unwrap(),
        Err(err) => {
            // 本番運用であればpager duty等に通知をした方が良い
            panic!("{}", err);
        }
    };
    dbg!(res);
}