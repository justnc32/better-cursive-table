// Crate Dependencies ---------------------------------------------------------
// ----------------------------------------------------------------------------
extern crate better_cursive_table;
extern crate cursive;

// STD Dependencies -----------------------------------------------------------
// ----------------------------------------------------------------------------
use std::cmp::Ordering;

// External Dependencies ------------------------------------------------------
// ----------------------------------------------------------------------------
use cursive::Cursive;
use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{Dialog, TextView};

// Modules --------------------------------------------------------------------
// ----------------------------------------------------------------------------
use better_cursive_table::{TableView, TableViewItem};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum BasicColumn {
    Task,
    Status,
    Owner,
    Priority,
}

#[derive(Clone, Debug)]
struct TaskRow {
    task: String,
    status: String,
    owner: String,
    priority: String,
}

impl TableViewItem<BasicColumn> for TaskRow {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::Task => self.task.clone(),
            BasicColumn::Status => self.status.clone(),
            BasicColumn::Owner => self.owner.clone(),
            BasicColumn::Priority => self.priority.clone(),
        }
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            BasicColumn::Task => self.task.cmp(&other.task),
            BasicColumn::Status => self.status.cmp(&other.status),
            BasicColumn::Owner => self.owner.cmp(&other.owner),
            BasicColumn::Priority => self.priority.cmp(&other.priority),
        }
    }
}

fn main() {
    let mut siv = cursive::default();

    // Sorting disabled: no header markers, no header selection, no sorting.
    let mut table = TableView::<TaskRow, BasicColumn>::new()
        .column(BasicColumn::Task, "Task", |c| c.align(HAlign::Center))
        .column(BasicColumn::Status, "Status", |c| c.align(HAlign::Center))
        .column(BasicColumn::Owner, "Owner", |c| c.align(HAlign::Center))
        .column(BasicColumn::Priority, "Priority", |c| {
            c.align(HAlign::Center)
        })
        .sortable(false);

    // Static items; order remains as inserted.
    table.set_items(vec![
        TaskRow {
            task: "Write docs".to_string(),
            status: "Done".to_string(),
            owner: "Alex".to_string(),
            priority: "Low".to_string(),
        },
        TaskRow {
            task: "Fix bug #42".to_string(),
            status: "In Progress".to_string(),
            owner: "Sam".to_string(),
            priority: "High".to_string(),
        },
        TaskRow {
            task: "Release v1.2".to_string(),
            status: "Queued".to_string(),
            owner: "Riley".to_string(),
            priority: "Medium".to_string(),
        },
    ]);

    // Submit still works with sorting off.
    table.set_on_submit(|siv: &mut Cursive, row: usize, index: usize| {
        let value = siv
            .call_on_name(
                "table",
                move |table: &mut TableView<TaskRow, BasicColumn>| {
                    format!("{:?}", table.borrow_item(index).unwrap())
                },
            )
            .unwrap();

        siv.add_layer(
            Dialog::around(TextView::new(value))
                .title(format!("Selected row # {}", row))
                .button("Close", |s| {
                    s.pop_layer();
                }),
        );
    });

    siv.add_layer(
        Dialog::around(table.with_name("table").min_size((60, 12))).title("Non-sortable Table"),
    );

    siv.run();
}
