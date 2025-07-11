use color_eyre::eyre::WrapErr;
use ratatui::crossterm::event::{self, Event as CrosstermEvent};
use std::{
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
        Self { receiver }
    }

    pub fn next(&self) -> color_eyre::Result<Event> {
        Ok(self.receiver.recv()?)
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
            if timeout == Duration::ZERO {
                last_tick = Instant::now();
                self.send(Event::Tick);
            }
            if event::poll(timeout).wrap_err("failed to poll for crossterm events")? {
                let event = event::read().wrap_err("failed to read crossterm event")?;
                self.send(Event::Crossterm(event));
            }
        }
    }

    fn send(&self, event: Event) {
        let _ = self.sender.send(event);
    }
}
