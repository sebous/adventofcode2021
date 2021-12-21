use std::collections::HashMap;

#[derive(Debug)]
struct Player {
    position: usize,
    score: usize,
}

#[derive(Debug)]
struct Game {
    dice_max_sides: usize,
    dice_value: usize,
    dice_thrown: usize,
    board: Vec<usize>,
    players: Vec<Player>,
    won: Option<usize>,
}

impl Game {
    fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    fn round(&mut self) {
        for (p_i, player) in self.players.iter_mut().enumerate() {
            let mut player_round_score = 0;
            for _ in 0..3 {
                self.dice_value = if self.dice_value % 100 == 0 {
                    1
                } else {
                    self.dice_value + 1
                };
                player_round_score += self.dice_value;
                self.dice_thrown += 1;
            }

            player.position = (player.position + player_round_score - 1) % self.board.len() + 1;
            player.score += player.position;
            if player.score >= 1000 {
                self.won = Some(p_i);
                break;
            }
        }
    }
}

fn part_one() {
    let mut game = Game {
        dice_max_sides: 100,
        dice_value: 0,
        dice_thrown: 0,
        board: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        won: None,
        players: vec![
            Player {
                position: 10,
                score: 0,
            },
            Player {
                position: 6,
                score: 0,
            },
        ],
    };

    while game.won.is_none() {
        game.round();
    }
    let losing_score = game
        .players
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != game.won.unwrap())
        .next()
        .unwrap()
        .1
        .score;

    println!("part 1: {}", losing_score * game.dice_thrown);
}

type Cache = HashMap<(u32, u32, u32, u32, usize), [u64; 2]>;

fn quantum_round(
    pos1: u32,
    pos2: u32,
    score1: u32,
    score2: u32,
    player_index: usize,
    cache: &mut Cache,
) -> [u64; 2] {
    if cache.contains_key(&(pos1, pos2, score1, score2, player_index)) {
        return *cache
            .get(&(pos1, pos2, score1, score2, player_index))
            .unwrap();
    }

    let mut wins = [0; 2];
    let mut rolls = vec![];

    for r1 in 1..4 {
        for r2 in 1..4 {
            for r3 in 1..4 {
                rolls.push(r1 + r2 + r3);
            }
        }
    }

    for roll in rolls {
        let mut pos = [pos1, pos2];
        let mut score = [score1, score2];

        pos[player_index] = (pos[player_index] + roll - 1) % 10 + 1;
        score[player_index] += pos[player_index];

        if score[player_index] >= 21 {
            wins[player_index] += 1;
        } else {
            let [win1, win2] = quantum_round(
                pos[0],
                pos[1],
                score[0],
                score[1],
                if player_index == 1 { 0 } else { 1 },
                cache,
            );
            wins[0] += win1;
            wins[1] += win2;
        }
    }

    cache.insert((pos1, pos2, score1, score2, player_index), wins);
    wins
}

fn part_two() {
    let mut cache: Cache = HashMap::new();
    let r = quantum_round(10, 6, 0, 0, 0, &mut cache);
    println!("part 2: {:?}", r.iter().max().unwrap());
}

pub fn run() {
    part_one();
    part_two();
}
