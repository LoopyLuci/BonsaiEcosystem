use bonsai_app_menu::{AppMenu, AppCategory};
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    println!("\n🪴 Bonsai Universal App Menu\n");

    let mut menu = AppMenu::new()?;
    println!("Discovered {} apps\n", menu.all_apps().len());

    loop {
        print!("\n> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        match input {
            "" => continue,
            "quit" | "exit" => break,
            "refresh" => {
                menu.refresh()?;
                println!("✓ App list refreshed");
            }
            "list" => {
                for app in menu.installed_apps() {
                    let status = if app.is_running { "🟢" } else { "⚫" };
                    println!("  {} {} - {}", status, app.name, app.description);
                }
            }
            cmd if cmd.starts_with("search ") => {
                let query = cmd.strip_prefix("search ").unwrap_or("");
                let results = menu.search(query);
                if results.is_empty() {
                    println!("No apps found matching '{}'", query);
                } else {
                    for app in results {
                        println!("  {} {}", app.icon, app.name);
                    }
                }
            }
            cmd if cmd.starts_with("launch ") => {
                let app_id = cmd.strip_prefix("launch ").unwrap_or("");
                match menu.launch(app_id) {
                    Ok(_) => println!("✓ Launching {}", app_id),
                    Err(e) => println!("✗ Error: {}", e),
                }
            }
            cmd if cmd.starts_with("info ") => {
                let app_id = cmd.strip_prefix("info ").unwrap_or("");
                if let Some(app) = menu.get_app(app_id) {
                    println!("\n📦 {}", app.name);
                    println!("   Description: {}", app.description);
                    println!("   Category: {:?}", app.category);
                    println!("   Status: {}", if app.is_running { "Running 🟢" } else { "Stopped ⚫" });
                    if let Some(port) = app.port {
                        println!("   Port: {}", port);
                    }
                    println!("   Version: {}", app.version);
                } else {
                    println!("App '{}' not found", app_id);
                }
            }
            "categories" => {
                let grouped = menu.grouped();
                for category in [
                    AppCategory::AI,
                    AppCategory::Infrastructure,
                    AppCategory::Media,
                    AppCategory::Development,
                    AppCategory::Knowledge,
                    AppCategory::Security,
                    AppCategory::Communication,
                    AppCategory::Utility,
                ] {
                    if let Some(apps) = grouped.get(&category) {
                        println!("\n{:?} ({})", category, apps.len());
                        for app in apps {
                            println!("  {} {}", app.icon, app.name);
                        }
                    }
                }
            }
            "help" => {
                println!(r#"
Commands:
  list              - List all installed apps
  refresh           - Refresh the app list
  search <query>    - Search for apps by name
  launch <app-id>   - Launch an app
  info <app-id>     - Show app details
  categories        - List apps by category
  help              - Show this help
  quit              - Exit
"#);
            }
            _ => {
                println!("Unknown command: '{}'. Type 'help' for commands.", input);
            }
        }
    }

    println!("\n👋 Goodbye!\n");
    Ok(())
}
