mod seeders;

use clap::Parser;
use gittype::{
    infrastructure::database::database::{Database, DatabaseInterface},
    Result,
};
use seeders::DatabaseSeeder;
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "seed_database")]
#[command(about = "Populate database with seed data for development")]
struct Args {
    /// Clear existing data before seeding
    #[arg(long)]
    clear: bool,
    /// Number of repositories to generate
    #[arg(long, default_value = "10")]
    repos: usize,
    /// Number of sessions to generate  
    #[arg(long, default_value = "1000")]
    sessions: usize,
    /// Number of stages to generate
    #[arg(long, default_value = "3000")]
    stages: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("🌱 Starting database seeding...");

    let db = Database::new()?;
    let database: Arc<dyn DatabaseInterface> = Arc::new(db);

    // Initialize database tables if needed
    database.init_tables()?;

    if args.clear {
        println!("🧹 Clearing existing data...");
        clear_database(&database)?;
    }

    println!(
        "📊 Generating {} repositories, {} sessions, {} stages...",
        args.repos, args.sessions, args.stages
    );

    let seeder = DatabaseSeeder::new(database);
    seeder.seed_with_counts(args.repos, args.sessions, args.stages)?;

    println!("✅ Seed data has been successfully loaded!");
    println!("💡 You can now use the application with sample data for development and testing.");

    Ok(())
}

fn clear_database(database: &Arc<dyn DatabaseInterface>) -> Result<()> {
    let conn = database.get_connection()?;

    // Disable foreign key checks temporarily
    conn.execute("PRAGMA foreign_keys = OFF", [])?;

    let tables = vec![
        "stage_results",
        "session_results",
        "stages",
        "challenges",
        "sessions",
        "repositories",
    ];

    for table in tables {
        conn.execute(&format!("DELETE FROM {}", table), [])?;
    }

    // Re-enable foreign key checks
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    Ok(())
}
