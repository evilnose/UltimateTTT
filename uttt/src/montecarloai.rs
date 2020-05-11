use std::fs::File;
use std::collections::HashMap;
use crate::ai::AI;
use serde::{Serialize, Deserialize};

use rand;

use crate::board::Board;
use crate::board::Player;

#[derive(Serialize, Deserialize)]
struct TreeNode {
    // the numerator and denominator
    // of the probability that the move
    // entering this node
    // leads to a win
    numerator: u64,
    denominator: u64,
    // maps moves from this node 
    // to the nodes that contain
    // their probability
    children: HashMap<usize, usize>,
}


#[derive(Serialize, Deserialize)]
pub struct MonteCarloAI {
   tree: Vec<TreeNode>,
}

impl MonteCarloAI {
    pub fn new() -> MonteCarloAI {
        let mut result = MonteCarloAI {
            tree: Vec::new(),
        };
        result.tree.push(TreeNode {
                numerator: 0,
                denominator: 0,
                children: HashMap::new(),
            });
        return result;
    }

    pub fn from_save(filename: String) -> MonteCarloAI {
        bincode::deserialize_from(File::create(filename).unwrap()).unwrap()
    }

    pub fn train(&mut self, games: usize) {
        for _i in 0..games {
            let board = Board::new(2);
            self.train_helper(&board, 0);
        }
    }

    fn train_helper(&mut self, board: &Board, node_index: usize) -> i64 {
        if board.winner == Player::O {
            return -1;
        } else if board.winner == Player::X {
            return 1;
        } else if board.winner == Player::DEAD {
            return 0;
        }
        let moves = board.get_moves();
        let mut new_board = board.clone();
        let next_move = moves[rand::random::<usize>() % moves.len()];
        new_board.make_move(next_move);
        //let node = &mut self.tree[node_index];
        if self.tree[node_index].children.len() == 0 {
            for m in moves {
                self.tree.push(
                    TreeNode {
                        numerator: 0,
                        denominator: 0,
                        children: HashMap::new(),
                    }
                );
                let idx = self.tree.len() - 1;
                self.tree[node_index].children.insert(m, idx);
            }
        }
        let move_node_index = *self.tree[node_index].children.get(&next_move).unwrap();
        let result = self.train_helper(&new_board, move_node_index);
        if result == 1 && board.get_to_move() == Player::X {
            self.tree[move_node_index].numerator += 1;
        } else if result == -1 && board.get_to_move() == Player::O {
            self.tree[move_node_index].numerator += 1;
        }
        self.tree[move_node_index].denominator += 1;
        return result;
    }

    pub fn save_to_file(&self, filename: String) {
        bincode::serialize_into(File::create(filename).unwrap(), &self).unwrap();
    }
}