use clap::ArgMatches;

pub fn render(sub_matches: &ArgMatches) {
    let previous_return_code = sub_matches
        .value_of("previous_return_code")
        .and_then(|r| r.parse().ok())
        .unwrap_or(0);
    let keymap = sub_matches.value_of("keymap").unwrap_or("US");


    if previous_return_code != 0 {
        print!("%F{{magenta}}");
    } else {
        print!("%F{{blue}}");
    }
    print!("❯%f ");
}
