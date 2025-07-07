use blockpedia::{color::ExtendedColorData, query_builder::*};
use crossterm::event::{self, Event as CEvent, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs};
use ratatui::Terminal;
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
            // Global keys
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Tab => app.next_tab(),
                KeyCode::BackTab => app.previous_tab(),
                KeyCode::Esc => {
                    app.modal_open = false;
                    app.input_mode = InputMode::Normal;
                    app.input_buffer.clear();
                }
                _ => {}
            }

            // Tab-specific input handling
            match app.current_tab {
                Tab::QueryBuilder => handle_query_builder_input(key, app),
                Tab::Results => handle_results_input(key, app),
                Tab::Examples => handle_examples_input(key, app),
                Tab::Help => handle_help_input(key, app),
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Title
    let title = Paragraph::new("🧱 Blockpedia Modern Query Builder")
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Tabs
    let titles = ["🔍 Query Builder", "📋 Results", "💡 Examples", "❓ Help"]
        .iter()
        .cloned()
        .map(Line::from)
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL))
        .select(app.current_tab as usize)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Blue),
        );

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(chunks[1]);

    f.render_widget(tabs, inner_chunks[0]);

    match app.current_tab {
        Tab::QueryBuilder => render_query_builder_tab(f, app, inner_chunks[1]),
        Tab::Results => render_results_tab(f, app, inner_chunks[1]),
        Tab::Examples => render_examples_tab(f, app, inner_chunks[1]),
        Tab::Help => render_help_tab(f, app, inner_chunks[1]),
    }

    // Status bar
    let help_text = match app.current_tab {
        Tab::QueryBuilder => {
            "[Enter] Execute query | [r] Reset | [s] Save query | [Tab] Switch tabs | [q] Quit"
        }
        Tab::Results => {
            "[↑/↓] Navigate | [Enter] Details | [e] Export | [Tab] Switch tabs | [q] Quit"
        }
        Tab::Examples => "[↑/↓] Navigate | [Enter] Load example | [Tab] Switch tabs | [q] Quit",
        Tab::Help => "[Tab] Switch tabs | [q] Quit",
    };
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);

    // Modal dialogs
    if app.modal_open {
        render_modal(f, app);
    }
}

