use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use crate::elo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub elo: f64,
    pub wins: u32,
    pub losses: u32,
    pub comparisons: u32,
}

impl Item {
    pub fn new(name: String) -> Self {
        Self {
            name,
            elo: elo::INITIAL_ELO,
            wins: 0,
            losses: 0,
            comparisons: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub items: Vec<Item>,
    pub total_comparisons: u32,
    pub source_file: String,
}

impl Session {
    fn new(source_file: &str, items: Vec<Item>) -> Self {
        Self {
            items,
            total_comparisons: 0,
            source_file: source_file.to_string(),
        }
    }

    pub fn session_path(source: &Path) -> PathBuf {
        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("~/.local/share"))
            .join("duel");
        std::fs::create_dir_all(&data_dir).ok();

        // Use the absolute path of the source file as a unique key
        let canonical = source.canonicalize().unwrap_or_else(|_| source.to_path_buf());
        let key = canonical.to_string_lossy().replace('/', "_").replace('\\', "_");
        data_dir.join(format!("{}.json", key))
    }

    pub fn load_or_create(source: &Path) -> Result<Self> {
        let session_path = Self::session_path(source);

        if session_path.exists() {
            let data = std::fs::read_to_string(&session_path)?;
            let mut session: Session = serde_json::from_str(&data)?;

            // Merge any new items added to the source file since last session
            let content = std::fs::read_to_string(source)?;
            let existing_names: std::collections::HashSet<&str> =
                session.items.iter().map(|i| i.name.as_str()).collect();

            let new_items: Vec<Item> = content
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty() && !existing_names.contains(*l))
                .map(|l| Item::new(l.to_string()))
                .collect();

            if !new_items.is_empty() {
                eprintln!(
                    "  {} new item(s) found in source file, added to session.",
                    new_items.len()
                );
                session.items.extend(new_items);
            }

            return Ok(session);
        }

        let content = std::fs::read_to_string(source)?;
        let items: Vec<Item> = content
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(|l| Item::new(l.to_string()))
            .collect();

        if items.len() < 2 {
            bail!("need at least 2 items to duel");
        }

        Ok(Session::new(source.to_str().unwrap_or(""), items))
    }

    pub fn save(&self, source: &Path) -> Result<()> {
        let session_path = Self::session_path(source);
        std::fs::write(session_path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    // Returns (left_idx, right_idx) — already randomized for display order.
    pub fn next_matchup(&self) -> (usize, usize) {
        let mut rng = thread_rng();

        let min_comparisons = self.items.iter().map(|i| i.comparisons).min().unwrap_or(0);
        let mut candidates: Vec<usize> = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, i)| i.comparisons == min_comparisons)
            .map(|(idx, _)| idx)
            .collect();
        candidates.shuffle(&mut rng);

        let a_idx = candidates[0];
        let a_elo = self.items[a_idx].elo;

        let b_idx = self
            .items
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != a_idx)
            .min_by(|(_, x), (_, y)| {
                (x.elo - a_elo)
                    .abs()
                    .partial_cmp(&(y.elo - a_elo).abs())
                    .unwrap()
            })
            .map(|(idx, _)| idx)
            .unwrap();

        // Randomly flip display order so position 1 isn't biased
        if rand::random::<bool>() {
            (a_idx, b_idx)
        } else {
            (b_idx, a_idx)
        }
    }

    pub fn record_result(&mut self, winner_idx: usize, loser_idx: usize) {
        let (new_winner_elo, new_loser_elo) =
            elo::update_ratings(self.items[winner_idx].elo, self.items[loser_idx].elo);

        self.items[winner_idx].elo = new_winner_elo;
        self.items[winner_idx].wins += 1;
        self.items[winner_idx].comparisons += 1;

        self.items[loser_idx].elo = new_loser_elo;
        self.items[loser_idx].losses += 1;
        self.items[loser_idx].comparisons += 1;

        self.total_comparisons += 1;
    }

    pub fn sorted_rankings(&self) -> Vec<&Item> {
        let mut sorted: Vec<&Item> = self.items.iter().collect();
        sorted.sort_by(|a, b| b.elo.partial_cmp(&a.elo).unwrap());
        sorted
    }

    pub fn target_comparisons(&self) -> u32 {
        let n = self.items.len() as f64;
        (n * n.log2().ceil()).ceil() as u32
    }

    pub fn min_comparisons(&self) -> u32 {
        self.items.iter().map(|i| i.comparisons).min().unwrap_or(0)
    }
}
