const TOP_STORIES_URL: &str = "https://hacker-news.firebaseio.com/v0/topstories.json";

#[derive(serde::Deserialize)]
struct Story {
    title: String,
    by: String,
    score: u32,
    url: Option<String>,
}

fn main() {
    println!("Top 10 Hacker News Stories");

    let client = reqwest::blocking::Client::new();

    let top_ids: Vec<u64> = client
        .get(TOP_STORIES_URL)
        .send()
        .unwrap()
        .json()
        .expect("Failed to parse story ids");

    for (i, id) in top_ids.iter().take(10).enumerate() {
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
    }
}
