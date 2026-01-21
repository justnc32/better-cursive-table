use better_cursive_table::{TableBuilder, TableDataRow, TableView};
use cursive::traits::*;
use cursive::views::{Dialog, DummyView, LinearLayout};
use rand::Rng;

fn make_table() -> TableView<TableDataRow<String>, usize> {
    let mut rng = rand::thread_rng();
    let data: Vec<Vec<String>> = (0..30)
        .map(|i| {
            vec![
                format!("Item {i}"),
                rng.gen_range(0..=999).to_string(),
                rng.gen_range(0..=999).to_string(),
            ]
        })
        .collect();

    TableBuilder::new()
        .column_header(vec!["Name", "Count", "Rate"])
        .data(data)
        .sortable(true)
        .build()
}

fn main() {
    let layout = LinearLayout::horizontal()
        .child(make_table())
        .child(DummyView::new().fixed_width(2))
        .child(make_table());

    let mut siv = cursive::default();
    siv.add_layer(Dialog::around(layout).title("Double Table"));
    siv.run();
}