use std::sync::{Arc, Mutex};

use better_cursive_table::{SelectMode, TableBuilder, TableDataRow, TableView};
use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout};
use rand::Rng;

fn random_cell(rng: &mut impl Rng) -> String {
    rng.gen_range(0..1000).to_string()
}

struct TableState {
    column_headers: Vec<String>,
    data: Vec<Vec<String>>,
}

impl TableState {
    fn new() -> Self {
        let mut state = Self {
            column_headers: Vec::new(),
            data: Vec::new(),
        };
        for _ in 0..3 {
            state.add_column();
        }
        for _ in 0..3 {
            state.add_row();
        }
        state
    }

    fn add_row(&mut self) {
        let mut rng = rand::thread_rng();
        let row = (0..self.column_headers.len())
            .map(|_| random_cell(&mut rng))
            .collect();
        self.data.push(row);
    }

    fn remove_row(&mut self) {
        self.data.pop();
    }

    fn add_column(&mut self) {
        let label = format!("C{}", self.column_headers.len() + 1);
        self.column_headers.push(label);
        let mut rng = rand::thread_rng();
        for row in &mut self.data {
            row.push(random_cell(&mut rng));
        }
    }

    fn remove_column(&mut self) {
        if self.column_headers.pop().is_some() {
            for row in &mut self.data {
                row.pop();
            }
        }
    }
}

fn build_table(
    state: &TableState,
    shared: &Arc<Mutex<TableState>>,
) -> TableView<TableDataRow<String>, usize> {
    let mut table = TableBuilder::new()
        .column_header(state.column_headers.clone())
        .data(state.data.clone())
        .sortable(false)
        .build();

    let shared = Arc::clone(shared);
    table.set_on_submit_cell(move |s, row, col| {
        open_cell_editor(s, Arc::clone(&shared), row, col);
    });
    table.set_selection_mode(SelectMode::Cell);

    table
}

fn refresh_table(siv: &mut cursive::Cursive, state: &Arc<Mutex<TableState>>) {
    let table = {
        let state_guard = state.lock().expect("table state lock");
        build_table(&state_guard, state)
    };
    siv.call_on_name("table", |view: &mut TableView<TableDataRow<String>, usize>| {
        *view = table;
    });
}

fn open_cell_editor(
    siv: &mut cursive::Cursive,
    state: Arc<Mutex<TableState>>,
    row: usize,
    col: usize,
) {
    let current = {
        let state = state.lock().expect("table state lock");
        state
            .data
            .get(row)
            .and_then(|r| r.get(col))
            .cloned()
            .unwrap_or_default()
    };

    let state_submit = Arc::clone(&state);
    let edit = EditView::new().content(current).on_submit(move |s, text| {
        if let Ok(mut state) = state_submit.lock() {
            if row < state.data.len() {
                let cols = state.column_headers.len();
                if cols > 0 && state.data[row].len() < cols {
                    state.data[row].resize(cols, String::new());
                }
                if col < state.data[row].len() {
                    state.data[row][col] = text.to_string();
                }
            }
        }
        refresh_table(s, &state_submit);
        s.pop_layer();
    });

    let title = format!("Edit R{} C{}", row + 1, col + 1);
    siv.add_layer(Dialog::around(edit).title(title).button("Cancel", |s| {
        s.pop_layer();
    }));
}

fn main() {
    let state = Arc::new(Mutex::new(TableState::new()));

    let table = {
        let state_guard = state.lock().expect("table state lock");
        build_table(&state_guard, &state).with_name("table")
    };

    let state_add_row = Arc::clone(&state);
    let add_row = Button::new("Add Row", move |s| {
        if let Ok(mut state) = state_add_row.lock() {
            state.add_row();
        }
        refresh_table(s, &state_add_row);
    });

    let state_remove_row = Arc::clone(&state);
    let remove_row = Button::new("Remove Row", move |s| {
        if let Ok(mut state) = state_remove_row.lock() {
            state.remove_row();
        }
        refresh_table(s, &state_remove_row);
    });

    let state_add_col = Arc::clone(&state);
    let add_col = Button::new("Add Col", move |s| {
        if let Ok(mut state) = state_add_col.lock() {
            state.add_column();
        }
        refresh_table(s, &state_add_col);
    });

    let state_remove_col = Arc::clone(&state);
    let remove_col = Button::new("Remove Col", move |s| {
        if let Ok(mut state) = state_remove_col.lock() {
            state.remove_column();
        }
        refresh_table(s, &state_remove_col);
    });

    let buttons = LinearLayout::horizontal()
        .child(add_row)
        .child(DummyView::new().fixed_width(2))
        .child(remove_row)
        .child(DummyView::new().fixed_width(2))
        .child(add_col)
        .child(DummyView::new().fixed_width(2))
        .child(remove_col);

    let layout = LinearLayout::vertical()
        .child(table)
        .child(DummyView::new().fixed_height(1))
        .child(buttons);

    let mut siv = cursive::default();
    siv.add_layer(Dialog::around(layout).title("TableBuilder"));
    siv.run();
}