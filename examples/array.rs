use better_cursive_table::ArrayBuilder;
use cursive::views::Dialog;
use rand::Rng;

fn random_row(rng: &mut impl Rng, cols: usize) -> Vec<String> {
    (0..cols)
        .map(|_| rng.gen_range(0..=99).to_string())
        .collect()
}

fn main() {
    let mut rng = rand::thread_rng();
    let array = ArrayBuilder::new()
        .column_header(vec!["X", "Y", "Z"])
        .add_row("A", random_row(&mut rng, 3))
        .add_row("B", random_row(&mut rng, 3))
        .add_row("C", random_row(&mut rng, 3))
        .add_row("D", random_row(&mut rng, 3))
        .build();

    let mut siv = cursive::default();
    siv.add_layer(Dialog::around(array).title("ArrayView 4x3"));
    siv.run();
}
