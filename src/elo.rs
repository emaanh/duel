pub const INITIAL_ELO: f64 = 1000.0;
const K_FACTOR: f64 = 32.0;

fn expected_score(rating_a: f64, rating_b: f64) -> f64 {
    1.0 / (1.0 + 10f64.powf((rating_b - rating_a) / 400.0))
}

pub fn update_ratings(winner_elo: f64, loser_elo: f64) -> (f64, f64) {
    let expected_winner = expected_score(winner_elo, loser_elo);
    let expected_loser = expected_score(loser_elo, winner_elo);

    let new_winner = winner_elo + K_FACTOR * (1.0 - expected_winner);
    let new_loser = loser_elo + K_FACTOR * (0.0 - expected_loser);

    (new_winner, new_loser)
}
