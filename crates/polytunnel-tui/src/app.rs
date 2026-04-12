use std::collections::HashSet;
use std::path::PathBuf;

use polytunnel_core::{
    DependencyScope, ProjectConfig, add_dependency_to_file, parse_add_coordinate,
    remove_dependency_from_file,
};
use polytunnel_maven::Coordinate;
use polytunnel_resolver::{DependencyGraph, Resolver};

/// Active tab in the TUI
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Dashboard,
    Dependencies,
    Tree,
}

impl Tab {
    pub const ALL: [Tab; 3] = [Tab::Dashboard, Tab::Dependencies, Tab::Tree];

    pub fn index(self) -> usize {
        match self {
            Tab::Dashboard => 0,
            Tab::Dependencies => 1,
            Tab::Tree => 2,
        }
    }

    pub fn next(self) -> Tab {
        match self {
            Tab::Dashboard => Tab::Dependencies,
            Tab::Dependencies => Tab::Tree,
            Tab::Tree => Tab::Dashboard,
        }
    }
}

/// Input mode for the TUI
#[derive(Clone, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    AddingCoord,
    AddingScope,
    ConfirmDelete,
}

/// A flattened dependency entry for display
pub struct DepEntry {
    pub ga_key: String,
    pub version: String,
    pub scope: DependencyScope,
}

/// Main application state
pub struct App {
    pub tab: Tab,
    pub running: bool,
    pub config: ProjectConfig,
    pub config_path: PathBuf,
    pub dep_list: Vec<DepEntry>,
    pub dep_selected: usize,
    pub tree_lines: Vec<String>,
    pub tree_loading: bool,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub scope_selected: usize,
    pub status_message: Option<String>,
}

const SCOPES: [DependencyScope; 4] = [
    DependencyScope::Compile,
    DependencyScope::Runtime,
    DependencyScope::Test,
    DependencyScope::Provided,
];

impl App {
    pub fn new(config_path: PathBuf) -> color_eyre::Result<Self> {
        let config = ProjectConfig::load(&config_path)
            .map_err(|e| color_eyre::eyre::eyre!("Failed to load config: {e}"))?;
        let dep_list = build_dep_list(&config);

        Ok(Self {
            tab: Tab::Dashboard,
            running: true,
            config,
            config_path,
            dep_list,
            dep_selected: 0,
            tree_lines: vec!["Press 'r' to resolve dependency tree".into()],
            tree_loading: false,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            scope_selected: 0,
            status_message: None,
        })
    }

    pub fn reload_config(&mut self) {
        match ProjectConfig::load(&self.config_path) {
            Ok(config) => {
                self.dep_list = build_dep_list(&config);
                self.config = config;
                if self.dep_selected >= self.dep_list.len() && !self.dep_list.is_empty() {
                    self.dep_selected = self.dep_list.len() - 1;
                }
            }
            Err(e) => {
                self.status_message = Some(format!("Config reload failed: {e}"));
            }
        }
    }

    pub fn switch_tab(&mut self, tab: Tab) {
        self.tab = tab;
    }

    pub fn next_tab(&mut self) {
        self.tab = self.tab.next();
    }

    pub fn select_up(&mut self) {
        if self.dep_selected > 0 {
            self.dep_selected -= 1;
        }
    }

    pub fn select_down(&mut self) {
        if !self.dep_list.is_empty() && self.dep_selected < self.dep_list.len() - 1 {
            self.dep_selected += 1;
        }
    }

    pub fn enter_add_mode(&mut self) {
        self.input_mode = InputMode::AddingCoord;
        self.input_buffer.clear();
        self.scope_selected = 0;
        self.status_message = None;
    }

    pub fn enter_delete_mode(&mut self) {
        if !self.dep_list.is_empty() {
            self.input_mode = InputMode::ConfirmDelete;
            self.status_message = None;
        }
    }

    pub fn cancel_input(&mut self) {
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
        self.status_message = None;
    }

    pub fn confirm_coord(&mut self) {
        match parse_add_coordinate(&self.input_buffer) {
            Ok(_) => {
                self.input_mode = InputMode::AddingScope;
                self.scope_selected = 0;
            }
            Err(e) => {
                self.status_message = Some(format!("Invalid coordinate: {e}"));
            }
        }
    }

