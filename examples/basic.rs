use better_cursive_table::TableBuilder;
use cursive::views::Dialog;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let data: Vec<Vec<String>> = (0..50)
        .map(|i| {
            vec![
                format!("Name {i}"),
                rng.gen_range(0..=255).to_string(),
                rng.gen_range(0..=255).to_string(),
            ]
        })
        .collect();

    let table = TableBuilder::new()
        .column_header(vec!["Name", "Count", "Rate"])
        .data(data)
        .sortable(true)
        .build();

    let mut siv = cursive::default();
    siv.add_layer(Dialog::around(table).title("Table View"));
    siv.run();
}