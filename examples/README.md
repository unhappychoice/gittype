# Development Tools

This directory contains development tools and utilities for GitType.

## Seed Database

Populate the database with sample data for development and testing.

### Usage

```bash
# Generate default dataset (10 repos, 1000 sessions, 3000 stages)
cargo run --example seed_database -- --clear

# Generate custom dataset
cargo run --example seed_database -- --clear --repos 5 --sessions 100 --stages 500

# Small dataset for quick testing
cargo run --example seed_database -- --clear --repos 2 --sessions 20 --stages 50
```

### Options

- `--clear`: Clear existing data before seeding
- `--repos <N>`: Number of repositories to generate (default: 10)
- `--sessions <N>`: Number of sessions to generate (default: 1000)  
- `--stages <N>`: Number of stages to generate (default: 3000)

### Generated Data

The tool generates realistic sample data including:

- **Repositories**: Various programming languages and project types
- **Sessions**: Mix of completed and incomplete typing sessions
- **Challenges**: Real code snippets in multiple languages
- **Stages**: Individual typing challenges with timing data
- **Results**: Performance metrics, rankings, and statistics

This data is useful for:
- Testing UI components with realistic data volumes
- Performance testing with large datasets
- Developing analytics and reporting features
- Manual testing of different user scenarios