use super::Algorithm;

#[derive(Debug, Clone)]
pub struct MergeSortState {
    bars: Vec<i32>,
    aux: Vec<i32>,
    width: usize,
    pair_start: usize,
    left_end: usize,
    right_end: usize,
    left_idx: usize,
    right_idx: usize,
    write_idx: usize,
    in_merge: bool,
    complete: bool,
    comparisons: Vec<(usize, usize)>,
    current_indices: Vec<usize>,
}

#[derive(Debug)]
pub struct MergeSort;

impl MergeSort {
    fn begin_merge(state: &mut MergeSortState) {
        let start = state.pair_start;
        let left_end = (start + state.width).min(state.bars.len());
        let right_end = (start + state.width * 2).min(state.bars.len());

        state.left_end = left_end;
        state.right_end = right_end;
        state.aux[start..right_end].copy_from_slice(&state.bars[start..right_end]);
        state.left_idx = start;
        state.right_idx = left_end;
        state.write_idx = start;
        state.in_merge = true;
    }

    fn advance_pair(state: &mut MergeSortState) {
        state.in_merge = false;
        state.pair_start += state.width * 2;

        if state.pair_start >= state.bars.len() {
            state.width *= 2;
            state.pair_start = 0;

            if state.width >= state.bars.len() {
                state.complete = true;
            }
        }
    }
}

impl Algorithm for MergeSort {
    type State = MergeSortState;

    fn name(&self) -> &'static str {
        "Merge Sort"
    }

    fn initial_state(&self, bars: Vec<i32>) -> Self::State {
        let n = bars.len();
        MergeSortState {
            aux: vec![0; n],
            bars,
            width: 1,
            pair_start: 0,
            left_end: 0,
            right_end: 0,
            left_idx: 0,
            right_idx: 0,
            write_idx: 0,
            in_merge: false,
            complete: n <= 1,
            comparisons: Vec::new(),
            current_indices: Vec::new(),
        }
    }

    fn step(&self, state: &mut Self::State) -> bool {
        if state.complete {
            return true;
        }

        let n = state.bars.len();
        state.comparisons.clear();
        state.current_indices.clear();

        if state.in_merge {
            if state.left_idx < state.left_end && state.right_idx < state.right_end {
                state.comparisons
                    .push((state.left_idx, state.right_idx));
                state.current_indices = vec![state.left_idx, state.right_idx, state.write_idx];

                if state.aux[state.left_idx] <= state.aux[state.right_idx] {
                    state.bars[state.write_idx] = state.aux[state.left_idx];
                    state.left_idx += 1;
                } else {
                    state.bars[state.write_idx] = state.aux[state.right_idx];
                    state.right_idx += 1;
                }

                state.write_idx += 1;
            } else if state.left_idx < state.left_end {
                state.current_indices = vec![state.left_idx, state.write_idx];
                state.bars[state.write_idx] = state.aux[state.left_idx];
                state.left_idx += 1;
                state.write_idx += 1;
            } else if state.right_idx < state.right_end {
                state.current_indices = vec![state.right_idx, state.write_idx];
                state.bars[state.write_idx] = state.aux[state.right_idx];
                state.right_idx += 1;
                state.write_idx += 1;
            } else {
                Self::advance_pair(state);
            }

            return state.complete;
        }

        if state.width >= n {
            state.complete = true;
            return true;
        }

        let left_end = (state.pair_start + state.width).min(n);
        let right_end = (state.pair_start + state.width * 2).min(n);

        if left_end >= right_end {
            Self::advance_pair(state);
            return state.complete;
        }

        Self::begin_merge(state);
        false
    }

    fn get_data(&self, state: &Self::State) -> Vec<i32> {
        state.bars.clone()
    }

    fn get_comparisons(&self, state: &Self::State) -> Vec<(usize, usize)> {
        state.comparisons.clone()
    }

    fn get_current_indices(&self, state: &Self::State) -> Vec<usize> {
        state.current_indices.clone()
    }
}
