use gittype::scoring::engine::ScoringEngine;
use std::time::Duration;

// Helper struct for test results
struct ScoringTestResult {
    actual_cpm: f64,
    accuracy: f64,
    mistakes: usize,
    score: f64,
    title: String,
}

// Helper function to test scoring with specific metrics (pure function approach)
fn test_scoring_with_metrics(target_cpm: f64, accuracy: f64, mistakes: usize) -> ScoringTestResult {
    // Fixed 1 second elapsed time for consistent testing
    let elapsed_secs = 1.0;

    // Calculate total chars based on CPM, accuracy, and mistakes
    let correct_chars = (target_cpm / 60.0) as usize; // chars per second
    let total_chars = if accuracy > 0.0 {
        ((correct_chars as f64) / (accuracy / 100.0)) as usize
    } else {
        correct_chars + mistakes
    };

    let score = ScoringEngine::calculate_score_from_metrics(
        target_cpm,
        accuracy,
        mistakes,
        elapsed_secs,
        total_chars,
    );

    let title = ScoringEngine::get_rank_for_score(score)
        .name()
        .to_string();

    ScoringTestResult {
        actual_cpm: target_cpm,
        accuracy,
        mistakes,
        score,
        title,
    }
}

macro_rules! define_performance_test_cases {
    ($(
        $tier:ident: [
            $(($cpm:expr, { $($condition:ident: ($score:expr, $rank:literal)),+ $(,)? })),* $(,)?
        ]
    ),* $(,)?) => {
        // Generate individual test functions for each case
        $($(
            paste::paste! {
                #[test]
                fn [<test_ $tier _ $cpm _cpm_ $( $score )+>]() {
                    // Test different performance conditions and their expected outcomes
                    $(
                        let expected_score = $score;
                        let expected_rank = $rank;

                        // Define mistakes based on condition
                        let (target_mistakes, accuracy) = match stringify!($condition) {
                            // Keep mistake ratios; raise accuracies as requested
                            "bad" => (($cpm as f64 * 0.05) as usize, 90.0),      // 5% mistakes, 90% accuracy
                            "normal" => (($cpm as f64 * 0.02) as usize, 95.0),   // 2% mistakes, 95% accuracy
                            "good" => (($cpm as f64 * 0.01) as usize, 98.0),     // 1% mistakes, 98% accuracy
                            "perfect" => (0, 100.0),                             // 0% mistakes, 100% accuracy
                            _ => (($cpm as f64 * 0.02) as usize, 80.0),         // default to normal
                        };

                        let result = test_scoring_with_metrics($cpm as f64, accuracy, target_mistakes);

                        println!("{}CPM, {} mistakes ({} condition) -> Actual CPM: {:.1}, Accuracy: {:.1}%, Score: {:.0}, Title: {}",
                                $cpm, target_mistakes, stringify!($condition), result.actual_cpm, result.accuracy,
                                result.score, result.title);

                        // Basic validations
                        assert!(result.actual_cpm >= 0.0, "CPM should be non-negative");
                        assert!(result.accuracy >= 0.0 && result.accuracy <= 100.0, "Accuracy should be 0-100%");
                        assert_eq!(result.mistakes, target_mistakes, "Mistakes should match: expected {}, got {}", target_mistakes, result.mistakes);
                        assert!(!result.title.is_empty(), "Title should not be empty");

                        println!("  {} condition: expecting score ~{}, rank '{}'",
                               stringify!($condition), expected_score, expected_rank);

                        // Assert exact title match (this should fail in RED phase)
                        assert_eq!(result.title, expected_rank,
                            "Expected title '{}' for {} condition with {}CPM and {} mistakes, but got '{}'",
                            expected_rank, stringify!($condition), $cpm, target_mistakes, result.title);

                        // Assert score is within reasonable range (±20% tolerance)
                        let score_tolerance = (expected_score as f64) * 0.2;
                        let score_min = (expected_score as f64) - score_tolerance;
                        let score_max = (expected_score as f64) + score_tolerance;
                        assert!(result.score >= score_min && result.score <= score_max,
                            "Expected score ~{} (±20%) for {} condition with {}CPM and {} mistakes, but got {:.0}",
                            expected_score, stringify!($condition), $cpm, target_mistakes, result.score);
                    )+
                }
            }
        )*)*
    };
}

