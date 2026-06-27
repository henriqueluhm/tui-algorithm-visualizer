use color_eyre::eyre::WrapErr;
use ratatui::crossterm::event::{self, Event as CrosstermEvent};
use std::{
    collections::VecDeque,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

const TICK_FPS: f64 = 30.0;

#[derive(Clone, Debug)]
pub enum Event {
    Tick,
    Crossterm(CrosstermEvent),
}

#[derive(Debug)]
pub struct EventHandler {
    receiver: mpsc::Receiver<Event>,
    pending: VecDeque<Event>,
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let actor = EventThread::new(sender.clone());

        thread::spawn(|| actor.run());

        Self {
            receiver,
            pending: VecDeque::new(),
        }
    }

    pub fn next(&mut self) -> color_eyre::Result<Event> {
        self.fill_pending();
        self.prioritize_input();
        self.coalesce_ticks();

        self.pending
            .pop_front()
            .ok_or_else(|| color_eyre::eyre::eyre!("event channel closed"))
    }

    pub fn discard_pending_ticks(&mut self) {
        self.pending.retain(|event| !matches!(event, Event::Tick));
    }

    fn fill_pending(&mut self) {
        if self.pending.is_empty() {
            match self.receiver.recv() {
                Ok(event) => self.pending.push_back(event),
                Err(_) => return,
            }
        }

        while let Ok(event) = self.receiver.try_recv() {
            self.pending.push_back(event);
        }
    }

    fn prioritize_input(&mut self) {
        let first_key = self
            .pending
            .iter()
            .position(|event| matches!(event, Event::Crossterm(_)));

        let Some(index) = first_key else {
            return;
        };

        if index > 0 {
            self.pending.drain(0..index);
        }
    }

    fn coalesce_ticks(&mut self) {
        let mut merged = VecDeque::with_capacity(self.pending.len());
        let mut has_tick = false;

        for event in self.pending.drain(..) {
            match event {
                Event::Tick => has_tick = true,
                other => {
                    if has_tick {
                        merged.push_back(Event::Tick);
                        has_tick = false;
                    }
                    merged.push_back(other);
                }
            }
        }

        if has_tick {
            merged.push_back(Event::Tick);
        }

        self.pending = merged;
    }
}

struct EventThread {
    sender: mpsc::Sender<Event>,
}

impl EventThread {
    fn new(sender: mpsc::Sender<Event>) -> Self {
        Self { sender }
    }

    fn run(self) -> color_eyre::Result<()> {
        let tick_interval = Duration::from_secs_f64(1.0 / TICK_FPS);
        let mut last_tick = Instant::now();

        loop {
            let timeout = tick_interval.saturating_sub(last_tick.elapsed());

            if event::poll(Duration::ZERO).wrap_err("failed to poll for crossterm events")? {
                let event = event::read().wrap_err("failed to read crossterm event")?;
                self.send(Event::Crossterm(event));
            } else if timeout == Duration::ZERO {
                last_tick = Instant::now();
                self.send(Event::Tick);
            } else if event::poll(timeout).wrap_err("failed to poll for crossterm events")? {
                let event = event::read().wrap_err("failed to read crossterm event")?;
                self.send(Event::Crossterm(event));
            }
        }
    }

    fn send(&self, event: Event) {
        let _ = self.sender.send(event);
    }
}
