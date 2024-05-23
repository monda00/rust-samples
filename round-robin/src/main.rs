fn generate_round_robin_pairs(players: Vec<&str>) -> Vec<Vec<(&str, &str)>> {
    let num_players = players.len();
    assert!(num_players % 2 == 0, "Number of players must be even");

    let mut rounds = Vec::new();
    let mut rotated_players = players.clone();

    for _ in 0..(num_players - 1) {
        let mut round = Vec::new();

        for i in 0..(num_players / 2) {
            round.push((rotated_players[i], rotated_players[num_players - 1 - i]));
        }

        rounds.push(round);

        // Rotate players except the first one
        let last_player = rotated_players.pop().unwrap();
        rotated_players.insert(1, last_player);
    }

    rounds
}

fn main() {
    let players = vec!["Player 1", "Player 2", "Player 3", "Player 4"];
    let rounds = generate_round_robin_pairs(players);

    for (i, round) in rounds.iter().enumerate() {
        println!("Round {}: {:?}", i + 1, round);
    }
}