// Define comprehensive test cases ensuring all 63 titles appear in normal condition
// Population thick areas (200-350CPM) get more titles, sparse areas (700+CPM) get fewer
// Score formula: (CPM × accuracy% × 10) × 2 + 100 + bonuses ≈ CPM × 20 + bonuses
define_performance_test_cases! {
    beginner: [
        // All 12 BEGINNER titles: Hello World, Syntax Error, Rubber Duck, Script Kid, Bash Newbie, CLI Wanderer, Tab Tamer, Bracket Juggler, Copy-Paste Engineer, Linter Apprentice, Unit Test Trainee, Code Monkey (0-200 CPM)
        (20, { normal: (670, "Hello World") }),
        (30, { normal: (955, "Syntax Error") }),
        (45, { normal: (1382, "Rubber Duck") }),
        (60, { normal: (1800, "Script Kid") }),
        (75, { normal: (2228, "Bash Newbie") }),
        (90, { normal: (2655, "CLI Wanderer") }),
        (105, { normal: (3072, "Tab Tamer") }),
        (120, { normal: (3500, "Bracket Juggler") }),
        (135, { normal: (3928, "Copy-Paste Engineer") }),
        (150, { normal: (4345, "Linter Apprentice") }),
        (165, { normal: (4772, "Unit Test Trainee") }),
        (180, { normal: (5200, "Code Monkey") }),
    ],

    intermediate: [
        // All 12 INTERMEDIATE titles (expanded, near-equal CPM steps)
        (200, { normal: (5760, "Ticket Picker") }),
        (205, { normal: (5902, "Junior Dev") }),
        (210, { normal: (6045, "Git Ninja") }),
        (215, { normal: (6188, "Merge Wrangler") }),
        (220, { normal: (6330, "API Crafter") }),
        (225, { normal: (6472, "Frontend Dev") }),
        (230, { normal: (6615, "Backend Dev") }),
        (235, { normal: (6758, "CI Tinkerer") }),
        (240, { normal: (6900, "Test Pilot") }),
        (245, { normal: (7042, "Build Tamer") }),
        (250, { normal: (7175, "Code Reviewer") }),
        (255, { normal: (7318, "Release Handler") }),
    ],

    advanced: [
        // All 12 ADVANCED titles (shifted lower, near-equal steps)
        (270, { normal: (7745, "Refactorer") }),
        (275, { normal: (7888, "Senior Dev") }),
        (280, { normal: (8030, "DevOps Engineer") }),
        (285, { normal: (8172, "Incident Responder") }),
        (290, { normal: (8315, "Reliability Guardian") }),
        (295, { normal: (8458, "Security Engineer") }),
        (300, { normal: (8590, "Performance Alchemist") }),
        (305, { normal: (8732, "Data Pipeline Master") }),
        (310, { normal: (8875, "Tech Lead") }),
        (315, { normal: (9018, "Architect") }),
        (320, { normal: (9160, "Protocol Artisan") }),
        (325, { normal: (9302, "Kernel Hacker") }),
    ],

    expert: [
        // All 12 EXPERT titles (expanded lower, near-equal steps; 340-395)
        (340, { normal: (9730, "Compiler") }),
        (345, { normal: (9872, "Bytecode Interpreter") }),
        (350, { normal: (10005, "Virtual Machine") }),
        (355, { normal: (10148, "Operating System") }),
        (360, { normal: (10290, "Filesystem") }),
        (365, { normal: (10432, "Network Stack") }),
        (370, { normal: (10575, "Database Engine") }),
        (375, { normal: (10718, "Query Optimizer") }),
        (380, { normal: (10860, "Cloud Platform") }),
        (385, { normal: (11002, "Container Orchestrator") }),
        (390, { normal: (11145, "Stream Processor") }),
        (395, { normal: (11288, "Quantum Computer") }),
    ],

    legendary: [
        // All 15 LEGENDARY titles (start from 400CPM; near-equal 20CPM steps until 680)
        (400, { normal: (11420, "GPU Cluster") }),
        (420, { normal: (11990, "DNS Overlord") }),
        (440, { normal: (12560, "CDN Sentinel") }),
        (460, { normal: (13120, "Load Balancer Primarch") }),
        (480, { normal: (13690, "Singularity") }),
        (500, { normal: (14250, "The Machine") }),
        (520, { normal: (14820, "Origin") }),
        (540, { normal: (15390, "SegFault") }),
        (560, { normal: (15950, "Buffer Overflow") }),
        (580, { normal: (16520, "Memory Leak") }),
        (600, { normal: (17080, "Null Pointer Exception") }),
        (620, { normal: (17650, "Undefined Behavior") }),
        (640, { normal: (18220, "Heisenbug") }),
        (660, { normal: (18780, "Blue Screen") }),
        (680, { normal: (19350, "Kernel Panic") }),
    ],
}

