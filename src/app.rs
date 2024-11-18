use crate::product::Product;
use tui::widgets::TableState;

pub struct App {
    pub products: Vec<Product>,
    pub state: TableState,
}

impl App {
    pub fn new(products: Vec<Product>) -> App {
        let mut app = App {
            products,
            state: TableState::default(),
        };
        app.state.select(Some(0));
        app
    }

    pub fn next(&mut self) {
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

    pub fn previous(&mut self) {
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
