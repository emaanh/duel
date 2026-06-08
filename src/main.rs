use std::io::{self, Write};
use std::path::Path;

use anyhow::Result;
use clap::Parser;

mod elo;
mod session;

use session::Session;

#[derive(Parser)]
#[command(name = "duel", about = "Rank anything with Elo-powered pairwise comparisons")]
struct Cli {
    /// Path to the file containing items (one per line)
    file: String,

    /// Show current rankings without dueling
    #[arg(long, short)]
    results: bool,

    /// Save results to a file
    #[arg(long, short)]
    output: Option<String>,
}

fn format_rankings(session: &Session) -> String {
    let rankings = session.sorted_rankings();
    let mut lines = vec![
        format!("Rankings  ({} comparisons total)\n", session.total_comparisons),
        format!("{:<5} {:<8} {:<6} {:<6}  Item", "Rank", "Score", "W", "L"),
        "─".repeat(50),
    ];

    for (i, item) in rankings.iter().enumerate() {
        lines.push(format!(
            "{:<5} {:<8.0} {:<6} {:<6}  {}",
            i + 1,
            item.elo,
            item.wins,
            item.losses,
            item.name
        ));
    }

    lines.join("\n")
}

fn print_rankings(session: &Session, output: Option<&str>) -> Result<()> {
    let text = format_rankings(session);
    println!("{}", text);

    if let Some(path) = output {
        std::fs::write(path, format!("{}\n", text))?;
        println!("\nSaved to {}", path);
    }

    Ok(())
}

fn progress_bar(done: u32, target: u32, width: usize) -> String {
    let filled = if target == 0 {
        width
    } else {
        ((done as f64 / target as f64) * width as f64).min(width as f64) as usize
    };
    let empty = width.saturating_sub(filled);
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

fn run_duel(session: &mut Session, source: &Path) -> Result<()> {
    println!(
        "duel  •  {} items  •  1/2 to pick  •  s to skip  •  q to quit\n",
        session.items.len(),
    );

    loop {
        let (left_idx, right_idx) = session.next_matchup();

        let left = session.items[left_idx].name.clone();
        let right = session.items[right_idx].name.clone();

        let done = session.total_comparisons;
        let target = session.target_comparisons();
        let min_seen = session.min_comparisons();
        let bar = progress_bar(done, target, 20);
        let pct = ((done as f64 / target as f64) * 100.0).min(100.0) as u32;
        println!(
            "  {} {}/{} ({}%)  •  least-seen item: {} duel{}",
            bar, done, target, pct,
            min_seen,
            if min_seen == 1 { "" } else { "s" }
        );
        println!("─────────────────────────────────────────");
        println!("  [1]  {}", left);
        println!();
        println!("            vs");
        println!();
        println!("  [2]  {}", right);
        println!("─────────────────────────────────────────");
        print!("  Pick (1/2/s/q): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => {
                session.record_result(left_idx, right_idx);
                session.save(source)?;
                println!();
            }
            "2" => {
                session.record_result(right_idx, left_idx);
                session.save(source)?;
                println!();
            }
            "s" | "S" => {
                println!("  skipped\n");
            }
            "q" | "Q" => {
                println!("\nProgress saved. Run again to continue.\n");
                break;
            }
            _ => {
                println!("  → type 1, 2, s, or q\n");
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let source = Path::new(&cli.file);

    if !source.exists() {
        anyhow::bail!("file not found: {}", cli.file);
    }

    let mut session = Session::load_or_create(source)?;

    if cli.results {
        print_rankings(&session, cli.output.as_deref())?;
        return Ok(());
    }

    run_duel(&mut session, source)?;
    print_rankings(&session, cli.output.as_deref())?;

    Ok(())
}
