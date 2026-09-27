use std::{env, num::NonZeroU32};

use softbuffer::{Context, Surface};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, OwnedDisplayHandle},
    window::{Fullscreen, Window, WindowAttributes, WindowId},
};

const GLYPH_HEIGHT: usize = 7;
const HEADER: &str = "-- STOP PEEKING --";
const FOOTER: &str = "PRESS ANY KEY TO RETURN";

fn glyph(character: char) -> [&'static str; GLYPH_HEIGHT] {
    match character.to_ascii_uppercase() {
        'A' => [
            " ### ", "#   #", "#   #", "#####", "#   #", "#   #", "#   #",
        ],
        'B' => [
            "#### ", "#   #", "#   #", "#### ", "#   #", "#   #", "#### ",
        ],
        'C' => [
            " ### ", "#   #", "#    ", "#    ", "#    ", "#   #", " ### ",
        ],
        'D' => [
            "#### ", "#   #", "#   #", "#   #", "#   #", "#   #", "#### ",
        ],
        'E' => [
            "#####", "#    ", "#    ", "#### ", "#    ", "#    ", "#####",
        ],
        'F' => [
            "#####", "#    ", "#    ", "#### ", "#    ", "#    ", "#    ",
        ],
        'G' => [
            " ### ", "#   #", "#    ", "# ###", "#   #", "#   #", " ### ",
        ],
        'H' => [
            "#   #", "#   #", "#   #", "#####", "#   #", "#   #", "#   #",
        ],
        'I' => [
            " ### ", "  #  ", "  #  ", "  #  ", "  #  ", "  #  ", " ### ",
        ],
        'J' => [
            "  ###", "    #", "    #", "    #", "#   #", "#   #", " ### ",
        ],
        'K' => [
            "#   #", "#  # ", "# #  ", "##   ", "# #  ", "#  # ", "#   #",
        ],
        'L' => [
            "#    ", "#    ", "#    ", "#    ", "#    ", "#    ", "#####",
        ],
        'M' => [
            "#   #", "## ##", "# # #", "#   #", "#   #", "#   #", "#   #",
        ],
        'N' => [
            "#   #", "##  #", "##  #", "# # #", "#  ##", "#  ##", "#   #",
        ],
        'O' => [
            " ### ", "#   #", "#   #", "#   #", "#   #", "#   #", " ### ",
        ],
        'P' => [
            "#### ", "#   #", "#   #", "#### ", "#    ", "#    ", "#    ",
        ],
        'Q' => [
            " ### ", "#   #", "#   #", "#   #", "# # #", "#  # ", " ## #",
        ],
        'R' => [
            "#### ", "#   #", "#   #", "#### ", "# #  ", "#  # ", "#   #",
        ],
        'S' => [
            " ####", "#    ", "#    ", " ### ", "    #", "    #", "#### ",
        ],
        'T' => [
            "#####", "  #  ", "  #  ", "  #  ", "  #  ", "  #  ", "  #  ",
        ],
        'U' => [
            "#   #", "#   #", "#   #", "#   #", "#   #", "#   #", " ### ",
        ],
        'V' => [
            "#   #", "#   #", "#   #", "#   #", "#   #", " # # ", "  #  ",
        ],
        'W' => [
            "#   #", "#   #", "#   #", "# # #", "# # #", "## ##", "#   #",
        ],
        'X' => [
            "#   #", "#   #", " # # ", "  #  ", " # # ", "#   #", "#   #",
        ],
        'Y' => [
            "#   #", "#   #", " # # ", "  #  ", "  #  ", "  #  ", "  #  ",
        ],
        'Z' => [
            "#####", "    #", "   # ", "  #  ", " #   ", "#    ", "#####",
        ],
        '0' => [
            " ### ", "#   #", "#  ##", "# # #", "##  #", "#   #", " ### ",
        ],
        '1' => [
            "  #  ", " ##  ", "# #  ", "  #  ", "  #  ", "  #  ", "#####",
        ],
        '2' => [
            " ### ", "#   #", "    #", "   # ", "  #  ", " #   ", "#####",
        ],
        '3' => [
            "#### ", "    #", "    #", " ### ", "    #", "    #", "#### ",
        ],
        '4' => [
            "#   #", "#   #", "#   #", "#####", "    #", "    #", "    #",
        ],
        '5' => [
            "#####", "#    ", "#    ", "#### ", "    #", "    #", "#### ",
        ],
        '6' => [
            " ### ", "#    ", "#    ", "#### ", "#   #", "#   #", " ### ",
        ],
        '7' => [
            "#####", "    #", "   # ", "  #  ", " #   ", " #   ", " #   ",
        ],
        '8' => [
            " ### ", "#   #", "#   #", " ### ", "#   #", "#   #", " ### ",
        ],
        '9' => [
            " ### ", "#   #", "#   #", " ####", "    #", "    #", " ### ",
        ],
        '!' => [
            "  #  ", "  #  ", "  #  ", "  #  ", "  #  ", "     ", "  #  ",
        ],
        '?' => [
            " ### ", "#   #", "    #", "   # ", "  #  ", "     ", "  #  ",
        ],
        '.' => [
            "     ", "     ", "     ", "     ", "     ", "  ## ", "  ## ",
        ],
        '-' => [
            "     ", "     ", "     ", "#####", "     ", "     ", "     ",
        ],
        '\'' => [
            "  ## ", "  ## ", "  #  ", "     ", "     ", "     ", "     ",
        ],
        '*' => [
            "     ", "# # #", " ### ", "#####", " ### ", "# # #", "     ",
        ],
        ',' => [
            "     ", "     ", "     ", "     ", "     ", "  ## ", " ##  ",
        ],
        ':' => [
            "     ", "  ## ", "  ## ", "     ", "  ## ", "  ## ", "     ",
        ],
        '/' => [
            "    #", "   # ", "   # ", "  #  ", " #   ", " #   ", "#    ",
        ],
        '(' => [
            "   ##", "  #  ", " #   ", " #   ", " #   ", "  #  ", "   ##",
        ],
        ')' => [
            "##   ", "  #  ", "   # ", "   # ", "   # ", "  #  ", "##   ",
        ],
        ' ' => [
            "     ", "     ", "     ", "     ", "     ", "     ", "     ",
        ],
        _ => [
            " ### ", "#   #", "    #", "   # ", "  #  ", "     ", "  #  ",
        ],
    }
}