    pub fn scope_up(&mut self) {
        if self.scope_selected > 0 {
            self.scope_selected -= 1;
        }
    }

    pub fn scope_down(&mut self) {
        if self.scope_selected < SCOPES.len() - 1 {
            self.scope_selected += 1;
        }
    }

    pub fn confirm_add(&mut self) {
        let coord_str = self.input_buffer.clone();
        let scope = SCOPES[self.scope_selected];

        match parse_add_coordinate(&coord_str) {
            Ok((ga_key, version)) => {
                let scope_arg = if scope == DependencyScope::Compile {
                    None
                } else {
                    Some(scope)
                };
                match add_dependency_to_file(&self.config_path, &ga_key, &version, scope_arg) {
                    Ok(()) => {
                        self.status_message = Some(format!("Added {ga_key}:{version}"));
                        self.reload_config();
                    }
                    Err(e) => {
                        self.status_message = Some(format!("Add failed: {e}"));
                    }
                }
            }
            Err(e) => {
                self.status_message = Some(format!("Invalid coordinate: {e}"));
            }
        }

        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
    }

    pub fn confirm_delete(&mut self) {
        if let Some(dep) = self.dep_list.get(self.dep_selected) {
            let ga_key = dep.ga_key.clone();
            match remove_dependency_from_file(&self.config_path, &ga_key) {
                Ok(()) => {
                    self.status_message = Some(format!("Removed {ga_key}"));
                    self.reload_config();
                }
                Err(e) => {
                    self.status_message = Some(format!("Remove failed: {e}"));
                }
            }
        }
        self.input_mode = InputMode::Normal;
    }

    pub async fn resolve_tree(&mut self) {
        self.tree_loading = true;
        self.tree_lines = vec!["Resolving...".into()];

        let root_coords = parse_root_coords(&self.config);
        let mut resolver = Resolver::new();

        match resolver.resolve(&root_coords).await {
            Ok(_resolved) => {
                self.tree_lines =
                    render_tree(&self.config.project.name, &root_coords, &resolver.graph);
            }
            Err(e) => {
                self.tree_lines = vec![format!("Resolution failed: {e}")];
            }
        }
        self.tree_loading = false;
    }

    pub fn selected_scope_name(&self) -> &'static str {
        scope_name(SCOPES[self.scope_selected])
    }
}

fn build_dep_list(config: &ProjectConfig) -> Vec<DepEntry> {
    let mut list: Vec<DepEntry> = config
        .dependencies
        .iter()
        .map(|(key, dep)| DepEntry {
            ga_key: key.clone(),
            version: dep.version().to_string(),
            scope: dep.scope(),
        })
        .collect();
    list.sort_by(|a, b| a.ga_key.cmp(&b.ga_key));
    list
}

fn parse_root_coords(config: &ProjectConfig) -> Vec<Coordinate> {
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

fn render_tree(
    project_name: &str,
    root_coords: &[Coordinate],
    graph: &DependencyGraph,
) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("{project_name} v0.1.0"));

    let mut printed = HashSet::new();

    for (i, coord) in root_coords.iter().enumerate() {
        let is_last = i == root_coords.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let child_prefix = if is_last { "    " } else { "│   " };

        lines.push(format!("{connector}{coord}"));
        printed.insert(coord.to_string());
        collect_children(graph, coord, child_prefix, &mut printed, &mut lines);
    }

    lines
}

fn collect_children(
    graph: &DependencyGraph,
    coord: &Coordinate,
    prefix: &str,
    printed: &mut HashSet<String>,
    lines: &mut Vec<String>,
) {
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
                lines.push(format!("{prefix}{connector}{child} (*)"));
            } else {
                lines.push(format!("{prefix}{connector}{child}"));
                printed.insert(child_key);
                let new_prefix = format!("{prefix}{child_prefix_ext}");
                collect_children(graph, child, &new_prefix, printed, lines);
            }
        }
    }
}

pub fn scope_name(scope: DependencyScope) -> &'static str {
    match scope {
        DependencyScope::Compile => "compile",
        DependencyScope::Runtime => "runtime",
        DependencyScope::Test => "test",
        DependencyScope::Provided => "provided",
    }
}