#[cfg(test)]
mod basic_functionality_tests {
    use super::*;

    #[test]
    fn test_scoring_engine_lifecycle() {
        let target_text = "hello world test";
        let mut engine = ScoringEngine::new(target_text.to_string());

        // Test initial state
        assert!(!engine.is_finished());

        engine.start();

        // Record some keystrokes
        engine.record_keystroke('h', 0);
        engine.record_keystroke('e', 1);
        engine.record_keystroke('l', 2);
        engine.record_keystroke('l', 3);
        engine.record_keystroke('o', 4);

        // Record a mistake
        engine.record_keystroke('x', 5); // should be space

        std::thread::sleep(Duration::from_millis(1000));

        engine.finish();
        assert!(engine.is_finished());

        let metrics = engine
            .calculate_result()
            .expect("Should calculate result");

        assert_eq!(engine.correct_chars(), 5);
        assert_eq!(engine.mistakes(), 1);
        assert_eq!(engine.total_chars(), 6);
        assert!(metrics.cpm > 0.0);
        assert!(metrics.accuracy > 0.0 && metrics.accuracy <= 100.0);
        assert!(!metrics.rank_name.is_empty());

        println!(
            "Lifecycle test: {} correct, {} mistakes, CPM: {:.1}, Title: {}",
            engine.correct_chars(),
            engine.mistakes(),
            metrics.cpm,
            metrics.rank_name
        );
    }

    #[test]
    fn test_engine_combination() {
        let mut engine1 = ScoringEngine::new("test session one".to_string());
        let mut engine2 = ScoringEngine::new("test session two".to_string());

        engine1.start();
        engine1.record_keystroke('t', 0);
        engine1.record_keystroke('e', 1);
        engine1.record_keystroke('s', 2);
        engine1.record_keystroke('t', 3);
        engine1.finish();

        engine2.start();
        engine2.record_keystroke('t', 0);
        engine2.record_keystroke('e', 1);
        engine2.record_keystroke('x', 2); // mistake
        engine2.finish();

        let combined = engine1 + engine2;
        let combined_metrics = combined
            .calculate_result()
            .expect("Should calculate combined result");

        assert_eq!(combined.correct_chars(), 6); // 4 + 2
        assert_eq!(combined.mistakes(), 1);
        assert!(combined_metrics.challenge_score > 0.0);
        assert!(!combined_metrics.rank_name.is_empty());

        println!(
            "Combination test: {} total correct, {} total mistakes, Score: {:.0}, Title: {}",
            combined.correct_chars(),
            combined.mistakes(),
            combined_metrics.challenge_score,
            combined_metrics.rank_name
        );
    }
}
