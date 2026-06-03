use crate::discovery::{BonsaiApp, AppCategory, discover_apps};
use crate::error::{MenuError, Result};
use std::collections::HashMap;
use std::process::Command;

pub struct AppMenu {
    apps: Vec<BonsaiApp>,
}

impl AppMenu {
    pub fn new() -> Result<Self> {
        let apps = discover_apps()?;
        Ok(Self { apps })
    }

    pub fn refresh(&mut self) -> Result<()> {
        self.apps = discover_apps()?;
        Ok(())
    }

    pub fn all_apps(&self) -> &[BonsaiApp] {
        &self.apps
    }

    pub fn installed_apps(&self) -> Vec<&BonsaiApp> {
        self.apps.iter()
            .filter(|app| app.is_installed)
            .collect()
    }

    pub fn running_apps(&self) -> Vec<&BonsaiApp> {
        self.apps.iter()
            .filter(|app| app.is_running)
            .collect()
    }

    pub fn apps_by_category(&self, category: AppCategory) -> Vec<&BonsaiApp> {
        self.apps.iter()
            .filter(|app| app.category == category)
            .collect()
    }

    pub fn grouped(&self) -> HashMap<AppCategory, Vec<&BonsaiApp>> {
        let mut grouped: HashMap<AppCategory, Vec<&BonsaiApp>> = HashMap::new();

        for app in &self.apps {
            grouped.entry(app.category).or_insert_with(Vec::new).push(app);
        }

        for apps in grouped.values_mut() {
            apps.sort_by(|a, b| a.name.cmp(&b.name));
        }

        grouped
    }

    pub fn search(&self, query: &str) -> Vec<&BonsaiApp> {
        let query = query.to_lowercase();
        self.apps.iter()
            .filter(|app| {
                app.name.to_lowercase().contains(&query)
                    || app.description.to_lowercase().contains(&query)
                    || app.id.to_lowercase().contains(&query)
            })
            .collect()
    }

    pub fn launch(&self, app_id: &str) -> Result<()> {
        let app = self.apps.iter()
            .find(|a| a.id == app_id)
            .ok_or_else(|| MenuError::AppNotFound(app_id.to_string()))?;

        if let Some(ref cmd) = app.launch_command {
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            if parts.is_empty() {
                return Err(MenuError::LaunchFailed("Empty command".into()));
            }

            match Command::new(parts[0])
                .args(&parts[1..])
                .spawn()
            {
                Ok(mut child) => {
                    // Don't wait; let the app run independently
                    let _ = std::thread::spawn(move || {
                        let _ = child.wait();
                    });
                    Ok(())
                }
                Err(e) => Err(MenuError::LaunchFailed(e.to_string()))
            }
        } else {
            Err(MenuError::LaunchFailed(
                format!("No launch command for '{}'", app_id)
            ))
        }
    }

    pub fn get_app(&self, app_id: &str) -> Option<&BonsaiApp> {
        self.apps.iter().find(|a| a.id == app_id)
    }

    pub fn html_grid(&self) -> String {
        let mut html = String::from(r#"
        <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); gap: 16px; padding: 20px;">
        "#);

        for app in &self.apps {
            html.push_str(&format!(
                r#"
                <div style="
                    background: linear-gradient(135deg, {color}30, {color}10);
                    border-left: 4px solid {color};
                    border-radius: 12px;
                    padding: 16px;
                    cursor: pointer;
                    transition: transform 0.2s;
                    text-align: center;
                " onclick="launch_app('{}')">
                    <div style="font-size: 48px; margin-bottom: 8px;">{}</div>
                    <div style="font-weight: 600; color: #fff; margin-bottom: 4px;">{}</div>
                    <div style="font-size: 12px; color: #aaa;">{}</div>
                    <div style="margin-top: 8px; font-size: 11px; color: {};">{}</div>
                </div>
                "#,
                app.id,
                app.icon,
                app.name,
                app.description,
                app.category.color_hex(),
                if app.is_running { "🟢 Running" } else { "⚫ Stopped" }
            ));
        }

        html.push_str("</div>");
        html
    }
}

impl Default for AppMenu {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            apps: Vec::new()
        })
    }
}
