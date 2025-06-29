use crate::algorithms::Algorithm;

#[derive(Debug, Clone)]
pub struct QuickSortCall {
    low: usize,
    high: usize,
    pivot_index: Option<usize>,
    partition_i: usize,
    partition_j: usize,
    partitioning: bool,
    pivot_placed: bool,
}

#[derive(Debug, Clone)]
pub struct QuickSortState {
    bars: Vec<i32>,
    call_stack: Vec<QuickSortCall>,
    complete: bool,
    current_indices: Vec<usize>,
    comparisons: Vec<(usize, usize)>,
    pivot_index: Option<usize>,
}

#[derive(Debug)]
pub struct QuickSort;

impl Algorithm for QuickSort {
    type State = QuickSortState;

    fn name(&self) -> &'static str {
        "Quick Sort"
    }

    fn initial_state(&self, bars: Vec<i32>) -> Self::State {
        let mut state = QuickSortState {
            bars,
            call_stack: Vec::new(),
            complete: false,
            current_indices: Vec::new(),
            comparisons: Vec::new(),
            pivot_index: None,
        };

        if state.bars.len() > 1 {
            state.call_stack.push(QuickSortCall {
                low: 0,
                high: state.bars.len() - 1,
                pivot_index: None,
                partition_i: 0,
                partition_j: 0,
                partitioning: false,
                pivot_placed: false,
            });
        } else {
            state.complete = true;
        }

        state
    }

    fn step(&self, state: &mut Self::State) -> bool {
        if state.complete || state.call_stack.is_empty() {
            state.complete = true;
            state.current_indices.clear();
            state.comparisons.clear();
            state.pivot_index = None;

            return true;
        }

        state.current_indices.clear();
        state.comparisons.clear();

        let mut current_call = state.call_stack.pop().unwrap();

        if current_call.low >= current_call.high {
            return self.step(state);
        }

        if !current_call.partitioning {
            current_call.partitioning = true;
            current_call.pivot_index = Some(current_call.high);
            current_call.partition_i = current_call.low;
            current_call.partition_j = current_call.low;

            state.pivot_index = current_call.pivot_index;
            state.current_indices.push(current_call.high);

            state.call_stack.push(current_call);

            return false;
        }

        if !current_call.pivot_placed {
            let pivot_idx = current_call.pivot_index.unwrap();
            let pivot_value = state.bars[pivot_idx];

            if current_call.partition_j <= current_call.high {
                if current_call.partition_j < current_call.high {
                    state
                        .comparisons
                        .push((current_call.partition_j, pivot_idx));
                    state.current_indices.push(current_call.partition_j);
                    state.current_indices.push(pivot_idx);
                    state.current_indices.push(current_call.partition_i);

                    if state.bars[current_call.partition_j] < pivot_value {
                        state
                            .bars
                            .swap(current_call.partition_i, current_call.partition_j);

                        current_call.partition_i += 1;
                    }

                    current_call.partition_j += 1;
                } else {
                    state.bars.swap(current_call.partition_i, pivot_idx);
                    current_call.pivot_placed = true;

                    if current_call.partition_i > current_call.low {
                        state.call_stack.push(QuickSortCall {
                            low: current_call.low,
                            high: current_call.partition_i - 1,
                            pivot_index: None,
                            partition_i: 0,
                            partition_j: 0,
                            partitioning: false,
                            pivot_placed: false,
                        });
                    }

                    if current_call.partition_i < current_call.high {
                        state.call_stack.push(QuickSortCall {
                            low: current_call.partition_i + 1,
                            high: current_call.high,
                            pivot_index: None,
                            partition_i: 0,
                            partition_j: 0,
                            partitioning: false,
                            pivot_placed: false,
                        });
                    }

                    state.pivot_index = Some(current_call.partition_i);
                    state.current_indices.push(current_call.partition_i);

                    return false;
                }

                state.call_stack.push(current_call);

                return false;
            }
        }

        false
    }

    fn get_data(&self, state: &Self::State) -> Vec<i32> {
        state.bars.clone()
    }

    fn get_comparisons(&self, state: &Self::State) -> Vec<(usize, usize)> {
        state.comparisons.clone()
    }

    fn get_current_indices(&self, state: &Self::State) -> Vec<usize> {
        let mut indices = state.current_indices.clone();

        if let Some(pivot) = state.pivot_index {
            if !indices.contains(&pivot) {
                indices.push(pivot);
            }
        }

        indices
    }
}
