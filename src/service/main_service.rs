use crate::property::file_property::{read_resource_file};
use crate::restclient::line_client;

pub async fn hoge() {
    read_resource_file("hoge.yml");

    line_client::show_profile().await;
}