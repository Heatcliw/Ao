use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::{
    app::{fmt_bytes, App},
    theme::THEMES,
};

impl App {
    pub(crate) fn draw(&self, frame: &mut Frame) {
        let theme = &THEMES[self.theme_index];
        let input_lines = self.buffer.content.lines().count().max(1) as u16;
        let input_height = input_lines + 2;

        let chunks = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(input_height),
        ])
        .split(frame.area());

        let before_cursor = &self.buffer.content[..self.buffer.cursor];

        let cursor_x = before_cursor
            .lines()
            .last()
            .unwrap_or("")
            .chars()
            .count() as u16;

        let cursor_y = before_cursor
            .matches('\n')
            .count() as u16;

        frame.set_cursor_position((
            chunks[1].x + 1 + cursor_x,
            chunks[1].y + 1 + cursor_y,
        ));

        let req_bytes = self
            .core
            .request
            .content
            .as_ref()
            .map(|content| content.len())
            .unwrap_or(0);

        let llama_status = if self.core.llama.server.is_some() {
            "ON"
        } else {
            "OFF"
        };

        let status = format!(
            " llama: {} | port: {} | ctx: {} | req: {} ",
            llama_status,
            self.core.llama.port,
            self.core.llama.context,
            fmt_bytes(req_bytes)
        );

        let hotkeys =
            "Input ↓ | Alt+Enter: перенос | Enter: выполнить | Ctrl+Q: выход | :help ";

        let lines: Vec<Line> = self
            .output
            .lines()
            .map(|line| {
                if line.starts_with("Ao: ") {
                    Line::from(Span::styled(
                        line.to_string(),
                        Style::default().fg(theme.assistant),
                    ))
                } else if line.starts_with("Error: ") {
                    Line::from(Span::styled(
                        line.to_string(),
                        Style::default().fg(theme.error),
                    ))
                } else {
                    Line::from(line.to_string())
                }
            })
            .collect();

        let content = Paragraph::new(lines)
            .style(
                Style::default()
                    .bg(theme.background)
                    .fg(theme.foreground),
            )
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(theme.border))
                    .title(Span::styled(
                        " Ao ",
                        Style::default().fg(theme.title),
                    )),
            )
            .scroll((self.scroll, 0))
            .wrap(ratatui::widgets::Wrap { trim: false });

        let input = Paragraph::new(self.buffer.content.as_str())
            .style(
                Style::default()
                    .bg(theme.background)
                    .fg(theme.input),
            )
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(theme.border))
                    .title(hotkeys)
                    .title_bottom(status),
            )
            .wrap(ratatui::widgets::Wrap { trim: false });

        frame.render_widget(content, chunks[0]);
        frame.render_widget(input, chunks[1]);
    }
}
