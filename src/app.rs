use crate::algorithms::AlgorithmType;
use crate::algorithms::bubble::BubbleSort;
use crate::algorithms::merge::MergeSort;
use crate::algorithms::quick::QuickSort;
use crate::algorithms::selection::SelectionSort;
use crate::event::{Event, EventHandler};
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{DefaultTerminal, Frame, layout::Rect};
use std::time::{Duration, Instant};

pub const MIN_BARS_SIZE: usize = 10;
const MAX_STEPS_PER_TICK: u32 = 500;
const SPEED_STEPS_MS: &[u64] = &[0, 1, 2, 5, 10, 20, 40, 60, 80, 100, 150, 200, 300, 500, 750, 1000];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppStatus {
    Running,
    Paused,
    Completed,
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub event_handler: EventHandler,
    pub algorithms: Vec<AlgorithmType>,
    pub current_algorithm: usize,
    pub app_status: AppStatus,
    pub speed: Duration,
    pub last_step: Instant,
    pub bars: Vec<i32>,
    pub viewport_width: u16,
    pub viewport_height: u16,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            event_handler: EventHandler::new(),
            algorithms: vec![
                AlgorithmType::BubbleSort(BubbleSort, None),
                AlgorithmType::QuickSort(QuickSort, None),
                AlgorithmType::MergeSort(MergeSort, None),
                AlgorithmType::SelectionSort(SelectionSort, None),
            ],
            current_algorithm: 0,
            app_status: AppStatus::Paused,
            speed: Duration::from_millis(100),
            last_step: Instant::now(),
            bars: (1..=50).collect(),
            viewport_width: 80,
            viewport_height: 24,
        }
    }
}

impl App {
    pub fn new() -> Self {
        let mut app = Self::default();
        app.shuffle_data();
        app.reset_algorithm();
        app
    }

    pub fn chart_inner_width(&self) -> u16 {
        self.viewport_width.saturating_sub(2)
    }

    pub fn max_bars(&self) -> usize {
        let inner = self.chart_inner_width() as usize;
        if inner == 0 {
            return MIN_BARS_SIZE;
        }

        let with_gap = inner / 2;
        if with_gap >= MIN_BARS_SIZE {
            with_gap
        } else {
            inner.max(MIN_BARS_SIZE)
        }
    }

    pub fn bar_chart_layout(&self, bar_count: usize) -> (u16, u16) {
        let inner = self.chart_inner_width() as usize;
        if bar_count == 0 || inner == 0 {
            return (1, 1);
        }

        if bar_count * 2 <= inner {
            (1, 1)
        } else {
            (1, 0)
        }
    }

    pub fn is_compact_layout(&self) -> bool {
        self.viewport_width < 100 || self.viewport_height < 28
    }

    pub fn is_tiny_layout(&self) -> bool {
        self.viewport_width < 60 || self.viewport_height < 20
    }

    pub fn speed_label(&self) -> String {
        if self.speed.as_millis() == 0 {
            "max".to_string()
        } else {
            format!("{}ms", self.speed.as_millis())
        }
    }

    pub fn update_viewport(&mut self, area: Rect) {
        self.viewport_width = area.width;
        self.viewport_height = area.height;
        self.clamp_bars_to_viewport();
    }

    fn clamp_bars_to_viewport(&mut self) {
        let max = self.max_bars();
        let mut changed = false;

        while self.bars.len() > max {
            if let Some((index, _)) = self.bars.iter().enumerate().max_by_key(|&(_, &val)| val) {
                self.bars.remove(index);
                changed = true;
            } else {
                break;
            }
        }

        if changed {
            self.reset_algorithm();
        }
    }

    fn shuffle_data(&mut self) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        std::time::SystemTime::now().hash(&mut hasher);
        let seed = hasher.finish() as usize;

