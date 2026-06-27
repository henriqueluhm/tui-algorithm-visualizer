use crate::app::{App, AppStatus, MIN_BARS_SIZE};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{BarChart, Block, BorderType, Paragraph, Widget, Wrap},
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let tiny = self.is_tiny_layout();
        let compact = self.is_compact_layout();

        let bottom_percent = if tiny {
            40
        } else if compact {
            30
        } else {
            20
        };

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(100 - bottom_percent),
                Constraint::Percentage(bottom_percent),
            ])
            .split(area);

        self.render_visualization(main_chunks[0], buf);

        if tiny {
            self.render_compact_footer(main_chunks[1], buf);
        } else if compact {
            let footer_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(4), Constraint::Min(3)])
                .split(main_chunks[1]);

            self.render_controls(footer_chunks[0], buf, true);
            self.render_info(footer_chunks[1], buf, true);
        } else {
            let footer_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(main_chunks[1]);

            self.render_controls(footer_chunks[0], buf, false);
            self.render_info(footer_chunks[1], buf, false);
        }
    }
}

impl App {
    fn render_visualization(&self, area: Rect, buf: &mut Buffer) {
        let max_bars = self.max_bars();
        let block = Block::bordered()
            .title(format!(
                "Algorithm Visualization (max {} bars)",
                max_bars
            ))
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let inner = block.inner(area);
        block.render(area, buf);

        let current_algo = self.get_current_algorithm();
        let bars = current_algo.get_data();
        let current_indices = current_algo.get_current_indices();
        let comparisons = current_algo.get_comparisons();

        if bars.is_empty() {
            return;
        }

        let max_value = *bars.iter().max().unwrap_or(&1) as u64;
        let (bar_width, bar_gap) = self.bar_chart_layout(bars.len());
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
            .bar_width(bar_width)
            .bar_gap(bar_gap)
            .bar_style(Style::default().fg(Color::White))
            .value_style(Style::default().fg(Color::White).bg(Color::White));

        bar_chart.render(inner, buf);
    }

    fn render_controls(&self, area: Rect, buf: &mut Buffer, compact: bool) {
        let block = Block::bordered()
            .title("Controls")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let controls = if compact {
            vec![
                Line::from(vec![
                    Span::styled("Space", key_style()),
                    Span::raw(" Play  "),
                    Span::styled("R", key_style()),
                    Span::raw(" Reset  "),
                    Span::styled("S", key_style()),
                    Span::raw(" Shuffle  "),
                    Span::styled("↑↓", key_style()),
                    Span::raw(" Speed  "),
                    Span::styled("←→", key_style()),
                    Span::raw(" Bars"),
                ]),
                Line::from(vec![
                    Span::styled("1", key_style()),
                    Span::raw(" Bubble  "),
                    Span::styled("2", key_style()),
                    Span::raw(" Quick  "),
                    Span::styled("3", key_style()),
                    Span::raw(" Merge  "),
                    Span::styled("4", key_style()),
                    Span::raw(" Selection  "),
                    Span::styled("Q", key_style()),
                    Span::raw(" Quit"),
                ]),
            ]
        } else {
            vec![
                control_line("Space", "Start/Pause"),
                control_line("R", "Reset"),
                control_line("S", "Shuffle & Reset"),
                control_line("↑/↓", "Speed Up/Down"),
                control_line("Q/Esc", "Quit"),
                control_line("←/→", "Decrease/Increase bars"),
                control_line("1/2/3/4", "Bubble / Quick / Merge / Selection"),
            ]
        };

        Paragraph::new(controls)
            .block(block)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }

