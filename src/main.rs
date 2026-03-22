use std::io;

use noise::{NoiseFn, SuperSimplex};
use ratzilla::ratatui::style::{Color, Style};
use ratzilla::ratatui::text::{Line, Span, Text};
use ratzilla::ratatui::widgets::Paragraph;
use ratzilla::ratatui::{Frame, Terminal};
use ratzilla::{CanvasBackend, WebRenderer};

fn main() -> io::Result<()> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let backend = CanvasBackend::new()?;
    let terminal = Terminal::new(backend)?;

    let perlin = SuperSimplex::new(42);
    let mut time: f64 = 0.0;

    terminal.draw_web(move |frame| {
        render(frame, &perlin, time);
        time += 0.003;
    });

    Ok(())
}

fn render(frame: &mut Frame, perlin: &SuperSimplex, time: f64) {
    let width = frame.area().width as usize;
    let height = frame.area().height as usize;

    let lines: Vec<Line> = (0..height)
        .map(|y| {
            Line::from(
                (0..width)
                    .map(|x| terrain_span(sample(perlin, x, y, time)))
                    .collect::<Vec<Span>>(),
            )
        })
        .collect();

    frame.render_widget(Paragraph::new(Text::from(lines)), frame.area());
}

fn sample(perlin: &SuperSimplex, x: usize, y: usize, time: f64) -> f64 {
    let nx = x as f64 * 0.03;
    let ny = y as f64 * 0.06;

    let o1 = perlin.get([nx, ny, time]);
    let o2 = perlin.get([nx * 2.0, ny * 2.0, time * 1.5]) * 0.50;
    let o3 = perlin.get([nx * 4.0, ny * 4.0, time * 2.0]) * 0.25;
    let o4 = perlin.get([nx * 8.0, ny * 8.0, time * 2.5]) * 0.125;

    let sum = o1 + o2 + o3 + o4;
    let max = 1.0 + 0.50 + 0.25 + 0.125;
    (sum + max) / (max * 2.0)
}

fn terrain_span(val: f64) -> Span<'static> {
    let (ch, r, g, b) = match val {
        v if v < 0.30 => ("≈", 0, 20, 80),     // deep sea
        v if v < 0.40 => ("≈", 30, 80, 180),   // sea
        v if v < 0.43 => ("~", 60, 140, 200),  // shallow water
        v if v < 0.46 => ("·", 210, 190, 140), // beach
        v if v < 0.55 => (",", 120, 160, 60),  // grass
        v if v < 0.70 => ("♣", 40, 110, 30),   // forest
        v if v < 0.76 => ("∧", 110, 100, 90),  // foothills
        v if v < 0.83 => ("▲", 140, 120, 100), // mountain
        v if v < 0.92 => ("▲", 180, 170, 160), // high mountain
        _ => ("*", 240, 240, 255),
    };
    Span::styled(ch, Style::default().fg(Color::Rgb(r, g, b)))
}
