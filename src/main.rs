mod process {
    pub mod nodejs;
    pub mod php;
}

use clap::{App, Arg};

fn main() {
    let matches = App::new("pinit")
        .version("1.0")
        .author("Sébastien Lemoine <sebastien.lemoine@oclock.io>")
        .about("Initialise des projets Node.js ou PHP avec des configurations prédéfinies")
        .arg(Arg::with_name("type")
            .short('t')
            .long("type")
            .help("Type du projet à initialiser (nodejs ou php)")
            .takes_value(true)
            .possible_values(&["nodejs", "php"])
            .required(true))
        .arg(Arg::with_name("name")
            .short('n')
            .long("name")
            .help("Nom du projet à initialiser")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("directory")
            .short('d')
            .long("directory")
            .help("Répertoire de destination du projet")
            .takes_value(true)
            .required(true))
        .get_matches();

    let project_type = matches.value_of("type").unwrap();
    let project_name = matches.value_of("name").unwrap();
    let destination_directory = matches.value_of("directory").unwrap();

    match project_type {
        "nodejs" => process::nodejs::initialize_node_project(project_name, destination_directory),
        "php" => process::php::initialize_php_project(project_name, destination_directory),
        _ => eprintln!("Type de projet non supporté."),
    }
}