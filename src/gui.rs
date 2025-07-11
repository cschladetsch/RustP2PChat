use crate::{config::Config, ChatError, P2PChat, Result};
use eframe::egui;
use egui::{Context, RichText, Ui, Response, Sense};
use std::collections::VecDeque;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;
use tokio::runtime::Runtime;
use tracing::{error, info};
use std::path::PathBuf;

// Helper function to load application icon
fn load_icon() -> egui::IconData {
    // Default icon - a simple chat bubble
    egui::IconData {
        rgba: vec![255; 32 * 32 * 4], // White 32x32 icon
        width: 32,
        height: 32,
    }
}

pub struct ChatMessage {
    pub text: String,
    pub sender: String,
    pub timestamp: SystemTime,
    pub is_encrypted: bool,
    pub is_file: bool,
}

impl Default for ChatMessage {
    fn default() -> Self {
        Self {
            text: String::new(),
            sender: String::new(),
            timestamp: SystemTime::now(),
            is_encrypted: false,
            is_file: false,
        }
    }
}

pub struct P2PChatApp {
    config: Config,
    messages: Arc<Mutex<VecDeque<ChatMessage>>>,
    current_message: String,
    connection_status: ConnectionStatus,
    show_settings: bool,

    // Connection settings
    listen_port: String,
    peer_address: String,
    nickname: String,
    enable_encryption: bool,

    // Chat backend communication
    message_sender: Option<mpsc::Sender<String>>,
    runtime: Option<Arc<Runtime>>,

    // UI state
    auto_scroll: bool,
    show_timestamps: bool,
    
    // Drag and drop state
    dropped_files: Arc<Mutex<Vec<egui::DroppedFile>>>,
}

#[derive(PartialEq, Default)]
enum ConnectionStatus {
    #[default]
    Disconnected,
    Connecting,
    Connected(String),
    #[allow(dead_code)]
    Error(String),
}

impl Default for P2PChatApp {
    fn default() -> Self {
        let config = Config::default();
        Self {
            listen_port: config.default_port.to_string(),
            peer_address: String::new(),
            nickname: config.nickname.clone().unwrap_or_else(|| "You".to_string()),
            enable_encryption: config.enable_encryption,
            config,
            messages: Arc::new(Mutex::new(VecDeque::new())),
            current_message: String::new(),
            connection_status: ConnectionStatus::default(),
            show_settings: false,
            message_sender: None,
            runtime: None,
            auto_scroll: true,
            show_timestamps: true,
            dropped_files: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl P2PChatApp {
    pub fn new() -> Self {
        Self::default()
    }

    fn add_message(&mut self, text: String, sender: String, is_encrypted: bool, is_file: bool) {
        let message = ChatMessage {
            text,
            sender,
            timestamp: SystemTime::now(),
            is_encrypted,
            is_file,
        };

        if let Ok(mut messages) = self.messages.lock() {
            messages.push_back(message);
            // Keep only last 1000 messages
            if messages.len() > 1000 {
                messages.pop_front();
            }
        }
    }

    fn connect_to_peer(&mut self, peer_addr: Option<String>) {
        if self.runtime.is_some() {
            self.disconnect();
        }

        self.connection_status = ConnectionStatus::Connecting;

        let port: u16 = self.listen_port.parse().unwrap_or(8080);
        let config = self.config.clone();
        let messages = self.messages.clone();

        // Create message channel
        let (tx, _rx) = mpsc::channel();
        self.message_sender = Some(tx);

        // Create runtime and spawn chat task
        let runtime = Arc::new(Runtime::new().expect("Failed to create Tokio runtime"));
        let rt_clone = runtime.clone();
        self.runtime = Some(runtime);

        let peer_addr_clone = peer_addr.clone();
        thread::spawn(move || {
            rt_clone.block_on(async {
                let mut chat = match P2PChat::new(config) {
                    Ok(chat) => chat,
                    Err(e) => {
                        error!("Failed to create chat: {}", e);
                        return;
                    }
                };

                // Add system message
                if let Ok(mut msgs) = messages.lock() {
                    msgs.push_back(ChatMessage {
                        text: if let Some(ref addr) = peer_addr_clone {
                            format!("Connecting to peer at {}...", addr)
                        } else {
                            format!("Listening on port {}...", port)
                        },
                        sender: "System".to_string(),
                        timestamp: SystemTime::now(),
                        is_encrypted: false,
                        is_file: false,
                    });
                }

                // Start chat (this will block)
                if let Err(e) = chat.start(port, peer_addr_clone).await {
                    error!("Chat error: {}", e);
                    if let Ok(mut msgs) = messages.lock() {
                        msgs.push_back(ChatMessage {
                            text: format!("Connection failed: {}", e),
                            sender: "System".to_string(),
                            timestamp: SystemTime::now(),
                            is_encrypted: false,
                            is_file: false,
                        });
                    }
                }
            });
        });

        // Update status
        if let Some(ref addr) = peer_addr {
            self.connection_status = ConnectionStatus::Connected(addr.clone());
        } else {
            self.connection_status = ConnectionStatus::Connected(format!("Listening on {}", port));
        }
    }

    fn disconnect(&mut self) {
        self.message_sender = None;
        self.runtime = None;
        self.connection_status = ConnectionStatus::Disconnected;
        self.add_message(
            "Disconnected".to_string(),
            "System".to_string(),
            false,
            false,
        );
    }

    fn send_message(&mut self) {
        if !self.current_message.trim().is_empty() {
            let message = self.current_message.clone();

            // Add to our message list
            self.add_message(
                message.clone(),
                self.nickname.clone(),
                self.enable_encryption,
                false,
            );

            // Send through channel (in a real implementation, this would go to the chat backend)
            if let Some(ref sender) = self.message_sender {
                let _ = sender.send(message);
            }

            self.current_message.clear();
        }
    }
    
    fn send_file(&mut self, path: PathBuf) {
        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file")
            .to_string();
            
        // Add to message list
        self.add_message(
            format!("Sending file: {}", filename),
            self.nickname.clone(),
            self.enable_encryption,
            true,
        );
        
        // In a real implementation, this would send the file through the chat backend
        if let Some(ref sender) = self.message_sender {
            let _ = sender.send(format!("/send {}", path.display()));
        }
    }
    
    fn handle_dropped_files(&mut self, ctx: &Context) {
        // Check for newly dropped files
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                let mut dropped = self.dropped_files.lock().unwrap();
                dropped.extend(i.raw.dropped_files.clone());
            }
        });
        
        // Process dropped files
        let files_to_process: Vec<egui::DroppedFile> = {
            let mut dropped = self.dropped_files.lock().unwrap();
            dropped.drain(..).collect()
        };
        
        for file in files_to_process {
            if let Some(path) = &file.path {
                self.send_file(path.clone());
            }
        }
    }
}

