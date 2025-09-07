use crate::ui::Colors;

/// Represents a rank with associated metadata
#[derive(Debug, Clone, PartialEq)]
pub struct Rank {
    pub name: String,
    pub tier: RankTier,
    pub min_score: u32,
    pub max_score: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RankTier {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
    Legendary,
}

impl RankTier {
    /// Get the color palette name for ASCII art generation
    pub fn color_palette(&self) -> &'static str {
        match self {
            RankTier::Beginner => "grad-blue",
            RankTier::Intermediate => "dawn",
            RankTier::Advanced => "forest",
            RankTier::Expert => "gold",
            RankTier::Legendary => "fire",
        }
    }

    /// Get the terminal color for this tier
    pub fn terminal_color(&self) -> crossterm::style::Color {
        match self {
            RankTier::Beginner => Colors::to_crossterm(Colors::INFO),
            RankTier::Intermediate => Colors::to_crossterm(Colors::BORDER),
            RankTier::Advanced => Colors::to_crossterm(Colors::CPM_WPM),
            RankTier::Expert => Colors::to_crossterm(Colors::ACCURACY),
            RankTier::Legendary => Colors::to_crossterm(Colors::ERROR),
        }
    }
}

impl Rank {
    /// Create a new rank
    pub fn new(name: impl Into<String>, tier: RankTier, min_score: u32, max_score: u32) -> Self {
        Self {
            name: name.into(),
            tier,
            min_score,
            max_score,
        }
    }

    /// Get the display name of the rank
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the tier of the rank
    #[allow(dead_code)]
    pub fn tier(&self) -> &RankTier {
        &self.tier
    }

    /// Check if a score falls within this rank's range
    #[allow(dead_code)]
    pub fn contains_score(&self, score: f64) -> bool {
        let score = score as u32;
        score >= self.min_score && score <= self.max_score
    }

