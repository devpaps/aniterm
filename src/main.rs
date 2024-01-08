use crate::structs::streaming::ApiStreamingResponse;
use crate::structs::{AnimeData, ApiResponse};
use clap::Parser;
use prettytable::{format, row, Table};
use serde::Deserialize;

mod structs;

/// Search for a anime or manga and display some information about it
#[derive(Parser, Debug, Deserialize)]
#[command(about, long_about = None, version)]
struct Cli {
    #[arg(long, short, num_args = 1..)]
    anime: Vec<String>,
    #[arg(long, short, num_args = 1..)]
    manga: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    match args.manga.is_empty() {
        true => get_url(args.anime, "anime").await?,
        false => get_url(args.manga, "manga").await?,
    };
    Ok(())
}

async fn get_url(
    request: Vec<String>,
    what_type: &str,
) -> Result<Option<AnimeData>, reqwest::Error> {
    let anime_name = request.join(" ");
    let mut url = String::new();
    match what_type {
        "anime" => {
            url = format!(
                "https://kitsu.io/api/edge/anime?filter%5Btext%5D={}&page%5Blimit%5D=1&page%5Boffset%5D=0",
                anime_name
            );
        }
        "manga" => {
            url = format!(
                "https://kitsu.io/api/edge/manga?filter%5Btext%5D={}&page%5Blimit%5D=1&page%5Boffset%5D=0",
                anime_name
            );
        }
        _ => println!("Error"),
    };

    let response: ApiResponse = reqwest::Client::new()
        .get(&url)
        .send()
        .await?
        .json()
        .await?;

    if response
        .data
        .as_ref()
        .expect("Something went wrong, try later")
        .is_empty()
    {
        println!("\n Nothing found, search again with a different name");
    }

    let id = response
        .data
        .as_ref()
        .and_then(|data| data.first())
        .and_then(|anime| anime.id.clone());

    let mut table4 = Table::new();
    table4.set_format(*format::consts::FORMAT_BORDERS_ONLY);

    if let Some(id) = id {
        let streaming_urls = format!(
            "https://kitsu.io/api/edge/anime/{:#?}/streaming-links",
            id.parse::<i32>().unwrap()
        );

        let streaming_links_response: ApiStreamingResponse = reqwest::Client::new()
            .get(&streaming_urls)
            .send()
            .await?
            .json()
            .await?;

        match what_type {
            "anime" => {
                if let Some(streaming_links) = streaming_links_response.data {
                    if streaming_links.is_empty() {
                        table4.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
                        table4.add_row(row!["Url:", "Not available"]);
                        table4.add_row(row!["Subs:", "Not available"]);
                        table4.add_row(row!["Dubs:", "Not available"]);
                    }
                    for streaming_link in streaming_links {
                        if let Some(attributes) = streaming_link.attributes {
                            let subs = attributes
                                .subs
                                .as_ref()
                                .map(|s| s.join(", "))
                                .unwrap_or("Not available".to_string());

                            let dubs = attributes
                                .dubs
                                .as_ref()
                                .map(|d| d.join(", "))
                                .unwrap_or("Not available".to_string());

                            let url = attributes.url.unwrap_or("Not available".to_string());

                            table4.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
                            table4.add_row(row!["Url:", url]);
                            table4.add_row(row!["Subs:", subs]);
                            table4.add_row(row!["Dubs:", dubs]);
                        }
                    }
                }
                println!("\n === Streaming Links ===");
                table4.printstd();
            }
            "manga" => {
                println!("\n === Streaming Links ===");
                table4.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
                table4.add_row(row!["Url:", "Not available"]);
                table4.add_row(row!["Subs:", "Not available"]);
                table4.add_row(row!["Dubs:", "Not available"]);
                table4.printstd();
            }
            _ => println!("Error getting streaming info"),
        };
    } else {
        table4.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table4.add_row(row!["Url:", "Not available"]);
        table4.add_row(row!["Subs:", "Not available"]);
        table4.add_row(row!["Dubs:", "Not available"]);
    }

    if let Some(anime_list) = response.data {
        anime_list.first().cloned().into_iter().for_each(|anime| {
            if let Some(attributes) = anime.attributes {
                let en_title = attributes
                    .titles
                    .as_ref()
                    .and_then(|t| t.en.clone())
                    .unwrap_or("Not available".to_string());
                let en_jp_title = attributes
                    .titles
                    .as_ref()
                    .and_then(|t| t.en_jp.clone())
                    .unwrap_or("Not available".to_string());
                let ja_jp_title = attributes
                    .titles
                    .as_ref()
                    .and_then(|t| t.ja_jp.clone())
                    .unwrap_or("Not available".to_string());
                let rating = attributes
                    .averageRating
                    .unwrap_or("Not available".to_string());
                let started = attributes.startDate.unwrap_or("Not available".to_string());
                let ended = attributes.endDate.unwrap_or("Not available".to_string());
                let meta_type = anime.type_.unwrap_or("Not available".to_string());
                let poster = attributes
                    .coverImage
                    .and_then(|p| p.original.clone())
                    .unwrap_or("Not available".to_string());
                let youtube_id = if attributes.youtubeVideoId.is_some() {
                    format!(
                        "https://www.youtube.com/watch?v={}",
                        attributes
                            .youtubeVideoId
                            .unwrap_or("Not available".to_string())
                    )
                } else {
                    "Not available".to_string()
                };

                let current_status = attributes.status.unwrap_or("Not available".to_string());
                let description = attributes
                    .description
                    .unwrap_or("Not available".to_string());

                let abbreviated_titles = attributes
                    .abbreviatedTitles
                    .and_then(|a| {
                        if a.is_empty() {
                            None
                        } else {
                            Some(a.join(",\n"))
                        }
                    })
                    .unwrap_or_else(|| "Not available".to_string());

                let episode_count = attributes
                    .episodeCount
                    .map(|e| e.to_string())
                    .unwrap_or("Not available".to_string());

                let episode_length = attributes
                    .episodeLength
                    .map(|e| e.to_string())
                    .unwrap_or("Not available".to_string());

                println!("\n === Information ===");
                let mut table = Table::new();
                table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
                table.add_row(row!["English Title", en_title]);
                table.add_row(row!["English (JP) Title", en_jp_title]);
                table.add_row(row!["Japanese Title", ja_jp_title]);
                table.add_row(row!["Abbreviated Titles", abbreviated_titles]);
                table.add_row(row!["Rating", rating]);
                table.add_row(row!["Started", started]);
                table.add_row(row!["Ended", ended]);
                table.add_row(row!["Status", current_status]);
                table.add_row(row!["Episode Count", episode_count]);
                table.add_row(row!["Episode Length", episode_length]);
                table.printstd();

                println!("\n === Media ===");
                let mut table2 = Table::new();
                table2.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
                table2.add_row(row!["Type", meta_type]);
                table2.add_row(row!["Poster", poster]);
                table2.add_row(row!["Youtube", youtube_id]);
                table2.printstd();

                println!("\n === Description ===");
                let mut table3 = Table::new();
                table3.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
                table3.add_row(row![description]);
                table3.add_empty_row();
                table3.printstd();
            }
        });
    }
    Ok(None)
}
