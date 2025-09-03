/// Represents a ranking title with associated metadata
#[derive(Debug, Clone, PartialEq)]
pub struct RankingTitle {
    pub name: String,
    pub tier: Rank,
    pub min_score: u32,
    pub max_score: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Rank {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
    Legendary,
}

impl Rank {
    /// Get the color palette name for ASCII art generation
    pub fn color_palette(&self) -> &'static str {
        match self {
            Rank::Beginner => "grad-blue",
            Rank::Intermediate => "dawn",
            Rank::Advanced => "forest",
            Rank::Expert => "gold",
            Rank::Legendary => "fire",
        }
    }

    /// Get the terminal color for this tier
    pub fn terminal_color(&self) -> crossterm::style::Color {
        use crossterm::style::Color;
        match self {
            Rank::Beginner => Color::Cyan,
            Rank::Intermediate => Color::Blue,
            Rank::Advanced => Color::Green,
            Rank::Expert => Color::Yellow,
            Rank::Legendary => Color::Red,
        }
    }
}

impl RankingTitle {
    /// Create a new ranking title
    pub fn new(name: impl Into<String>, tier: Rank, min_score: u32, max_score: u32) -> Self {
        Self {
            name: name.into(),
            tier,
            min_score,
            max_score,
        }
    }

    /// Get the display name of the ranking title
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the tier of the ranking title
    #[allow(dead_code)]
    pub fn tier(&self) -> &Rank {
        &self.tier
    }

    /// Check if a score falls within this ranking title's range
    #[allow(dead_code)]
    pub fn contains_score(&self, score: f64) -> bool {
        let score = score as u32;
        score >= self.min_score && score <= self.max_score
    }

