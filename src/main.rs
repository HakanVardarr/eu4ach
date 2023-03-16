use eu4ach::run;

fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => {
            eprintln!("ERROR: {e}");
        }
    }
}