impl eframe::App for P2PChatApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Handle dropped files
        self.handle_dropped_files(ctx);
        // Top menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Connection", |ui| {
                    if ui.button("Connect to Peer").clicked() {
                        if !self.peer_address.is_empty() {
                            self.connect_to_peer(Some(self.peer_address.clone()));
                        }
                        ui.close_menu();
                    }
                    if ui.button("Start Listening").clicked() {
                        self.connect_to_peer(None);
                        ui.close_menu();
                    }
                    if ui.button("Disconnect").clicked() {
                        self.disconnect();
                        ui.close_menu();
                    }
                });

                ui.menu_button("Settings", |ui| {
                    if ui.button("Open Settings").clicked() {
                        self.show_settings = true;
                        ui.close_menu();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Connection status indicator
                    let (status_text, status_color) = match &self.connection_status {
                        ConnectionStatus::Disconnected => ("Disconnected", egui::Color32::RED),
                        ConnectionStatus::Connecting => ("Connecting...", egui::Color32::YELLOW),
                        ConnectionStatus::Connected(addr) => (addr.as_str(), egui::Color32::GREEN),
                        ConnectionStatus::Error(err) => (err.as_str(), egui::Color32::RED),
                    };

                    ui.colored_label(status_color, status_text);
                });
            });
        });

        // Settings window
        let mut show_settings = self.show_settings;
        if show_settings {
            egui::Window::new("Settings")
                .open(&mut show_settings)
                .default_width(400.0)
                .show(ctx, |ui| {
                    self.show_settings_ui(ui);
                });
        }
        self.show_settings = show_settings;

        // Main chat area with drag and drop support
        let response = egui::CentralPanel::default()
            .show(ctx, |ui| {
            ui.vertical(|ui| {
                // Connection controls
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Listen Port:");
                        ui.text_edit_singleline(&mut self.listen_port);

                        ui.separator();

                        ui.label("Peer Address:");
                        ui.text_edit_singleline(&mut self.peer_address);

                        if ui.button("Connect").clicked() {
                            if !self.peer_address.is_empty() {
                                self.connect_to_peer(Some(self.peer_address.clone()));
                            } else {
                                self.connect_to_peer(None);
                            }
                        }
                    });
                });

                ui.separator();

                // Chat messages area
                let available_height = ui.available_height() - 60.0; // Reserve space for input
                egui::ScrollArea::vertical()
                    .max_height(available_height)
                    .auto_shrink([false, false])
                    .stick_to_bottom(self.auto_scroll)
                    .show(ui, |ui| {
                        if let Ok(messages) = self.messages.lock() {
                            for msg in messages.iter() {
                                self.show_message(ui, msg);
                            }
                        }
                    });

                ui.separator();

                // Message input area
                ui.horizontal(|ui| {
                    let response = ui.text_edit_singleline(&mut self.current_message);

                    // Handle Enter key
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.send_message();
                        response.request_focus();
                    }

                    if ui.button("Send").clicked() {
                        self.send_message();
                        response.request_focus();
                    }

                    // File attachment button
                    if ui.button("📎").on_hover_text("Attach file").clicked() {
                        // Open file dialog
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.send_file(path);
                        }
                    }

                    // Encryption indicator
                    let lock_symbol = if self.enable_encryption {
                        "🔒"
                    } else {
                        "🔓"
                    };
                    ui.label(lock_symbol);
                });
            });
            
            // Create an invisible drop target that covers the entire panel
            let response = ui.allocate_response(ui.available_size(), Sense::hover());
            response
        })
        .inner;
        
        // Visual feedback for drag and drop
        if response.hovered() && ctx.input(|i| !i.raw.dropped_files.is_empty()) {
            // Draw overlay when files are being dragged over
            let painter = ctx.layer_painter(egui::LayerId::new(
                egui::Order::Foreground,
                egui::Id::new("drag_drop_overlay"),
            ));
            let rect = response.rect;
            painter.rect_filled(
                rect,
                egui::Rounding::same(8.0),
                egui::Color32::from_rgba_premultiplied(0, 0, 0, 100),
            );
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Drop files here to send",
                egui::FontId::proportional(24.0),
                egui::Color32::WHITE,
            );
        }

        // Request repaint to keep UI responsive
        ctx.request_repaint();
    }
}

