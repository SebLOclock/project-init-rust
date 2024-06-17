use std::process::Command;
use std::fs;

pub fn initialize_php_project(project_name: &str, destination_directory: &str) {
    let project_path = format!("{}/{}", destination_directory, project_name);

    // Crée le répertoire de projet
    fs::create_dir_all(&project_path).unwrap();

    // Créer un fichier index.php simple
    fs::write(format!("{}/index.php", &project_path), "<?php echo 'Hello, World!'; ?>").unwrap();

    // Initialiser Composer dans le projet PHP (assurez-vous que Composer est installé)
    Command::new("composer")
        .arg("init")
        .arg("--no-interaction") // Exécute sans interaction
        .current_dir(&project_path)
        .status()
        .expect("Failed to initialize Composer");

    println!("Projet PHP initialisé dans {}", project_path);
}