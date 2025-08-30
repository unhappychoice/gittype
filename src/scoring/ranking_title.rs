/// Represents a ranking title with associated metadata
#[derive(Debug, Clone, PartialEq)]
pub struct RankingTitle {
    pub name: String,
    pub tier: RankingTier,
    pub min_score: u32,
    pub max_score: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RankingTier {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
    Legendary,
}

impl RankingTier {
    /// Get the color palette name for ASCII art generation
    pub fn color_palette(&self) -> &'static str {
        match self {
            RankingTier::Beginner => "grad-blue",
            RankingTier::Intermediate => "dawn",
            RankingTier::Advanced => "forest",
            RankingTier::Expert => "gold",
            RankingTier::Legendary => "fire",
        }
    }
}

impl RankingTitle {
    /// Create a new ranking title
    pub fn new(name: impl Into<String>, tier: RankingTier, min_score: u32, max_score: u32) -> Self {
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
    pub fn tier(&self) -> &RankingTier {
        &self.tier
    }

    /// Check if a score falls within this ranking title's range
    pub fn contains_score(&self, score: f64) -> bool {
        let score = score as u32;
        score >= self.min_score && score <= self.max_score
    }

    /// Get the color palette name for ASCII art generation
    pub fn color_palette(&self) -> &'static str {
        self.tier.color_palette()
    }

    /// Get all ranking titles in order from lowest to highest score (matches engine.rs exactly)
    pub fn all_titles() -> Vec<RankingTitle> {
        vec![
            // Beginner Level (clean boundaries, ~even progression)
            RankingTitle::new("Hello World", RankingTier::Beginner, 0, 800),
            RankingTitle::new("Syntax Error", RankingTier::Beginner, 801, 1200),
            RankingTitle::new("Rubber Duck", RankingTier::Beginner, 1201, 1600),
            RankingTitle::new("Script Kid", RankingTier::Beginner, 1601, 2000),
            RankingTitle::new("Bash Newbie", RankingTier::Beginner, 2001, 2450),
            RankingTitle::new("CLI Wanderer", RankingTier::Beginner, 2451, 2900),
            RankingTitle::new("Tab Tamer", RankingTier::Beginner, 2901, 3300),
            RankingTitle::new("Bracket Juggler", RankingTier::Beginner, 3301, 3700),
            RankingTitle::new("Copy-Paste Engineer", RankingTier::Beginner, 3701, 4150),
            RankingTitle::new("Linter Apprentice", RankingTier::Beginner, 4151, 4550),
            RankingTitle::new("Unit Test Trainee", RankingTier::Beginner, 4551, 5000),
            RankingTitle::new("Code Monkey", RankingTier::Beginner, 5001, 5600),

            // Intermediate Level (clean midpoints rounded to 50/100)
            RankingTitle::new("Ticket Picker", RankingTier::Intermediate, 5601, 5850),
            RankingTitle::new("Junior Dev", RankingTier::Intermediate, 5851, 6000),
            RankingTitle::new("Git Ninja", RankingTier::Intermediate, 6001, 6100),
            RankingTitle::new("Merge Wrangler", RankingTier::Intermediate, 6101, 6250),
            RankingTitle::new("API Crafter", RankingTier::Intermediate, 6251, 6400),
            RankingTitle::new("Frontend Dev", RankingTier::Intermediate, 6401, 6550),
            RankingTitle::new("Backend Dev", RankingTier::Intermediate, 6551, 6700),
            RankingTitle::new("CI Tinkerer", RankingTier::Intermediate, 6701, 6850),
            RankingTitle::new("Test Pilot", RankingTier::Intermediate, 6851, 7000),
            RankingTitle::new("Build Tamer", RankingTier::Intermediate, 7001, 7100),
            RankingTitle::new("Code Reviewer", RankingTier::Intermediate, 7101, 7250),
            RankingTitle::new("Release Handler", RankingTier::Intermediate, 7251, 7500),

            // Advanced Level (clean midpoints rounded)
            RankingTitle::new("Refactorer", RankingTier::Advanced, 7501, 7800),
            RankingTitle::new("Senior Dev", RankingTier::Advanced, 7801, 8000),
            RankingTitle::new("DevOps Engineer", RankingTier::Advanced, 8001, 8100),
            RankingTitle::new("Incident Responder", RankingTier::Advanced, 8101, 8250),
            RankingTitle::new("Reliability Guardian", RankingTier::Advanced, 8251, 8400),
            RankingTitle::new("Security Engineer", RankingTier::Advanced, 8401, 8500),
            RankingTitle::new("Performance Alchemist", RankingTier::Advanced, 8501, 8650),
            RankingTitle::new("Data Pipeline Master", RankingTier::Advanced, 8651, 8800),
            RankingTitle::new("Tech Lead", RankingTier::Advanced, 8801, 8950),
            RankingTitle::new("Architect", RankingTier::Advanced, 8951, 9100),
            RankingTitle::new("Protocol Artisan", RankingTier::Advanced, 9101, 9250),
            RankingTitle::new("Kernel Hacker", RankingTier::Advanced, 9251, 9500),

            // Expert Level
            RankingTitle::new("Compiler", RankingTier::Expert, 9501, 9800),
            RankingTitle::new("Bytecode Interpreter", RankingTier::Expert, 9801, 9950),
            RankingTitle::new("Virtual Machine", RankingTier::Expert, 9951, 10100),
            RankingTitle::new("Operating System", RankingTier::Expert, 10101, 10200),
            RankingTitle::new("Filesystem", RankingTier::Expert, 10201, 10350),
            RankingTitle::new("Network Stack", RankingTier::Expert, 10351, 10500),
            RankingTitle::new("Database Engine", RankingTier::Expert, 10501, 10650),
            RankingTitle::new("Query Optimizer", RankingTier::Expert, 10651, 10800),
            RankingTitle::new("Cloud Platform", RankingTier::Expert, 10801, 10950),
            RankingTitle::new("Container Orchestrator", RankingTier::Expert, 10951, 11100),
            RankingTitle::new("Stream Processor", RankingTier::Expert, 11101, 11200),
            RankingTitle::new("Quantum Computer", RankingTier::Expert, 11201, 11400),

            // Legendary Level
            RankingTitle::new("GPU Cluster", RankingTier::Legendary, 11401, 11700),
            RankingTitle::new("DNS Overlord", RankingTier::Legendary, 11701, 12250),
            RankingTitle::new("CDN Sentinel", RankingTier::Legendary, 12251, 12800),
            RankingTitle::new("Load Balancer Primarch", RankingTier::Legendary, 12801, 13400),
            RankingTitle::new("Singularity", RankingTier::Legendary, 13401, 13950),
            RankingTitle::new("The Machine", RankingTier::Legendary, 13951, 14500),
            RankingTitle::new("Origin", RankingTier::Legendary, 14501, 15100),
            RankingTitle::new("SegFault", RankingTier::Legendary, 15101, 15650),
            RankingTitle::new("Buffer Overflow", RankingTier::Legendary, 15651, 16200),
            RankingTitle::new("Memory Leak", RankingTier::Legendary, 16201, 16800),
            RankingTitle::new("Null Pointer Exception", RankingTier::Legendary, 16801, 17350),
            RankingTitle::new("Undefined Behavior", RankingTier::Legendary, 17351, 17900),
            RankingTitle::new("Heisenbug", RankingTier::Legendary, 17901, 18500),
            RankingTitle::new("Blue Screen", RankingTier::Legendary, 18501, 19100),
            RankingTitle::new("Kernel Panic", RankingTier::Legendary, 19101, u32::MAX),
        ]
    }

