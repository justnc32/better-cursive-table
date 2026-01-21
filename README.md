# better_cursive_table

TableView and ArrayView widgets for the [cursive](https://crates.io/crates/cursive) TUI framework.
This is a maintained fork of [cursive_table_view](https://github.com/BonsaiDen/cursive_table_view)
with all original functionality plus a toggle to disable sorting and header selection.

Links: [crates.io](https://crates.io/crates/better-cursive-table) | [docs.rs](https://docs.rs/better-cursive-table) | [GitHub](https://github.com/justnc32/better-cursive-table)

## Highlights

- TableView with multi-column sort and keyboard/mouse navigation
- ArrayView for labeled rows + columns (a 2D grid with headers)
- Chainable column configuration (alignment, width, default order)
- Callbacks for sort, row select, and submit
- Optional non-sortable mode for "static" tables


![TableView example](examples/images/basic.png)
![Non-sortable table example](examples/images/non_sortable.png)
![ArrayView example](examples/images/array.png)

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
better_cursive_table = "0.2"
cursive = "0.21"
```

## Quick start (TableView)

```rust
use std::cmp::Ordering;
use cursive::align::HAlign;
use cursive::traits::*;
use better_cursive_table::{TableView, TableViewItem};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Column {
    Name,
    Count,
    Rate,
}

#[derive(Clone, Debug)]
struct Row {
    name: String,
    count: usize,
    rate: usize,
}

impl TableViewItem<Column> for Row {
    fn to_column(&self, column: Column) -> String {
        match column {
            Column::Name => self.name.clone(),
            Column::Count => self.count.to_string(),
            Column::Rate => self.rate.to_string(),
        }
    }

    fn cmp(&self, other: &Self, column: Column) -> Ordering {
        match column {
            Column::Name => self.name.cmp(&other.name),
            Column::Count => self.count.cmp(&other.count),
            Column::Rate => self.rate.cmp(&other.rate),
        }
    }
}

let mut table = TableView::<Row, Column>::new()
    .column(Column::Name, "Name", |c| c.width_percent(30))
    .column(Column::Count, "Count", |c| c.align(HAlign::Center))
    .column(Column::Rate, "Rate", |c| {
        c.ordering(Ordering::Greater).align(HAlign::Right).width_percent(20)
    })
    .default_column(Column::Name);

table.set_items(vec![
    Row { name: "Alpha".into(), count: 3, rate: 10 },
    Row { name: "Beta".into(), count: 1, rate: 42 },
]);
```

## Sorting disabled

Disable header selection and sort indicators entirely:

```rust
let table = TableView::<Row, Column>::new()
    .column(Column::Name, "Name", |c| c)
    .column(Column::Count, "Count", |c| c)
    .sortable(false);
```

## ArrayView (row + column headers)

```rust
use better_cursive_table::ArrayBuilder;

let array = ArrayBuilder::new()
    .array_name("My Array")
    .column_header(vec!["X", "Y", "Z"])
    .add_row("Row A", vec![1, 2, 3])
    .add_row("Row B", vec![4, 5])
    .build();
```

## Callbacks

```rust
use std::cmp::Ordering;
use cursive::Cursive;

table.set_on_sort(|_siv: &mut Cursive, column, order: Ordering| {
    // React to sorting (e.g., update status line)
});

table.set_on_select(|_siv: &mut Cursive, row, index| {
    // Row changed
});

table.set_on_submit(|_siv: &mut Cursive, row, index| {
    // Enter key / click on focused row
});
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
