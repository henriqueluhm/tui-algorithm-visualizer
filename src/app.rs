use crate::algorithms::AlgorithmType;
use crate::algorithms::bubble::BubbleSort;
use crate::algorithms::quick::QuickSort;
use crate::event::{Event, EventHandler};
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use std::time::{Duration, Instant};

const MAX_BARS_SIZE: u8 = 100;
const MIN_BARS_SIZE: u8 = 10;

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
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            event_handler: EventHandler::new(),
            algorithms: vec![
                AlgorithmType::BubbleSort(BubbleSort, None),
                AlgorithmType::QuickSort(QuickSort, None),
            ],
            current_algorithm: 0,
            app_status: AppStatus::Paused,
            speed: Duration::from_millis(100),
            last_step: Instant::now(),
            bars: (1..=50).collect(),
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
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
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
                _ => {}
            }
        }
    }

    pub fn tick(&mut self) {
        if self.app_status == AppStatus::Running && self.last_step.elapsed() >= self.speed {
            let is_completed = self.algorithms[self.current_algorithm].step();

            if is_completed {
                self.app_status = AppStatus::Completed;
            }

            self.last_step = Instant::now();
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    fn toggle_running(&mut self) {
        match self.app_status {
            AppStatus::Running => self.app_status = AppStatus::Paused,
            AppStatus::Paused => self.app_status = AppStatus::Running,
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
        if self.bars.len() as u8 == MAX_BARS_SIZE {
            return;
        }
        self.reset_algorithm();

        self.bars.push((self.bars.len() + 1) as i32);
        self.shuffle_data_and_reset();
    }

    fn decrease_length(&mut self) {
        if self.bars.len() as u8 == MIN_BARS_SIZE {
            return;
        }
        self.reset_algorithm();

        if let Some((index, _)) = self.bars.iter().enumerate().max_by_key(|&(_, &val)| val) {
            self.bars.remove(index);
        }
        self.shuffle_data_and_reset();
    }

    fn increase_speed(&mut self) {
        if self.speed.as_millis() == 1 {
            return;
        }

        if self.speed.as_millis() == 20 {
            self.speed = Duration::from_millis(1);
            return;
        }

        self.speed =
            Duration::from_millis((self.speed.as_millis() as u64).saturating_sub(20).max(1));
    }

    fn decrease_speed(&mut self) {
        if self.speed.as_millis() == 1 {
            self.speed = Duration::from_millis(20);
            return;
        }

        self.speed = Duration::from_millis((self.speed.as_millis() as u64 + 20).min(1000));
    }

    pub fn get_current_algorithm(&self) -> &AlgorithmType {
        &self.algorithms[self.current_algorithm]
    }
}