        for i in 0..self.bars.len() {
            let j = (seed + i * 17) % self.bars.len();
            self.bars.swap(i, j);
        }
    }

    fn reset_algorithm(&mut self) {
        self.algorithms[self.current_algorithm].reset_with_data(self.bars.clone());
        self.app_status = AppStatus::Paused;
        self.last_step = Instant::now();
        self.event_handler.discard_pending_ticks();
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| {
                self.update_viewport(frame.area());
                self.render(frame);
            })?;
            self.handle_event()?;
        }

        Ok(())
    }

    pub fn render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn handle_event(&mut self) -> color_eyre::Result<()> {
        match self.event_handler.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => self.handle_crossterm_event(event),
        }

        Ok(())
    }

    pub fn handle_crossterm_event(&mut self, event: crossterm::event::Event) {
        if let crossterm::event::Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                KeyCode::Char('c' | 'C') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.quit();
                }
                KeyCode::Char(' ') => self.toggle_running(),
                KeyCode::Char('r') => self.reset(),
                KeyCode::Char('s') => self.shuffle_data_and_reset(),
                KeyCode::Right => self.increase_length(),
                KeyCode::Left => self.decrease_length(),
                KeyCode::Up => self.increase_speed(),
                KeyCode::Down => self.decrease_speed(),
                KeyCode::Char('1') => self.select_algorithm(0),
                KeyCode::Char('2') => self.select_algorithm(1),
                KeyCode::Char('3') => self.select_algorithm(2),
                KeyCode::Char('4') => self.select_algorithm(3),
                _ => {}
            }
        }
    }

    pub fn tick(&mut self) {
        if self.app_status != AppStatus::Running {
            return;
        }

        let speed_ms = self.speed.as_millis();
        if speed_ms > 0 && self.last_step.elapsed() < self.speed {
            return;
        }

        let steps = if speed_ms == 0 {
            MAX_STEPS_PER_TICK
        } else {
            (self.last_step.elapsed().as_millis() / speed_ms).max(1) as u32
        }
        .min(MAX_STEPS_PER_TICK);

        for _ in 0..steps {
            if self.algorithms[self.current_algorithm].step() {
                self.app_status = AppStatus::Completed;
                break;
            }
        }

        self.last_step = Instant::now();
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    fn toggle_running(&mut self) {
        match self.app_status {
            AppStatus::Running => {
                self.app_status = AppStatus::Paused;
                self.last_step = Instant::now();
            }
            AppStatus::Paused => {
                self.app_status = AppStatus::Running;
                self.last_step = Instant::now();
            }
            AppStatus::Completed => {
                self.app_status = AppStatus::Paused;
                self.shuffle_data_and_reset();
            }
        };
    }

    fn reset(&mut self) {
        self.reset_algorithm();
    }

    fn shuffle_data_and_reset(&mut self) {
        self.shuffle_data();
        self.reset_algorithm();
    }

    fn select_algorithm(&mut self, index: usize) {
        if index < self.algorithms.len() {
            self.current_algorithm = index;
            self.reset_algorithm();
        }
    }

    fn increase_length(&mut self) {
        if self.bars.len() >= self.max_bars() {
            return;
        }

        self.bars.push((self.bars.len() + 1) as i32);
        self.shuffle_data_and_reset();
    }

    fn decrease_length(&mut self) {
        if self.bars.len() <= MIN_BARS_SIZE {
            return;
        }

        if let Some((index, _)) = self.bars.iter().enumerate().max_by_key(|&(_, &val)| val) {
            self.bars.remove(index);
        }
        self.shuffle_data_and_reset();
    }

    fn speed_index(&self) -> usize {
        let current = self.speed.as_millis() as u64;
        SPEED_STEPS_MS
            .iter()
            .position(|&ms| ms == current)
            .unwrap_or(1)
    }

    fn set_speed_index(&mut self, index: usize) {
        self.speed = Duration::from_millis(SPEED_STEPS_MS[index]);
    }

    fn increase_speed(&mut self) {
        let index = self.speed_index();
        if index > 0 {
            self.set_speed_index(index - 1);
        }
    }

    fn decrease_speed(&mut self) {
        let index = self.speed_index();
        if index + 1 < SPEED_STEPS_MS.len() {
            self.set_speed_index(index + 1);
        }
    }

    pub fn get_current_algorithm(&self) -> &AlgorithmType {
        &self.algorithms[self.current_algorithm]
    }
}
