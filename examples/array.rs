// Crate Dependencies ---------------------------------------------------------
// ----------------------------------------------------------------------------
extern crate better_cursive_table;
extern crate cursive;

// STD Dependencies -----------------------------------------------------------
// ----------------------------------------------------------------------------

// External Dependencies ------------------------------------------------------
// ----------------------------------------------------------------------------
use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::Dialog;

// Modules --------------------------------------------------------------------
// ----------------------------------------------------------------------------
use better_cursive_table::{ArrayView, ArrayViewItem};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum AxisColumn {
    X,
    Y,
    Z,
}

#[derive(Clone, Debug)]
struct ArrayRow {
    row: String,
    values: [i32; 3],
}

impl ArrayViewItem<AxisColumn> for ArrayRow {
    fn to_column(&self, column: AxisColumn) -> String {
        match column {
            AxisColumn::X => self.values[0].to_string(),
            AxisColumn::Y => self.values[1].to_string(),
            AxisColumn::Z => self.values[2].to_string(),
        }
    }

    fn to_row(&self) -> String {
        self.row.clone()
    }
}

fn main() {
    let mut array = ArrayView::<ArrayRow, AxisColumn>::new()
        .row_header(|h| h.width(4).align(HAlign::Center))
        .column(AxisColumn::X, "X", |c| c.align(HAlign::Center))
        .column(AxisColumn::Y, "Y", |c| c.align(HAlign::Center))
        .column(AxisColumn::Z, "Z", |c| c.align(HAlign::Center));

    array.set_items(vec![
        ArrayRow {
            row: "A".to_string(),
            values: [1, 2, 3],
        },
        ArrayRow {
            row: "B".to_string(),
            values: [4, 5, 6],
        },
        ArrayRow {
            row: "C".to_string(),
            values: [7, 8, 9],
        },
        ArrayRow {
            row: "D".to_string(),
            values: [10, 11, 12],
        },
    ]);

    let mut siv = cursive::default();
    siv.add_layer(Dialog::around(array.min_size((24, 10))).title("ArrayView 4x3"));
    siv.run();
}
