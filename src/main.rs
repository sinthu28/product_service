use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{error::Error, fs::File, io::{self, Read}};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Product {
    _id: String,
    actual_price: String,
    average_rating: String,
    brand: String,
    category: String,
    crawled_at: String,
    description: String,
    discount: String,
    images: Vec<String>,
    out_of_stock: bool,
    pid: String,
    product_details: Vec<std::collections::HashMap<String, String>>,
    seller: String,
    selling_price: String,
    sub_category: String,
    title: String,
    url: String,
}

struct App {
    products: Vec<Product>,
    state: TableState,
}

impl App {
    fn new(products: Vec<Product>) -> App {
        let mut app = App {
            products,
            state: TableState::default(),
        };
        app.state.select(Some(0));
        app
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.products.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.products.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

// fn parse_json_product(json_str: &str) -> Result<Product, Box<dyn Error>> {
//     // First parse as a generic JSON Value
//     let v: Value = serde_json::from_str(json_str)?;
    
//     // Then convert to our Product type
//     let product: Product = serde_json::from_value(v)?;
    
//     Ok(product)
// }

fn parse_json_product(json_str: &str) -> Result<Vec<Product>, Box<dyn Error>> {
    let v: Value = serde_json::from_str(json_str)?;
    let products: Vec<Product> = serde_json::from_value(v)?;
    Ok(products)
  }

// fn read_json_file(file_path: &str) -> Result<Product, Box<dyn Error>> {
//     // Read the file content into a string
//     let mut file = File::open(file_path)?;
//     let mut json_str = String::new();
//     file.read_to_string(&mut json_str)?;
    
//     // Parse JSON string into Product
//     parse_json_product(&json_str)
// }

fn read_json_file(file_path: &str) -> Result<Vec<Product>, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)?;
    let products = parse_json_product(&json_str)?;
    Ok(products)
  }

fn main() -> Result<(), Box<dyn Error>> {
    // Read data from file
    // let product = match read_json_file("data/product.json") {
    //     Ok(prod) => prod,
    //     Err(e) => {
    //         eprintln!("Error reading product data: {}", e);
    //         return Err(e);
    //     }
    // };

    let products = match read_json_file("data/product.json") {
        Ok(prods) => prods,
        Err(e) => {
          eprintln!("Error reading product data: {}", e);
          return Err(e);
        }
      };

    // let products = vec![product];

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let app = App::new(products);
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
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
        "Title",
        "Brand",
        "Price",
        "Discount",
        "Rating",
        "Category",
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


// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use std::{error::Error, io};
// use tui::{
//     backend::{Backend, CrosstermBackend},
//     layout::{Constraint, Layout},
//     style::{Color, Style},
//     widgets::{Block, Borders, Row, Table},
//     Frame, Terminal,
// };
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Product {
//     product_name: String,
//     price: f64,
//     stock: u32,
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     // Create sample product data
//     let products = vec![
//         Product {
//             product_name: "Laptop".to_string(),
//             price: 999.99,
//             stock: 10,
//         },
//         Product {
//             product_name: "Smartphone".to_string(),
//             price: 699.99,
//             stock: 25,
//         },
//         Product {
//             product_name: "Headphones".to_string(),
//             price: 199.99,
//             stock: 50,
//         },
//         Product {
//             product_name: "Keyboard".to_string(),
//             price: 49.99,
//             stock: 100,
//         },
//         Product {
//             product_name: "Mouse".to_string(),
//             price: 29.99,
//             stock: 150,
//         },
//     ];

//     // Initialize terminal
//     enable_raw_mode()?;
//     let mut stdout = io::stdout();
//     execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;

//     // Run the TUI loop
//     let res = run_app(&mut terminal, products);

//     // Restore terminal
//     disable_raw_mode()?;
//     execute!(
//         terminal.backend_mut(),
//         LeaveAlternateScreen,
//         DisableMouseCapture
//     )?;
//     terminal.show_cursor()?;

//     if let Err(err) = res {
//         println!("{:?}", err)
//     }

//     Ok(())
// }

// fn run_app<B: Backend>(terminal: &mut Terminal<B>, products: Vec<Product>) -> io::Result<()> {
//     loop {
//         terminal.draw(|f| ui(f, &products))?;

//         if event::poll(std::time::Duration::from_millis(200))? {
//             if let event::Event::Key(key) = event::read()? {
//                 if key.code == KeyCode::Char('q') {
//                     return Ok(());  // Exit on 'q' key
//                 }
//             }
//         }
//     }
// }

// fn ui<B: Backend>(f: &mut Frame<B>, products: &[Product]) {
//     // Create layout
//     let chunks = Layout::default()
//         .constraints([Constraint::Percentage(100)].as_ref())
//         .split(f.size());

//     // Build rows from product data
//     let rows: Vec<Row> = products.iter().map(|p| {
//         Row::new(vec![
//             p.product_name.clone(),
//             format!("{:.2}", p.price),
//             format!("{}", p.stock),
//         ])
//     }).collect();

//     // Create a table
//     let table = Table::new(rows)
//         .header(Row::new(vec!["Product Name", "Price", "Stock"]).style(Style::default().fg(Color::Yellow)))
//         .block(Block::default().borders(Borders::ALL).title("Product List"))
//         .widths(&[
//             Constraint::Percentage(50),
//             Constraint::Percentage(25),
//             Constraint::Percentage(25),
//         ]);

//     // Render the table
//     f.render_widget(table, chunks[0]);
// }

