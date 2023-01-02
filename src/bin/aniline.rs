use aniline::Client;
use clap::{Parser, Subcommand};
use crossterm::style::Stylize;
use std::collections::HashMap;
use std::io::{self};
use terminal_menu::{button, menu, mut_menu, run, TerminalMenuItem};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Searches an anime on myanimelist.org
    Search {
        query: String,
        /// Show all information available for an anime
        #[arg(short, long)]
        all: bool,
        /// Number of animes to show
        #[arg(short, long, default_value_t = 5)]
        limit: u8,
    },
}

fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();
    let client = Client::new();

    match &cli.command {
        Commands::Search {
            query,
            all: _,
            limit,
        } => {
            let search_result = client.search(query);
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
