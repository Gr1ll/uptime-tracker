mod url_check;

use std::time::Duration;

use reqwest::Error;
use url_check::check_url;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut interval = tokio::time::interval(Duration::from_secs(10));
    loop {
        interval.tick().await;

        check_url::init().await?;
    }
}
