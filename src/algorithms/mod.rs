use std::fmt::Debug;

use crate::algorithms::{
    bubble::{BubbleSort, BubbleSortState},
    merge::{MergeSort, MergeSortState},
    quick::{QuickSort, QuickSortState},
    selection::{SelectionSort, SelectionSortState},
};

pub mod bubble;
pub mod merge;
pub mod quick;
pub mod selection;

trait Algorithm: Debug {
    type State: Clone;

    fn name(&self) -> &'static str;
    fn initial_state(&self, bars: Vec<i32>) -> Self::State;
    fn step(&self, state: &mut Self::State) -> bool;
    fn get_data(&self, state: &Self::State) -> Vec<i32>;
    fn get_comparisons(&self, state: &Self::State) -> Vec<(usize, usize)>;
    fn get_current_indices(&self, state: &Self::State) -> Vec<usize>;
}

#[derive(Debug)]
pub enum AlgorithmType {
    BubbleSort(BubbleSort, Option<BubbleSortState>),
    QuickSort(QuickSort, Option<QuickSortState>),
    MergeSort(MergeSort, Option<MergeSortState>),
    SelectionSort(SelectionSort, Option<SelectionSortState>),
}

impl AlgorithmType {
    pub fn name(&self) -> &'static str {
        match self {
            AlgorithmType::BubbleSort(algo, _) => algo.name(),
            AlgorithmType::QuickSort(algo, _) => algo.name(),
            AlgorithmType::MergeSort(algo, _) => algo.name(),
            AlgorithmType::SelectionSort(algo, _) => algo.name(),
        }
    }

    pub fn reset_with_data(&mut self, bars: Vec<i32>) {
        match self {
            AlgorithmType::BubbleSort(algo, state) => {
                *state = Some(algo.initial_state(bars));
            }
            AlgorithmType::QuickSort(algo, state) => {
                *state = Some(algo.initial_state(bars));
            }
            AlgorithmType::MergeSort(algo, state) => {
                *state = Some(algo.initial_state(bars));
            }
            AlgorithmType::SelectionSort(algo, state) => {
                *state = Some(algo.initial_state(bars));
            }
        }
    }

    pub fn step(&mut self) -> bool {
        match self {
            AlgorithmType::BubbleSort(algo, Some(state)) => algo.step(state),
            AlgorithmType::QuickSort(algo, Some(state)) => algo.step(state),
            AlgorithmType::MergeSort(algo, Some(state)) => algo.step(state),
            AlgorithmType::SelectionSort(algo, Some(state)) => algo.step(state),
            _ => true,
        }
    }

    pub fn get_data(&self) -> Vec<i32> {
        match self {
            AlgorithmType::BubbleSort(algo, Some(state)) => algo.get_data(state),
            AlgorithmType::QuickSort(algo, Some(state)) => algo.get_data(state),
            AlgorithmType::MergeSort(algo, Some(state)) => algo.get_data(state),
            AlgorithmType::SelectionSort(algo, Some(state)) => algo.get_data(state),
            _ => Vec::new(),
        }
    }

    pub fn get_current_indices(&self) -> Vec<usize> {
        match self {
            AlgorithmType::BubbleSort(algo, Some(state)) => algo.get_current_indices(state),
            AlgorithmType::QuickSort(algo, Some(state)) => algo.get_current_indices(state),
            AlgorithmType::MergeSort(algo, Some(state)) => algo.get_current_indices(state),
            AlgorithmType::SelectionSort(algo, Some(state)) => algo.get_current_indices(state),
            _ => Vec::new(),
        }
    }

    pub fn get_comparisons(&self) -> Vec<(usize, usize)> {
        match self {
            AlgorithmType::BubbleSort(algo, Some(state)) => algo.get_comparisons(state),
            AlgorithmType::QuickSort(algo, Some(state)) => algo.get_comparisons(state),
            AlgorithmType::MergeSort(algo, Some(state)) => algo.get_comparisons(state),
            AlgorithmType::SelectionSort(algo, Some(state)) => algo.get_comparisons(state),
            _ => Vec::new(),
        }
    }
}
