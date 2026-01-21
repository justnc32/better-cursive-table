use better_cursive_table::TableBuilder;
use cursive::views::Dialog;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let data: Vec<Vec<String>> = (0..10)
        .map(|_| {
            vec![
                rng.gen_range(0..=999).to_string(),
                rng.gen_range(0..=999).to_string(),
                rng.gen_range(0..=999).to_string(),
            ]
        })
        .collect();

    let table = TableBuilder::new()
        .column_header(vec!["A", "B", "C"])
        .data(data)
        .sortable(false)
        .build();

    let mut siv = cursive::default();
    siv.add_layer(Dialog::around(table).title("Non-sortable").padding_lrtb(0, 0, 0, 0));
    siv.run();
}
