use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Tabs};

use polytunnel_core::DependencyScope;

use crate::app::{App, InputMode, Tab, scope_name};

pub fn render(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    let layout = Layout::vertical([
        Constraint::Length(3), // tab bar
        Constraint::Min(1),    // content
        Constraint::Length(1), // status / input
        Constraint::Length(1), // keybinds
    ])
    .split(area);

    render_tabs(frame, app, layout[0]);

    match app.tab {
        Tab::Dashboard => render_dashboard(frame, app, layout[1]),
        Tab::Dependencies => render_dependencies(frame, app, layout[1]),
        Tab::Tree => render_tree(frame, app, layout[1]),
    }

    render_status(frame, app, layout[2]);
    render_keybinds(frame, app, layout[3]);
}

fn render_tabs(frame: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = Tab::ALL
        .iter()
        .map(|t| {
            let name = match t {
                Tab::Dashboard => "Dashboard",
                Tab::Dependencies => "Dependencies",
                Tab::Tree => "Tree",
            };
            Line::from(name)
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(" polytunnel "))
        .select(app.tab.index())
        .highlight_style(Style::default().fg(Color::Cyan).bold());

    frame.render_widget(tabs, area);
}

fn render_dashboard(frame: &mut Frame, app: &App, area: Rect) {
    let layout = Layout::vertical([
        Constraint::Length(6), // project info
        Constraint::Length(8), // build config
        Constraint::Min(1),    // repositories
    ])
    .split(area);

    // Project info
    let project_text = vec![
        Line::from(vec![
            Span::styled("  Name:  ", Style::default().fg(Color::DarkGray)),
            Span::styled(&app.config.project.name, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  Java:  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                &app.config.project.java_version,
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Deps:  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                app.config.dependencies.len().to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
    ];
    let project_block = Paragraph::new(project_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Project ")
            .title_style(Style::default().fg(Color::Cyan)),
    );
    frame.render_widget(project_block, layout[0]);

    // Build config
    let build = &app.config.build;
    let build_text = vec![
        Line::from(vec![
            Span::styled("  Source:     ", Style::default().fg(Color::DarkGray)),
            Span::raw(build.source_dirs.join(", ")),
        ]),
        Line::from(vec![
            Span::styled("  Output:     ", Style::default().fg(Color::DarkGray)),
            Span::raw(&build.output_dir),
        ]),
        Line::from(vec![
            Span::styled("  Test src:   ", Style::default().fg(Color::DarkGray)),
            Span::raw(build.test_source_dirs.join(", ")),
        ]),
        Line::from(vec![
            Span::styled("  Test out:   ", Style::default().fg(Color::DarkGray)),
            Span::raw(&build.test_output_dir),
        ]),
        Line::from(vec![
            Span::styled("  Framework:  ", Style::default().fg(Color::DarkGray)),
            Span::raw(&build.test_framework),
        ]),
    ];
    let build_block = Paragraph::new(build_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Build Config ")
            .title_style(Style::default().fg(Color::Cyan)),
    );
    frame.render_widget(build_block, layout[1]);

    // Repositories
    let repo_lines: Vec<Line> = if app.config.repositories.is_empty() {
        vec![Line::from("  (none)")]
    } else {
        app.config
            .repositories
            .iter()
            .map(|r| {
                Line::from(vec![
                    Span::styled(format!("  {}  ", r.name), Style::default().fg(Color::Green)),
                    Span::styled(&r.url, Style::default().fg(Color::DarkGray)),
                ])
            })
            .collect()
    };
    let repo_block = Paragraph::new(repo_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Repositories ")
            .title_style(Style::default().fg(Color::Cyan)),
    );
    frame.render_widget(repo_block, layout[2]);
}

fn render_dependencies(frame: &mut Frame, app: &mut App, area: Rect) {
    if app.dep_list.is_empty() {
        let empty = Paragraph::new("  No dependencies. Press 'a' to add one.").block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Dependencies ")
                .title_style(Style::default().fg(Color::Cyan)),
        );
        frame.render_widget(empty, area);
        return;
    }

    let header = Row::new(vec![
        Cell::from("  GroupId:ArtifactId").style(Style::default().bold()),
        Cell::from("Version").style(Style::default().bold()),
        Cell::from("Scope").style(Style::default().bold()),
    ])
    .height(1);

    let rows: Vec<Row> = app
        .dep_list
        .iter()
        .map(|dep| {
            let scope_color = match dep.scope {
                DependencyScope::Test => Color::Magenta,
                DependencyScope::Runtime => Color::Blue,
                DependencyScope::Provided => Color::Yellow,
                DependencyScope::Compile => Color::DarkGray,
            };
            Row::new(vec![
                Cell::from(format!("  {}", dep.ga_key)),
                Cell::from(dep.version.as_str()),
                Cell::from(scope_name(dep.scope)).style(Style::default().fg(scope_color)),
            ])
        })
        .collect();

    let widths = [
        Constraint::Percentage(50),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Dependencies ")
                .title_style(Style::default().fg(Color::Cyan)),
        )
        .row_highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    let mut state = TableState::default().with_selected(Some(app.dep_selected));
    frame.render_stateful_widget(table, area, &mut state);
}

fn render_tree(frame: &mut Frame, app: &App, area: Rect) {
    let lines: Vec<Line> = app
        .tree_lines
        .iter()
        .map(|l| Line::from(format!("  {l}")))
        .collect();

    let title = if app.tree_loading {
        " Tree (resolving...) "
    } else {
        " Tree "
    };

    let tree = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .title_style(Style::default().fg(Color::Cyan)),
    );
    frame.render_widget(tree, area);
}

fn render_status(frame: &mut Frame, app: &App, area: Rect) {
    match &app.input_mode {
        InputMode::AddingCoord => {
            let input_line = Line::from(vec![
                Span::styled(" Add: ", Style::default().fg(Color::Yellow).bold()),
                Span::raw(&app.input_buffer),
                Span::styled("_", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    "  (groupId:artifactId:version)",
                    Style::default().fg(Color::DarkGray),
                ),
            ]);
            frame.render_widget(Paragraph::new(input_line), area);
        }
        InputMode::AddingScope => {
            let scope_line = Line::from(vec![
                Span::styled(" Scope: ", Style::default().fg(Color::Yellow).bold()),
                Span::styled(
                    app.selected_scope_name(),
                    Style::default().fg(Color::Cyan).bold(),
                ),
                Span::styled(
                    "  (Up/Down to change, Enter to confirm)",
                    Style::default().fg(Color::DarkGray),
                ),
            ]);
            frame.render_widget(Paragraph::new(scope_line), area);
        }
        InputMode::ConfirmDelete => {
            let ga = app
                .dep_list
                .get(app.dep_selected)
                .map(|d| d.ga_key.as_str())
                .unwrap_or("?");
            let confirm_line = Line::from(vec![
                Span::styled(
                    format!(" Delete {ga}? "),
                    Style::default().fg(Color::Red).bold(),
                ),
                Span::styled("(y/n)", Style::default().fg(Color::DarkGray)),
            ]);
            frame.render_widget(Paragraph::new(confirm_line), area);
        }
        InputMode::Normal => {
            if let Some(msg) = &app.status_message {
                let style = if msg.starts_with("Added") || msg.starts_with("Removed") {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::Red)
                };
                let status = Paragraph::new(Line::from(Span::styled(format!(" {msg}"), style)));
                frame.render_widget(status, area);
            }
        }
    }
}

fn render_keybinds(frame: &mut Frame, app: &App, area: Rect) {
    let hints = match app.input_mode {
        InputMode::AddingCoord => vec![("Enter", "confirm"), ("Esc", "cancel")],
        InputMode::AddingScope => vec![
            ("Up/Down", "select"),
            ("Enter", "confirm"),
            ("Esc", "cancel"),
        ],
        InputMode::ConfirmDelete => vec![("y", "yes"), ("n", "no")],
        InputMode::Normal => match app.tab {
            Tab::Dashboard => vec![("1/2/3", "tab"), ("Tab", "next"), ("q", "quit")],
            Tab::Dependencies => vec![
                ("Up/Down", "navigate"),
                ("a", "add"),
                ("d", "delete"),
                ("1/2/3", "tab"),
                ("q", "quit"),
            ],
            Tab::Tree => vec![("r", "refresh"), ("1/2/3", "tab"), ("q", "quit")],
        },
    };

    let spans: Vec<Span> = hints
        .iter()
        .enumerate()
        .flat_map(|(i, (key, desc))| {
            let mut s = vec![
                Span::styled(
                    format!(" {key} "),
                    Style::default().fg(Color::Yellow).bold(),
                ),
                Span::styled(*desc, Style::default().fg(Color::DarkGray)),
            ];
            if i < hints.len() - 1 {
                s.push(Span::raw("  "));
            }
            s
        })
        .collect();

    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}
