use std::sync::{Arc, Mutex};

use better_cursive_table::{ArrayBuilder, ArrayDataRow, ArrayView};
use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout};
use rand::Rng;

fn random_cell(rng: &mut impl Rng) -> String {
    rng.gen_range(0..1000).to_string()
}

struct ArrayState {
    row_headers: Vec<String>,
    column_headers: Vec<String>,
    data: Vec<Vec<String>>,
}

impl ArrayState {
    fn new() -> Self {
        let mut state = Self {
            row_headers: Vec::new(),
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
        let label = format!("Row {}", self.row_headers.len() + 1);
        let mut rng = rand::thread_rng();
        let row = (0..self.column_headers.len())
            .map(|_| random_cell(&mut rng))
            .collect();
        self.row_headers.push(label);
        self.data.push(row);
    }

    fn remove_row(&mut self) {
        if self.data.pop().is_some() {
            self.row_headers.pop();
        }
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

fn build_array(
    state: &ArrayState,
    shared: &Arc<Mutex<ArrayState>>,
) -> ArrayView<ArrayDataRow<String>, usize> {
    let mut array = ArrayBuilder::new()
        .row_header(state.row_headers.clone())
        .column_header(state.column_headers.clone())
        .data(state.data.clone())
        .build();

    let shared = Arc::clone(shared);
    array.set_on_submit(move |s, row, col| {
        open_cell_editor(s, Arc::clone(&shared), row, col);
    });

    array
}

fn refresh_array(siv: &mut cursive::Cursive, state: &Arc<Mutex<ArrayState>>) {
    let array = {
        let state_guard = state.lock().expect("array state lock");
        build_array(&state_guard, state)
    };
    siv.call_on_name(
        "array",
        |view: &mut ArrayView<ArrayDataRow<String>, usize>| {
            *view = array;
        },
    );
}

fn open_cell_editor(
    siv: &mut cursive::Cursive,
    state: Arc<Mutex<ArrayState>>,
    row: usize,
    col: usize,
) {
    let current = {
        let state = state.lock().expect("array state lock");
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
        refresh_array(s, &state_submit);
        s.pop_layer();
    });

    let title = format!("Edit R{} C{}", row + 1, col + 1);
    siv.add_layer(Dialog::around(edit).title(title).button("Cancel", |s| {
        s.pop_layer();
    }));
}

fn main() {
    let state = Arc::new(Mutex::new(ArrayState::new()));

    let array = {
        let state_guard = state.lock().expect("array state lock");
        build_array(&state_guard, &state).with_name("array")
    };

    let state_add_row = Arc::clone(&state);
    let add_row = Button::new("Add Row", move |s| {
        if let Ok(mut state) = state_add_row.lock() {
            state.add_row();
        }
        refresh_array(s, &state_add_row);
    });

    let state_remove_row = Arc::clone(&state);
    let remove_row = Button::new("Remove Row", move |s| {
        if let Ok(mut state) = state_remove_row.lock() {
            state.remove_row();
        }
        refresh_array(s, &state_remove_row);
    });

    let state_add_col = Arc::clone(&state);
    let add_col = Button::new("Add Col", move |s| {
        if let Ok(mut state) = state_add_col.lock() {
            state.add_column();
        }
        refresh_array(s, &state_add_col);
    });

    let state_remove_col = Arc::clone(&state);
    let remove_col = Button::new("Remove Col", move |s| {
        if let Ok(mut state) = state_remove_col.lock() {
            state.remove_column();
        }
        refresh_array(s, &state_remove_col);
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
        .child(array)
        .child(DummyView::new().fixed_height(1))
        .child(buttons);

    let mut siv = cursive::default();
    siv.add_layer(Dialog::around(layout).title("ArrayBuilder"));
    siv.run();
}
