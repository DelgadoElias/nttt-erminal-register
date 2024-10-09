use clap::{Parser, Subcommand};
use crossterm::{event, execute, terminal, ExecutableCommand};
use once_cell::sync::Lazy;
use ratatui::{backend::CrosstermBackend, layout::*, style::*, widgets::*, Terminal};
use std::collections::HashMap;
use std::sync::Mutex;

/// Comandos CLI de la aplicación
#[derive(Parser)]
#[command(name = "nttt", version = "0.1")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Registra un nuevo proyecto con un nombre y un puerto
    Register { name: String, port: u16 },

    /// Inicia el modo TUI
    Start,
}

// Cache para almacenar proyectos registrados
static CACHE: Lazy<Mutex<HashMap<String, u16>>> = Lazy::new(|| Mutex::new(HashMap::new()));

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Register { name, port } => {
            let mut cache = CACHE.lock().unwrap();
            cache.insert(name.clone(), *port);
            println!("Registrado: {} en el puerto {}", name, port);
        }
        Commands::Start => start_tui()?,
    }

    Ok(())
}

fn start_tui() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
    let mut selected = 0;

    loop {
        terminal.draw(|f| ui(f, selected))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    event::KeyCode::Esc => break,
                    event::KeyCode::Down => {
                        selected = (selected + 1) % CACHE.lock().unwrap().len();
                    }
                    event::KeyCode::Up => {
                        selected = if selected == 0 {
                            CACHE.lock().unwrap().len() - 1
                        } else {
                            selected - 1
                        };
                    }
                    event::KeyCode::Char('d') => {
                        let mut cache = CACHE.lock().unwrap();
                        let name_to_remove = cache.keys().nth(selected).cloned();
                        if let Some(name) = name_to_remove {
                            cache.remove(&name);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn ui(f: &mut ratatui::Frame, selected: usize) {
    let size = f.size(); // Utiliza el método area() para obtener el tamaño del marco
    let cache = CACHE.lock().unwrap();
    
    let items: Vec<ListItem> = cache
        .iter()
        .map(|(name, port)| ListItem::new(format!("{} - Puerto {}", name, port)))
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Proyectos"))
        .highlight_style(Style::default().bg(Color::Blue))
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    list_state.select(Some(selected));

    // Renderiza el widget con el estado adecuado
    f.render_stateful_widget(list, size, &mut list_state);
}

