use clap::ArgMatches;

pub fn main(matches: &ArgMatches) {

    let item_to_run = match matches.get_one::<String>("runnable") {
        Some(val) => val,
        None => {
            // enter a vic env with the system shell
            enter_vic_env();
            return;
        },
    };

    // check if the item to run is a nix flake url
    if item_to_run.contains("#") {
        run_from_nix(item_to_run);
        return;
    }
}

fn enter_vic_env() {
}
