use crate::app::{App, AppStatus};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{BarChart, Block, BorderType, Paragraph, Widget},
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let algo_visualization_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(area);

        let controls_and_info_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(algo_visualization_area[1]);

        self.render_visualization(algo_visualization_area[0], buf);

        self.render_controls(controls_and_info_area[0], buf);
        self.render_info(controls_and_info_area[1], buf);
    }
}

impl App {
    fn render_visualization(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Algorithm Visualization")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let inner = block.inner(area);
        block.render(area, buf);

        let current_algo = self.get_current_algorithm();
        let bars = current_algo.get_data();
        let current_indices = current_algo.get_current_indices();
        let comparisons = current_algo.get_comparisons();

        if !bars.is_empty() {
            let max_value = *bars.iter().max().unwrap_or(&1) as u64;
            let bar_data: Vec<(&str, u64)> = bars
                .iter()
                .enumerate()
                .map(|(i, &value)| {
                    let label = if current_indices.contains(&i) {
                        "●"
                    } else if comparisons.iter().any(|(a, b)| *a == i || *b == i) {
                        "◐"
                    } else {
                        " "
                    };
                    (label, value as u64)
                })
                .collect();

            let bar_chart = BarChart::default()
                .data(&bar_data)
                .max(max_value)
                .bar_width(1)
                .bar_gap(1)
                .bar_style(Style::default().fg(Color::White))
                .value_style(Style::default().fg(Color::White).bg(Color::White));

            bar_chart.render(inner, buf);
        }
    }

    fn render_controls(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Controls")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let controls = vec![
            Line::from(vec![
                Span::styled(
                    "Space",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" - Start/Pause"),
            ]),
            Line::from(vec![
                Span::styled(
                    "R",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" - Reset"),
            ]),
            Line::from(vec![
                Span::styled(
                    "S",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" - Shuffle & Reset"),
            ]),
            Line::from(vec![
                Span::styled(
                    "↑/↓",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" - Speed Up/Down"),
            ]),
            Line::from(vec![
                Span::styled(
                    "Q/Esc",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" - Quit"),
            ]),
            Line::from(vec![
                Span::styled(
                    "←/→",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" - Decrease/Increase bars"),
            ]),
            Line::from(vec![
                Span::styled(
                    "1/2",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" - Bubble/Quick sort"),
            ]),
        ];

        let paragraph = Paragraph::new(controls)
            .block(block)
            .alignment(Alignment::Left);

        paragraph.render(area, buf);
    }

    fn render_info(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Information")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let status = match self.app_status {
            AppStatus::Running => "Running",
            AppStatus::Paused => "Paused",
            AppStatus::Completed => "Completed",
        };

        let speed_ms = self.speed.as_millis();
        let current_algo = self.get_current_algorithm();

        let info = vec![
            Line::from(vec![
                Span::raw("Algorithm: "),
                Span::styled(current_algo.name(), Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::raw("Bars: "),
                Span::styled(
                    format!("{}", self.bars.len()),
                    Style::default().fg(Color::Cyan),
                ),
            ]),
            Line::from(vec![
                Span::raw("Status: "),
                Span::styled(
                    status,
                    Style::default().fg(match self.app_status {
                        AppStatus::Running => Color::Yellow,
                        AppStatus::Paused => Color::Red,
                        AppStatus::Completed => Color::Green,
                    }),
                ),
            ]),
            Line::from(vec![
                Span::raw("Speed: "),
                Span::styled(
                    format!("{}ms", speed_ms),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
        ];

        let paragraph = Paragraph::new(info).block(block).alignment(Alignment::Left);

        paragraph.render(area, buf);
    }
}
