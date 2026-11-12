use unicode_width::UnicodeWidthStr;
use ratatui::{
    Frame,
    buffer::Buffer as RatatuiBuffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::{
    app::{App, MessageRole, fmt_bytes},
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

        let cursor_y = before_cursor.matches('\n').count() as u16;

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
            " llama: {} | port: {} | ctx: {} | mod: {} ",
            llama_status,
            self.core.llama.port,
            self.core.llama.context, 
            self.core.client.model_name(),
        );

        let hotkeys =
            "Input ↓ | Alt+Enter: перенос | Enter: выполнить | Ctrl+Q: выход | :help ";

        let content_block = Block::bordered()
            .border_style(Style::default().fg(theme.border))
            .title(Span::styled(" Ao ", Style::default().fg(theme.title)))
            .title_bottom(
                Line::from(Span::styled(
                    format!(" v{} ", self.core.project_info.version),
                    Style::default().fg(theme.title),
                ))
                .alignment(Alignment::Right),
            );

        let inner = content_block.inner(chunks[0]);

        // Сначала рассчитываем высоту каждого сообщения.
        let mut heights = Vec::with_capacity(self.messages.len());
        let mut total_height: u16 = 0;

        for message in &self.messages {
            let text_width = inner.width.saturating_sub(2).max(1);

            let content_height = message
                .content
                .lines()
                .map(|line| {
                    let width = UnicodeWidthStr::width(line).max(1);
                    width.div_ceil(text_width as usize)
                })
                .sum::<usize>()
                .max(1) as u16;
            let message_height = content_height.saturating_add(2);

            heights.push(message_height);
            total_height = total_height
                .saturating_add(message_height);
        }

        // Виртуальный буфер всей истории.
        let mut virtual_buffer = RatatuiBuffer::empty(Rect::new(
            0,
            0,
            inner.width,
            total_height.max(1),
        ));

        let mut virtual_y: u16 = 0;

        // Отрисовываем сообщения в виртуальный буфер.
        for (message, height) in self.messages.iter().zip(heights.iter()) {
            let (name, color): (&str, _) = match message.role {
                MessageRole::User => (&self.username, theme.input),
                MessageRole::Assistant => ("Ao", theme.assistant),
                MessageRole::System => ("System", theme.foreground),
                MessageRole::Error => ("Error", theme.error),
            };

            let mut parts = vec![fmt_bytes(message.stats.bytes)];

            if let Some(tokens) = message.stats.tokens {
                parts.push(format!("{} T", tokens));
            }

            if let Some(tps) = message.stats.tps {
                parts.push(format!("{:.1} TPS", tps));
            }

            if let Some(time) = message.stats.generation_time {
                parts.push(format!("{:.2} s", time));
            }

            let stats = parts.join(" · ");

            let block = Block::bordered()
                .border_style(Style::default().fg(theme.border))
                .title(Span::styled(
                    format!(" {} ", name),
                    Style::default().fg(color),
                ))
                .title_bottom(
                    Line::from(Span::styled(
                        format!(" {} ", stats),
                        Style::default().fg(theme.title),
                    ))
                    .alignment(Alignment::Right),
                );

            let area = Rect::new(
                0,
                virtual_y,
                inner.width,
                *height,
            );

            let paragraph = Paragraph::new(message.content.as_str())
                .style(
                    Style::default()
                        .bg(theme.background)
                        .fg(color),
                )
                .block(block)
                .wrap(Wrap { trim: false });

            paragraph.render(area, &mut virtual_buffer);

            virtual_y = virtual_y.saturating_add(*height);
        }

        // Рисуем внешнюю рамку Ao.
        frame.render_widget(content_block, chunks[0]);

        // Ограничиваем прокрутку реальной высотой истории.
        let max_scroll = total_height.saturating_sub(inner.height);
        let scroll = self.scroll.min(max_scroll);

        // Копируем видимую часть виртуального буфера на экран.
        let visible_height = inner.height.min(total_height.saturating_sub(scroll));

        for y in 0..visible_height {
            for x in 0..inner.width {
                let source = virtual_buffer.get(x, scroll + y).clone();

                *frame.buffer_mut().get_mut(inner.x + x, inner.y + y) = source;
            }
        }

        // Поле ввода.
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
            .wrap(Wrap { trim: false });

        frame.render_widget(input, chunks[1]);
    }
}

