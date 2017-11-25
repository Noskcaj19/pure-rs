use clap::ArgMatches;

pub fn render(sub_matches: &ArgMatches) {
    let previous_return_code = sub_matches
        .value_of("previous_return_code")
        .and_then(|r| r.parse().ok())
        .unwrap_or(0);
    let prompt = if previous_return_code != 0 {
        "%F{red}❯"
    } else {
        "%F{blue}❯"
    };
    print!("{}%f ", prompt);
}
