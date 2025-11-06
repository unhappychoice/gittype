use gittype::domain::models::ui::rank_messages::{
    get_colored_messages_for_rank, get_hacking_messages_for_rank, ColoredMessage,
};

#[test]
fn get_hacking_messages_for_known_rank() {
    let messages = get_hacking_messages_for_rank("Hello World");
    assert!(!messages.is_empty());
    assert_eq!(messages.len(), 4);
}

#[test]
fn get_hacking_messages_for_unknown_rank() {
    let messages = get_hacking_messages_for_rank("Unknown Rank Name");
    assert!(!messages.is_empty());
    assert_eq!(messages.len(), 4); // Default fallback messages
}

#[test]
fn get_colored_messages_for_known_rank() {
    let messages = get_colored_messages_for_rank("Hello World");
    assert!(!messages.is_empty());
    assert_eq!(messages.len(), 4);
}

#[test]
fn get_colored_messages_for_unknown_rank() {
    let messages = get_colored_messages_for_rank("Unknown Rank Name");
    assert!(!messages.is_empty());
    assert_eq!(messages.len(), 4); // Default fallback messages
}

#[test]
fn colored_message_has_text_and_color() {
    let messages = get_colored_messages_for_rank("Hello World");
    let first = &messages[0];
    assert!(!first.text.is_empty());
}

#[test]
fn get_messages_for_beginner_tier_ranks() {
    let ranks = vec![
        "Hello World",
        "Syntax Error",
        "Rubber Duck",
        "Script Kid",
        "Bash Newbie",
    ];

    for rank in ranks {
        let messages = get_hacking_messages_for_rank(rank);
        assert_eq!(messages.len(), 4, "Rank '{}' should have 4 messages", rank);
    }
}

#[test]
fn get_messages_for_intermediate_tier_ranks() {
    let ranks = vec!["Junior Dev", "Git Ninja", "API Crafter", "Frontend Dev"];

    for rank in ranks {
        let messages = get_hacking_messages_for_rank(rank);
        assert_eq!(messages.len(), 4, "Rank '{}' should have 4 messages", rank);
    }
}

#[test]
fn get_messages_for_advanced_tier_ranks() {
    let ranks = vec!["Senior Dev", "DevOps Engineer", "Security Engineer"];

    for rank in ranks {
        let messages = get_hacking_messages_for_rank(rank);
        assert_eq!(messages.len(), 4, "Rank '{}' should have 4 messages", rank);
    }
}

#[test]
fn get_messages_for_expert_tier_ranks() {
    let ranks = vec!["Compiler", "Operating System", "Database Engine"];

    for rank in ranks {
        let messages = get_hacking_messages_for_rank(rank);
        assert_eq!(messages.len(), 4, "Rank '{}' should have 4 messages", rank);
    }
}

#[test]
fn get_messages_for_legendary_tier_ranks() {
    let ranks = vec!["Singularity", "The Machine", "Origin", "SegFault"];

    for rank in ranks {
        let messages = get_hacking_messages_for_rank(rank);
        assert_eq!(messages.len(), 4, "Rank '{}' should have 4 messages", rank);
    }
}

#[test]
fn colored_message_clone() {
    let message = ColoredMessage {
        text: "test".to_string(),
        color: ratatui::style::Color::White,
    };

    let cloned = message.clone();
    assert_eq!(message.text, cloned.text);
}

#[test]
fn all_messages_start_with_gt_symbol() {
    let messages = get_hacking_messages_for_rank("Hello World");
    for message in messages {
        assert!(
            message.starts_with('>'),
            "Message should start with '>' but got: {}",
            message
        );
    }
}

#[test]
fn all_messages_end_with_ellipsis_or_period() {
    let messages = get_hacking_messages_for_rank("Hello World");
    for message in messages {
        assert!(
            message.ends_with("...") || message.ends_with('.') || message.ends_with('!'),
            "Message should end with appropriate punctuation: {}",
            message
        );
    }
}

#[test]
fn messages_are_not_empty() {
    let ranks = vec![
        "Hello World",
        "Junior Dev",
        "Senior Dev",
        "Compiler",
        "Singularity",
    ];

    for rank in ranks {
        let messages = get_hacking_messages_for_rank(rank);
        for message in messages {
            assert!(!message.is_empty(), "Message should not be empty");
            assert!(
                message.len() > 5,
                "Message should have meaningful content: {}",
                message
            );
        }
    }
}

#[test]
fn colored_messages_match_regular_messages() {
    let rank = "Hello World";
    let regular = get_hacking_messages_for_rank(rank);
    let colored = get_colored_messages_for_rank(rank);

    assert_eq!(regular.len(), colored.len());
    for (i, (reg, col)) in regular.iter().zip(colored.iter()).enumerate() {
        assert_eq!(reg, &col.text, "Message at index {} should match", i);
    }
}