    fn render_info(&self, area: Rect, buf: &mut Buffer, compact: bool) {
        let block = Block::bordered()
            .title("Information")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let status = match self.app_status {
            AppStatus::Running => "Running",
            AppStatus::Paused => "Paused",
            AppStatus::Completed => "Completed",
        };

        let current_algo = self.get_current_algorithm();
        let status_color = match self.app_status {
            AppStatus::Running => Color::Yellow,
            AppStatus::Paused => Color::Red,
            AppStatus::Completed => Color::Green,
        };

        let info = if compact {
            vec![Line::from(vec![
                Span::raw("Algo: "),
                Span::styled(current_algo.name(), Style::default().fg(Color::Cyan)),
                Span::raw("  Bars: "),
                Span::styled(format!("{}", self.bars.len()), Style::default().fg(Color::Cyan)),
                Span::raw("/"),
                Span::styled(format!("{}", self.max_bars()), Style::default().fg(Color::DarkGray)),
                Span::raw("  Status: "),
                Span::styled(status, Style::default().fg(status_color)),
                Span::raw("  Speed: "),
                Span::styled(self.speed_label(), Style::default().fg(Color::Yellow)),
            ])]
        } else {
            vec![
                Line::from(vec![
                    Span::raw("Algorithm: "),
                    Span::styled(current_algo.name(), Style::default().fg(Color::Cyan)),
                ]),
                Line::from(vec![
                    Span::raw("Bars: "),
                    Span::styled(format!("{}", self.bars.len()), Style::default().fg(Color::Cyan)),
                    Span::raw(" (max "),
                    Span::styled(format!("{}", self.max_bars()), Style::default().fg(Color::Cyan)),
                    Span::raw(", min "),
                    Span::styled(format!("{MIN_BARS_SIZE}"), Style::default().fg(Color::Cyan)),
                    Span::raw(")"),
                ]),
                Line::from(vec![
                    Span::raw("Status: "),
                    Span::styled(status, Style::default().fg(status_color)),
                ]),
                Line::from(vec![
                    Span::raw("Speed: "),
                    Span::styled(self.speed_label(), Style::default().fg(Color::Yellow)),
                ]),
                Line::from(vec![
                    Span::raw("Terminal: "),
                    Span::styled(
                        format!("{}×{}", self.viewport_width, self.viewport_height),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]),
            ]
        };

        Paragraph::new(info)
            .block(block)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }

    fn render_compact_footer(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Controls & Info")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let status = match self.app_status {
            AppStatus::Running => "Running",
            AppStatus::Paused => "Paused",
            AppStatus::Completed => "Done",
        };

        let current_algo = self.get_current_algorithm();
        let lines = vec![
            Line::from(vec![
                Span::styled(current_algo.name(), Style::default().fg(Color::Cyan)),
                Span::raw(" | "),
                Span::raw("Bars "),
                Span::styled(format!("{}", self.bars.len()), Style::default().fg(Color::Cyan)),
                Span::raw("/"),
                Span::styled(format!("{}", self.max_bars()), Style::default().fg(Color::DarkGray)),
                Span::raw(" | "),
                Span::styled(status, Style::default().fg(Color::Yellow)),
                Span::raw(" | Speed "),
                Span::styled(self.speed_label(), Style::default().fg(Color::Yellow)),
            ]),
            Line::from(vec![
                Span::styled("Sp", key_style()),
                Span::raw(" Play "),
                Span::styled("R", key_style()),
                Span::raw(" Reset "),
                Span::styled("S", key_style()),
                Span::raw(" Shuf "),
                Span::styled("↑↓", key_style()),
                Span::raw(" Spd "),
                Span::styled("←→", key_style()),
                Span::raw(" Bars"),
            ]),
            Line::from(vec![
                Span::styled("1", key_style()),
                Span::raw(" Bub "),
                Span::styled("2", key_style()),
                Span::raw(" Qck "),
                Span::styled("3", key_style()),
                Span::raw(" Mrg "),
                Span::styled("4", key_style()),
                Span::raw(" Sel "),
                Span::styled("Q", key_style()),
                Span::raw(" Quit"),
            ]),
        ];

        Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}

fn key_style() -> Style {
    Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD)
}

fn control_line(key: &str, action: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(key.to_string(), key_style()),
        Span::raw(format!(" - {action}")),
    ])
}
