use shadesign::*;

#[derive(Parser)]
#[command(name="shadesign")]
#[command(about="Word overlap chuncher")]
enum Commands {
    /// Main command
    Crunch(CrunchCommand),
}

impl Commands {
    fn run(&mut self) {
        match self {
            Commands::Crunch(cmd) => cmd.run(),
        }
    }
}


fn main() {
    Commands::parse().run();
}
