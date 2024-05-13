use comfy_table::Table;

pub fn view_plugins() -> String {
    let mut table = Table::new();
    table
        .set_header(vec!["Plugin Name", "Description", "Install command"])
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
    return table.to_string();
}

pub fn display_commands() -> String {
    let mut table = Table::new();
    table
        .set_header(vec!["Command", "Number to run the command"])
        .add_row(vec!["Install Plugin/Theme", "1"])
        .add_row(vec!["View Plugins", "2"])
        .add_row(vec!["View Themes (colors)", "3"])
        .add_row(vec!["View Help", "4"])
        .add_row(vec!["View Help", "5"]);
    return table.to_string();
}
