use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "haifu", about = "OTA update server")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the OTA update server
    Serve,
}

/// An OTA release entry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OtaRelease {
    pub device: String,
    pub build_number: String,
    pub channel: String,
    pub url: String,
    pub size: u64,
}

/// Index of OTA releases, supporting multi-device and multi-channel lookups.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReleaseIndex {
    pub releases: Vec<OtaRelease>,
}

impl ReleaseIndex {
    /// Create a new empty release index.
    #[must_use]
    pub fn new() -> Self {
        Self {
            releases: Vec::new(),
        }
    }

    /// Add a release. If a release with the same device, channel, and
    /// build number already exists, it is replaced.
    pub fn add(&mut self, release: OtaRelease) {
        if let Some(existing) = self.releases.iter_mut().find(|r| {
            r.device == release.device
                && r.channel == release.channel
                && r.build_number == release.build_number
        }) {
            *existing = release;
        } else {
            self.releases.push(release);
        }
    }

    /// Get the latest release for a device (highest build number across all channels).
    #[must_use]
    pub fn latest_for(&self, device: &str) -> Option<&OtaRelease> {
        self.releases
            .iter()
            .filter(|r| r.device == device)
            .max_by(|a, b| a.build_number.cmp(&b.build_number))
    }

    /// Get the distinct set of channels across all releases.
    #[must_use]
    pub fn channels(&self) -> Vec<String> {
        let mut channels: Vec<String> = self
            .releases
            .iter()
            .map(|r| r.channel.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        channels.sort();
        channels
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Serve => {
            println!("haifu: starting OTA server");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_release(device: &str, build: &str, channel: &str) -> OtaRelease {
        OtaRelease {
            device: device.to_string(),
            build_number: build.to_string(),
            channel: channel.to_string(),
            url: format!("https://ota.example.com/{device}/{build}"),
            size: 1024,
        }
    }

    #[test]
    fn add_release() {
        let mut index = ReleaseIndex::new();
        index.add(test_release("husky", "TQ3A.230901.001", "stable"));
        assert_eq!(index.releases.len(), 1);
        assert_eq!(index.releases[0].device, "husky");
    }

    #[test]
    fn latest_for_device() {
        let mut index = ReleaseIndex::new();
        index.add(test_release("husky", "TQ3A.230801.001", "stable"));
        index.add(test_release("husky", "TQ3A.230901.001", "stable"));
        index.add(test_release("shiba", "TQ3A.230901.001", "stable"));

        let latest = index.latest_for("husky").unwrap();
        assert_eq!(latest.build_number, "TQ3A.230901.001");
    }

    #[test]
    fn empty_index() {
        let index = ReleaseIndex::new();
        assert!(index.latest_for("husky").is_none());
        assert!(index.channels().is_empty());
    }

    #[test]
    fn multiple_channels() {
        let mut index = ReleaseIndex::new();
        index.add(test_release("husky", "001", "stable"));
        index.add(test_release("husky", "002", "beta"));
        index.add(test_release("shiba", "001", "canary"));

        let channels = index.channels();
        assert_eq!(channels, vec!["beta", "canary", "stable"]);
    }

    #[test]
    fn add_duplicate_replaces() {
        let mut index = ReleaseIndex::new();
        index.add(OtaRelease {
            device: "husky".to_string(),
            build_number: "001".to_string(),
            channel: "stable".to_string(),
            url: "https://old.example.com".to_string(),
            size: 100,
        });
        index.add(OtaRelease {
            device: "husky".to_string(),
            build_number: "001".to_string(),
            channel: "stable".to_string(),
            url: "https://new.example.com".to_string(),
            size: 200,
        });
        assert_eq!(index.releases.len(), 1);
        assert_eq!(index.releases[0].url, "https://new.example.com");
        assert_eq!(index.releases[0].size, 200);
    }
}
