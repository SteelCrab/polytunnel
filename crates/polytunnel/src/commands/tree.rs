use color_eyre::eyre::Result;
use polytunnel_core::ProjectConfig;
use polytunnel_maven::Coordinate;
use polytunnel_resolver::{DependencyGraph, Resolver};
use std::collections::HashSet;
use std::path::Path;

pub async fn cmd_tree(verbose: bool) -> Result<()> {
    do_tree(Path::new("polytunnel.toml"), verbose).await
}

pub(crate) async fn do_tree(config_path: &Path, verbose: bool) -> Result<()> {
    let config = ProjectConfig::load(config_path)?;

    let root_coords = parse_root_coords(&config);

    let mut resolver = Resolver::new();
    resolver
        .resolve(&root_coords)
        .await
        .map_err(|e| color_eyre::eyre::eyre!("Dependency resolution failed: {}", e))?;

    let lines = render_tree(&config.project.name, &root_coords, &resolver.graph, verbose);
    for line in lines {
        println!("{}", line);
    }

    Ok(())
}

pub(crate) fn parse_root_coords(config: &ProjectConfig) -> Vec<Coordinate> {
    let mut coords: Vec<Coordinate> = config
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
    coords.sort_by_key(|c| c.to_string());
    coords
}

pub(crate) fn render_tree(
    project_name: &str,
    root_coords: &[Coordinate],
    graph: &DependencyGraph,
    verbose: bool,
) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("{} v0.1.0", project_name));

    let mut printed = HashSet::new();

    for (i, coord) in root_coords.iter().enumerate() {
        let is_last = i == root_coords.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let child_prefix = if is_last { "    " } else { "│   " };

        lines.push(format!("{}{}", connector, coord));
        printed.insert(coord.to_string());
        collect_children(
            graph,
            coord,
            child_prefix,
            &mut printed,
            verbose,
            &mut lines,
        );
    }

    lines
}

fn collect_children(
    graph: &DependencyGraph,
    coord: &Coordinate,
    prefix: &str,
    printed: &mut HashSet<String>,
    verbose: bool,
    lines: &mut Vec<String>,
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
                lines.push(format!("{}{}{} (*)", prefix, connector, child));
            } else {
                lines.push(format!("{}{}{}", prefix, connector, child));
                printed.insert(child_key);
                let new_prefix = format!("{}{}", prefix, child_prefix_ext);
                collect_children(graph, child, &new_prefix, printed, verbose, lines);
            }
        }
    }
}
