use futures::future::join_all;
use reqwest::Error;

struct UrlCheck {
    url: String,
    is_alive: bool,
}

pub async fn init() -> Result<(), Error> {
    let urls = vec![
        is_url_alive("https://cyrilk.dev"),
        is_url_alive("https://api.cyrilk.dev/about-me"),
        is_url_alive("https://goblinfpv.com"),
    ];
    let mut urls_unavailable = Vec::new();

    let result = join_all(urls).await;

    result.iter().for_each(|item| {
        if item.is_alive {
            println!("available {:?}", item.url);
        } else {
            println!("unavailable {:?}", item.url);
            urls_unavailable.push(&item.url);
        }
    });

    Ok(())
}

async fn is_url_alive(url: &str) -> UrlCheck {
    let response = reqwest::get(url).await;

    match response {
        Ok(res) => UrlCheck {
            url: url.to_string(),
            is_alive: res.status().is_success(),
        },
        Err(_) => UrlCheck {
            url: url.to_string(),
            is_alive: false,
        },
    }
}
