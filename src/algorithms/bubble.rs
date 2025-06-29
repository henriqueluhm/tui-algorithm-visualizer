use super::Algorithm;

#[derive(Debug, Clone)]
pub struct BubbleSortState {
    bars: Vec<i32>,
    i: usize,
    j: usize,
    complete: bool,
    comparisons: Vec<(usize, usize)>,
    current_indices: Vec<usize>,
}

#[derive(Debug)]
pub struct BubbleSort;

impl BubbleSort {}

impl Algorithm for BubbleSort {
    type State = BubbleSortState;

    fn name(&self) -> &'static str {
        "Bubble Sort"
    }

    fn initial_state(&self, bars: Vec<i32>) -> Self::State {
        BubbleSortState {
            bars,
            i: 0,
            j: 0,
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

        if state.j < n - state.i - 1 {
            state.current_indices = vec![state.j, state.j + 1];
            state.comparisons.push((state.j, state.j + 1));

            if state.bars[state.j] > state.bars[state.j + 1] {
                state.bars.swap(state.j, state.j + 1);
            }

            state.j += 1;
        } else {
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
