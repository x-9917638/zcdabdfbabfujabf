const TOP_STORIES_URL: &str = "https://hacker-news.firebaseio.com/v0/topstories.json";
const NEW_STORIES_URL: &str = "https://hacker-news.firebaseio.com/v0/newstories.json";
const BEST_STORIES_URL: &str = "https://hacker-news.firebaseio.com/v0/beststories.json";

const ASK_STORIES_URL: &str = "https://hacker-news.firebaseio.com/v0/askstories.json";
const SHOW_STORIES_URL: &str = "https://hacker-news.firebaseio.com/v0/showstories.json";
const JOB_STORIES_URL: &str = "https://hacker-news.firebaseio.com/v0/jobstories.json";

mod cli;

use clap::Parser;
use cli::Cli;
use reqwest::blocking::Client;

#[derive(serde::Deserialize)]
struct Story {
    title: String,
    by: String,
    score: u32,
    descendants: Option<u32>,
    url: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let client = reqwest::blocking::Client::new();

    let url = {
        if let Some(story_type) = cli.arg {
            use cli::StoryType as S;
            match story_type {
                S::Ask => {
                    println!("Newest 10 Ask Hacker News Stories");
                    ASK_STORIES_URL
                }
                S::Job => {
                    println!("Newest 10 Hacker News Jobs");
                    JOB_STORIES_URL
                }
                S::Best => {
                    println!("Best 10 Hacker News Stories");
                    BEST_STORIES_URL
                }
                S::New => {
                    println!("Newest 10 Hacker News Stories");
                    NEW_STORIES_URL
                }
                S::Show => {
                    println!("Newest 10 Show Hacker News Stories");
                    SHOW_STORIES_URL
                }
                S::Top => {
                    println!("Top 10 Hacker News Stories");
                    TOP_STORIES_URL
                }
            }
        } else {
            println!("Top 10 Hacker News Stories");
            TOP_STORIES_URL
        }
    };
    let ids = get_ids(&client, url);

    for (i, id) in ids.iter().take(10).enumerate() {
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{id}.json");

        let story: Story = client
            .get(url)
            .send()
            .unwrap()
            .json()
            .expect(&format!("Failed to parse story with id {id}"));

        let link = story.url.unwrap_or("(No URL)".to_string());
        println!(
            "{}, {}, {} points by {}",
            i + 1,
            story.title,
            story.score,
            story.by
        );
        if let Some(descendants) = story.descendants {
            println!("  {} Comments", descendants);
        }
        println!("  {}\n", link);
    }
}

fn get_ids(client: &Client, url: &str) -> Vec<u64> {
    client
        .get(url)
        .send()
        .unwrap()
        .json()
        .expect("Failed to parse story ids")
}
