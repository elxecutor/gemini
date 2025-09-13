use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, Paragraph, Wrap,
    },
    Frame,
};
use unicode_width::{UnicodeWidthStr, UnicodeWidthChar};

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub content: String,
    pub is_user: bool,
    pub timestamp: std::time::SystemTime,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub messages: Vec<ChatMessage>,
    pub input: String,
    pub input_cursor: usize,
    pub scroll_offset: usize,
    pub is_loading: bool,
    pub status_message: String,
    pub animation_frame: usize,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            input: String::new(),
            input_cursor: 0,
            scroll_offset: 0,
            is_loading: false,
            status_message: "Ready to chat with Gemini! 🚀".to_string(),
            animation_frame: 0,
        }
    }
}

impl AppState {
    pub fn add_message(&mut self, content: String, is_user: bool) {
        self.messages.push(ChatMessage {
            content,
            is_user,
            timestamp: std::time::SystemTime::now(),
        });
        // Auto-scroll to bottom
        self.scroll_offset = self.messages.len().saturating_sub(1);
    }

    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.input_cursor, c);
        self.input_cursor += 1;
    }

    pub fn delete_char(&mut self) {
        if self.input_cursor > 0 {
            self.input.remove(self.input_cursor - 1);
            self.input_cursor -= 1;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.input_cursor > 0 {
            self.input_cursor -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.input_cursor < self.input.len() {
            self.input_cursor += 1;
        }
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.input_cursor = 0;
    }

    pub fn increment_animation(&mut self) {
        self.animation_frame = (self.animation_frame + 1) % 100;
    }
}

pub fn ui(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),   // Title
            Constraint::Min(0),      // Chat area
            Constraint::Length(3),   // Input
            Constraint::Length(3),   // Status
        ])
        .split(f.area());

    // Crazy animated title
    render_title(f, chunks[0], app.animation_frame);

    // Chat messages area
    render_chat_area(f, chunks[1], app);

    // Input area
    render_input_area(f, chunks[2], app);

    // Status bar
    render_status_bar(f, chunks[3], app);
}

fn render_title(f: &mut Frame, area: Rect, frame: usize) {
    let rainbow_colors = [
        Color::Red,
        Color::Yellow,
        Color::Green,
        Color::Cyan,
        Color::Blue,
        Color::Magenta,
    ];
    
    let title_text = "GEMINI CHAT TUI";
    let mut spans = Vec::new();
    
    for (i, ch) in title_text.char_indices() {
        let color_index = (i + frame / 5) % rainbow_colors.len();
        spans.push(Span::styled(
            ch.to_string(),
            Style::default()
                .fg(rainbow_colors[color_index])
                .add_modifier(Modifier::BOLD),
        ));
    }
    
    let title = Paragraph::new(Line::from(spans))
        .block(Block::default().borders(Borders::ALL).border_style(
            Style::default().fg(rainbow_colors[frame % rainbow_colors.len()])
        ))
        .style(Style::default().bg(Color::Black))
        .wrap(Wrap { trim: true });
    
    f.render_widget(title, area);
}

