use gittype::models::rank::Rank;

macro_rules! rank_name_tests {
    ($($test:ident => $index:expr, $expected:expr),+ $(,)?) => {
        $(
            #[test]
            fn $test() {
                let ranks = Rank::all_ranks();
                assert_eq!(ranks[$index].name(), $expected);
            }
        )+
    };
}

macro_rules! rank_for_score_tests {
    ($($test:ident => $index:expr),+ $(,)?) => {
        $(
            #[test]
            fn $test() {
                let ranks = Rank::all_ranks();
                let rank = &ranks[$index];
                let mut sample = rank.min_score as f64 + 0.5;
                let max = rank.max_score as f64;
                if sample > max {
                    sample = rank.min_score as f64;
                }
                let found = Rank::for_score(sample);
                assert_eq!(found.name(), rank.name());
            }
        )+
    };
}

rank_name_tests! {
    rank_name_00 => 0, "Hello World",
    rank_name_01 => 1, "Syntax Error",
    rank_name_02 => 2, "Rubber Duck",
    rank_name_03 => 3, "Script Kid",
    rank_name_04 => 4, "Bash Newbie",
    rank_name_05 => 5, "CLI Wanderer",
    rank_name_06 => 6, "Tab Tamer",
    rank_name_07 => 7, "Bracket Juggler",
    rank_name_08 => 8, "Copy-Paste Engineer",
    rank_name_09 => 9, "Linter Apprentice",
    rank_name_10 => 10, "Unit Test Trainee",
    rank_name_11 => 11, "Code Monkey",
    rank_name_12 => 12, "Ticket Picker",
    rank_name_13 => 13, "Junior Dev",
    rank_name_14 => 14, "Git Ninja",
    rank_name_15 => 15, "Merge Wrangler",
    rank_name_16 => 16, "API Crafter",
    rank_name_17 => 17, "Frontend Dev",
    rank_name_18 => 18, "Backend Dev",
    rank_name_19 => 19, "CI Tinkerer",
    rank_name_20 => 20, "Test Pilot",
    rank_name_21 => 21, "Build Tamer",
    rank_name_22 => 22, "Code Reviewer",
    rank_name_23 => 23, "Release Handler",
    rank_name_24 => 24, "Refactorer",
    rank_name_25 => 25, "Senior Dev",
    rank_name_26 => 26, "DevOps Engineer",
    rank_name_27 => 27, "Incident Responder",
    rank_name_28 => 28, "Reliability Guardian",
    rank_name_29 => 29, "Security Engineer",
    rank_name_30 => 30, "Performance Alchemist",
    rank_name_31 => 31, "Data Pipeline Master",
    rank_name_32 => 32, "Tech Lead",
    rank_name_33 => 33, "Architect",
    rank_name_34 => 34, "Protocol Artisan",
    rank_name_35 => 35, "Kernel Hacker",
    rank_name_36 => 36, "Compiler",
    rank_name_37 => 37, "Bytecode Interpreter",
    rank_name_38 => 38, "Virtual Machine",
    rank_name_39 => 39, "Operating System",
    rank_name_40 => 40, "Filesystem",
    rank_name_41 => 41, "Network Stack",
    rank_name_42 => 42, "Database Engine",
    rank_name_43 => 43, "Query Optimizer",
    rank_name_44 => 44, "Cloud Platform",
    rank_name_45 => 45, "Container Orchestrator",
    rank_name_46 => 46, "Stream Processor",
    rank_name_47 => 47, "Quantum Computer",
    rank_name_48 => 48, "GPU Cluster",
    rank_name_49 => 49, "DNS Overlord",
    rank_name_50 => 50, "CDN Sentinel",
    rank_name_51 => 51, "Load Balancer Primarch",
    rank_name_52 => 52, "Singularity",
    rank_name_53 => 53, "The Machine",
    rank_name_54 => 54, "Origin",
    rank_name_55 => 55, "SegFault",
    rank_name_56 => 56, "Buffer Overflow",
    rank_name_57 => 57, "Memory Leak",
    rank_name_58 => 58, "Null Pointer Exception",
    rank_name_59 => 59, "Undefined Behavior",
    rank_name_60 => 60, "Heisenbug",
    rank_name_61 => 61, "Blue Screen",
    rank_name_62 => 62, "Kernel Panic",
}

rank_for_score_tests! {
    rank_for_score_00 => 0,
    rank_for_score_01 => 1,
    rank_for_score_02 => 2,
    rank_for_score_03 => 3,
    rank_for_score_04 => 4,
    rank_for_score_05 => 5,
    rank_for_score_06 => 6,
    rank_for_score_07 => 7,
    rank_for_score_08 => 8,
    rank_for_score_09 => 9,
    rank_for_score_10 => 10,
    rank_for_score_11 => 11,
    rank_for_score_12 => 12,
    rank_for_score_13 => 13,
    rank_for_score_14 => 14,
    rank_for_score_15 => 15,
    rank_for_score_16 => 16,
    rank_for_score_17 => 17,
    rank_for_score_18 => 18,
    rank_for_score_19 => 19,
    rank_for_score_20 => 20,
    rank_for_score_21 => 21,
    rank_for_score_22 => 22,
    rank_for_score_23 => 23,
    rank_for_score_24 => 24,
    rank_for_score_25 => 25,
    rank_for_score_26 => 26,
    rank_for_score_27 => 27,
    rank_for_score_28 => 28,
    rank_for_score_29 => 29,
    rank_for_score_30 => 30,
    rank_for_score_31 => 31,
    rank_for_score_32 => 32,
    rank_for_score_33 => 33,
    rank_for_score_34 => 34,
    rank_for_score_35 => 35,
    rank_for_score_36 => 36,
    rank_for_score_37 => 37,
    rank_for_score_38 => 38,
    rank_for_score_39 => 39,
    rank_for_score_40 => 40,
    rank_for_score_41 => 41,
    rank_for_score_42 => 42,
    rank_for_score_43 => 43,
    rank_for_score_44 => 44,
    rank_for_score_45 => 45,
    rank_for_score_46 => 46,
    rank_for_score_47 => 47,
    rank_for_score_48 => 48,
    rank_for_score_49 => 49,
    rank_for_score_50 => 50,
    rank_for_score_51 => 51,
    rank_for_score_52 => 52,
    rank_for_score_53 => 53,
    rank_for_score_54 => 54,
    rank_for_score_55 => 55,
    rank_for_score_56 => 56,
    rank_for_score_57 => 57,
    rank_for_score_58 => 58,
    rank_for_score_59 => 59,
    rank_for_score_60 => 60,
    rank_for_score_61 => 61,
    rank_for_score_62 => 62,
}
