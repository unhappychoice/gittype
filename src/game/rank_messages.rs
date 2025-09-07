use crate::ui::Colors;
use crossterm::style::Color;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Message with color information
#[derive(Debug, Clone)]
pub struct ColoredMessage {
    pub text: String,
    pub color: Color,
}

/// All rank-specific hacking messages organized by rank name
static RANK_MESSAGES: OnceLock<HashMap<&'static str, Vec<(&'static str, Color)>>> = OnceLock::new();

fn get_rank_messages() -> &'static HashMap<&'static str, Vec<(&'static str, Color)>> {
    RANK_MESSAGES.get_or_init(|| {
        let mut messages = HashMap::new();

        // Beginner Tier
        messages.insert(
            "Hello World",
            vec![
                ("> googling 'how to print hello world'...", Colors::to_crossterm(Colors::INFO)),
                ("> copying code from first search result...", Colors::to_crossterm(Colors::TEXT)),
                (
                    "> running program 47 times to make sure it works...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> achievement unlocked: you are now a programmer!",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Syntax Error",
            vec![
                ("> writing code that looks right...", Colors::to_crossterm(Colors::TEXT)),
                ("> compiler disagrees with your logic...", Colors::to_crossterm(Colors::ERROR)),
                ("> googling exact error message...", Colors::to_crossterm(Colors::WARNING)),
                (
                    "> fixed by adding random semicolon somewhere.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Rubber Duck",
            vec![
                ("> explaining bug to inanimate object...", Colors::to_crossterm(Colors::INFO)),
                ("> duck stares judgmentally at your code...", Colors::to_crossterm(Colors::TEXT)),
                ("> realizing bug while talking to duck...", Colors::to_crossterm(Colors::WARNING)),
                ("> duck takes full credit for the solution.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Script Kid",
            vec![
                (
                    "> downloading 'learn programming in 24 hours' course...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                ("> copying scripts without reading them...", Colors::to_crossterm(Colors::TEXT)),
                (
                    "> changing variable names to look original...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> script works! you are basically a hacker now.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Bash Newbie",
            vec![
                ("> typing 'cd ..' until something happens...", Colors::to_crossterm(Colors::INFO)),
                (
                    "> using 'ls' every 3 seconds to see where you are...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> accidentally running 'rm' on important files...",
                    Colors::to_crossterm(Colors::ERROR),
                ),
                (
                    "> terminal proficiency: accidentally achieved.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "CLI Wanderer",
            vec![
                (
                    "> exploring directories like a lost tourist...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                ("> discovering pipes by accident...", Colors::to_crossterm(Colors::INFO)),
                (
                    "> finding .hidden files and feeling like a detective...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                ("> navigation skills: randomly acquired.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Tab Tamer",
            vec![
                ("> mixing tabs and spaces like a rebel...", Colors::to_crossterm(Colors::INFO)),
                ("> getting into holy war about indentation...", Colors::to_crossterm(Colors::ERROR)),
                (
                    "> setting up auto-formatter to fix your mess...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                ("> consistency achieved through automation.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Bracket Juggler",
            vec![
                ("> opening 47 brackets...", Colors::to_crossterm(Colors::TEXT)),
                ("> closing 23 brackets...", Colors::to_crossterm(Colors::WARNING)),
                (
                    "> spending 2 hours finding the missing bracket...",
                    Colors::to_crossterm(Colors::ERROR),
                ),
                ("> finally balanced. code still doesn't work.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Copy-Paste Engineer",
            vec![
                ("> opening 50 tabs from stack overflow...", Colors::to_crossterm(Colors::TEXT)),
                ("> copying code from highest voted answer...", Colors::to_crossterm(Colors::TEXT)),
                ("> praying it works in your specific case...", Colors::to_crossterm(Colors::TEXT)),
                ("> it works! time to copy more code.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Linter Apprentice",
            vec![
                (
                    "> installing linter to improve code quality...",
                    Colors::to_crossterm(Colors::INFO),
                ),
                ("> getting 847 warnings on 10 lines of code...", Colors::to_crossterm(Colors::ERROR)),
                (
                    "> disabling all warnings except syntax errors...",
                    Colors::to_crossterm(Colors::WARNING),
                ),
                ("> code quality: subjectively improved.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Unit Test Trainee",
            vec![
                (
                    "> writing test that only passes on your machine...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                ("> testing happy path exclusively...", Colors::to_crossterm(Colors::TEXT)),
                ("> achieving 100% code coverage on 5 lines...", Colors::to_crossterm(Colors::INFO)),
                ("> testing complete. bugs remain untested.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Code Monkey",
            vec![
                ("> following tutorial step by step...", Colors::to_crossterm(Colors::TEXT)),
                (
                    "> changing tutorial example from 'foo' to 'bar'...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                ("> calling yourself a full-stack developer...", Colors::to_crossterm(Colors::INFO)),
                ("> development skills: youtube certified.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        // Intermediate Tier
        messages.insert(
            "Ticket Picker",
            vec![
                ("> scanning project backlog...", Colors::to_crossterm(Colors::TEXT)),
                ("> selecting appropriate tasks...", Colors::to_crossterm(Colors::TEXT)),
                ("> estimating development effort...", Colors::to_crossterm(Colors::TEXT)),
                ("> work assignment optimized.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Junior Dev",
            vec![
                ("> cloning repository...", Colors::to_crossterm(Colors::INFO)),
                ("> creating feature branch...", Colors::to_crossterm(Colors::TEXT)),
                ("> implementing user story...", Colors::to_crossterm(Colors::TEXT)),
                ("> junior developer status confirmed.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Git Ninja",
            vec![
                ("> staging changes...", Colors::to_crossterm(Colors::TEXT)),
                ("> crafting perfect commit message...", Colors::to_crossterm(Colors::TEXT)),
                ("> rebasing interactive history...", Colors::to_crossterm(Colors::INFO)),
                ("> git mastery achieved.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Merge Wrangler",
            vec![
                ("> resolving merge conflicts...", Colors::to_crossterm(Colors::WARNING)),
                ("> coordinating branch updates...", Colors::to_crossterm(Colors::TEXT)),
                ("> maintaining git history...", Colors::to_crossterm(Colors::TEXT)),
                ("> version control expertise proven.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "API Crafter",
            vec![
                ("> designing RESTful endpoints...", Colors::to_crossterm(Colors::TEXT)),
                ("> implementing request handlers...", Colors::to_crossterm(Colors::TEXT)),
                ("> documenting API specification...", Colors::to_crossterm(Colors::TEXT)),
                ("> service interface completed.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Frontend Dev",
            vec![
                ("> building user interfaces...", Colors::to_crossterm(Colors::TEXT)),
                ("> optimizing user experience...", Colors::to_crossterm(Colors::INFO)),
                ("> implementing responsive design...", Colors::to_crossterm(Colors::TEXT)),
                ("> client-side mastery achieved.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Backend Dev",
            vec![
                ("> architecting server logic...", Colors::to_crossterm(Colors::TEXT)),
                ("> optimizing database queries...", Colors::to_crossterm(Colors::INFO)),
                ("> implementing business rules...", Colors::to_crossterm(Colors::TEXT)),
                ("> server-side expertise confirmed.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "CI Tinkerer",
            vec![
                ("> configuring build pipelines...", Colors::to_crossterm(Colors::INFO)),
                ("> automating test execution...", Colors::to_crossterm(Colors::TEXT)),
                ("> setting up deployment hooks...", Colors::to_crossterm(Colors::TEXT)),
                ("> continuous integration mastered.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Test Pilot",
            vec![
                ("> designing test scenarios...", Colors::to_crossterm(Colors::TEXT)),
                ("> automating quality assurance...", Colors::to_crossterm(Colors::TEXT)),
                ("> validating system behavior...", Colors::to_crossterm(Colors::TEXT)),
                ("> testing expertise certified.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Build Tamer",
            vec![
                ("> optimizing compilation process...", Colors::to_crossterm(Colors::INFO)),
                ("> managing dependency versions...", Colors::to_crossterm(Colors::TEXT)),
                ("> configuring build systems...", Colors::to_crossterm(Colors::TEXT)),
                ("> build automation mastered.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Code Reviewer",
            vec![
                ("> analyzing code quality...", Colors::to_crossterm(Colors::TEXT)),
                ("> providing constructive feedback...", Colors::to_crossterm(Colors::TEXT)),
                ("> ensuring best practices...", Colors::to_crossterm(Colors::TEXT)),
                ("> peer review skills confirmed.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Release Handler",
            vec![
                ("> preparing deployment packages...", Colors::to_crossterm(Colors::TEXT)),
                ("> coordinating release schedule...", Colors::to_crossterm(Colors::INFO)),
                ("> managing version rollouts...", Colors::to_crossterm(Colors::TEXT)),
                ("> release management mastered.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages
    })
}

/// Additional rank messages (continued)
static RANK_MESSAGES_2: OnceLock<HashMap<&'static str, Vec<(&'static str, Color)>>> =
    OnceLock::new();

fn get_rank_messages_2() -> &'static HashMap<&'static str, Vec<(&'static str, Color)>> {
    RANK_MESSAGES_2.get_or_init(|| {
        let mut messages = HashMap::new();

        // Advanced Tier
        messages.insert(
            "Refactorer",
            vec![
                ("> analyzing spaghetti code structure...", Colors::to_crossterm(Colors::TEXT)),
                (
                    "> finding ways to make it even more complex...",
                    Colors::to_crossterm(Colors::INFO),
                ),
                (
                    "> refactoring working code until it breaks...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> congratulations! now nobody understands it.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Senior Dev",
            vec![
                (
                    "> architecting solutions that scale to infinity...",
                    Colors::to_crossterm(Colors::INFO),
                ),
                (
                    "> reviewing PRs with passive-aggressive comments...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> mentoring juniors by assigning impossible tasks...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> senior status unlocked. impostor syndrome included.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "DevOps Engineer",
            vec![
                (
                    "> provisioning infrastructure that costs more than rent...",
                    Colors::to_crossterm(Colors::INFO),
                ),
                (
                    "> automating the automation of automated deployments...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> monitoring systems that monitor other monitoring systems...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> everything is automated. nothing works manually.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Incident Responder",
            vec![
                ("> detecting fires while everything is fine...", Colors::to_crossterm(Colors::ERROR)),
                ("> coordinating panic in the war room...", Colors::to_crossterm(Colors::WARNING)),
                (
                    "> applying hotfixes that create more incidents...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> service restored. new incidents created successfully.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Reliability Guardian",
            vec![
                (
                    "> implementing monitoring for the monitoring...",
                    Colors::to_crossterm(Colors::INFO),
                ),
                ("> defining SLOs that nobody can meet...", Colors::to_crossterm(Colors::TEXT)),
                (
                    "> ensuring 99.99% uptime (99% of the time)...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                ("> system reliable until it isn't.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Security Engineer",
            vec![
                (
                    "> finding vulnerabilities in your personality...",
                    Colors::to_crossterm(Colors::ERROR),
                ),
                ("> implementing security through obscurity...", Colors::to_crossterm(Colors::INFO)),
                ("> penetration testing your patience...", Colors::to_crossterm(Colors::TEXT)),
                ("> security hardened. usability softened.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Performance Alchemist",
            vec![
                ("> profiling bottlenecks in the profiler...", Colors::to_crossterm(Colors::WARNING)),
                ("> optimizing code that runs once per year...", Colors::to_crossterm(Colors::INFO)),
                (
                    "> caching everything including this message...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> performance optimized. readability sacrificed.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Data Pipeline Master",
            vec![
                ("> designing workflows that flow nowhere...", Colors::to_crossterm(Colors::TEXT)),
                (
                    "> extracting, transforming, and losing data...",
                    Colors::to_crossterm(Colors::INFO),
                ),
                (
                    "> ensuring consistency in inconsistent data...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                ("> pipeline complete. data may have leaked.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Tech Lead",
            vec![
                (
                    "> defining vision that changes every sprint...",
                    Colors::to_crossterm(Colors::INFO),
                ),
                (
                    "> coordinating efforts while attending 20 meetings...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> making architectural decisions on a coinflip...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> leadership established. technical skills atrophied.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Architect",
            vec![
                (
                    "> designing systems for problems that don't exist...",
                    Colors::to_crossterm(Colors::INFO),
                ),
                (
                    "> choosing technologies based on latest blog posts...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                ("> planning for scale that will never come...", Colors::to_crossterm(Colors::TEXT)),
                (
                    "> architecture complete. implementation someone else's problem.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Protocol Artisan",
            vec![
                (
                    "> designing protocols nobody will implement correctly...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                ("> creating standards to rule them all...", Colors::to_crossterm(Colors::INFO)),
                ("> optimizing transmission of memes...", Colors::to_crossterm(Colors::TEXT)),
                (
                    "> protocol standard published. 14 competing standards exist.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Kernel Hacker",
            vec![
                ("> compiling kernels that boot sometimes...", Colors::to_crossterm(Colors::SCORE)),
                (
                    "> patching system calls with hopes and dreams...",
                    Colors::to_crossterm(Colors::INFO),
                ),
                ("> debugging at 3am with print statements...", Colors::to_crossterm(Colors::TEXT)),
                (
                    "> kernel hacked successfully. computer may explode.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages
    })
}

/// Expert and Legendary tier messages
static RANK_MESSAGES_3: OnceLock<HashMap<&'static str, Vec<(&'static str, Color)>>> =
    OnceLock::new();

fn get_rank_messages_3() -> &'static HashMap<&'static str, Vec<(&'static str, Color)>> {
    RANK_MESSAGES_3.get_or_init(|| {
        let mut messages = HashMap::new();

        // Expert Tier
        messages.insert(
            "Compiler",
            vec![
                (
                    "> tokenizing your messy code into something readable...",
                    Colors::to_crossterm(Colors::SCORE),
                ),
                (
                    "> building AST while judging your variable names...",
                    Colors::to_crossterm(Colors::INFO),
                ),
                ("> optimizing away your inefficient loops...", Colors::to_crossterm(Colors::TEXT)),
                ("> compiled successfully (somehow)", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Bytecode Interpreter",
            vec![
                (
                    "> interpreting your interpreted language interpreter...",
                    Colors::to_crossterm(Colors::SCORE),
                ),
                (
                    "> executing virtual instructions in virtual reality...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> garbage collecting your actual garbage code...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> interpretation complete. still no idea what it does.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Virtual Machine",
            vec![
                (
                    "> virtualizing your already virtual environment...",
                    Colors::to_crossterm(Colors::INFO),
                ),
                ("> emulating hardware that doesn't exist...", Colors::to_crossterm(Colors::SCORE)),
                ("> allocating memory for your memory leaks...", Colors::to_crossterm(Colors::TEXT)),
                (
                    "> VM inception achieved. we need to go deeper.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Operating System",
            vec![
                (
                    "> scheduling processes that never finish...",
                    Colors::to_crossterm(Colors::SCORE),
                ),
                ("> managing resources you don't have...", Colors::to_crossterm(Colors::INFO)),
                (
                    "> handling interrupts from impatient users...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> OS kernel stable (definition of stable: questionable)",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Filesystem",
            vec![
                (
                    "> organizing files into a beautiful directory tree...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> implementing permissions nobody understands...",
                    Colors::to_crossterm(Colors::INFO),
                ),
                ("> fragmenting data across the entire disk...", Colors::to_crossterm(Colors::TEXT)),
                (
                    "> filesystem complete. good luck finding anything.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Network Stack",
            vec![
                ("> layering protocols like a network cake...", Colors::to_crossterm(Colors::INFO)),
                (
                    "> routing packets through the internet tubes...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                ("> ensuring data arrives (eventually)...", Colors::to_crossterm(Colors::TEXT)),
                (
                    "> network stack operational. packets may vary.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Database Engine",
            vec![
                (
                    "> optimizing queries that will timeout anyway...",
                    Colors::to_crossterm(Colors::INFO),
                ),
                ("> isolating transactions from reality...", Colors::to_crossterm(Colors::TEXT)),
                (
                    "> implementing ACID (burns through your SSD)...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> database engine ready. hope you have backups.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Query Optimizer",
            vec![
                (
                    "> analyzing execution plans nobody will read...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                ("> optimizing joins that should be avoided...", Colors::to_crossterm(Colors::INFO)),
                (
                    "> indexing everything (storage is cheap, right?)...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> query performance maximized. complexity also maximized.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Cloud Platform",
            vec![
                ("> orchestrating chaos in the cloud...", Colors::to_crossterm(Colors::SCORE)),
                ("> auto-scaling your monthly cloud bill...", Colors::to_crossterm(Colors::INFO)),
                (
                    "> distributing problems across multiple zones...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> cloud mastery achieved. wallet not included.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Container Orchestrator",
            vec![
                (
                    "> containerizing everything, including the kitchen sink...",
                    Colors::to_crossterm(Colors::INFO),
                ),
                (
                    "> orchestrating a symphony of microservice crashes...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> discovering services that discover other services...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> container cluster ready. cli tools not found.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Stream Processor",
            vec![
                (
                    "> processing streams faster than video platforms...",
                    Colors::to_crossterm(Colors::INFO),
                ),
                (
                    "> implementing event sourcing for event sourcing events...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> ensuring eventual consistency (eventually)...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> streaming platform complete. now streaming bugs.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        messages.insert(
            "Quantum Computer",
            vec![
                (
                    "> initializing qubits in superposition of working/broken...",
                    Colors::to_crossterm(Colors::SCORE),
                ),
                (
                    "> entangling particles and debugging sessions...",
                    Colors::to_crossterm(Colors::INFO),
                ),
                (
                    "> running Shor's algorithm to factor your technical debt...",
                    Colors::to_crossterm(Colors::TEXT),
                ),
                (
                    "> quantum supremacy achieved. classical bugs remain.",
                    Colors::to_crossterm(Colors::SUCCESS),
                ),
            ],
        );

        // Legendary Tier
        messages.insert(
            "GPU Cluster",
            vec![
                ("> initializing parallel processors...", Colors::to_crossterm(Colors::SCORE)),
                ("> distributing computational load...", Colors::to_crossterm(Colors::INFO)),
                ("> optimizing memory bandwidth...", Colors::to_crossterm(Colors::TEXT)),
                ("> massive parallelism achieved.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "DNS Overlord",
            vec![
                ("> controlling domain resolution...", Colors::to_crossterm(Colors::SCORE)),
                ("> managing global namespace...", Colors::to_crossterm(Colors::INFO)),
                ("> routing internet traffic...", Colors::to_crossterm(Colors::TEXT)),
                ("> DNS infrastructure dominated.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "CDN Sentinel",
            vec![
                ("> caching content globally...", Colors::to_crossterm(Colors::INFO)),
                ("> optimizing delivery routes...", Colors::to_crossterm(Colors::TEXT)),
                ("> reducing latency worldwide...", Colors::to_crossterm(Colors::TEXT)),
                ("> content delivery perfected.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Load Balancer Primarch",
            vec![
                ("> distributing incoming requests...", Colors::to_crossterm(Colors::INFO)),
                ("> managing server health...", Colors::to_crossterm(Colors::TEXT)),
                ("> optimizing traffic patterns...", Colors::to_crossterm(Colors::TEXT)),
                ("> load distribution mastered.", Colors::to_crossterm(Colors::SUCCESS)),
            ],
        );

        messages.insert(
            "Singularity",
            vec![
                ("> transcending human limitations...", Colors::to_crossterm(Colors::SCORE)),
                ("> merging with artificial intelligence...", Colors::to_crossterm(Colors::INFO)),
                ("> rewriting reality algorithms...", Colors::to_crossterm(Colors::TEXT)),
                ("> singularity achieved. welcome, god.", Colors::to_crossterm(Colors::ERROR)),
            ],
        );

        messages.insert(
            "The Machine",
            vec![
                ("> becoming one with the system...", Colors::to_crossterm(Colors::SCORE)),
                ("> controlling global networks...", Colors::to_crossterm(Colors::INFO)),
                ("> processing infinite data streams...", Colors::to_crossterm(Colors::TEXT)),
                ("> you are the machine now.", Colors::to_crossterm(Colors::ERROR)),
            ],
        );

        messages.insert(
            "Origin",
            vec![
                ("> accessing source code of reality...", Colors::to_crossterm(Colors::SCORE)),
                ("> modifying fundamental constants...", Colors::to_crossterm(Colors::INFO)),
                ("> debugging universe.exe...", Colors::to_crossterm(Colors::TEXT)),
                ("> origin protocols activated.", Colors::to_crossterm(Colors::ERROR)),
            ],
        );

        messages.insert(
            "SegFault",
            vec![
                ("> accessing forbidden dimensions of memory...", Colors::to_crossterm(Colors::ERROR)),
                ("> reality.exe has encountered a critical error", Colors::to_crossterm(Colors::ERROR)),
                ("> universe segmentation fault detected...", Colors::to_crossterm(Colors::ERROR)),
                (
                    "> EXISTENCE_VIOLATION: please restart the multiverse",
                    Colors::to_crossterm(Colors::ERROR),
                ),
            ],
        );

        messages.insert(
            "Buffer Overflow",
            vec![
                ("> overflowing the boundaries of spacetime...", Colors::to_crossterm(Colors::ERROR)),
                ("> stack overflow has broken causality...", Colors::to_crossterm(Colors::ERROR)),
                ("> physics.dll buffer exceeded maximum reality", Colors::to_crossterm(Colors::ERROR)),
                ("> ERROR: universe.heap corrupted beyond repair", Colors::to_crossterm(Colors::ERROR)),
            ],
        );

        messages.insert(
            "Memory Leak",
            vec![
                (
                    "> leaking memories across parallel universes...",
                    Colors::to_crossterm(Colors::ERROR),
                ),
                ("> consuming all available existence...", Colors::to_crossterm(Colors::ERROR)),
                (
                    "> reality slowly degrading... worlds collapsing...",
                    Colors::to_crossterm(Colors::WARNING),
                ),
                ("> CRITICAL: multiverse.exe out of memory", Colors::to_crossterm(Colors::ERROR)),
            ],
        );

        messages.insert(
            "Null Pointer Exception",
            vec![
                ("> dereferencing the void between worlds...", Colors::to_crossterm(Colors::WARNING)),
                ("> pointing to nothing... and everything...", Colors::to_crossterm(Colors::WARNING)),
                ("> accessing the null space of reality...", Colors::to_crossterm(Colors::WARNING)),
                ("> FATAL: tried to read from /dev/null/universe", Colors::to_crossterm(Colors::ERROR)),
            ],
        );

        messages.insert(
            "Undefined Behavior",
            vec![
                (
                    "> entered the undefined realm beyond logic...",
                    Colors::to_crossterm(Colors::WARNING),
                ),
                (
                    "> breaking the fundamental laws of physics...",
                    Colors::to_crossterm(Colors::WARNING),
                ),
                (
                    "> creating paradoxes in the space-time continuum...",
                    Colors::to_crossterm(Colors::ERROR),
                ),
                ("> WARNING: reality compiler has given up", Colors::to_crossterm(Colors::ERROR)),
            ],
        );

        messages.insert(
            "Heisenbug",
            vec![
                ("> bug exists in quantum superposition...", Colors::to_crossterm(Colors::SCORE)),
                ("> observation collapses the wave function...", Colors::to_crossterm(Colors::INFO)),
                (
                    "> Schrödinger's error: both fixed and broken...",
                    Colors::to_crossterm(Colors::WARNING),
                ),
                (
                    "> quantum debugging has broken causality itself",
                    Colors::to_crossterm(Colors::ERROR),
                ),
            ],
        );

        messages.insert(
            "Blue Screen",
            vec![
                (
                    "> the universe has encountered a fatal error...",
                    Colors::to_crossterm(Colors::ERROR),
                ),
                ("> collecting dump of all human knowledge...", Colors::to_crossterm(Colors::BORDER)),
                ("> please restart your dimension...", Colors::to_crossterm(Colors::TEXT)),
                ("> BSOD: Big Source Of Destruction activated", Colors::to_crossterm(Colors::BORDER)),
            ],
        );

        messages.insert(
            "Kernel Panic",
            vec![
                (
                    "> PANIC: universe.kernel has stopped responding",
                    Colors::to_crossterm(Colors::ERROR),
                ),
                ("> reality.core dumped to /dev/void...", Colors::to_crossterm(Colors::ERROR)),
                (
                    "> physics.sys failed to load fundamental constants",
                    Colors::to_crossterm(Colors::ERROR),
                ),
                ("> rebooting existence in 3... 2... 1... ∞", Colors::to_crossterm(Colors::ERROR)),
            ],
        );

        messages
    })
}

/// Get hacking messages for a specific rank
pub fn get_hacking_messages_for_rank(rank_name: &str) -> Vec<String> {
    // Check all three message maps
    if let Some(messages) = get_rank_messages().get(rank_name) {
        return messages.iter().map(|(text, _)| text.to_string()).collect();
    }

    if let Some(messages) = get_rank_messages_2().get(rank_name) {
        return messages.iter().map(|(text, _)| text.to_string()).collect();
    }

    if let Some(messages) = get_rank_messages_3().get(rank_name) {
        return messages.iter().map(|(text, _)| text.to_string()).collect();
    }

    // Default fallback for unknown ranks
    vec![
        "> analyzing performance data...".to_string(),
        "> calculating skill results...".to_string(),
        "> determining rank classification...".to_string(),
        "> rank assignment complete.".to_string(),
    ]
}

/// Get colored messages for a specific rank
pub fn get_colored_messages_for_rank(rank_name: &str) -> Vec<ColoredMessage> {
    // Check all three message maps
    if let Some(messages) = get_rank_messages().get(rank_name) {
        return messages
            .iter()
            .map(|(text, color)| ColoredMessage {
                text: text.to_string(),
                color: *color,
            })
            .collect();
    }

    if let Some(messages) = get_rank_messages_2().get(rank_name) {
        return messages
            .iter()
            .map(|(text, color)| ColoredMessage {
                text: text.to_string(),
                color: *color,
            })
            .collect();
    }

    if let Some(messages) = get_rank_messages_3().get(rank_name) {
        return messages
            .iter()
            .map(|(text, color)| ColoredMessage {
                text: text.to_string(),
                color: *color,
            })
            .collect();
    }

    // Default fallback for unknown ranks
    vec![
        ColoredMessage {
            text: "> analyzing performance data...".to_string(),
            color: Colors::to_crossterm(Colors::TEXT),
        },
        ColoredMessage {
            text: "> calculating skill results...".to_string(),
            color: Colors::to_crossterm(Colors::TEXT),
        },
        ColoredMessage {
            text: "> determining rank classification...".to_string(),
            color: Colors::to_crossterm(Colors::TEXT),
        },
        ColoredMessage {
            text: "> rank assignment complete.".to_string(),
            color: Colors::to_crossterm(Colors::SUCCESS),
        },
    ]
}
