extern crate rand;

use rand::rngs::EntropyRng;
use rand::RngCore;

fn available_moves_indexes(moves_sizes: &Vec<usize>) -> Vec<usize> {
    moves_sizes
        .iter()
        .enumerate()
        .filter(|&(_, size)| *size != 0usize)
        .map(|(index, _)| index)
        .collect::<Vec<_>>()
}

fn generate_random_value(start_value: u32, final_value: u32) -> usize {
    let mut rng = EntropyRng::new(); //tmp
    let value = rng.next_u32(); 
    ( (value % final_value) + start_value ) as usize
}

fn generate_moves_sequence(moves_sizes: Vec<usize>, number_of_moves: usize) -> Vec<usize> {
    let moves_available = available_moves_indexes(&moves_sizes);
    let mut empty_moves: Vec<usize> = Vec::with_capacity(number_of_moves);
    (0..number_of_moves).fold((moves_sizes, moves_available), |(cols, moves_left), _| {
        // let chosen_move = moves_left[rng.gen_range(0, moves_left.len())];
        let chosen_move = moves_left[generate_random_value(0, moves_left.len() as u32)];
        let new_cols = cols.iter().enumerate().map(|(index, col_sizes)| {
            if index == chosen_move {
                (*col_sizes as i32 - 1) as usize
            } else {
                *col_sizes
            }
        }).collect::<Vec<_>>();
        empty_moves.push(chosen_move);
        (new_cols.clone(), available_moves_indexes(&new_cols))
    });
    empty_moves
}

pub fn generate_n_moves_sequences(moves_sizes: Vec<usize>, number_of_moves: usize, number_of_sequences: usize) -> Vec<Vec<usize>> {
    (0..number_of_sequences)
        .map(|_| generate_moves_sequence(moves_sizes.clone(), number_of_moves))
        .collect::<Vec<_>>()
}
