extern crate yansi;
use yansi::Paint as _;
extern crate comfy_table;
use comfy_table::*;

pub fn table() {
    table
        .set_header(vec!["Header1", "Header2", "Header3"])
        .add_row(vec![
            "This is a text",
            "This is another text",
            "This is the third text",
        ])
        .add_row(vec![
            "This is another text",
            "Now\nadd some\nmulti line stuff",
            "This is awesome",
        ]);

    println!("{table}");
}
