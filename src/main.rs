use Monkey::REPL;

fn main() {
    let mut repl = REPL::new();
    if let Ok(()) = repl.run() {
        std::process::exit(0);
    }
}
