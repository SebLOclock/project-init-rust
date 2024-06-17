use std::process::Command;
use std::fs;

pub fn initialize_node_project(project_name: &str, destination_directory: &str) {
    let project_path = format!("{}/{}", destination_directory, project_name);

    // Crée le répertoire de projet si nécessaire
    fs::create_dir_all(&project_path).unwrap();

    Command::new("npx")
        .args(["create-node-app", project_name])
        .current_dir(&destination_directory)
        .status()
        .expect("Failed to initialize Node.js project");

    Command::new("npm")
        .args(["install", "express", "postgres", "sequelize", "ejs"])
        .current_dir(&project_path)
        .status()
        .expect("Failed to install packages");

    // Créer les dossiers d'arborescence
    fs::create_dir_all(format!("{}/src/controllers", &project_path)).unwrap();
    fs::create_dir_all(format!("{}/src/models", &project_path)).unwrap();
    fs::create_dir_all(format!("{}/src/routers", &project_path)).unwrap();
    fs::create_dir_all(format!("{}/public", &project_path)).unwrap();
}