fn render_query_builder_tab(f: &mut ratatui::Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(12), Constraint::Min(0)])
        .split(chunks[0]);

    // Query operations panel
    let operations = vec![
        "🔍 Query Operations:",
        "",
        "[1] 🏗️  Filter: Only solid blocks",
        "[2] 🌱 Filter: Survival obtainable only",
        "[3] 🎨 Filter: Blocks with color data",
        "[4] 🚫 Filter: Exclude transparent blocks",
        "[5] 🔧 Filter: Exclude tile entities",
        "[6] 📝 Filter: By property (name:value)",
        "[7] 🎯 Filter: By color similarity (#hex)",
        "[8] 🔤 Filter: By name pattern (*wildcards*)",
        "[9] 📊 Limit: Number of results",
        "[0] ⚡ Generate: Color gradient",
    ];
    let operations_text: Vec<Line> = operations.iter().map(|s| Line::from(*s)).collect();
    let operations_paragraph = Paragraph::new(operations_text)
        .block(Block::default().borders(Borders::ALL).title("Operations"));
    f.render_widget(operations_paragraph, left_chunks[0]);

    // Current query display
    let query_display = format_current_query(app);
    let query_text: Vec<Line> = query_display
        .iter()
        .map(|s| Line::from(s.clone()))
        .collect();
    let query_paragraph = Paragraph::new(query_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Current Query"),
        )
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(query_paragraph, left_chunks[1]);

    // Results preview
    let preview_text = if app.query_results.is_empty() {
        vec![
            "No query executed yet.".to_string(),
            "".to_string(),
            "Build a query using operations 1-0, then press [Enter] to execute.".to_string(),
        ]
    } else {
        let mut preview = vec![
            format!("Results: {} blocks found", app.query_results.len()),
            "".to_string(),
        ];
        for (i, block) in app.query_results.iter().take(10).enumerate() {
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
            preview.push(format!("{}. {}{}", i + 1, block.id(), indicator_text));
        }
        if app.query_results.len() > 10 {
            preview.push(format!("... and {} more", app.query_results.len() - 10));
        }
        preview
    };

    let preview_lines: Vec<Line> = preview_text.iter().map(|s| Line::from(s.clone())).collect();
    let preview_paragraph = Paragraph::new(preview_lines)
        .block(Block::default().borders(Borders::ALL).title("Preview"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(preview_paragraph, chunks[1]);
}

fn render_results_tab(f: &mut ratatui::Frame, app: &App, area: Rect) {
    if app.query_results.is_empty() {
        let empty_message = Paragraph::new(
            "No results to display.\n\nGo to the Query Builder tab to create and execute a query.",
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Results"));
        f.render_widget(empty_message, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Results list
    let items: Vec<ListItem> = app
        .query_results
        .iter()
        .enumerate()
        .map(|(i, block)| {
            let mut style = Style::default();
            if i == app.selected_result_index {
                style = style.bg(Color::Blue).fg(Color::White);
            }

            let mut indicators = Vec::new();
            if block.extras.color.is_some() {
                indicators.push("🎨");
            }
            if !block.properties.is_empty() {
                indicators.push("⚙️");
            }
            if block.extras.mock_data.is_some() {
                indicators.push("🔧");
            }

            let indicator_text = if indicators.is_empty() {
                String::new()
            } else {
                format!(" {}", indicators.join(""))
            };

            ListItem::new(format!("{}{}", block.id(), indicator_text)).style(style)
        })
        .collect();

    let results_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Results ({} blocks)", app.query_results.len())),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(results_list, chunks[0]);

    // Block details
    if let Some(selected_block) = app.get_selected_result() {
        let mut details = vec![format!("🆔 ID: {}", selected_block.id()), "".to_string()];

        // Properties
        if selected_block.properties.is_empty() {
            details.push("⚙️ Properties: None".to_string());
        } else {
            details.push("⚙️ Properties:".to_string());
            for (prop, values) in selected_block.properties {
                details.push(format!("   • {}: {}", prop, values.join(", ")));
            }
        }

        details.push("".to_string());

        // Color information
        if let Some(color) = &selected_block.extras.color {
            details.push("🎨 Color Data:".to_string());
            details.push(format!("   • RGB: {:?}", color.rgb));
            details.push(format!(
                "   • Hex: #{:02X}{:02X}{:02X}",
                color.rgb[0], color.rgb[1], color.rgb[2]
            ));
            details.push(format!(
                "   • Oklab: [{:.2}, {:.2}, {:.2}]",
                color.oklab[0], color.oklab[1], color.oklab[2]
            ));
        } else {
            details.push("🎨 Color Data: None".to_string());
        }

        details.push("".to_string());

        // Default state
        if selected_block.default_state.is_empty() {
            details.push("🏗️ Default State: None".to_string());
        } else {
            details.push("🏗️ Default State:".to_string());
            for (key, value) in selected_block.default_state {
                details.push(format!("   • {}: {}", key, value));
            }
        }

        // Mock data if present
        if let Some(mock_data) = selected_block.extras.mock_data {
            details.push("".to_string());
            details.push(format!("🔧 Mock Data: {}", mock_data));
        }

        let details_text: Vec<Line> = details.iter().map(|s| Line::from(s.clone())).collect();
        let details_paragraph = Paragraph::new(details_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Block Details"),
            )
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(details_paragraph, chunks[1]);
    }
}

fn render_examples_tab(f: &mut ratatui::Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Examples list
    let examples = get_example_queries();
    let items: Vec<ListItem> = examples
        .iter()
        .enumerate()
        .map(|(i, example)| {
            let mut style = Style::default();
            if i == app.selected_example_index {
                style = style.bg(Color::Green).fg(Color::Black);
            }
            ListItem::new(format!("{}. {}", i + 1, example.name)).style(style)
        })
        .collect();

    let examples_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Example Queries"),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(examples_list, chunks[0]);

    // Example details
    if let Some(example) = examples.get(app.selected_example_index) {
        let mut details = vec![
            format!("📝 {}", example.name),
            "".to_string(),
            format!("📋 Description:"),
            format!("   {}", example.description),
            "".to_string(),
            format!("🔧 Query Steps:"),
        ];

        for (i, step) in example.steps.iter().enumerate() {
            details.push(format!("   {}. {}", i + 1, step));
        }

        details.push("".to_string());
        details.push("💡 Use Case:".to_string());
        details.push(format!("   {}", example.use_case));

        let details_text: Vec<Line> = details.iter().map(|s| Line::from(s.clone())).collect();
        let details_paragraph = Paragraph::new(details_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Example Details"),
            )
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(details_paragraph, chunks[1]);
    }
}

fn render_help_tab(f: &mut ratatui::Frame, _app: &App, area: Rect) {
    let help_content = vec![
        "🧱 Blockpedia Modern Query Builder - Help",
        "",
        "📚 OVERVIEW:",
        "This modern CLI demonstrates the new BlockQuery system that allows",
        "you to build complex, chainable queries for Minecraft block data.",
        "",
        "🔍 QUERY BUILDER TAB:",
        "• Use number keys (1-0) to add filters and operations",
        "• Press Enter to execute the current query",
        "• Press 'r' to reset the query",
        "• Filters can be chained together for complex queries",
        "",
        "📊 FEATURES:",
        "• Solid blocks filtering (excludes stairs, slabs, etc.)",
        "• Survival mode filtering (excludes creative-only blocks)",
        "• Color-based filtering and similarity search",
        "• Property-based filtering with flexible operators",
        "• Pattern matching with wildcards (*)",
        "• Gradient generation between block colors",
        "• Bidirectional chaining (filter → gradient → filter)",
        "",
        "🎨 COLOR FEATURES:",
        "• 472+ blocks with real texture color data",
        "• Multiple color spaces (RGB, HSL, Oklab, Lab)",
        "• Color similarity search with tolerance",
        "• Gradient generation with easing functions",
        "",
        "⌨️  KEYBOARD SHORTCUTS:",
        "• Tab/Shift+Tab: Switch between tabs",
        "• q: Quit application",
        "• Esc: Cancel current operation/close modals",
        "• ↑/↓: Navigate lists",
        "• Enter: Select/Execute",
        "",
        "💡 EXAMPLES TAB:",
        "Contains pre-built query examples showing different use cases",
        "from simple filtering to complex gradient generation.",
        "",
        "🚀 ADVANCED USAGE:",
        "The query system supports method chaining:",
        "AllBlocks::new()",
        "  .only_solid()",
        "  .with_color()",
        "  .generate_gradient(config)",
        "  .sort_by_name()",
        "  .limit(10)",
        "",
        "This allows for powerful, readable queries that can be built",
        "step by step and modified as needed.",
    ];

    let help_text: Vec<Line> = help_content.iter().map(|s| Line::from(*s)).collect();
    let help_paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help & Documentation"),
        )
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(help_paragraph, area);
}

fn render_modal(f: &mut ratatui::Frame, app: &App) {
    let area = centered_rect(60, 20, f.size());
    f.render_widget(Clear, area);

    let modal_title = match app.modal_type {
        ModalType::PropertyInput => "Enter Property Filter",
        ModalType::ColorInput => "Enter Color Filter",
        ModalType::PatternInput => "Enter Name Pattern",
        ModalType::LimitInput => "Enter Limit",
        ModalType::GradientConfig => "Configure Gradient",
        ModalType::SaveQuery => "Save Query",
    };

    let prompt = match app.modal_type {
        ModalType::PropertyInput => "Format: property:value (e.g., delay:1, facing:north)",
        ModalType::ColorInput => "Format: #RRGGBB (e.g., #FF0000 for red)",
        ModalType::PatternInput => "Pattern with wildcards (e.g., *stone*, minecraft:*)",
        ModalType::LimitInput => "Maximum number of results (e.g., 10, 50)",
        ModalType::GradientConfig => "Number of gradient steps (e.g., 5, 10)",
        ModalType::SaveQuery => "Query name (not yet implemented)",
    };

    let content = vec![
        prompt.to_string(),
        "".to_string(),
        format!("Input: {}_", app.input_buffer),
        "".to_string(),
        "[Enter] Confirm | [Esc] Cancel".to_string(),
    ];

    let content_text: Vec<Line> = content.iter().map(|s| Line::from(s.clone())).collect();
    let modal_paragraph = Paragraph::new(content_text)
        .block(Block::default().borders(Borders::ALL).title(modal_title))
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(modal_paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn handle_query_builder_input(key: crossterm::event::KeyEvent, app: &mut App) {
    match app.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('1') => app.add_solid_filter(),
            KeyCode::Char('2') => app.add_survival_filter(),
            KeyCode::Char('3') => app.add_color_filter(),
            KeyCode::Char('4') => app.add_transparent_exclusion(),
            KeyCode::Char('5') => app.add_tile_entity_exclusion(),
            KeyCode::Char('6') => app.open_property_input_modal(),
            KeyCode::Char('7') => app.open_color_input_modal(),
            KeyCode::Char('8') => app.open_pattern_input_modal(),
            KeyCode::Char('9') => app.open_limit_input_modal(),
            KeyCode::Char('0') => app.open_gradient_config_modal(),
            KeyCode::Enter => app.execute_query(),
            KeyCode::Char('r') => app.reset_query(),
            KeyCode::Char('s') => app.open_save_query_modal(),
            _ => {}
        },
        InputMode::Modal => match key.code {
            KeyCode::Enter => app.confirm_modal_input(),
            KeyCode::Backspace => {
                app.input_buffer.pop();
            }
            KeyCode::Char(c) => {
                app.input_buffer.push(c);
            }
            _ => {}
        },
    }
}

fn handle_results_input(key: crossterm::event::KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Down => app.next_result(),
        KeyCode::Up => app.previous_result(),
        KeyCode::Enter => {
            // Could implement detailed view
        }
        KeyCode::Char('e') => {
            // Could implement export functionality
        }
        _ => {}
    }
}

fn handle_examples_input(key: crossterm::event::KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Down => app.next_example(),
        KeyCode::Up => app.previous_example(),
        KeyCode::Enter => app.load_example(),
        _ => {}
    }
}

fn handle_help_input(_key: crossterm::event::KeyEvent, _app: &mut App) {
    // Help tab is read-only
}

#[derive(Debug, Clone, Copy)]
enum Tab {
    QueryBuilder = 0,
    Results = 1,
    Examples = 2,
    Help = 3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum InputMode {
    Normal,
    Modal,
}

#[derive(Debug, Clone, Copy)]
enum ModalType {
    PropertyInput,
    ColorInput,
    PatternInput,
    LimitInput,
    GradientConfig,
    SaveQuery,
}

struct App {
    current_tab: Tab,
    current_query: QueryState,
    query_results: Vec<&'static blockpedia::BlockFacts>,
    selected_result_index: usize,
    selected_example_index: usize,
    input_mode: InputMode,
    modal_open: bool,
    modal_type: ModalType,
    input_buffer: String,
}

#[derive(Debug, Clone)]
struct QueryState {
    operations: Vec<QueryOperation>,
}

#[derive(Debug, Clone)]
enum QueryOperation {
    OnlySolid,
    SurvivalOnly,
    WithColor,
    ExcludeTransparent,
    ExcludeTileEntities,
    PropertyFilter { property: String, value: String },
    ColorSimilarity { hex: String, tolerance: f32 },
    PatternMatch(String),
    Limit(usize),
    GenerateGradient { steps: usize },
}

impl Default for App {
    fn default() -> App {
        App {
            current_tab: Tab::QueryBuilder,
            current_query: QueryState {
                operations: Vec::new(),
            },
            query_results: Vec::new(),
            selected_result_index: 0,
            selected_example_index: 0,
            input_mode: InputMode::Normal,
            modal_open: false,
            modal_type: ModalType::PropertyInput,
            input_buffer: String::new(),
        }
    }
}

impl App {
    fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::QueryBuilder => Tab::Results,
            Tab::Results => Tab::Examples,
            Tab::Examples => Tab::Help,
            Tab::Help => Tab::QueryBuilder,
        };
    }

    fn previous_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::QueryBuilder => Tab::Help,
            Tab::Results => Tab::QueryBuilder,
            Tab::Examples => Tab::Results,
            Tab::Help => Tab::Examples,
        };
    }

    fn next_result(&mut self) {
        if !self.query_results.is_empty() {
            self.selected_result_index =
                (self.selected_result_index + 1) % self.query_results.len();
        }
    }

    fn previous_result(&mut self) {
        if !self.query_results.is_empty() {
            if self.selected_result_index == 0 {
                self.selected_result_index = self.query_results.len() - 1;
            } else {
                self.selected_result_index -= 1;
            }
        }
    }

    fn next_example(&mut self) {
        let examples = get_example_queries();
        if !examples.is_empty() {
            self.selected_example_index = (self.selected_example_index + 1) % examples.len();
        }
    }

    fn previous_example(&mut self) {
        let examples = get_example_queries();
        if !examples.is_empty() {
            if self.selected_example_index == 0 {
                self.selected_example_index = examples.len() - 1;
            } else {
                self.selected_example_index -= 1;
            }
        }
    }

    fn get_selected_result(&self) -> Option<&'static blockpedia::BlockFacts> {
        self.query_results.get(self.selected_result_index).copied()
    }

    // Query operations
    fn add_solid_filter(&mut self) {
        self.current_query
            .operations
            .push(QueryOperation::OnlySolid);
    }

    fn add_survival_filter(&mut self) {
        self.current_query
            .operations
            .push(QueryOperation::SurvivalOnly);
    }

    fn add_color_filter(&mut self) {
        self.current_query
            .operations
            .push(QueryOperation::WithColor);
    }

    fn add_transparent_exclusion(&mut self) {
        self.current_query
            .operations
            .push(QueryOperation::ExcludeTransparent);
    }

    fn add_tile_entity_exclusion(&mut self) {
        self.current_query
            .operations
            .push(QueryOperation::ExcludeTileEntities);
    }

    fn open_property_input_modal(&mut self) {
        self.modal_type = ModalType::PropertyInput;
        self.modal_open = true;
        self.input_mode = InputMode::Modal;
        self.input_buffer.clear();
    }

    fn open_color_input_modal(&mut self) {
        self.modal_type = ModalType::ColorInput;
        self.modal_open = true;
        self.input_mode = InputMode::Modal;
        self.input_buffer.clear();
    }

    fn open_pattern_input_modal(&mut self) {
        self.modal_type = ModalType::PatternInput;
        self.modal_open = true;
        self.input_mode = InputMode::Modal;
        self.input_buffer.clear();
    }

    fn open_limit_input_modal(&mut self) {
        self.modal_type = ModalType::LimitInput;
        self.modal_open = true;
        self.input_mode = InputMode::Modal;
        self.input_buffer.clear();
    }

    fn open_gradient_config_modal(&mut self) {
        self.modal_type = ModalType::GradientConfig;
        self.modal_open = true;
        self.input_mode = InputMode::Modal;
        self.input_buffer.clear();
    }

    fn open_save_query_modal(&mut self) {
        self.modal_type = ModalType::SaveQuery;
        self.modal_open = true;
        self.input_mode = InputMode::Modal;
        self.input_buffer.clear();
    }

    fn confirm_modal_input(&mut self) {
        match self.modal_type {
            ModalType::PropertyInput => {
                let parts: Vec<&str> = self.input_buffer.split(':').collect();
                if parts.len() == 2 {
                    self.current_query
                        .operations
                        .push(QueryOperation::PropertyFilter {
                            property: parts[0].trim().to_string(),
                            value: parts[1].trim().to_string(),
                        });
                }
            }
            ModalType::ColorInput => {
                if self.input_buffer.starts_with('#') && self.input_buffer.len() == 7 {
                    self.current_query
                        .operations
                        .push(QueryOperation::ColorSimilarity {
                            hex: self.input_buffer.clone(),
                            tolerance: 30.0, // Default tolerance
                        });
                }
            }
            ModalType::PatternInput => {
                if !self.input_buffer.is_empty() {
                    self.current_query
                        .operations
                        .push(QueryOperation::PatternMatch(self.input_buffer.clone()));
                }
            }
            ModalType::LimitInput => {
                if let Ok(limit) = self.input_buffer.parse::<usize>() {
                    self.current_query
                        .operations
                        .push(QueryOperation::Limit(limit));
                }
            }
            ModalType::GradientConfig => {
                if let Ok(steps) = self.input_buffer.parse::<usize>() {
                    self.current_query
                        .operations
                        .push(QueryOperation::GenerateGradient { steps });
                }
            }
            ModalType::SaveQuery => {
                // TODO: Implement query saving
            }
        }

        self.modal_open = false;
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
    }

    fn execute_query(&mut self) {
        let mut query = AllBlocks::new();

        for operation in &self.current_query.operations {
            match operation {
                QueryOperation::OnlySolid => {
                    query = query.only_solid();
                }
                QueryOperation::SurvivalOnly => {
                    query = query.survival_only();
                }
                QueryOperation::WithColor => {
                    query = query.with_color();
                }
                QueryOperation::ExcludeTransparent => {
                    query = query.exclude_transparent();
                }
                QueryOperation::ExcludeTileEntities => {
                    query = query.exclude_tile_entities();
                }
                QueryOperation::PropertyFilter { property, value } => {
                    query = query.with_property_value(property, value);
                }
                QueryOperation::ColorSimilarity { hex, tolerance } => {
                    if let Ok(target_r) = u8::from_str_radix(&hex[1..3], 16) {
                        if let Ok(target_g) = u8::from_str_radix(&hex[3..5], 16) {
                            if let Ok(target_b) = u8::from_str_radix(&hex[5..7], 16) {
                                let target_color =
                                    ExtendedColorData::from_rgb(target_r, target_g, target_b);
                                query = query.similar_to_color(target_color, *tolerance);
                            }
                        }
                    }
                }
                QueryOperation::PatternMatch(pattern) => {
                    query = query.matching(pattern);
                }
                QueryOperation::Limit(limit) => {
                    query = query.limit(*limit);
                }
                QueryOperation::GenerateGradient { steps } => {
                    let config = GradientConfig::new(*steps)
                        .with_color_space(ColorSpace::Oklab)
                        .with_easing(EasingFunction::Linear);
                    query = query.generate_gradient(config);
                }
            }
        }

        self.query_results = query.collect();
        self.selected_result_index = 0;
        self.current_tab = Tab::Results;
    }

    fn reset_query(&mut self) {
        self.current_query.operations.clear();
        self.query_results.clear();
        self.selected_result_index = 0;
    }

    fn load_example(&mut self) {
        let examples = get_example_queries();
        if let Some(example) = examples.get(self.selected_example_index) {
            self.current_query.operations = example.operations.clone();
            self.current_tab = Tab::QueryBuilder;
        }
    }
}

