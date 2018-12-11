#[macro_use]
extern crate error_chain;
extern crate regex;

use std::io::Read;

mod errors {
    error_chain!{}
}

use errors::*;

pub fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let game = parse_game(&data).unwrap();
    println!("part_1: {}", part_1(&game));
    println!("part_1: {}", part_2(&game));
}

#[derive(Debug)]
struct Game {
    players: usize,
    last_marble: usize,
}

fn parse_game(data: &str) -> Result<Game> {
    let re = regex::Regex::new(
        r"^(?P<players>\d+) players; last marble is worth (?P<marble>\d+) points$",
    ).unwrap();
    let caps = re.captures(data).chain_err(|| "no captures")?;
    let players: usize = caps
        .name("players")
        .chain_err(|| "missing players")?
        .as_str()
        .parse()
        .chain_err(|| "unable to parse players")?;
    let last_marble: usize = caps
        .name("marble")
        .chain_err(|| "missing marble")?
        .as_str()
        .parse()
        .chain_err(|| "unable to parse marble")?;
    Ok(Game {
        players: players,
        last_marble: last_marble,
    })
}

fn part_1(game: &Game) -> usize {
    let mut circle = Circle::new(game.players);
    for _ in 0..game.last_marble {
        circle.expand();
    }
    circle.high_score()
}

fn part_2(game: &Game) -> usize {
    let mut circle = Circle::new(game.players);
    for _ in 0..100 * game.last_marble {
        circle.expand();
    }
    circle.high_score()
}

#[derive(Debug)]
pub struct Circle {
    scores: Vec<usize>,
    succ: Vec<usize>,
    pred: Vec<usize>,
    curr_id: usize,
    curr_player: usize,
    marbles: usize,
}

impl Circle {
    pub fn new(players: usize) -> Circle {
        Circle {
            scores: std::iter::repeat(0).take(players).collect(),
            succ: vec![0],
            pred: vec![0],
            curr_id: 0,
            curr_player: 0,
            marbles: 1,
        }
    }

    pub fn expand(&mut self) {
        // println!("{}: {:?}", self.curr_player, self.marbles());
        self.curr_player = 1 + (self.curr_player % self.scores.len());
        if (self.curr_id + 1) % 23 == 0 {
            let would_be_placed_id = self.curr_id + 1;
            let mut deletion_node_id = self.curr_id;
            for _ in 0..7 {
                deletion_node_id = self.pred[deletion_node_id];
            }
            self.curr_id = self.delete(deletion_node_id);
            self.scores[self.curr_player - 1] += deletion_node_id + would_be_placed_id;
            // not linked to anything
            self.succ.push(0);
            self.pred.push(0);
        } else {
            let node_id = self.succ[self.curr_id];
            self.curr_id = self.insert_after(node_id);
        }
    }

    pub fn current_marble(&self) -> usize {
        self.curr_id
    }

    pub fn high_score(&self) -> usize {
        *self.scores.iter().max().unwrap()
    }

    pub fn marbles(&self) -> Vec<usize> {
        let mut result = vec![0];
        let mut prev = result[0];
        let mut i = 1;
        while self.succ[prev] != 0 {
            result.push(self.succ[prev]);
            prev = result[i];
            i += 1;
        }
        result
    }

    fn insert_after(&mut self, node_id: usize) -> usize {
        let new_next_id = self.succ.len();
        let old_next_id = self.succ[node_id];
        self.succ.push(old_next_id);
        self.succ[node_id] = new_next_id;
        self.pred.push(node_id);
        self.pred[old_next_id] = new_next_id;
        new_next_id
    }

    fn delete(&mut self, node_id: usize) -> usize {
        let old_pred = self.pred[node_id];
        let old_succ = self.succ[node_id];
        self.succ[old_pred] = old_succ;
        self.pred[old_succ] = old_pred;
        old_succ
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        use parse_game;
        assert_eq!(part_1(parse_game("9 players; last marble is worth 25 points").unwrap()), 32);
        assert_eq!(part_1(parse_game("10 players; last marble is worth 1618 points").unwrap()), 8317);
        assert_eq!(part_1(parse_game("13 players; last marble is worth 7999 points").unwrap()), 146373);
        assert_eq!(part_1(parse_game("17 players; last marble is worth 1104 points").unwrap()), 2764);
        assert_eq!(part_1(parse_game("21 players; last marble is worth 6111 points").unwrap()), 54718);
        assert_eq!(part_1(parse_game("30 players; last marble is worth 5807 points").unwrap()), 37305);
    }
}
