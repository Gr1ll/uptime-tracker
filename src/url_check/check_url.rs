use std::{
    sync::{Mutex, OnceLock},
    time::Duration,
};

use futures::future::join_all;
use reqwest::Error;
use tokio::task::JoinHandle;

struct UrlCheck {
    url: String,
    is_alive: bool,
}

static HANDLES: OnceLock<Mutex<Vec<JoinHandle<()>>>> = OnceLock::new();

pub async fn init() -> Result<(), Error> {
    let urls = vec![
        is_url_alive("https://cyrilk.dev"),
        is_url_alive("https://api.cyrilk.dev/about-me"),
        is_url_alive("https://goblinfpv.com"),
    ];
    let mut urls_unavailable = Vec::new();
    let mut handles = get_handles().lock().unwrap();

    for handle in handles.iter() {
        handle.abort();
    }
    handles.clear();

    let result = join_all(urls).await;

    result.iter().for_each(|item| {
        if item.is_alive {
            println!("sending no alert but, available {:?}", item.url);
        } else {
            println!("sending alert unavailable {:?}", item.url);
            urls_unavailable.push(item.url.clone());
        }
    });

    let handle = tokio::spawn(check_unavailable_urls(urls_unavailable));
    handles.push(handle);
    Ok(())
}

async fn check_unavailable_urls(mut urls_unavailable: Vec<String>) {
    if urls_unavailable.is_empty() {
        return;
    }

    let mut interval = tokio::time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        let result = join_all(urls_unavailable.iter().map(|item| is_url_alive(item))).await;

        result.iter().filter(|item| item.is_alive).for_each(|item| {
            println!("url {} back online", item.url);
            let alive_item_position = urls_unavailable.iter().position(|x| x == &item.url);
            urls_unavailable.remove(alive_item_position.unwrap());
        });

        if urls_unavailable.is_empty() {
            return;
        }
    }
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

fn get_handles() -> &'static Mutex<Vec<JoinHandle<()>>> {
    HANDLES.get_or_init(|| Mutex::new(Vec::new()))
}
