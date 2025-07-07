use blockpedia::{queries::*, BlockFacts, BLOCKS};
use crossterm::event::{self, Event as CEvent, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{
    BarChart, Block, Borders, Cell, List, ListItem, Paragraph, Row, Table, Tabs,
};
use ratatui::Terminal;
use std::collections::HashMap;
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize terminal
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Application state
    let mut app = App::default();

    // Main loop
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::cursor::Show
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let CEvent::Key(key) = event::read()? {
            match app.current_tab {
                Tab::Blocks => handle_blocks_input(key, app),
                Tab::Colors => handle_colors_input(key, app),
                Tab::Query => handle_query_input(key, app),
                Tab::Properties => handle_properties_input(key, app),
                Tab::Statistics => handle_statistics_input(key, app),
                Tab::Sources => handle_sources_input(key, app),
            }

            // Global keys
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Tab => app.next_tab(),
                KeyCode::BackTab => app.previous_tab(),
                _ => {}
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints::<&[ratatui::layout::Constraint]>(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    // Title
    let title = Paragraph::new("🧱 Blockpedia Interactive CLI")
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main content area with tabs
    let titles = [
        "📦 Blocks",
        "🎨 Colors",
        "🔍 Query",
        "⚙️ Properties",
        "📊 Stats",
        "🌐 Sources",
    ]
    .iter()
    .cloned()
    .map(Line::from)
    .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Navigation"))
        .select(app.current_tab as usize)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints::<&[ratatui::layout::Constraint]>(
            [Constraint::Length(3), Constraint::Min(0)].as_ref(),
        )
        .split(chunks[1]);

    f.render_widget(tabs, inner_chunks[0]);

    match app.current_tab {
        Tab::Blocks => render_blocks_tab(f, app, inner_chunks[1]),
        Tab::Colors => render_colors_tab(f, app, inner_chunks[1]),
        Tab::Query => render_query_tab(f, app, inner_chunks[1]),
        Tab::Properties => render_properties_tab(f, app, inner_chunks[1]),
        Tab::Statistics => render_statistics_tab(f, app, inner_chunks[1]),
        Tab::Sources => render_sources_tab(f, app, inner_chunks[1]),
    }

    // Status bar
    let help_text = match app.current_tab {
        Tab::Blocks => "[↑/↓] Navigate | [Enter] Select | [Tab] Switch tabs | [q] Quit",
        Tab::Colors => "[1-4] Color queries | [↑/↓] Navigate | [Tab] Switch tabs | [q] Quit",
        Tab::Query => "[1-5] Query types | [Tab] Switch tabs | [q] Quit",
        Tab::Properties => "[↑/↓] Navigate | [Tab] Switch tabs | [q] Quit",
        Tab::Statistics => "[Tab] Switch tabs | [q] Quit",
        Tab::Sources => "[i] Info | [r] Refresh | [Tab] Switch tabs | [q] Quit",
    };
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);
}

fn render_blocks_tab(f: &mut ratatui::Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints::<&[ratatui::layout::Constraint]>(
            [Constraint::Percentage(50), Constraint::Percentage(50)].as_ref(),
        )
        .split(area);

    // Block list
    let items: Vec<ListItem> = app
        .filtered_blocks
        .iter()
        .enumerate()
        .map(|(i, block)| {
            let mut style = Style::default();
            if i == app.selected_block_index {
                style = style.bg(Color::Yellow).fg(Color::Black);
            }

            // Add fetcher data indicators
            let mut indicators = Vec::new();
            if block.extras.mock_data.is_some() {
                indicators.push("🔧");
            }
            if block.extras.color.is_some() {
                indicators.push("🎨");
            }

            let indicator_text = if indicators.is_empty() {
                String::new()
            } else {
                format!(" {}", indicators.join(""))
            };

            ListItem::new(format!("{}{}", block.id(), indicator_text)).style(style)
        })
        .collect();

    let blocks_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(format!(
            "Blocks ({}/{})",
            app.filtered_blocks.len(),
            BLOCKS.len()
        )))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(blocks_list, chunks[0]);

    // Block details
    if let Some(selected_block) = app.get_selected_block() {
        let mut details = vec![format!("ID: {}", selected_block.id()), String::new()];

        // Properties
        if selected_block.properties.is_empty() {
            details.push("Properties: None".to_string());
        } else {
            details.push("Properties:".to_string());
            for (prop, values) in selected_block.properties {
                details.push(format!("  {}: {:?}", prop, values));
            }
        }

        details.push(String::new());

        // Default state
        if selected_block.default_state.is_empty() {
            details.push("Default State: None".to_string());
        } else {
            details.push("Default State:".to_string());
            for (key, value) in selected_block.default_state {
                details.push(format!("  {}: {}", key, value));
            }
        }

        // Fetcher data
        if selected_block.extras.mock_data.is_some() || selected_block.extras.color.is_some() {
            details.push(String::new());
            details.push("Extra Data:".to_string());

            if let Some(mock_val) = selected_block.extras.mock_data {
                details.push(format!("  Mock Data: {}", mock_val));
            }

            if let Some(color) = selected_block.extras.color {
                details.push(format!("  RGB Color: {:?}", color.rgb));
                details.push(format!(
                    "  Oklab: [{:.2}, {:.2}, {:.2}]",
                    color.oklab[0], color.oklab[1], color.oklab[2]
                ));
            }
        }

        let details_text: Vec<Line> = details.iter().map(|s| Line::from(s.clone())).collect();
        let details_paragraph = Paragraph::new(details_text)
            .block(Block::default().borders(Borders::ALL).title("Details"))
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(details_paragraph, chunks[1]);
    }
}

