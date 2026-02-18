use color_eyre::eyre::Result;
use colored::*;
use polytunnel_core::ProjectConfig;
use polytunnel_maven::Coordinate;
use polytunnel_resolver::{DependencyGraph, Resolver};
use std::collections::HashSet;
use std::path::Path;

pub async fn cmd_tree(verbose: bool) -> Result<()> {
    let config = ProjectConfig::load(Path::new("polytunnel.toml"))?;

    let mut root_coords: Vec<Coordinate> = config
        .dependencies
        .iter()
        .filter_map(|(key, dep)| {
            let parts: Vec<&str> = key.split(':').collect();
            if parts.len() >= 2 {
                Some(Coordinate::new(parts[0], parts[1], dep.version()))
            } else {
                None
            }
        })
        .collect();
    root_coords.sort_by_key(|c| c.to_string());

    let mut resolver = Resolver::new();
    resolver
        .resolve(&root_coords)
        .await
        .map_err(|e| color_eyre::eyre::eyre!("Dependency resolution failed: {}", e))?;

    println!("{} v0.1.0", config.project.name.bold());

    let graph = &resolver.graph;
    let mut printed = HashSet::new();

    for (i, coord) in root_coords.iter().enumerate() {
        let is_last = i == root_coords.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let child_prefix = if is_last { "    " } else { "│   " };

        println!("{}{}", connector, coord.to_string().cyan());
        printed.insert(coord.to_string());
        print_children(graph, coord, child_prefix, &mut printed, verbose);
    }

    Ok(())
}

fn print_children(
    graph: &DependencyGraph,
    coord: &Coordinate,
    prefix: &str,
    printed: &mut HashSet<String>,
    verbose: bool,
) {
    let _ = verbose;
    let key = coord.to_string();
    if let Some(node) = graph.get(&key) {
        let mut children = node.dependencies.clone();
        children.sort_by_key(|c| c.to_string());

        for (i, child) in children.iter().enumerate() {
            let is_last = i == children.len() - 1;
            let connector = if is_last { "└── " } else { "├── " };
            let child_prefix_ext = if is_last { "    " } else { "│   " };

            let child_key = child.to_string();
            if printed.contains(&child_key) {
                println!("{}{}{} (*)", prefix, connector, child.to_string().dimmed());
            } else {
                println!("{}{}{}", prefix, connector, child);
                printed.insert(child_key);
                let new_prefix = format!("{}{}", prefix, child_prefix_ext);
                print_children(graph, child, &new_prefix, printed, verbose);
            }
        }
    }
}
