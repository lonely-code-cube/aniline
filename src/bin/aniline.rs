use aniline::{AnilineError, Client};
use aniline::{Video, Videos};
use clap::{Parser, Subcommand};
use crossterm::style::Stylize;
use spinners::{Spinner, Spinners};
use std::collections::HashMap;
use std::process;
use terminal_menu::{button, label, menu, mut_menu, run, TerminalMenuItem};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search an anime on myanimelist.org
    Search {
        query: String,
        /// Show all information available for an anime
        #[arg(short, long)]
        all: bool,
        /// Number of animes to show
        #[arg(short, long, default_value_t = 5)]
        limit: u8,
    },
    /// Play an anime
    Play {
        query: String,
        /// Play in VLC [default MPV]
        #[arg(short, long)]
        vlc: bool,
        /// Select video quality
        #[arg(short, long, default_value_t = String::from("default"))]
        quality: String,
    },
}

fn main() -> Result<(), AnilineError> {
    let client = Client::new();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Search {
            query,
            all: _,
            limit,
        } => {
            let mut sp = Spinner::new(
                Spinners::Dots12,
                format!("{}", "Searching anime".yellow().bold()),
            );
            let search_result = client.search(query);
            sp.stop();
            print!("\x1b[2K\r");
            match search_result {
                Err(e) => {
                    println!("\n{}\n", "Error occured while fetching resources".red());
                    println!(
                        "Please open an issue at {}",
                        "https://github.com/lonely-code-cube/aniline/issues".bold()
                    );
                    println!("\n{}", format!("{}", e).red());
                }
                Ok(data) => {
                    let mut selections = Vec::<TerminalMenuItem>::new();
                    let mut items = HashMap::new();
                    for i in 0..*limit {
                        let anime = data.data.get(i as usize);
                        if let Some(anime) = anime {
                            let string = format!(
                                "{} ({})",
                                anime.title.clone(),
                                format!("{}", anime.mal_id).bold()
                            );
                            selections.push(button(&string));
                            items.insert(string, anime);
                        } else {
                            break;
                        }
                    }
                    // let esc_event = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
                    // let menu_event = KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE);
                    let anime = *items.get(&get_selection(selections)).unwrap();
                    println!(
                        "\n\t{} [{}]\n\n{} {} (by {})\n{} #{}\n{} {}\n{} {}\n{} {}\n{} {}\n\n{}\n",
                        anime.title.clone().bold().underlined().green(),
                        format!("{}", anime.mal_id).bold(),
                        "Rating:".green(),
                        format!("{}", anime.score.unwrap_or(0.0)).bold(),
                        format!("{}", anime.scored_by.unwrap_or(0)).bold(),
                        "Rank:".green(),
                        format!("{}", anime.rank.unwrap_or(0)).bold(),
                        "Episodes:".green(),
                        format!("{}", anime.episodes.unwrap_or(0)).bold(),
                        "Status:".green(),
                        format!("{}", anime.status.clone().unwrap_or("Unknown".to_string())).bold(),
                        "Duration:".green(),
                        format!(
                            "{}",
                            anime.duration.clone().unwrap_or("Unknown".to_string())
                        )
                        .bold(),
                        "Genres:".green(),
                        format!(
                            "{}",
                            anime
                                .genres
                                .clone()
                                .unwrap_or(vec![])
                                .iter()
                                .map(|x| x.name.clone().unwrap())
                                .collect::<Vec<String>>()
                                .join(", ")
                                .bold()
                        ),
                        format!(
                            "{}",
                            anime.synopsis.clone().unwrap_or("Unknown".to_string())
                        ),
                    );
                }
            }
        }
        Commands::Play {
            query,
            vlc,
            quality,
        } => {
            let mut items = HashMap::new();
            let mut selections = Vec::<TerminalMenuItem>::new();
            let mut sp = Spinner::new(
                Spinners::Dots12,
                format!("{}", " [1/4] Searching anime".yellow().bold()),
            );
            for i in client.get_all_anime_query_res(query)? {
                selections.push(button(&i.name));
                items.insert(i.name.to_owned(), i);
            }
            sp.stop();
            println!();
            if selections.len() == 0 {
                println!("{}", "\nNo anime found".red().bold());
                process::exit(0);
            }
            let anime = items.get(&get_selection(selections)).unwrap();
            let mut selections = Vec::<TerminalMenuItem>::new();
            selections.push(label("Select episode"));
            for i in 1..=anime.available_episodes.sub {
                selections.push(button(i.to_string()));
            }
            if selections.len() == 0 {
                println!("{}", "\nNo episodes available".red().bold());
                process::exit(0);
            }
            let episode = get_selection(selections).parse::<usize>().unwrap();
            let mut sp = Spinner::new(
                Spinners::Dots12,
                format!("{}", " [2/4] Getting video ID".yellow().bold()),
            );
            let id = client.get_all_anime_video_id(anime, episode)?;
            sp.stop();
            println!();
            match id {
                None => {
                    println!("{}", "\nStreaming links yeilded 0 results.".red().bold());
                    process::exit(0);
                }
                Some((_service, id)) => {
                    let mut sp = Spinner::new(
                        Spinners::Dots12,
                        format!("{}", " [3/4] Getting video URLs".yellow().bold()),
                    );
                    let cvideos = client.get_video_urls(&id)?;
                    sp.stop();
                    println!();
                    if cvideos.links.len() == 0 {
                        println!("{}", "\nVideo URL could not be obtained".red().bold());
                        process::exit(0);
                    }
                    let videos = cvideos.clone();
                    let video = match quality.as_str() {
                        "1080" => get_quality(videos, "Mp4-1080p"),
                        "720" => get_quality(videos, "Mp4-720p"),
                        "480" => get_quality(videos, "Mp4-480p"),
                        "360" => get_quality(videos, "Mp4-360p"),
                        "270" => get_quality(videos, "Mp4-270p"),
                        "144" => get_quality(videos, "Mp4-144p"),
                        "default" => Some(videos.links[0].clone()),
                        _ => None,
                    };
                    let video = match video {
                        Some(v) => v,
                        None => {
                            println!(
                                "{} {}",
                                "Specified video quality not found, defaulting to"
                                    .yellow()
                                    .bold(),
                                format!("{}", cvideos.links[0].res).green().bold()
                            );
                            cvideos.links[0].clone()
                        }
                    };
                    println!("{}", "    [4/4] Playback in progress".yellow().bold());
                    play(*vlc, video, &anime.name)?;
                }
            }
        }
    }

    Ok(())
}

fn get_selection(selections: Vec<TerminalMenuItem>) -> String {
    let selections = menu(selections);
    run(&selections);
    let menu = mut_menu(&selections);
    let selection = menu.selected_item_name();
    selection.to_string()
}

fn get_quality(videos: Videos, quality: &str) -> Option<Video> {
    for video in videos.links {
        if quality == &video.res {
            return Some(video);
        }
    }
    None
}

fn play(vlc: bool, video: Video, title: &str) -> Result<(), AnilineError> {
    use subprocess::{Popen, PopenConfig, Redirection};
    match vlc {
        false => {
            Popen::create(
                &["mpv", &video.link, &format!("--title=\"{}\"", title)],
                PopenConfig {
                    stdin: Redirection::Pipe,
                    ..Default::default()
                },
            )?;
        }
        true => {
            Popen::create(
                &["vlc", &video.link, &format!("--meta-title=\"{}\"", title)],
                PopenConfig {
                    stdin: Redirection::Pipe,
                    ..Default::default()
                },
            )?;
        }
    }
    Ok(())
}
