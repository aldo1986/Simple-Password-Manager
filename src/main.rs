use eframe::egui;
use std::error::Error;

mod model;
mod crypto;
mod storage;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
     eframe::run_native(
        "Vault GUI",
        options,
        Box::new(|_cc| -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(Box::new(MyApp::default()))
        }),
    )

}

struct MyApp {
    password_entries: Vec<model::PasswordEntry>,
    master_password: String,
    new_entry: model::PasswordEntry,
    message: String,
    unlocked: bool,
    entry_to_delete: Option<usize>,
    show_confirm_delete: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            password_entries: Vec::new(),
            master_password: String::new(),
            new_entry: model::PasswordEntry {
                service: String::new(),
                username: String::new(),
                password: crypto::generate_password(32),
            },
            message: String::new(),
            unlocked: false,
            entry_to_delete: None,
            show_confirm_delete: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.unlocked {
                ui.heading("Ingresa tu contraseña maestra");
                ui.add(egui::TextEdit::singleline(&mut self.master_password).password(true));
                if ui.button("Desbloquear").clicked() {
                    self.password_entries = storage::load_entries(&self.master_password);
                    self.unlocked = true;
                }
            } else {
                ui.heading("Vault");
                ui.vertical(|ui| {
                    ui.label("Servicio:");
                    ui.text_edit_singleline(&mut self.new_entry.service);
                    ui.label("Usuario:");
                    ui.text_edit_singleline(&mut self.new_entry.username);
                    ui.label("Contraseña:");
                    ui.text_edit_singleline(&mut self.new_entry.password);
                    if ui.button("Generar contraseña").clicked() {
                        self.new_entry.password = crypto::generate_password(32); // o el largo que prefieras
                        }
                });

                if ui.button("Agregar entrada").clicked() {
                    self.password_entries.push(self.new_entry.clone());
                    storage::save_entries(&self.password_entries, &self.master_password);
                    self.new_entry = model::PasswordEntry::default();
                }

                ui.separator();
                for (i, entry) in self.password_entries.iter().enumerate() {
                    let index = i;
                    ui.group(|ui| {
                        ui.label(format!("Servicio: {}", entry.service));
                        ui.label(format!("Usuario: {}", entry.username));
                        ui.horizontal(|ui| {
                            ui.label("Contraseña:");
                            if ui.button("Mostrar").clicked() {
                                self.message = entry.password.clone();
                            }
                            if ui.button("Copiar").clicked() {
                                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                                    let _ = clipboard.set_text(entry.password.clone());
                                }
                            }
                            if ui.button("Eliminar").clicked() {
                                self.entry_to_delete = Some(index);
                                self.show_confirm_delete = true;
                            }
                        });
                    });
                    
                }
                if self.show_confirm_delete {
    egui::Window::new("Confirmar eliminación")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.label("¿Estás seguro de que deseas eliminar esta entrada?");
            ui.horizontal(|ui| {
                if ui.button("Sí, eliminar").clicked() {
                    if let Some(index) = self.entry_to_delete {
                        self.password_entries.remove(index);
                        storage::save_entries(&self.password_entries, &self.master_password);
                    }
                    self.entry_to_delete = None;
                    self.show_confirm_delete = false;
                }
                if ui.button("Cancelar").clicked() {
                    self.entry_to_delete = None;
                    self.show_confirm_delete = false;
                }
            });
        });
}

                if !self.message.is_empty() {
                    ui.label(format!("→ {}", self.message));
                }

            }
        });
    }
}