fn wrap_words(message: &str, width: usize) -> Vec<String> {
    let width = width.max(1);
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in message.split_whitespace() {
        let word_length = word.chars().count();
        if word_length > width {
            if !current.is_empty() {
                lines.push(current);
            }
            let mut chunk = String::new();
            for character in word.chars() {
                chunk.push(character);
                if chunk.chars().count() == width {
                    lines.push(chunk);
                    chunk = String::new();
                }
            }
            current = chunk;
        } else {
            let separator = usize::from(!current.is_empty());
            if current.chars().count() + separator + word_length > width {
                lines.push(current);
                current = String::new();
            }
            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(word);
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn layout(message: &str, width: usize, height: usize) -> (Vec<String>, usize) {
    let usable_width = width * 4 / 5;
    let usable_height = height * 3 / 5;
    for scale in (1..=(height / GLYPH_HEIGHT).max(1)).rev() {
        let columns = usable_width / (6 * scale);
        if columns == 0 {
            continue;
        }
        let lines = wrap_words(message, columns);
        let text_height =
            lines.len() * GLYPH_HEIGHT * scale + lines.len().saturating_sub(1) * scale * 2;
        if text_height <= usable_height {
            return (lines, scale);
        }
    }
    (wrap_words(message, (usable_width / 6).max(1)), 1)
}

fn fill(buffer: &mut [u32], width: usize, x: usize, y: usize, w: usize, h: usize, color: u32) {
    for row in y..y.saturating_add(h) {
        for column in x..x.saturating_add(w) {
            if let Some(pixel) = buffer.get_mut(row.saturating_mul(width).saturating_add(column)) {
                *pixel = color;
            }
        }
    }
}

fn draw_text(buffer: &mut [u32], width: usize, text: &str, y: usize, scale: usize, color: u32) {
    let text_width = text.chars().count() * 6 * scale;
    let mut x = width.saturating_sub(text_width) / 2;
    for character in text.chars() {
        for (row, pixels) in glyph(character).iter().enumerate() {
            for (column, pixel) in pixels.chars().enumerate() {
                if pixel == '#' {
                    fill(
                        buffer,
                        width,
                        x + column * scale,
                        y + row * scale,
                        scale,
                        scale,
                        color,
                    );
                }
            }
        }
        x += 6 * scale;
    }
}

fn draw_screen(buffer: &mut [u32], width: usize, height: usize, message: &str) {
    const BACKGROUND: u32 = 0x13151A;
    const TEXT: u32 = 0xF2EFE9;
    const ACCENT: u32 = 0xFF7755;
    const MUTED: u32 = 0x8C939F;

    buffer.fill(BACKGROUND);
    let rule_width = width * 3 / 5;
    fill(
        buffer,
        width,
        (width - rule_width) / 2,
        height / 7,
        rule_width,
        3,
        ACCENT,
    );
    draw_text(buffer, width, HEADER, height / 7 + 24, 3, MUTED);

    let (lines, scale) = layout(message, width, height);
    let line_height = GLYPH_HEIGHT * scale;
    let content_height = lines.len() * line_height + lines.len().saturating_sub(1) * scale * 2;
    let mut y = height.saturating_sub(content_height) / 2;
    for line in lines {
        draw_text(buffer, width, &line, y, scale, TEXT);
        y += line_height + scale * 2;
    }

    draw_text(buffer, width, FOOTER, height * 6 / 7, 2, MUTED);
}

struct App {
    message: String,
    surfaces: Vec<Surface<OwnedDisplayHandle, Window>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.surfaces.is_empty() {
            return;
        }
        let mut monitors = event_loop
            .available_monitors()
            .map(Some)
            .collect::<Vec<_>>();
        if monitors.is_empty() {
            monitors.push(None);
        }
        for monitor in monitors {
            let attributes = WindowAttributes::default()
                .with_decorations(false)
                .with_fullscreen(Some(Fullscreen::Borderless(monitor)));
            let window = event_loop
                .create_window(attributes)
                .expect("failed to open overlay");
            window.set_cursor_visible(false);
            let context =
                Context::new(event_loop.owned_display_handle()).expect("failed to create display");
            let surface = Surface::new(&context, window).expect("failed to create surface");
            surface.window().request_redraw();
            self.surfaces.push(surface);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            }
            | WindowEvent::MouseInput {
                state: ElementState::Pressed,
                ..
            } => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(surface) = self
                    .surfaces
                    .iter_mut()
                    .find(|surface| surface.window().id() == id)
                {
                    if let (Some(width), Some(height)) =
                        (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                    {
                        surface
                            .resize(width, height)
                            .expect("failed to resize overlay");
                    }
                    surface.window().request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(surface) = self
                    .surfaces
                    .iter_mut()
                    .find(|surface| surface.window().id() == id)
                {
                    let size = surface.window().inner_size();
                    if let (Some(width), Some(height)) =
                        (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                    {
                        surface
                            .resize(width, height)
                            .expect("failed to resize overlay");
                        let mut buffer = surface.buffer_mut().expect("failed to draw overlay");
                        draw_screen(
                            &mut buffer,
                            size.width as usize,
                            size.height as usize,
                            &self.message,
                        );
                        buffer.present().expect("failed to present overlay");
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let message = env::args().skip(1).collect::<Vec<_>>().join(" ");
    if message.is_empty() || matches!(message.as_str(), "-h" | "--help") {
        eprintln!("Usage: stoppeaking <message>");
        eprintln!("Example: stoppeaking 'Back to the task'");
        return;
    }

    let event_loop = EventLoop::new().expect("failed to start event loop");
    event_loop
        .run_app(&mut App {
            message,
            surfaces: Vec::new(),
        })
        .expect("failed to run overlays");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_scales_short_messages_up() {
        let (lines, scale) = layout("FOCUS", 1920, 1080);
        assert_eq!(lines, ["FOCUS"]);
        assert!(scale > 10);
    }

    #[test]
    fn layout_keeps_long_messages_within_bounds() {
        let (lines, scale) = layout(
            "IF YOU'RE READING THIS YOU SHOULDN'T BE PEEKING",
            1920,
            1080,
        );
        assert!(
            lines
                .iter()
                .all(|line| line.chars().count() * 6 * scale <= 1920 * 4 / 5)
        );
    }

    #[test]
    fn common_message_symbols_have_glyphs() {
        assert_ne!(glyph('\''), glyph(' '));
        assert_ne!(glyph('*'), glyph(' '));
        assert_ne!(glyph('/'), glyph(' '));
    }
}