    /// Find the ranking title for a given score
    pub fn for_score(score: f64) -> RankingTitle {
        Self::all_titles()
            .into_iter()
            .find(|title| title.contains_score(score))
            .unwrap_or_else(|| {
                // Fallback to highest title if score exceeds all ranges
                RankingTitle::new("Kernel Panic", RankingTier::Legendary, 40001, u32::MAX)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ranking_title_creation() {
        let title = RankingTitle::new("Hello World", RankingTier::Beginner, 0, 800);
        assert_eq!(title.name(), "Hello World");
        assert_eq!(title.tier(), &RankingTier::Beginner);
        assert_eq!(title.min_score, 0);
        assert_eq!(title.max_score, 800);
    }

    #[test]
    fn test_contains_score() {
        let title = RankingTitle::new("Hello World", RankingTier::Beginner, 0, 800);
        assert!(title.contains_score(400.0));
        assert!(title.contains_score(0.0));
        assert!(title.contains_score(800.0));
        assert!(!title.contains_score(801.0));
    }

    #[test]
    fn test_color_palette() {
        let beginner = RankingTitle::new("Hello World", RankingTier::Beginner, 0, 800);
        assert_eq!(beginner.color_palette(), "grad-blue");

        let expert = RankingTitle::new("Virtual Machine", RankingTier::Expert, 16351, 17000);
        assert_eq!(expert.color_palette(), "gold");
    }

    #[test]
    fn test_for_score() {
        let title = RankingTitle::for_score(400.0);
        assert_eq!(title.name(), "Hello World");

        let title = RankingTitle::for_score(6300.0);
        assert_eq!(title.name(), "API Crafter");

        let title = RankingTitle::for_score(50000.0);
        assert_eq!(title.name(), "Kernel Panic");
    }

    #[test]
    fn test_all_titles_coverage() {
        let titles = RankingTitle::all_titles();
        assert!(!titles.is_empty());

        // Test that scores are covered from 0 to a high value
        for score in [0, 100, 5000, 10000, 20000, 30000, 40000] {
            let title = RankingTitle::for_score(score as f64);
            assert!(!title.name().is_empty());
        }
    }

    #[test]
    fn test_no_gaps_in_ranges() {
        let titles = RankingTitle::all_titles();
        for i in 0..(titles.len() - 1) {
            let current = &titles[i];
            let next = &titles[i + 1];
            assert_eq!(current.max_score + 1, next.min_score,
                "Gap found between {} (max: {}) and {} (min: {})",
                current.name(), current.max_score, next.name(), next.min_score);
        }
    }
}