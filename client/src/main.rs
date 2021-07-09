use clap::{App, Arg};

fn main() {
    let matches = App::new("Tasks")
        .version("0.1.0")
        .author("Erik Reinert")
        .about("Super awesome task management tool")
        .subcommand(
            App::new("new")
                .about("Creates a new command")
                .version("0.1.0")
                .author("Erik Reinert")
                .arg(
                    Arg::new("pomodoro")
                        .long("pomodoro")
                        .short('p')
                        .about("Set desired pomodoro amount"),
                ),
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("new") {
        if matches.is_present("pomodoro") {
            println!("lets set a custom task")
        }
    }
}
