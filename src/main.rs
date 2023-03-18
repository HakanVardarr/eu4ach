use crossterm::Result;
use eu4ach::{run, Categories};

fn main() -> Result<()> {
    let categories = Categories::new()
        .add_category("Very Hard")
        .add_category("     Hard")
        .add_category("   Medium")
        .add_category("     Easy")
        .add_category("Very Easy")
        .add_category("   Random")
        .add_category(" Complete")
        .add_category("    Track")
        .add_category("  Current")
        .add_category("    Clear");

    run(categories)
}
