use crate::protocol::Command;
use crate::peer::PeerManager;
use crate::config::Config;
use crate::error::Result;

pub struct CommandHandler {
    config: Config,
}

impl CommandHandler {
    pub fn new(config: Config) -> Self {
        CommandHandler { config }
    }

    pub fn parse_command(input: &str) -> Option<Command> {
        if !input.starts_with('/') {
            return None;
        }

        let parts: Vec<&str> = input[1..].split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        match parts[0] {
            "quit" | "exit" => Some(Command::Quit),
            "help" | "?" => Some(Command::Help),
            "info" => Some(Command::Info),
            "peers" | "list" => Some(Command::ListPeers),
            "nick" | "nickname" => {
                if parts.len() > 1 {
                    Some(Command::SetNickname(parts[1..].join(" ")))
                } else {
                    None
                }
            }
            "send" | "file" => {
                if parts.len() > 1 {
                    Some(Command::SendFile(parts[1..].join(" ")))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub async fn handle_command(
        &mut self,
        command: Command,
        peer_manager: &PeerManager,
    ) -> Result<String> {
        match command {
            Command::Help => Ok(self.get_help_text()),
            Command::Info => Ok(self.get_info_text(peer_manager).await),
            Command::ListPeers => Ok(self.list_peers(peer_manager).await),
            Command::SetNickname(nick) => {
                self.config.nickname = Some(nick.clone());
                self.config.save()?;
                Ok(format!("Nickname set to: {}", nick))
            }
            Command::Quit => Ok("Goodbye!".to_string()),
            Command::SendFile(path) => Ok(format!("Preparing to send file: {}", path)),
        }
    }

    fn get_help_text(&self) -> String {
        r#"Available commands:
  /help, /?          - Show this help message
  /info              - Show connection information
  /peers, /list      - List connected peers
  /nick <name>       - Set your nickname
  /send <file>       - Send a file to peer(s)
  /quit, /exit       - Exit the chat

Type normally to send messages to all connected peers."#.to_string()
    }

    async fn get_info_text(&self, peer_manager: &PeerManager) -> String {
        let peer_count = peer_manager.peer_count().await;
        let nickname = self.config.nickname.as_deref().unwrap_or("Anonymous");
        
        format!(
            "Chat Information:
  Nickname: {}
  Connected peers: {}
  Encryption: {}
  Buffer size: {} bytes
  Max file size: {} MB",
            nickname,
            peer_count,
            if self.config.enable_encryption { "Enabled" } else { "Disabled" },
            self.config.buffer_size,
            self.config.max_file_size_mb
        )
    }

    async fn list_peers(&self, peer_manager: &PeerManager) -> String {
        let peers = peer_manager.list_peers().await;
        if peers.is_empty() {
            return "No peers connected.".to_string();
        }

        let mut result = format!("Connected peers ({}):\n", peers.len());
        for peer in peers {
            let nickname = peer.nickname.as_deref().unwrap_or("Anonymous");
            result.push_str(&format!("  - {} ({}) from {}\n", nickname, peer.id, peer.address));
        }
        result
    }
}