fn render_colors_tab(f: &mut ratatui::Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints::<&[ratatui::layout::Constraint]>(
            [Constraint::Length(10), Constraint::Min(0)].as_ref(),
        )
        .split(area);

    // Color query options
    let color_options = vec![
        "🎨 Color System Features",
        "",
        "[1] 📊 Color Coverage Analysis - See how many blocks have color data",
        "[2] 🎯 Color Palette Analysis - Group blocks by color families",
        "[3] 🔍 Color Similarity Search - Find blocks similar to stone gray",
        "[4] 📈 Color Statistics - Overall color data analysis",
        "[5] 🌈 Gradient Palettes - Generate gradients between colors/blocks",
        "[6] 🎭 Themed Palettes - Pre-made palettes (sunset, ocean, fire, etc.)",
        "[7] 🧱 Block Palettes - Get actual Minecraft blocks for building!",
        "[8] 🏗️ Architectural Palettes - Building style recommendations",
        "",
        "💡 Our color system covers 472+ blocks with real texture data!",
    ];
    let options_text: Vec<Line> = color_options.iter().map(|s| Line::from(*s)).collect();
    let options_paragraph = Paragraph::new(options_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("🎨 Color Queries"),
    );
    f.render_widget(options_paragraph, chunks[0]);

    // Color query results
    let results_text: Vec<Line> = app
        .query_results
        .iter()
        .map(|s| Line::from(s.clone()))
        .collect();
    let results_paragraph = Paragraph::new(results_text)
        .block(Block::default().borders(Borders::ALL).title("Results"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(results_paragraph, chunks[1]);
}

fn render_query_tab(f: &mut ratatui::Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints::<&[ratatui::layout::Constraint]>(
            [Constraint::Length(8), Constraint::Min(0)].as_ref(),
        )
        .split(area);

    // Query options
    let search_status = match app.input_mode {
        InputMode::Searching => format!(" (Searching: {})", app.search_query),
        InputMode::QueryBuilding => " (Building Query)".to_string(),
        InputMode::Normal => String::new(),
    };

    let query_options = vec![
        format!("🔍 Advanced Search & Query Building{}", search_status),
        "".to_string(),
        "Quick Searches:".to_string(),
        "[n] 📝 Search by Name (e.g., 'stone', 'wool')".to_string(),
        "[p] ⚙️  Search by Property (format: property:value)".to_string(),
        "[c] 🎨 Search by Color (format: #RRGGBB)".to_string(),
        "".to_string(),
        "Advanced:".to_string(),
        "[a] 🛠️  Advanced Query Builder".to_string(),
        "[r] 🔄 Reset all filters".to_string(),
        "".to_string(),
        "Legacy Queries:".to_string(),
        "[1] Find blocks by property | [2] Pattern search | [3] Rare properties".to_string(),
    ];

    // Handle input mode display
    let mut display_options = query_options.clone();
    if app.input_mode == InputMode::Searching {
        display_options.push("".to_string());
        display_options.push(format!(
            "✏️  Type your {} query:",
            match app.search_mode {
                SearchMode::ByName => "name",
                SearchMode::ByProperty => "property:value",
                SearchMode::ByColor => "#hex color",
                SearchMode::Advanced => "advanced",
            }
        ));
        display_options.push(format!("   > {}_", app.search_query));
        display_options.push("[Enter] Execute | [Esc] Cancel".to_string());
    }
    let options_text: Vec<Line> = display_options
        .iter()
        .map(|s| Line::from(s.clone()))
        .collect();
    let options_paragraph = Paragraph::new(options_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Query Options"),
    );
    f.render_widget(options_paragraph, chunks[0]);

    // Query results
    let results_text: Vec<Line> = app
        .query_results
        .iter()
        .map(|s| Line::from(s.clone()))
        .collect();
    let results_paragraph = Paragraph::new(results_text)
        .block(Block::default().borders(Borders::ALL).title("Results"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(results_paragraph, chunks[1]);
}

fn render_properties_tab(f: &mut ratatui::Frame, _app: &App, area: Rect) {
    let all_properties = get_all_properties();

    let rows: Vec<Row> = all_properties
        .iter()
        .map(|(prop, values)| {
            Row::new(vec![
                Cell::from(prop.clone()),
                Cell::from(values.len().to_string()),
                Cell::from(values.join(", ")),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(15),
        Constraint::Length(8),
        Constraint::Min(20),
    ];
    let table = Table::new(rows, widths)
        .header(
            Row::new(vec!["Property", "Count", "Values"]).style(Style::default().fg(Color::Yellow)),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("All Properties"),
        );

    f.render_widget(table, area);
}

fn render_statistics_tab(f: &mut ratatui::Frame, _app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints::<&[ratatui::layout::Constraint]>(
            [Constraint::Length(6), Constraint::Min(0)].as_ref(),
        )
        .split(area);

    // Basic stats
    let stats = get_property_stats();
    let stats_text = vec![
        format!("Total blocks: {}", BLOCKS.len()),
        format!("Unique properties: {}", stats.total_unique_properties),
        format!(
            "Blocks with no properties: {}",
            stats.blocks_with_no_properties
        ),
        format!(
            "Average properties per block: {:.2}",
            stats.average_properties_per_block
        ),
    ];
    let stats_lines: Vec<Line> = stats_text.iter().map(|s| Line::from(s.clone())).collect();
    let stats_paragraph = Paragraph::new(stats_lines)
        .block(Block::default().borders(Borders::ALL).title("Statistics"));
    f.render_widget(stats_paragraph, chunks[0]);

    // Property frequency chart
    let all_props = get_all_properties();
    let property_counts: Vec<_> = all_props
        .iter()
        .map(|(prop, values)| (prop.as_str(), values.len() as u64))
        .take(10)
        .collect();

    let barchart = BarChart::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Property Value Counts (Top 10)"),
        )
        .data(&property_counts)
        .bar_width(9)
        .bar_style(Style::default().fg(Color::Green))
        .value_style(
            Style::default()
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(barchart, chunks[1]);
}

fn handle_blocks_input(key: crossterm::event::KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Down => app.next_block(),
        KeyCode::Up => app.previous_block(),
        KeyCode::Enter => {
            // Could implement block selection action here
        }
        _ => {}
    }
}

fn handle_query_input(key: crossterm::event::KeyEvent, app: &mut App) {
    // Handle input mode first
    match app.input_mode {
        InputMode::Searching => {
            match key.code {
                KeyCode::Enter => {
                    app.execute_search();
                }
                KeyCode::Esc => {
                    app.input_mode = InputMode::Normal;
                    app.search_query.clear();
                }
                KeyCode::Backspace => {
                    app.search_query.pop();
                }
                KeyCode::Char(c) => {
                    app.search_query.push(c);
                }
                _ => {}
            }
            return;
        }
        InputMode::Normal => {
            match key.code {
                // New search hotkeys
                KeyCode::Char('n') => app.start_search(SearchMode::ByName),
                KeyCode::Char('p') => app.start_search(SearchMode::ByProperty),
                KeyCode::Char('c') => app.start_search(SearchMode::ByColor),
                KeyCode::Char('a') => app.start_search(SearchMode::Advanced),
                KeyCode::Char('r') => app.reset_filters(),

                // Legacy queries
                KeyCode::Char('1') => app.run_property_query(),
                KeyCode::Char('2') => app.run_pattern_search(),
                KeyCode::Char('3') => app.run_rare_properties_query(),
                KeyCode::Char('4') => app.run_families_query(),
                KeyCode::Char('5') => app.run_statistics_query(),

                // Navigation for filtered results
                KeyCode::Down => app.next_block(),
                KeyCode::Up => app.previous_block(),

                _ => {}
            }
        }
        InputMode::QueryBuilding => {
            // TODO: Implement advanced query builder interface
            match key.code {
                KeyCode::Esc => {
                    app.input_mode = InputMode::Normal;
                }
                _ => {}
            }
        }
    }
}

fn handle_properties_input(_key: crossterm::event::KeyEvent, _app: &mut App) {
    // Properties tab is read-only for now
}

fn handle_statistics_input(_key: crossterm::event::KeyEvent, _app: &mut App) {
    // Statistics tab is read-only for now
}

fn get_all_properties() -> HashMap<String, Vec<String>> {
    let mut all_props = HashMap::new();
    for block in BLOCKS.values() {
        for (prop, values) in block.properties {
            let entry = all_props.entry(prop.to_string()).or_insert_with(Vec::new);
            for value in values.iter() {
                if !entry.contains(&value.to_string()) {
                    entry.push(value.to_string());
                }
            }
        }
    }
    for values in all_props.values_mut() {
        values.sort();
    }
    all_props
}

#[derive(Debug, Clone, Copy)]
enum Tab {
    Blocks = 0,
    Colors = 1,
    Query = 2,
    Properties = 3,
    Statistics = 4,
    Sources = 5,
}

struct App {
    current_tab: Tab,
    filtered_blocks: Vec<&'static BlockFacts>,
    selected_block_index: usize,
    query_results: Vec<String>,
    // Search and filtering
    search_query: String,
    search_mode: SearchMode,
    query_builder: QueryBuilder,
    input_mode: InputMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SearchMode {
    ByName,
    ByProperty,
    ByColor,
    Advanced,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum InputMode {
    Normal,
    Searching,
    QueryBuilding,
}

#[derive(Debug, Clone, Default)]
struct QueryBuilder {
    property_filters: Vec<PropertyFilter>,
    color_filter: Option<ColorFilter>,
    name_pattern: String,
    current_field: QueryField,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum QueryField {
    NamePattern,
    PropertyName,
    PropertyValue,
    ColorHex,
}

#[derive(Debug, Clone)]
struct PropertyFilter {
    property: String,
    value: String,
    operator: FilterOperator,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FilterOperator {
    Equals,
    Contains,
    NotEquals,
    Exists,
}

#[derive(Debug, Clone)]
struct ColorFilter {
    hex_color: String,
    tolerance: f32,
}

impl Default for QueryField {
    fn default() -> Self {
        QueryField::NamePattern
    }
}

impl App {
    fn default() -> App {
        App {
            current_tab: Tab::Blocks,
            filtered_blocks: BLOCKS.values().cloned().collect(),
            selected_block_index: 0,
            query_results: vec!["Select a query option above to see results".to_string()],
            search_query: String::new(),
            search_mode: SearchMode::ByName,
            query_builder: QueryBuilder::default(),
            input_mode: InputMode::Normal,
        }
    }

    fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Blocks => Tab::Colors,
            Tab::Colors => Tab::Query,
            Tab::Query => Tab::Properties,
            Tab::Properties => Tab::Statistics,
            Tab::Statistics => Tab::Sources,
            Tab::Sources => Tab::Blocks,
        };
    }

    fn previous_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Blocks => Tab::Sources,
            Tab::Colors => Tab::Blocks,
            Tab::Query => Tab::Colors,
            Tab::Properties => Tab::Query,
            Tab::Statistics => Tab::Properties,
            Tab::Sources => Tab::Statistics,
        };
    }

    fn next_block(&mut self) {
        if !self.filtered_blocks.is_empty() {
            self.selected_block_index =
                (self.selected_block_index + 1) % self.filtered_blocks.len();
        }
    }

    fn previous_block(&mut self) {
        if !self.filtered_blocks.is_empty() {
            if self.selected_block_index == 0 {
                self.selected_block_index = self.filtered_blocks.len() - 1;
            } else {
                self.selected_block_index -= 1;
            }
        }
    }

    fn get_selected_block(&self) -> Option<&'static BlockFacts> {
        self.filtered_blocks.get(self.selected_block_index).copied()
    }

    fn run_property_query(&mut self) {
        let results: Vec<_> = find_blocks_by_property("delay", "1").collect();
        self.query_results = vec![
            "Query: Blocks with delay=1".to_string(),
            format!("Found {} blocks:", results.len()),
        ];
        for block in results {
            self.query_results.push(format!("  - {}", block.id()));
        }
    }

    fn run_pattern_search(&mut self) {
        let results: Vec<_> = search_blocks("minecraft:*").collect();
        self.query_results = vec![
            "Query: Search pattern 'minecraft:*'".to_string(),
            format!("Found {} blocks:", results.len()),
        ];
        for block in results.iter().take(10) {
            self.query_results.push(format!("  - {}", block.id()));
        }
        if results.len() > 10 {
            self.query_results
                .push(format!("  ... and {} more", results.len() - 10));
        }
    }

    fn run_rare_properties_query(&mut self) {
        let rare_props = find_rare_properties(0.5);
        self.query_results = vec![
            "Query: Rare properties (< 50% frequency)".to_string(),
            format!("Found {} rare properties:", rare_props.len()),
        ];
        for (prop, count) in rare_props {
            self.query_results
                .push(format!("  - {}: {} blocks", prop, count));
        }
    }

    fn run_families_query(&mut self) {
        let families = get_block_families();
        self.query_results = vec![
            "Query: Block families".to_string(),
            format!("Found {} families:", families.len()),
        ];
        for (family, blocks) in families {
            self.query_results
                .push(format!("  - {}: {} blocks", family, blocks.len()));
        }
    }

    fn run_statistics_query(&mut self) {
        let stats = get_property_stats();
        self.query_results = vec![
            "Query: Property statistics".to_string(),
            format!("Total unique properties: {}", stats.total_unique_properties),
            format!(
                "Most common property: {} ({} blocks)",
                stats.most_common_property.0, stats.most_common_property.1
            ),
            format!(
                "Blocks with no properties: {}",
                stats.blocks_with_no_properties
            ),
            format!(
                "Average properties per block: {:.2}",
                stats.average_properties_per_block
            ),
        ];
    }

    fn run_color_coverage_query(&mut self) {
        let total_blocks = BLOCKS.len();
        let blocks_with_color: Vec<_> = BLOCKS
            .values()
            .filter(|b| b.extras.color.is_some())
            .collect();
        let coverage_percentage = (blocks_with_color.len() as f64 / total_blocks as f64) * 100.0;

        self.query_results = vec![
            "🎨 Query: Color Coverage Analysis".to_string(),
            "".to_string(),
            format!("Total blocks: {}", total_blocks),
            format!("Blocks with color data: {}", blocks_with_color.len()),
            format!("Coverage: {:.1}%", coverage_percentage),
            "".to_string(),
            "Sample colored blocks:".to_string(),
        ];

        for block in blocks_with_color.iter().take(10) {
            if let Some(color) = &block.extras.color {
                let hex = format!(
                    "#{:02X}{:02X}{:02X}",
                    color.rgb[0], color.rgb[1], color.rgb[2]
                );
                self.query_results
                    .push(format!("  • {} → {}", block.id(), hex));
            }
        }

        if blocks_with_color.len() > 10 {
            self.query_results
                .push(format!("  ... and {} more", blocks_with_color.len() - 10));
        }
    }

    fn run_color_palette_query(&mut self) {
        let colored_blocks: Vec<_> = BLOCKS
            .values()
            .filter(|b| b.extras.color.is_some())
            .collect();

        // Group by color families
        let mut red_blocks = Vec::new();
        let mut blue_blocks = Vec::new();
        let mut green_blocks = Vec::new();
        let mut other_blocks = Vec::new();

        for block in colored_blocks {
            if let Some(color) = &block.extras.color {
                let (r, g, b) = (color.rgb[0], color.rgb[1], color.rgb[2]);
                if r > g && r > b {
                    red_blocks.push((block, color));
                } else if b > r && b > g {
                    blue_blocks.push((block, color));
                } else if g > r && g > b {
                    green_blocks.push((block, color));
                } else {
                    other_blocks.push((block, color));
                }
            }
        }

        self.query_results = vec![
            "🎨 Query: Color Palette Analysis".to_string(),
            "".to_string(),
            format!("🔴 Red-dominant blocks: {}", red_blocks.len()),
            format!("🔵 Blue-dominant blocks: {}", blue_blocks.len()),
            format!("🟢 Green-dominant blocks: {}", green_blocks.len()),
            format!("⚪ Other colors: {}", other_blocks.len()),
            "".to_string(),
        ];

        // Show examples from each category
        if !red_blocks.is_empty() {
            self.query_results.push("Red blocks:".to_string());
            for (block, color) in red_blocks.iter().take(3) {
                let hex = format!(
                    "#{:02X}{:02X}{:02X}",
                    color.rgb[0], color.rgb[1], color.rgb[2]
                );
                self.query_results
                    .push(format!("  • {} → {}", block.id(), hex));
            }
        }

        if !blue_blocks.is_empty() {
            self.query_results.push("Blue blocks:".to_string());
            for (block, color) in blue_blocks.iter().take(3) {
                let hex = format!(
                    "#{:02X}{:02X}{:02X}",
                    color.rgb[0], color.rgb[1], color.rgb[2]
                );
                self.query_results
                    .push(format!("  • {} → {}", block.id(), hex));
            }
        }

        if !green_blocks.is_empty() {
            self.query_results.push("Green blocks:".to_string());
            for (block, color) in green_blocks.iter().take(3) {
                let hex = format!(
                    "#{:02X}{:02X}{:02X}",
                    color.rgb[0], color.rgb[1], color.rgb[2]
                );
                self.query_results
                    .push(format!("  • {} → {}", block.id(), hex));
            }
        }
    }

    fn run_color_similarity_query(&mut self) {
        // Find blocks similar to stone color (gray)
        let target_color = [125, 125, 125]; // Stone color
        let colored_blocks: Vec<_> = BLOCKS
            .values()
            .filter(|b| b.extras.color.is_some())
            .collect();

        let mut similar_blocks = Vec::new();

        for block in colored_blocks {
            if let Some(color) = &block.extras.color {
                let distance = {
                    let dr = (color.rgb[0] as f32) - (target_color[0] as f32);
                    let dg = (color.rgb[1] as f32) - (target_color[1] as f32);
                    let db = (color.rgb[2] as f32) - (target_color[2] as f32);
                    (dr * dr + dg * dg + db * db).sqrt()
                };

                if distance < 50.0 {
                    // Threshold for similarity
                    similar_blocks.push((block, color, distance));
                }
            }
        }

        similar_blocks.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

        self.query_results = vec![
            "🎨 Query: Color Similarity Search".to_string(),
            "".to_string(),
            format!(
                "Target: Stone gray RGB({}, {}, {})",
                target_color[0], target_color[1], target_color[2]
            ),
            format!("Found {} similar blocks:", similar_blocks.len()),
            "".to_string(),
        ];

        for (block, color, distance) in similar_blocks.iter().take(10) {
            let hex = format!(
                "#{:02X}{:02X}{:02X}",
                color.rgb[0], color.rgb[1], color.rgb[2]
            );
            self.query_results.push(format!(
                "  • {} → {} (distance: {:.1})",
                block.id(),
                hex,
                distance
            ));
        }
    }

    fn run_color_analysis_query(&mut self) {
        let colored_blocks: Vec<_> = BLOCKS
            .values()
            .filter(|b| b.extras.color.is_some())
            .collect();

        // Calculate color statistics
        let mut avg_red = 0.0;
        let mut avg_green = 0.0;
        let mut avg_blue = 0.0;
        let mut brightest = (String::new(), 0u32);
        let mut darkest = (String::new(), 255u32 * 3);

        for block in &colored_blocks {
            if let Some(color) = &block.extras.color {
                avg_red += color.rgb[0] as f64;
                avg_green += color.rgb[1] as f64;
                avg_blue += color.rgb[2] as f64;

                let brightness = color.rgb[0] as u32 + color.rgb[1] as u32 + color.rgb[2] as u32;
                if brightness > brightest.1 {
                    brightest = (block.id().to_string(), brightness);
                }
                if brightness < darkest.1 {
                    darkest = (block.id().to_string(), brightness);
                }
            }
        }

        let count = colored_blocks.len() as f64;
        avg_red /= count;
        avg_green /= count;
        avg_blue /= count;

        self.query_results = vec![
            "🎨 Query: Color Analysis".to_string(),
            "".to_string(),
            format!("Analyzed {} colored blocks", colored_blocks.len()),
            "".to_string(),
            "Average color:".to_string(),
            format!("  RGB({:.0}, {:.0}, {:.0})", avg_red, avg_green, avg_blue),
            format!(
                "  #{:02X}{:02X}{:02X}",
                avg_red as u8, avg_green as u8, avg_blue as u8
            ),
            "".to_string(),
            format!(
                "Brightest block: {} (brightness: {})",
                brightest.0, brightest.1
            ),
            format!("Darkest block: {} (brightness: {})", darkest.0, darkest.1),
        ];
    }

    fn run_gradient_palette_query(&mut self) {
        use blockpedia::color::palettes::{GradientMethod, PaletteGenerator};

        // Get some colored blocks to create gradients
        let colored_blocks: Vec<_> = BLOCKS
            .values()
            .filter(|b| b.extras.color.is_some())
            .collect();

        if colored_blocks.len() < 2 {
            self.query_results = vec![
                "❌ Gradient Palette Generation".to_string(),
                "".to_string(),
                "Not enough colored blocks to generate gradients.".to_string(),
                "Need at least 2 blocks with color data.".to_string(),
            ];
            return;
        }

        // Pick two interesting blocks for demonstration
        let stone_block = colored_blocks
            .iter()
            .find(|b| b.id().contains("stone"))
            .unwrap_or(&colored_blocks[0]);
        let grass_block = colored_blocks
            .iter()
            .find(|b| b.id().contains("grass"))
            .unwrap_or(&colored_blocks[1]);

        let stone_color = stone_block.extras.color.unwrap().to_extended();
        let grass_color = grass_block.extras.color.unwrap().to_extended();

        // Generate a gradient between stone and grass
        let gradient = PaletteGenerator::generate_block_gradient_palette(
            stone_color,
            grass_color,
            10,
            GradientMethod::LinearOklab,
        );

        self.query_results = vec![
            "🌈 Query: Gradient Palette Generation".to_string(),
            "".to_string(),
            format!("Gradient from {} to {}", stone_block.id(), grass_block.id()),
            format!("Method: Perceptual (Oklab) - 10 steps"),
            "".to_string(),
            "Generated palette:".to_string(),
        ];

        for (i, color) in gradient.iter().enumerate() {
            let hex = format!(
                "#{:02X}{:02X}{:02X}",
                color.rgb[0], color.rgb[1], color.rgb[2]
            );
            let step = i as f32 / (gradient.len() - 1) as f32;
            self.query_results
                .push(format!("  {:2}. {} (step {:.1})", i + 1, hex, step));
        }

        self.query_results.extend(vec![
            "".to_string(),
            "💡 Gradient methods available:".to_string(),
            "  • Linear RGB - Simple RGB interpolation".to_string(),
            "  • Linear HSL - Hue-based interpolation".to_string(),
            "  • Linear Oklab - Perceptually uniform".to_string(),
            "  • Cubic Bezier - Smooth curves".to_string(),
        ]);
    }

    fn run_themed_palette_query(&mut self) {
        use blockpedia::color::palettes::PaletteGenerator;

        // Generate some themed palettes
        let sunset = PaletteGenerator::generate_sunset_palette(8);
        let ocean = PaletteGenerator::generate_ocean_palette(6);
        let fire = PaletteGenerator::generate_fire_palette(5);

        self.query_results = vec![
            "🎭 Query: Themed Color Palettes".to_string(),
            "".to_string(),
            "Pre-designed palettes for creative builds:".to_string(),
            "".to_string(),
            "🌅 Sunset Palette (8 colors):".to_string(),
        ];

        for (i, color) in sunset.iter().enumerate() {
            let hex = format!(
                "#{:02X}{:02X}{:02X}",
                color.rgb[0], color.rgb[1], color.rgb[2]
            );
            self.query_results.push(format!("  {}. {}", i + 1, hex));
        }

        self.query_results.push("".to_string());
        self.query_results
            .push("🌊 Ocean Depths Palette (6 colors):".to_string());

        for (i, color) in ocean.iter().enumerate() {
            let hex = format!(
                "#{:02X}{:02X}{:02X}",
                color.rgb[0], color.rgb[1], color.rgb[2]
            );
            self.query_results.push(format!("  {}. {}", i + 1, hex));
        }

        self.query_results.push("".to_string());
        self.query_results
            .push("🔥 Fire Palette (5 colors):".to_string());

        for (i, color) in fire.iter().enumerate() {
            let hex = format!(
                "#{:02X}{:02X}{:02X}",
                color.rgb[0], color.rgb[1], color.rgb[2]
            );
            self.query_results.push(format!("  {}. {}", i + 1, hex));
        }

        self.query_results.extend(vec![
            "".to_string(),
            "Available themed palettes:".to_string(),
            "  🌅 Sunset - Warm gradient from red to midnight blue".to_string(),
            "  🌊 Ocean - Cool blue depths".to_string(),
            "  🔥 Fire - Warm yellows to deep reds".to_string(),
            "  🌲 Forest - Natural greens (available via API)".to_string(),
            "  ⚫ Monochrome - Shades of any base color".to_string(),
        ]);
    }

    fn run_block_palette_query(&mut self) {
        use blockpedia::color::block_palettes::BlockPaletteGenerator;

        // Get some colored blocks to create block palettes
        let colored_blocks: Vec<_> = BLOCKS
            .values()
            .filter(|b| b.extras.color.is_some())
            .collect();

        if colored_blocks.len() < 2 {
            self.query_results = vec![
                "❌ Block Palette Generation".to_string(),
                "".to_string(),
                "Not enough colored blocks to generate palettes.".to_string(),
                "Need at least 2 blocks with color data.".to_string(),
            ];
            return;
        }

        // Pick two interesting blocks for a gradient demonstration
        let stone_block = colored_blocks
            .iter()
            .find(|b| b.id().contains("stone"))
            .unwrap_or(&colored_blocks[0]);
        let grass_block = colored_blocks
            .iter()
            .find(|b| b.id().contains("grass"))
            .unwrap_or(&colored_blocks[1]);

        self.query_results = vec![
            "🧱 Query: Block Palette Generation".to_string(),
            "".to_string(),
            "Generate actual Minecraft block recommendations for building!".to_string(),
            "".to_string(),
        ];

        // Generate a gradient palette
        if let Some(gradient_palette) =
            BlockPaletteGenerator::generate_block_gradient(stone_block, grass_block, 5)
        {
            self.query_results.push(format!(
                "🌈 {} ({})",
                gradient_palette.name,
                gradient_palette.blocks.len()
            ));
            self.query_results.push(gradient_palette.description);
            self.query_results.push("".to_string());

            for (i, rec) in gradient_palette.blocks.iter().enumerate() {
                let block_name = rec
                    .block
                    .id()
                    .strip_prefix("minecraft:")
                    .unwrap_or(rec.block.id());
                self.query_results.push(format!(
                    "  {}. {} {} - {}",
                    i + 1,
                    rec.color.hex_string(),
                    block_name.replace('_', " "),
                    rec.usage_notes
                ));
            }
        }

        // Generate a monochrome palette
        if let Some(mono_palette) =
            BlockPaletteGenerator::generate_monochrome_palette(stone_block, 7)
        {
            self.query_results.push("".to_string());
            self.query_results.push(format!(
                "⚫ {} ({})",
                mono_palette.name,
                mono_palette.blocks.len()
            ));
            self.query_results.push(mono_palette.description);
            self.query_results.push("".to_string());

            for (i, rec) in mono_palette.blocks.iter().enumerate() {
                let block_name = rec
                    .block
                    .id()
                    .strip_prefix("minecraft:")
                    .unwrap_or(rec.block.id());
                self.query_results.push(format!(
                    "  {}. {} {} ({})",
                    i + 1,
                    rec.color.hex_string(),
                    block_name.replace('_', " "),
                    format!("{:?}", rec.role)
                ));
            }
        }

        self.query_results.extend(vec![
            "".to_string(),
            "💡 Block palette types available:".to_string(),
            "  • Gradient - Smooth color transitions between blocks".to_string(),
            "  • Monochrome - Tonal variations of a base block".to_string(),
            "  • Complementary - High contrast color combinations".to_string(),
            "  • Natural biomes - Forest, desert, ocean, mountain".to_string(),
            "  • Architectural styles - Medieval, modern, rustic".to_string(),
        ]);
    }

    fn run_architectural_palette_query(&mut self) {
        use blockpedia::color::block_palettes::BlockPaletteGenerator;

        self.query_results = vec![
            "🏗️ Query: Architectural Block Palettes".to_string(),
            "".to_string(),
            "Professional building palettes for different architectural styles:".to_string(),
            "".to_string(),
        ];

        // Generate palettes for different architectural styles
        let styles = ["medieval", "modern", "rustic"];

        for style in &styles {
            if let Some(palette) = BlockPaletteGenerator::generate_architectural_palette(style) {
                let icon = match *style {
                    "medieval" => "🏰",
                    "modern" => "🏢",
                    "rustic" => "🏡",
                    _ => "🏗️",
                };

                self.query_results.push(format!(
                    "{} {} ({})",
                    icon,
                    palette.name,
                    palette.blocks.len()
                ));
                self.query_results.push(palette.description);

                for (_i, rec) in palette.blocks.iter().take(4).enumerate() {
                    let block_name = rec
                        .block
                        .id()
                        .strip_prefix("minecraft:")
                        .unwrap_or(rec.block.id());
                    self.query_results.push(format!(
                        "  • {} {} - {}",
                        rec.color.hex_string(),
                        block_name.replace('_', " "),
                        match rec.role {
                            blockpedia::color::block_palettes::BlockRole::Primary =>
                                "Main structure",
                            blockpedia::color::block_palettes::BlockRole::Secondary => "Supporting",
                            blockpedia::color::block_palettes::BlockRole::Accent => "Details",
                            _ => "Other",
                        }
                    ));
                }
                self.query_results.push("".to_string());
            }
        }

        // Generate natural biome palettes
        if let Some(forest_palette) = BlockPaletteGenerator::generate_natural_palette("forest") {
            self.query_results
                .push("🌲 Forest Biome Palette".to_string());
            self.query_results.push(forest_palette.description);

            for rec in forest_palette.blocks.iter().take(3) {
                let block_name = rec
                    .block
                    .id()
                    .strip_prefix("minecraft:")
                    .unwrap_or(rec.block.id());
                self.query_results.push(format!(
                    "  • {} {}",
                    rec.color.hex_string(),
                    block_name.replace('_', " ")
                ));
            }
        }

        self.query_results.extend(vec![
            "".to_string(),
            "Available styles:".to_string(),
            "  🏰 Medieval - Cobblestone, oak, stone bricks".to_string(),
            "  🏢 Modern - Concrete, glass, clean lines".to_string(),
            "  🏡 Rustic - Natural materials, country style".to_string(),
            "  🏭 Industrial - Metal, gray concrete, tech blocks".to_string(),
            "".to_string(),
            "Natural biomes:".to_string(),
            "  🌲 Forest, 🏜️ Desert, 🌊 Ocean, ⛰️ Mountain, 🔥 Nether, 🌌 End".to_string(),
        ]);
    }

    // Search and filtering methods
    fn start_search(&mut self, mode: SearchMode) {
        self.search_mode = mode;
        self.input_mode = InputMode::Searching;
        self.search_query.clear();
    }

    fn execute_search(&mut self) {
        match self.search_mode {
            SearchMode::ByName => self.search_by_name(),
            SearchMode::ByProperty => self.search_by_property(),
            SearchMode::ByColor => self.search_by_color(),
            SearchMode::Advanced => self.execute_advanced_query(),
        }
        self.input_mode = InputMode::Normal;
    }

    fn search_by_name(&mut self) {
        let query = self.search_query.to_lowercase();
        let results: Vec<_> = BLOCKS
            .values()
            .filter(|block| block.id().to_lowercase().contains(&query))
            .collect();

        self.filtered_blocks = results.into_iter().cloned().collect();
        self.selected_block_index = 0;

        self.query_results = vec![
            format!("🔍 Name Search: '{}'", self.search_query),
            "".to_string(),
            format!("Found {} blocks:", self.filtered_blocks.len()),
        ];

        for block in self.filtered_blocks.iter().take(20) {
            let mut indicators = Vec::new();
            if block.extras.color.is_some() {
                indicators.push("🎨");
            }
            if !block.properties.is_empty() {
                indicators.push("⚙️");
            }

            let indicator_text = if indicators.is_empty() {
                String::new()
            } else {
                format!(" {}", indicators.join(""))
            };

            self.query_results
                .push(format!("  • {}{}", block.id(), indicator_text));
        }

        if self.filtered_blocks.len() > 20 {
            self.query_results.push(format!(
                "  ... and {} more",
                self.filtered_blocks.len() - 20
            ));
        }
    }

    fn search_by_property(&mut self) {
        // Parse property:value format
        let parts: Vec<&str> = self.search_query.split(':').collect();
        if parts.len() != 2 {
            self.query_results = vec![
                "❌ Property Search Error".to_string(),
                "".to_string(),
                "Format: property:value".to_string(),
                "Examples:".to_string(),
                "  • delay:1".to_string(),
                "  • facing:north".to_string(),
                "  • waterlogged:true".to_string(),
            ];
            return;
        }

        let prop_name = parts[0].trim();
        let prop_value = parts[1].trim();

        let results: Vec<_> = BLOCKS
            .values()
            .filter(|block| {
                if let Some(values) = block.properties.iter().find(|(name, _)| *name == prop_name) {
                    values.1.contains(&prop_value)
                } else {
                    false
                }
            })
            .collect();

        self.filtered_blocks = results.into_iter().cloned().collect();
        self.selected_block_index = 0;

        self.query_results = vec![
            format!("🔍 Property Search: {} = {}", prop_name, prop_value),
            "".to_string(),
            format!("Found {} blocks:", self.filtered_blocks.len()),
        ];

        for block in self.filtered_blocks.iter().take(15) {
            let mut details = vec![block.id().to_string()];

            // Show the matching property
            if let Some((_, values)) = block.properties.iter().find(|(name, _)| *name == prop_name)
            {
                details.push(format!("({}: {})", prop_name, values.join(", ")));
            }

            // Add color indicator
            if let Some(color) = &block.extras.color {
                let hex = format!(
                    "#{:02X}{:02X}{:02X}",
                    color.rgb[0], color.rgb[1], color.rgb[2]
                );
                details.push(format!("🎨 {}", hex));
            }

            self.query_results
                .push(format!("  • {}", details.join(" ")));
        }

        if self.filtered_blocks.len() > 15 {
            self.query_results.push(format!(
                "  ... and {} more",
                self.filtered_blocks.len() - 15
            ));
        }
    }

    fn search_by_color(&mut self) {
        // Parse hex color format (#RRGGBB)
        let hex = self.search_query.trim();
        if !hex.starts_with('#') || hex.len() != 7 {
            self.query_results = vec![
                "❌ Color Search Error".to_string(),
                "".to_string(),
                "Format: #RRGGBB (hex color)".to_string(),
                "Examples:".to_string(),
                "  • #FF0000 (red)".to_string(),
                "  • #00FF00 (green)".to_string(),
                "  • #7D7D7D (stone gray)".to_string(),
            ];
            return;
        }

        // Parse target color
        let target_r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
        let target_g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0);
        let target_b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0);

        let colored_blocks: Vec<_> = BLOCKS
            .values()
            .filter(|block| block.extras.color.is_some())
            .collect();

        let mut similar_blocks = Vec::new();

        for block in colored_blocks {
            if let Some(color) = &block.extras.color {
                let distance = {
                    let dr = (color.rgb[0] as f32) - (target_r as f32);
                    let dg = (color.rgb[1] as f32) - (target_g as f32);
                    let db = (color.rgb[2] as f32) - (target_b as f32);
                    (dr * dr + dg * dg + db * db).sqrt()
                };

                similar_blocks.push((block, color, distance));
            }
        }

        // Sort by similarity (closest first)
        similar_blocks.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

        // Take the closest matches
        self.filtered_blocks = similar_blocks
            .iter()
            .take(50)
            .map(|(block, _, _)| **block)
            .collect();
        self.selected_block_index = 0;

        self.query_results = vec![
            format!(
                "🎨 Color Search: {} → RGB({}, {}, {})",
                hex, target_r, target_g, target_b
            ),
            "".to_string(),
            format!("Found {} similar blocks:", self.filtered_blocks.len()),
        ];

        for (block, color, distance) in similar_blocks.iter().take(15) {
            let block_hex = format!(
                "#{:02X}{:02X}{:02X}",
                color.rgb[0], color.rgb[1], color.rgb[2]
            );
            self.query_results.push(format!(
                "  • {} → {} (Δ {:.1})",
                block.id(),
                block_hex,
                distance
            ));
        }

        if similar_blocks.len() > 15 {
            self.query_results
                .push(format!("  ... and {} more", similar_blocks.len() - 15));
        }
    }

    fn execute_advanced_query(&mut self) {
        // Execute the built query
        let mut results: Vec<_> = BLOCKS.values().collect();

        // Apply name pattern filter
        if !self.query_builder.name_pattern.is_empty() {
            let pattern = self.query_builder.name_pattern.to_lowercase();
            results = results
                .into_iter()
                .filter(|block| block.id().to_lowercase().contains(&pattern))
                .collect();
        }

        // Apply property filters
        for filter in &self.query_builder.property_filters {
            results = results
                .into_iter()
                .filter(|block| self.apply_property_filter(block, filter))
                .collect();
        }

        // Apply color filter
        if let Some(color_filter) = &self.query_builder.color_filter {
            results = results
                .into_iter()
                .filter(|block| self.apply_color_filter(block, color_filter))
                .collect();
        }

        self.filtered_blocks = results.into_iter().cloned().collect();
        self.selected_block_index = 0;

        // Generate results summary
        self.query_results = vec![
            "🔍 Advanced Query Results".to_string(),
            "".to_string(),
            format!("Found {} blocks matching:", self.filtered_blocks.len()),
        ];

        if !self.query_builder.name_pattern.is_empty() {
            self.query_results.push(format!(
                "  • Name contains: '{}'",
                self.query_builder.name_pattern
            ));
        }

        for filter in &self.query_builder.property_filters {
            let op_text = match filter.operator {
                FilterOperator::Equals => "=",
                FilterOperator::Contains => "contains",
                FilterOperator::NotEquals => "≠",
                FilterOperator::Exists => "exists",
            };

            let value_text = if filter.operator == FilterOperator::Exists {
                String::new()
            } else {
                format!(" '{}'", filter.value)
            };

            self.query_results.push(format!(
                "  • Property: {} {}{}",
                filter.property, op_text, value_text
            ));
        }

        if let Some(color_filter) = &self.query_builder.color_filter {
            self.query_results.push(format!(
                "  • Color similar to: {} (±{:.0})",
                color_filter.hex_color, color_filter.tolerance
            ));
        }

        self.query_results.push("".to_string());

        // Show first few results
        for block in self.filtered_blocks.iter().take(10) {
            let mut details = vec![block.id().to_string()];

            if let Some(color) = &block.extras.color {
                let hex = format!(
                    "#{:02X}{:02X}{:02X}",
                    color.rgb[0], color.rgb[1], color.rgb[2]
                );
                details.push(format!("🎨 {}", hex));
            }

            if !block.properties.is_empty() {
                details.push(format!("⚙️  {} props", block.properties.len()));
            }

            self.query_results
                .push(format!("  • {}", details.join(" ")));
        }

        if self.filtered_blocks.len() > 10 {
            self.query_results.push(format!(
                "  ... and {} more",
                self.filtered_blocks.len() - 10
            ));
        }
    }

    fn apply_property_filter(&self, block: &BlockFacts, filter: &PropertyFilter) -> bool {
        match filter.operator {
            FilterOperator::Exists => block
                .properties
                .iter()
                .any(|(name, _)| name == &filter.property),
            FilterOperator::Equals => {
                if let Some((_, values)) = block
                    .properties
                    .iter()
                    .find(|(name, _)| name == &filter.property)
                {
                    values.contains(&filter.value.as_str())
                } else {
                    false
                }
            }
            FilterOperator::Contains => {
                if let Some((_, values)) = block
                    .properties
                    .iter()
                    .find(|(name, _)| name == &filter.property)
                {
                    values.iter().any(|v| v.contains(&filter.value))
                } else {
                    false
                }
            }
            FilterOperator::NotEquals => {
                if let Some((_, values)) = block
                    .properties
                    .iter()
                    .find(|(name, _)| name == &filter.property)
                {
                    !values.contains(&filter.value.as_str())
                } else {
                    true
                }
            }
        }
    }

    fn apply_color_filter(&self, block: &BlockFacts, filter: &ColorFilter) -> bool {
        if let Some(color) = &block.extras.color {
            if let Ok(target_r) = u8::from_str_radix(&filter.hex_color[1..3], 16) {
                if let Ok(target_g) = u8::from_str_radix(&filter.hex_color[3..5], 16) {
                    if let Ok(target_b) = u8::from_str_radix(&filter.hex_color[5..7], 16) {
                        let distance = {
                            let dr = (color.rgb[0] as f32) - (target_r as f32);
                            let dg = (color.rgb[1] as f32) - (target_g as f32);
                            let db = (color.rgb[2] as f32) - (target_b as f32);
                            (dr * dr + dg * dg + db * db).sqrt()
                        };
                        return distance <= filter.tolerance;
                    }
                }
            }
        }
        false
    }

    fn reset_filters(&mut self) {
        self.filtered_blocks = BLOCKS.values().cloned().collect();
        self.selected_block_index = 0;
        self.query_builder = QueryBuilder::default();
        self.search_query.clear();
        self.input_mode = InputMode::Normal;

        self.query_results = vec![
            "🔄 Filters Reset".to_string(),
            "".to_string(),
            format!("Showing all {} blocks", self.filtered_blocks.len()),
        ];
    }
}

fn handle_colors_input(key: crossterm::event::KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char('1') => app.run_color_coverage_query(),
        KeyCode::Char('2') => app.run_color_palette_query(),
        KeyCode::Char('3') => app.run_color_similarity_query(),
        KeyCode::Char('4') => app.run_color_analysis_query(),
        KeyCode::Char('5') => app.run_gradient_palette_query(),
        KeyCode::Char('6') => app.run_themed_palette_query(),
        KeyCode::Char('7') => app.run_block_palette_query(),
        KeyCode::Char('8') => app.run_architectural_palette_query(),
        KeyCode::Down => app.next_block(),
        KeyCode::Up => app.previous_block(),
        _ => {}
    }
}

fn handle_sources_input(key: crossterm::event::KeyEvent, _app: &mut App) {
    match key.code {
        KeyCode::Char('i') => {
            // Info action - would open detailed info about current source
        }
        KeyCode::Char('r') => {
            // Refresh action - would reload source information
        }
        _ => {}
    }
}

fn render_sources_tab(f: &mut ratatui::Frame, _app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints::<&[ratatui::layout::Constraint]>(
            [Constraint::Length(8), Constraint::Min(0)].as_ref(),
        )
        .split(area);

    // Data source information
    let source_info = vec![
        "🌐 Data Source Management",
        "",
        "Current build was compiled with data from one of these sources:",
        "• PrismarineJS: Complete block states and properties (~1058 blocks)",
        "• MCPropertyEncyclopedia: Rich metadata and descriptions (~288 blocks)",
        "",
        "💡 To switch data sources, rebuild with:",
        "   BLOCKPEDIA_DATA_SOURCE=MCPropertyEncyclopedia cargo build",
    ];

    let source_lines: Vec<Line> = source_info.iter().map(|s| Line::from(*s)).collect();
    let source_paragraph = Paragraph::new(source_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Source Information"),
    );
    f.render_widget(source_paragraph, chunks[0]);

    // Current source stats (detect based on block count)
    let total_blocks = BLOCKS.len();
    let current_source = if total_blocks > 1000 {
        "PrismarineJS"
    } else if total_blocks > 200 {
        "MCPropertyEncyclopedia"
    } else {
        "Test Data"
    };

    let stats_info = vec![
        format!("📊 Current Statistics"),
        "".to_string(),
        format!("Active Source: {}", current_source),
        format!("Total Blocks: {}", total_blocks),
        "".to_string(),
        "Block Analysis:".to_string(),
    ];

    // Add some quick analysis
    let mut enhanced_stats = stats_info;
    let blocks_with_properties = BLOCKS.values().filter(|b| !b.properties.is_empty()).count();
    let blocks_with_color = BLOCKS.values().filter(|b| b.extras.color.is_some()).count();
    let blocks_with_mock_data = BLOCKS
        .values()
        .filter(|b| b.extras.mock_data.is_some())
        .count();

    enhanced_stats.extend(vec![
        format!(
            "• Blocks with properties: {} ({:.1}%)",
            blocks_with_properties,
            (blocks_with_properties as f64 / total_blocks as f64) * 100.0
        ),
        format!(
            "• Blocks with color data: {} ({:.1}%)",
            blocks_with_color,
            (blocks_with_color as f64 / total_blocks as f64) * 100.0
        ),
        format!(
            "• Blocks with extra data: {} ({:.1}%)",
            blocks_with_mock_data,
            (blocks_with_mock_data as f64 / total_blocks as f64) * 100.0
        ),
        "".to_string(),
        "🔄 Use [r] to refresh this information".to_string(),
        "ℹ️  Use [i] for detailed source information".to_string(),
    ]);

    let stats_lines: Vec<Line> = enhanced_stats
        .iter()
        .map(|s| Line::from(s.clone()))
        .collect();
    let stats_paragraph = Paragraph::new(stats_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Current Build Stats"),
        )
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(stats_paragraph, chunks[1]);
}