fn format_current_query(app: &App) -> Vec<String> {
    if app.current_query.operations.is_empty() {
        return vec![
            "No operations added yet.".to_string(),
            "".to_string(),
            "Use number keys 1-0 to add filters and operations:".to_string(),
            "• 1: Only solid blocks".to_string(),
            "• 2: Survival obtainable only".to_string(),
            "• 3: Blocks with color data".to_string(),
            "• 4: Exclude transparent blocks".to_string(),
            "• 5: Exclude tile entities".to_string(),
            "• 6: Filter by property".to_string(),
            "• 7: Filter by color".to_string(),
            "• 8: Filter by name pattern".to_string(),
            "• 9: Limit results".to_string(),
            "• 0: Generate gradient".to_string(),
        ];
    }

    let mut result = vec!["Current query chain:".to_string(), "".to_string()];

    for (i, operation) in app.current_query.operations.iter().enumerate() {
        let description = match operation {
            QueryOperation::OnlySolid => "🏗️ Only solid blocks".to_string(),
            QueryOperation::SurvivalOnly => "🌱 Survival obtainable only".to_string(),
            QueryOperation::WithColor => "🎨 Blocks with color data".to_string(),
            QueryOperation::ExcludeTransparent => "🚫 Exclude transparent blocks".to_string(),
            QueryOperation::ExcludeTileEntities => "🔧 Exclude tile entities".to_string(),
            QueryOperation::PropertyFilter { property, value } => {
                format!("📝 Property: {} = {}", property, value)
            }
            QueryOperation::ColorSimilarity { hex, tolerance } => {
                format!("🎯 Color similar to {} (±{:.0})", hex, tolerance)
            }
            QueryOperation::PatternMatch(pattern) => {
                format!("🔤 Pattern: {}", pattern)
            }
            QueryOperation::Limit(limit) => {
                format!("📊 Limit: {} results", limit)
            }
            QueryOperation::GenerateGradient { steps } => {
                format!("⚡ Generate gradient: {} steps", steps)
            }
        };

        result.push(format!("{}. {}", i + 1, description));
    }

    result.push("".to_string());
    result.push("Press [Enter] to execute this query.".to_string());
    result.push("Press [r] to reset the query.".to_string());

    result
}