    /// Get the color palette name for ASCII art generation
    pub fn color_palette(&self) -> &'static str {
        self.tier.color_palette()
    }

    /// Get the terminal color for this ranking title
    pub fn terminal_color(&self) -> crossterm::style::Color {
        self.tier.terminal_color()
    }

    /// Get all ranking titles in order from lowest to highest score
    pub fn all_titles() -> Vec<RankingTitle> {
        vec![
            // Beginner Level (clean boundaries, ~even progression)
            RankingTitle::new("Hello World", Rank::Beginner, 0, 800),
            RankingTitle::new("Syntax Error", Rank::Beginner, 801, 1200),
            RankingTitle::new("Rubber Duck", Rank::Beginner, 1201, 1600),
            RankingTitle::new("Script Kid", Rank::Beginner, 1601, 2000),
            RankingTitle::new("Bash Newbie", Rank::Beginner, 2001, 2450),
            RankingTitle::new("CLI Wanderer", Rank::Beginner, 2451, 2900),
            RankingTitle::new("Tab Tamer", Rank::Beginner, 2901, 3300),
            RankingTitle::new("Bracket Juggler", Rank::Beginner, 3301, 3700),
            RankingTitle::new("Copy-Paste Engineer", Rank::Beginner, 3701, 4150),
            RankingTitle::new("Linter Apprentice", Rank::Beginner, 4151, 4550),
            RankingTitle::new("Unit Test Trainee", Rank::Beginner, 4551, 5000),
            RankingTitle::new("Code Monkey", Rank::Beginner, 5001, 5600),
            // Intermediate Level
            RankingTitle::new("Ticket Picker", Rank::Intermediate, 5601, 5850),
            RankingTitle::new("Junior Dev", Rank::Intermediate, 5851, 6000),
            RankingTitle::new("Git Ninja", Rank::Intermediate, 6001, 6100),
            RankingTitle::new("Merge Wrangler", Rank::Intermediate, 6101, 6250),
            RankingTitle::new("API Crafter", Rank::Intermediate, 6251, 6400),
            RankingTitle::new("Frontend Dev", Rank::Intermediate, 6401, 6550),
            RankingTitle::new("Backend Dev", Rank::Intermediate, 6551, 6700),
            RankingTitle::new("CI Tinkerer", Rank::Intermediate, 6701, 6850),
            RankingTitle::new("Test Pilot", Rank::Intermediate, 6851, 7000),
            RankingTitle::new("Build Tamer", Rank::Intermediate, 7001, 7100),
            RankingTitle::new("Code Reviewer", Rank::Intermediate, 7101, 7250),
            RankingTitle::new("Release Handler", Rank::Intermediate, 7251, 7500),
            // Advanced Level
            RankingTitle::new("Refactorer", Rank::Advanced, 7501, 7800),
            RankingTitle::new("Senior Dev", Rank::Advanced, 7801, 8000),
            RankingTitle::new("DevOps Engineer", Rank::Advanced, 8001, 8100),
            RankingTitle::new("Incident Responder", Rank::Advanced, 8101, 8250),
            RankingTitle::new("Reliability Guardian", Rank::Advanced, 8251, 8400),
            RankingTitle::new("Security Engineer", Rank::Advanced, 8401, 8500),
            RankingTitle::new("Performance Alchemist", Rank::Advanced, 8501, 8650),
            RankingTitle::new("Data Pipeline Master", Rank::Advanced, 8651, 8800),
            RankingTitle::new("Tech Lead", Rank::Advanced, 8801, 8950),
            RankingTitle::new("Architect", Rank::Advanced, 8951, 9100),
            RankingTitle::new("Protocol Artisan", Rank::Advanced, 9101, 9250),
            RankingTitle::new("Kernel Hacker", Rank::Advanced, 9251, 9500),
            // Expert Level
            RankingTitle::new("Compiler", Rank::Expert, 9501, 9800),
            RankingTitle::new("Bytecode Interpreter", Rank::Expert, 9801, 9950),
            RankingTitle::new("Virtual Machine", Rank::Expert, 9951, 10100),
            RankingTitle::new("Operating System", Rank::Expert, 10101, 10200),
            RankingTitle::new("Filesystem", Rank::Expert, 10201, 10350),
            RankingTitle::new("Network Stack", Rank::Expert, 10351, 10500),
            RankingTitle::new("Database Engine", Rank::Expert, 10501, 10650),
            RankingTitle::new("Query Optimizer", Rank::Expert, 10651, 10800),
            RankingTitle::new("Cloud Platform", Rank::Expert, 10801, 10950),
            RankingTitle::new("Container Orchestrator", Rank::Expert, 10951, 11100),
            RankingTitle::new("Stream Processor", Rank::Expert, 11101, 11200),
            RankingTitle::new("Quantum Computer", Rank::Expert, 11201, 11400),
            // Legendary Level
            RankingTitle::new("GPU Cluster", Rank::Legendary, 11401, 11700),
            RankingTitle::new("DNS Overlord", Rank::Legendary, 11701, 12250),
            RankingTitle::new("CDN Sentinel", Rank::Legendary, 12251, 12800),
            RankingTitle::new("Load Balancer Primarch", Rank::Legendary, 12801, 13400),
            RankingTitle::new("Singularity", Rank::Legendary, 13401, 13950),
            RankingTitle::new("The Machine", Rank::Legendary, 13951, 14500),
            RankingTitle::new("Origin", Rank::Legendary, 14501, 15100),
            RankingTitle::new("SegFault", Rank::Legendary, 15101, 15650),
            RankingTitle::new("Buffer Overflow", Rank::Legendary, 15651, 16200),
            RankingTitle::new("Memory Leak", Rank::Legendary, 16201, 16800),
            RankingTitle::new("Null Pointer Exception", Rank::Legendary, 16801, 17350),
            RankingTitle::new("Undefined Behavior", Rank::Legendary, 17351, 17900),
            RankingTitle::new("Heisenbug", Rank::Legendary, 17901, 18500),
            RankingTitle::new("Blue Screen", Rank::Legendary, 18501, 19100),
            RankingTitle::new("Kernel Panic", Rank::Legendary, 19101, u32::MAX),
        ]
    }

    /// Find the ranking title for a given score
    #[allow(dead_code)]
    pub fn for_score(score: f64) -> RankingTitle {
        Self::all_titles()
            .into_iter()
            .find(|title| title.contains_score(score))
            .unwrap_or_else(|| {
                // Fallback to highest title if score exceeds all ranges
                RankingTitle::new("Kernel Panic", Rank::Legendary, 40001, u32::MAX)
            })
    }
}