impl P2PChatApp {
    fn show_message(&self, ui: &mut Ui, msg: &ChatMessage) {
        ui.horizontal(|ui| {
            // Timestamp
            if self.show_timestamps {
                if let Ok(duration) = msg.timestamp.duration_since(std::time::UNIX_EPOCH) {
                    let datetime = chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0)
                        .unwrap_or_default();
                    ui.label(
                        RichText::new(format!("[{}]", datetime.format("%H:%M:%S")))
                            .color(egui::Color32::GRAY)
                            .size(10.0),
                    );
                }
            }

            // Sender name
            let sender_color = if msg.sender == "System" {
                egui::Color32::YELLOW
            } else if msg.sender == self.nickname {
                egui::Color32::GREEN
            } else {
                egui::Color32::LIGHT_BLUE
            };

            ui.label(
                RichText::new(format!("{}:", msg.sender))
                    .color(sender_color)
                    .strong(),
            );

            // Message content
            let mut message_text = RichText::new(&msg.text);

            if msg.is_file {
                message_text = message_text.color(egui::Color32::LIGHT_YELLOW);
                ui.label("📁");
            }

            ui.label(message_text);

            // Encryption indicator
            if msg.is_encrypted {
                ui.label(RichText::new("🔒").color(egui::Color32::GREEN));
            }
        });
    }

    fn show_settings_ui(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label("General Settings");

            ui.horizontal(|ui| {
                ui.label("Nickname:");
                ui.text_edit_singleline(&mut self.nickname);
            });

            ui.checkbox(&mut self.enable_encryption, "Enable encryption");
            ui.checkbox(&mut self.auto_scroll, "Auto-scroll messages");
            ui.checkbox(&mut self.show_timestamps, "Show timestamps");
        });

        ui.separator();

        ui.group(|ui| {
            ui.label("Connection Settings");

            ui.horizontal(|ui| {
                ui.label("Default Port:");
                ui.text_edit_singleline(&mut self.listen_port);
            });
        });

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Save Settings").clicked() {
                // Update config
                self.config.nickname = Some(self.nickname.clone());
                self.config.enable_encryption = self.enable_encryption;
                if let Ok(port) = self.listen_port.parse::<u16>() {
                    self.config.default_port = port;
                }

                // Save to file
                if let Err(e) = self.config.save() {
                    error!("Failed to save config: {}", e);
                } else {
                    info!("Settings saved successfully");
                }
            }

            if ui.button("Reset to Defaults").clicked() {
                self.config = Config::default();
                self.listen_port = self.config.default_port.to_string();
                self.nickname = self
                    .config
                    .nickname
                    .clone()
                    .unwrap_or_else(|| "You".to_string());
                self.enable_encryption = self.config.enable_encryption;
            }
        });
    }
}

pub fn run_gui() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0])
            .with_title("Rust P2P Chat - Drag & Drop Files")
            .with_drag_and_drop(true)
            .with_icon(load_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "Rust P2P Chat",
        options,
        Box::new(|_cc| Ok(Box::new(P2PChatApp::new()))),
    )
    .map_err(|e| {
        ChatError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        ))
    })?;

    Ok(())
}