struct ExampleQuery {
    name: String,
    description: String,
    operations: Vec<QueryOperation>,
    steps: Vec<String>,
    use_case: String,
}

fn get_example_queries() -> Vec<ExampleQuery> {
    vec![
        ExampleQuery {
            name: "Solid Building Blocks".to_string(),
            description: "Find blocks suitable for construction".to_string(),
            operations: vec![
                QueryOperation::OnlySolid,
                QueryOperation::SurvivalOnly,
                QueryOperation::WithColor,
                QueryOperation::ExcludeTransparent,
                QueryOperation::Limit(20),
            ],
            steps: vec![
                "Filter to only solid blocks".to_string(),
                "Keep only survival-obtainable blocks".to_string(),
                "Include only blocks with color data".to_string(),
                "Exclude transparent blocks".to_string(),
                "Limit to 20 results".to_string(),
            ],
            use_case: "Perfect for finding blocks to use in large building projects where you need solid, opaque materials with known colors.".to_string(),
        },
        ExampleQuery {
            name: "Stone Color Palette".to_string(),
            description: "Create a color palette around stone-like blocks".to_string(),
            operations: vec![
                QueryOperation::PatternMatch("*stone*".to_string()),
                QueryOperation::WithColor,
                QueryOperation::GenerateGradient { steps: 8 },
            ],
            steps: vec![
                "Find all blocks with 'stone' in the name".to_string(),
                "Keep only those with color data".to_string(),
                "Generate an 8-step gradient between them".to_string(),
            ],
            use_case: "Great for creating natural stone gradients for architecture or terrain builds.".to_string(),
        },
        ExampleQuery {
            name: "Red Blocks Search".to_string(),
            description: "Find blocks similar to red color".to_string(),
            operations: vec![
                QueryOperation::ColorSimilarity {
                    hex: "#FF0000".to_string(), 
                    tolerance: 50.0
                },
                QueryOperation::SurvivalOnly,
                QueryOperation::Limit(15),
            ],
            steps: vec![
                "Search for blocks similar to red (#FF0000)".to_string(),
                "Keep only survival-obtainable blocks".to_string(),
                "Limit to 15 results".to_string(),
            ],
            use_case: "Useful for finding all red-ish blocks available in survival mode for themed builds.".to_string(),
        },
        ExampleQuery {
            name: "Redstone Components".to_string(),
            description: "Find all redstone-related blocks".to_string(),
            operations: vec![
                QueryOperation::PatternMatch("*redstone*".to_string()),
                QueryOperation::SurvivalOnly,
            ],
            steps: vec![
                "Search for blocks containing 'redstone'".to_string(),
                "Keep only survival-obtainable blocks".to_string(),
            ],
            use_case: "Perfect for redstone engineers who want to see all available redstone components.".to_string(),
        },
        ExampleQuery {
            name: "Advanced Gradient".to_string(),
            description: "Complex query with gradient generation".to_string(),
            operations: vec![
                QueryOperation::OnlySolid,
                QueryOperation::WithColor,
                QueryOperation::ExcludeTransparent,
                QueryOperation::GenerateGradient { steps: 12 },
                QueryOperation::Limit(10),
            ],
            steps: vec![
                "Start with solid blocks only".to_string(),
                "Require color data".to_string(),
                "Exclude transparent blocks".to_string(),
                "Generate a 12-step gradient".to_string(),
                "Limit final result to 10 blocks".to_string(),
            ],
            use_case: "Demonstrates bidirectional chaining: filter → gradient → filter. Perfect for creating sophisticated color palettes.".to_string(),
        },
    ]
}