fn render_chat_area(f: &mut Frame, area: Rect, app: &AppState) {
    let mut items = Vec::new();
    
    for (_i, message) in app.messages.iter().enumerate() {
        let timestamp = format_timestamp(&message.timestamp);
        
        if message.is_user {
            // User message (right-aligned, blue bubble)
            let max_width = area.width.saturating_sub(10) as usize; // More conservative width
            let wrapped_content = wrap_text(&message.content, max_width);
            
            // Calculate the width needed for this bubble
            let content_width = wrapped_content.iter()
                .map(|line| format!("You: {}", line).width())
                .max()
                .unwrap_or(10)
                .min(max_width);
            
            let bubble_width = content_width + 4; // Add padding
            let timestamp_header = format!("You {}", timestamp);
            let header_width = timestamp_header.width() + 4;
            let actual_width = bubble_width.max(header_width).min(max_width + 4);
            
            // Create top border
            let top_border = format!("╭─ {} {}╮", 
                timestamp_header,
                "─".repeat(actual_width.saturating_sub(timestamp_header.width() + 5))
            );
            
            let mut lines = vec![
                Line::from(vec![
                    Span::raw(" ".repeat(area.width.saturating_sub(actual_width as u16 + 2) as usize)),
                    Span::styled(top_border, Style::default().fg(Color::Cyan)),
                ]),
            ];
            
            // Add content lines with markdown parsing
            for line in wrapped_content {
                let content_prefix = "You: ";
                let right_padding_size = actual_width.saturating_sub(content_prefix.width() + line.width() + 2);
                let right_padding = " ".repeat(right_padding_size);
                let left_padding = " ".repeat(area.width.saturating_sub(actual_width as u16 + 2) as usize);
                
                let mut line_spans = vec![
                    Span::raw(left_padding),
                    Span::styled("│ ", Style::default().fg(Color::Cyan)),
                    Span::styled(content_prefix, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                ];
                
                // Parse markdown for user messages too
                line_spans.extend(parse_markdown_spans(&line));
                
                line_spans.push(Span::raw(right_padding));
                line_spans.push(Span::styled(" │", Style::default().fg(Color::Cyan)));
                
                lines.push(Line::from(line_spans));
            }
            
            // Create bottom border
            let bottom_border = format!("╰{}╯", "─".repeat(actual_width));
            lines.push(Line::from(vec![
                Span::raw(" ".repeat(area.width.saturating_sub(actual_width as u16 + 2) as usize)),
                Span::styled(bottom_border, Style::default().fg(Color::Cyan)),
            ]));
            lines.push(Line::from(""));
            
            items.push(ListItem::new(lines));
        } else {
            // Gemini message (left-aligned, green bubble)
            let max_width = area.width.saturating_sub(8) as usize; // More conservative width
            let wrapped_content = wrap_text(&message.content, max_width);
            
            // Calculate the width needed for this bubble
            let content_width = wrapped_content.iter()
                .map(|line| line.width())
                .max()
                .unwrap_or(10)
                .min(max_width);
            
            let timestamp_header = format!("🤖 Gemini {}", timestamp);
            let header_width = timestamp_header.width() + 4;
            let actual_width = content_width.max(header_width).min(max_width);
            
            // Create top border
            let top_border = format!("╭─ {} {}╮",
                timestamp_header,
                "─".repeat(actual_width.saturating_sub(timestamp_header.width() + 5))
            );
            
            let mut lines = vec![
                Line::from(vec![
                    Span::styled(top_border, Style::default().fg(Color::Green)),
                ]),
            ];
            
            // Add content lines with markdown parsing
            for line in wrapped_content {
                let padding_size = actual_width.saturating_sub(line.width() + 2);
                let padding = " ".repeat(padding_size);
                
                let mut line_spans = vec![
                    Span::styled("│ ", Style::default().fg(Color::Green)),
                ];
                
                // Parse markdown and add spans
                line_spans.extend(parse_markdown_spans(&line));
                
                // Add padding and closing border
                line_spans.push(Span::raw(padding));
                line_spans.push(Span::styled(" │", Style::default().fg(Color::Green)));
                
                lines.push(Line::from(line_spans));
            }
            
            // Create bottom border
            let bottom_border = format!("╰{}╯", "─".repeat(actual_width));
            lines.push(Line::from(vec![
                Span::styled(bottom_border, Style::default().fg(Color::Green)),
            ]));
            lines.push(Line::from(""));
            
            items.push(ListItem::new(lines));
        }
    }
    
    // Add loading animation if waiting for response
    if app.is_loading {
        let loading_chars = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let loading_char = loading_chars[app.animation_frame % loading_chars.len()];
        
        let loading_text = format!("{} Processing your message...", loading_char);
        let timestamp_header = "Gemini is thinking...";
        let content_width = loading_text.width().max(timestamp_header.width());
        let actual_width = content_width + 4;
        
        // Create top border
        let top_border = format!("╭─ {} {}╮",
            timestamp_header,
            "─".repeat(actual_width.saturating_sub(timestamp_header.width() + 5))
        );
        
        // Create content line with padding
        let padding_size = actual_width.saturating_sub(loading_text.width() + 2);
        let padding = " ".repeat(padding_size);
        
        // Create bottom border
        let bottom_border = format!("╰{}╯", "─".repeat(actual_width));
        
        items.push(ListItem::new(vec![
            Line::from(vec![
                Span::styled(top_border, Style::default().fg(Color::Yellow)),
            ]),
            Line::from(vec![
                Span::styled("│ ", Style::default().fg(Color::Yellow)),
                Span::styled(loading_text, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(padding),
                Span::styled(" │", Style::default().fg(Color::Yellow)),
            ]),
            Line::from(vec![
                Span::styled(bottom_border, Style::default().fg(Color::Yellow)),
            ]),
            Line::from(""),
        ]));
    }
    
    let chat_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Chat")
                .border_style(Style::default().fg(Color::White))
        )
        .style(Style::default().bg(Color::Black));
    
    f.render_widget(chat_list, area);
}

fn render_input_area(f: &mut Frame, area: Rect, app: &AppState) {
    let input_text = if app.input.is_empty() {
        "Type your message here... (Press Enter to send, Ctrl+C to quit)"
    } else {
        &app.input
    };
    
    let input_style = if app.input.is_empty() {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default().fg(Color::White)
    };
    
    let input = Paragraph::new(input_text)
        .style(input_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Your Message")
                .border_style(Style::default().fg(Color::Magenta))
        );
    
    f.render_widget(input, area);
    
    // Render cursor
    if !app.input.is_empty() {
        let cursor_x = area.x + 1 + app.input_cursor as u16;
        let cursor_y = area.y + 1;
        f.set_cursor_position((cursor_x, cursor_y));
    }
}

fn render_status_bar(f: &mut Frame, area: Rect, app: &AppState) {
    let status_color = if app.is_loading {
        Color::Yellow
    } else {
        Color::Green
    };
    
    let status = Paragraph::new(app.status_message.as_str())
        .style(Style::default().fg(status_color).add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Status")
                .border_style(Style::default().fg(status_color))
        );
    
    f.render_widget(status, area);
}

fn format_timestamp(timestamp: &std::time::SystemTime) -> String {
    use std::time::UNIX_EPOCH;
    
    if let Ok(duration) = timestamp.duration_since(UNIX_EPOCH) {
        let secs = duration.as_secs();
        let hours = (secs / 3600) % 24;
        let minutes = (secs / 60) % 60;
        let seconds = secs % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        "??:??:??".to_string()
    }
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    
    // Handle minimum width to prevent issues
    let min_width = 10;
    let actual_width = width.max(min_width);
    
    for word in text.split_whitespace() {
        // Handle very long words by breaking them
        if word.width() > actual_width {
            // If we have content in current line, push it first
            if !current_line.is_empty() {
                lines.push(current_line);
                current_line = String::new();
            }
            
            // Break the long word into chunks
            let mut remaining = word;
            while !remaining.is_empty() {
                let mut chunk_end = 0;
                let mut chunk_width = 0;
                
                for (i, c) in remaining.char_indices() {
                    let char_width = c.width().unwrap_or(0);
                    if chunk_width + char_width > actual_width {
                        break;
                    }
                    chunk_width += char_width;
                    chunk_end = i + c.len_utf8();
                }
                
                if chunk_end == 0 {
                    // Single character is too wide, take it anyway
                    chunk_end = remaining.chars().next().unwrap().len_utf8();
                }
                
                let chunk = &remaining[..chunk_end];
                lines.push(chunk.to_string());
                remaining = &remaining[chunk_end..];
            }
        } else {
            // Normal word processing
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.width() + 1 + word.width() <= actual_width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line);
                current_line = word.to_string();
            }
        }
    }
    
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    if lines.is_empty() {
        lines.push(String::new());
    }
    
    lines
}

fn parse_markdown_spans(text: &str) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let mut chars = text.chars().peekable();
    let mut current_text = String::new();
    
    while let Some(ch) = chars.next() {
        if ch == '*' && chars.peek() == Some(&'*') {
            // Found start of bold text
            chars.next(); // consume second *
            
            // Push any accumulated normal text
            if !current_text.is_empty() {
                spans.push(Span::styled(current_text.clone(), Style::default().fg(Color::White)));
                current_text.clear();
            }
            
            // Collect bold text until next **
            let mut bold_text = String::new();
            let mut found_end = false;
            
            while let Some(ch) = chars.next() {
                if ch == '*' && chars.peek() == Some(&'*') {
                    chars.next(); // consume second *
                    found_end = true;
                    break;
                }
                bold_text.push(ch);
            }
            
            if found_end {
                spans.push(Span::styled(
                    bold_text,
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                ));
            } else {
                // No closing **, treat as regular text
                current_text.push_str("**");
                current_text.push_str(&bold_text);
            }
        } else {
            current_text.push(ch);
        }
    }
    
    // Push any remaining normal text
    if !current_text.is_empty() {
        spans.push(Span::styled(current_text, Style::default().fg(Color::White)));
    }
    
    spans
}