use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame, Terminal,
};
use crossterm::event::{self, KeyCode};
use std::io;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let event::Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(2)
        .split(f.size());

    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(Color::Yellow);
    let normal_style = Style::default().fg(Color::White);

    let header_cells = [
        "Title", "Brand", "Price", "Discount", "Rating", "Category",
    ]
    .iter()
    .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));

    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);

    let rows = app.products.iter().map(|item| {
        let cells = vec![
            Cell::from(item.title.clone()),
            Cell::from(item.brand.clone()),
            Cell::from(item.actual_price.clone()),
            Cell::from(item.discount.clone()),
            Cell::from(item.average_rating.clone()),
            Cell::from(item.category.clone()),
        ];
        Row::new(cells).height(2)
    });

    let table = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Product Details"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(25),
            Constraint::Percentage(15),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(30),
        ]);

    f.render_stateful_widget(table, rects[0], &mut app.state);
}
