use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pair(String, String);

#[derive(Debug, Clone)]
struct Match {
    team1: Pair,
    team2: Pair,
}

impl Pair {
    fn new(a: &str, b: &str) -> Self {
        if a < b {
            Pair(a.to_string(), b.to_string())
        } else {
            Pair(b.to_string(), a.to_string())
        }
    }
}

// 試合を生成する関数
fn generate_next_match(
    players: &[&str],
    pair_history: &HashMap<Pair, i32>,
    player_match_count: &HashMap<String, i32>
) -> Match {
    let mut max_match_count_selected_player = 0;
    let mut selected_players = Vec::new();

    //　最も試合数の少ない４人を選択
    // ソートして最も試合数の少ない４人を選択
    for player in players {
        let match_count = player_match_count.get(*player).unwrap();
        // println!("player: {:?}, match_count: {:?}", player, match_count);
        // ４人以下の場合はそのまま追加
        if selected_players.len() < 4 {
            selected_players.push(*player);
            // ４人の中で最も試合数が多い人を記録
            if max_match_count_selected_player < *match_count {
                max_match_count_selected_player = *match_count;
            }
        } else if *match_count < max_match_count_selected_player {
            selected_players.sort_by_key(|player| player_match_count.get(*player).unwrap());
            // println!("sorted selected_players: {:?}", selected_players);
            selected_players.pop();
            selected_players.push(*player);
            // ４人の中で最も試合数が多い人を記録
            max_match_count_selected_player = *selected_players.iter().map(|player| player_match_count.get(*player).unwrap()).max().unwrap();
        }
    }
    // println!("max_match_count_selected_player: {:?}", max_match_count_selected_player);
    // println!("selected_players: {:?}", selected_players);
    // println!("=========================\n");

    // 選択した４人のペアの中で最も組んだことがないペアを選択
    let pairs = selected_players.iter().combinations(2).map(|v| Pair::new(v[0], v[1])).collect::<Vec<_>>();
    let pairs = pairs.iter().map(|pair| (pair.clone(), pair_history.get(pair).unwrap())).collect::<Vec<_>>();
    let pairs = pairs.iter().sorted_by_key(|pair| pair.1).collect::<Vec<_>>();

    // 試合の組み合わせを生成
    let team1 = pairs[0].0.clone();
    let mut team2 = Pair("".to_string(), "".to_string());
    for pair in pairs.iter() {
        if team1.0 != pair.0.0 && team1.1 != pair.0.1 && team1.0 != pair.0.1 && team1.1 != pair.0.0{
            team2 = pair.0.clone();
            break;
        }
    };

    Match {
        team1,
        team2,
    }
}

// 試合のリストを生成する関数
fn generate_matches(players: &[&str], num_matches: usize) -> Vec<Match> {
    let pairs = players.iter().combinations(2).map(|v| Pair::new(v[0], v[1])).collect::<Vec<_>>();
    let mut pair_history: HashMap<Pair, i32> = HashMap::new();
    let mut player_match_count: HashMap<String, i32> = HashMap::new();
    let mut matches = Vec::new();

    pair_history.extend(pairs.iter().map(|pair| (pair.clone(), 0)));
    player_match_count.extend(players.iter().map(|player| (player.to_string(), 0)));

    for _ in 0..num_matches {
        let next_match = generate_next_match(players, &pair_history, &player_match_count);
        matches.push(next_match.clone());
        pair_history.entry(next_match.team1.clone()).and_modify(|e| *e += 1);
        pair_history.entry(next_match.team2.clone()).and_modify(|e| *e += 1);
        player_match_count.entry(next_match.team1.0.clone()).and_modify(|e| *e += 1);
        player_match_count.entry(next_match.team1.1.clone()).and_modify(|e| *e += 1);
        player_match_count.entry(next_match.team2.0.clone()).and_modify(|e| *e += 1);
        player_match_count.entry(next_match.team2.1.clone()).and_modify(|e| *e += 1);
    }

    println!("player_match_count: {:?}", player_match_count);

    matches
}

fn main() {
    let players = vec!["Player1", "Player2", "Player3", "Player4", "Player5", "Player6", "Player7", "Player8"];
    let num_matches = 7; // 生成する試合数
    let matches = generate_matches(&players, num_matches);

    for (i, match_) in matches.iter().enumerate() {
        println!("Match {}: {:?} vs {:?}", i + 1, match_.team1, match_.team2);
    }
}