    /// Get the color palette name for ASCII art generation
    pub fn color_palette(&self) -> &'static str {
        self.tier.color_palette()
    }

    /// Get the terminal color for this rank
    pub fn terminal_color(&self) -> crossterm::style::Color {
        self.tier.terminal_color()
    }

    /// Get all ranks in order from lowest to highest score
    pub fn all_ranks() -> Vec<Rank> {
        vec![
            // Beginner Level (clean boundaries, ~even progression)
            Rank::new("Hello World", RankTier::Beginner, 0, 800),
            Rank::new("Syntax Error", RankTier::Beginner, 801, 1200),
            Rank::new("Rubber Duck", RankTier::Beginner, 1201, 1600),
            Rank::new("Script Kid", RankTier::Beginner, 1601, 2000),
            Rank::new("Bash Newbie", RankTier::Beginner, 2001, 2450),
            Rank::new("CLI Wanderer", RankTier::Beginner, 2451, 2900),
            Rank::new("Tab Tamer", RankTier::Beginner, 2901, 3300),
            Rank::new("Bracket Juggler", RankTier::Beginner, 3301, 3700),
            Rank::new("Copy-Paste Engineer", RankTier::Beginner, 3701, 4150),
            Rank::new("Linter Apprentice", RankTier::Beginner, 4151, 4550),
            Rank::new("Unit Test Trainee", RankTier::Beginner, 4551, 5000),
            Rank::new("Code Monkey", RankTier::Beginner, 5001, 5600),
            // Intermediate Level
            Rank::new("Ticket Picker", RankTier::Intermediate, 5601, 5850),
            Rank::new("Junior Dev", RankTier::Intermediate, 5851, 6000),
            Rank::new("Git Ninja", RankTier::Intermediate, 6001, 6100),
            Rank::new("Merge Wrangler", RankTier::Intermediate, 6101, 6250),
            Rank::new("API Crafter", RankTier::Intermediate, 6251, 6400),
            Rank::new("Frontend Dev", RankTier::Intermediate, 6401, 6550),
            Rank::new("Backend Dev", RankTier::Intermediate, 6551, 6700),
            Rank::new("CI Tinkerer", RankTier::Intermediate, 6701, 6850),
            Rank::new("Test Pilot", RankTier::Intermediate, 6851, 7000),
            Rank::new("Build Tamer", RankTier::Intermediate, 7001, 7100),
            Rank::new("Code Reviewer", RankTier::Intermediate, 7101, 7250),
            Rank::new("Release Handler", RankTier::Intermediate, 7251, 7500),
            // Advanced Level
            Rank::new("Refactorer", RankTier::Advanced, 7501, 7800),
            Rank::new("Senior Dev", RankTier::Advanced, 7801, 8000),
            Rank::new("DevOps Engineer", RankTier::Advanced, 8001, 8100),
            Rank::new("Incident Responder", RankTier::Advanced, 8101, 8250),
            Rank::new("Reliability Guardian", RankTier::Advanced, 8251, 8400),
            Rank::new("Security Engineer", RankTier::Advanced, 8401, 8500),
            Rank::new("Performance Alchemist", RankTier::Advanced, 8501, 8650),
            Rank::new("Data Pipeline Master", RankTier::Advanced, 8651, 8800),
            Rank::new("Tech Lead", RankTier::Advanced, 8801, 8950),
            Rank::new("Architect", RankTier::Advanced, 8951, 9100),
            Rank::new("Protocol Artisan", RankTier::Advanced, 9101, 9250),
            Rank::new("Kernel Hacker", RankTier::Advanced, 9251, 9500),
            // Expert Level
            Rank::new("Compiler", RankTier::Expert, 9501, 9800),
            Rank::new("Bytecode Interpreter", RankTier::Expert, 9801, 9950),
            Rank::new("Virtual Machine", RankTier::Expert, 9951, 10100),
            Rank::new("Operating System", RankTier::Expert, 10101, 10200),
            Rank::new("Filesystem", RankTier::Expert, 10201, 10350),
            Rank::new("Network Stack", RankTier::Expert, 10351, 10500),
            Rank::new("Database Engine", RankTier::Expert, 10501, 10650),
            Rank::new("Query Optimizer", RankTier::Expert, 10651, 10800),
            Rank::new("Cloud Platform", RankTier::Expert, 10801, 10950),
            Rank::new("Container Orchestrator", RankTier::Expert, 10951, 11100),
            Rank::new("Stream Processor", RankTier::Expert, 11101, 11200),
            Rank::new("Quantum Computer", RankTier::Expert, 11201, 11400),
            // Legendary Level
            Rank::new("GPU Cluster", RankTier::Legendary, 11401, 11700),
            Rank::new("DNS Overlord", RankTier::Legendary, 11701, 12250),
            Rank::new("CDN Sentinel", RankTier::Legendary, 12251, 12800),
            Rank::new("Load Balancer Primarch", RankTier::Legendary, 12801, 13400),
            Rank::new("Singularity", RankTier::Legendary, 13401, 13950),
            Rank::new("The Machine", RankTier::Legendary, 13951, 14500),
            Rank::new("Origin", RankTier::Legendary, 14501, 15100),
            Rank::new("SegFault", RankTier::Legendary, 15101, 15650),
            Rank::new("Buffer Overflow", RankTier::Legendary, 15651, 16200),
            Rank::new("Memory Leak", RankTier::Legendary, 16201, 16800),
            Rank::new("Null Pointer Exception", RankTier::Legendary, 16801, 17350),
            Rank::new("Undefined Behavior", RankTier::Legendary, 17351, 17900),
            Rank::new("Heisenbug", RankTier::Legendary, 17901, 18500),
            Rank::new("Blue Screen", RankTier::Legendary, 18501, 19100),
            Rank::new("Kernel Panic", RankTier::Legendary, 19101, u32::MAX),
        ]
    }

    /// Find the rank for a given score
    #[allow(dead_code)]
    pub fn for_score(score: f64) -> Rank {
        Self::all_ranks()
            .into_iter()
            .find(|rank| rank.contains_score(score))
            .unwrap_or_else(|| {
                // Fallback to highest rank if score exceeds all ranges
                Rank::new("Kernel Panic", RankTier::Legendary, 40001, u32::MAX)
            })
    }
}
