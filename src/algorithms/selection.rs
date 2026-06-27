use super::Algorithm;

#[derive(Debug, Clone)]
pub struct SelectionSortState {
    bars: Vec<i32>,
    i: usize,
    j: usize,
    min_idx: usize,
    complete: bool,
    comparisons: Vec<(usize, usize)>,
    current_indices: Vec<usize>,
}

#[derive(Debug)]
pub struct SelectionSort;

impl Algorithm for SelectionSort {
    type State = SelectionSortState;

    fn name(&self) -> &'static str {
        "Selection Sort"
    }

    fn initial_state(&self, bars: Vec<i32>) -> Self::State {
        SelectionSortState {
            bars,
            i: 0,
            j: 0,
            min_idx: 0,
            complete: false,
            comparisons: Vec::new(),
            current_indices: Vec::new(),
        }
    }

    fn step(&self, state: &mut Self::State) -> bool {
        if state.complete {
            return true;
        }

        let n = state.bars.len();
        if n <= 1 {
            state.complete = true;
            return true;
        }

        state.comparisons.clear();
        state.current_indices.clear();

        if state.i >= n - 1 {
            state.complete = true;
            return true;
        }

        if state.j == 0 {
            state.j = state.i + 1;
            state.min_idx = state.i;
        }

        if state.j < n {
            state.current_indices = vec![state.i, state.j, state.min_idx];
            state.comparisons.push((state.j, state.min_idx));

            if state.bars[state.j] < state.bars[state.min_idx] {
                state.min_idx = state.j;
            }

            state.j += 1;
        } else {
            if state.min_idx != state.i {
                state.current_indices = vec![state.i, state.min_idx];
                state.comparisons.push((state.i, state.min_idx));
                state.bars.swap(state.i, state.min_idx);
            }

            state.i += 1;
            state.j = 0;

            if state.i >= n - 1 {
                state.complete = true;
                state.current_indices.clear();
                state.comparisons.clear();
            }
        }

        state.complete
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
