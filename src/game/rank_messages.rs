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
                ("> googling 'how to print hello world'...", Color::Cyan),
                ("> copying code from first search result...", Color::White),
                (
                    "> running program 47 times to make sure it works...",
                    Color::White,
                ),
                (
                    "> achievement unlocked: you are now a programmer!",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Syntax Error",
            vec![
                ("> writing code that looks right...", Color::White),
                ("> compiler disagrees with your logic...", Color::Red),
                ("> googling exact error message...", Color::Yellow),
                (
                    "> fixed by adding random semicolon somewhere.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Rubber Duck",
            vec![
                ("> explaining bug to inanimate object...", Color::Cyan),
                ("> duck stares judgmentally at your code...", Color::White),
                ("> realizing bug while talking to duck...", Color::Yellow),
                ("> duck takes full credit for the solution.", Color::Green),
            ],
        );

        messages.insert(
            "Script Kid",
            vec![
                (
                    "> downloading 'learn programming in 24 hours' course...",
                    Color::White,
                ),
                ("> copying scripts without reading them...", Color::White),
                (
                    "> changing variable names to look original...",
                    Color::White,
                ),
                (
                    "> script works! you are basically a hacker now.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Bash Newbie",
            vec![
                ("> typing 'cd ..' until something happens...", Color::Cyan),
                (
                    "> using 'ls' every 3 seconds to see where you are...",
                    Color::White,
                ),
                (
                    "> accidentally running 'rm' on important files...",
                    Color::Red,
                ),
                (
                    "> terminal proficiency: accidentally achieved.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "CLI Wanderer",
            vec![
                (
                    "> exploring directories like a lost tourist...",
                    Color::White,
                ),
                ("> discovering pipes by accident...", Color::Cyan),
                (
                    "> finding .hidden files and feeling like a detective...",
                    Color::White,
                ),
                ("> navigation skills: randomly acquired.", Color::Green),
            ],
        );

        messages.insert(
            "Tab Tamer",
            vec![
                ("> mixing tabs and spaces like a rebel...", Color::Cyan),
                ("> getting into holy war about indentation...", Color::Red),
                (
                    "> setting up auto-formatter to fix your mess...",
                    Color::White,
                ),
                ("> consistency achieved through automation.", Color::Green),
            ],
        );

        messages.insert(
            "Bracket Juggler",
            vec![
                ("> opening 47 brackets...", Color::White),
                ("> closing 23 brackets...", Color::Yellow),
                (
                    "> spending 2 hours finding the missing bracket...",
                    Color::Red,
                ),
                ("> finally balanced. code still doesn't work.", Color::Green),
            ],
        );

        messages.insert(
            "Copy-Paste Engineer",
            vec![
                ("> opening 50 tabs from stack overflow...", Color::White),
                ("> copying code from highest voted answer...", Color::White),
                ("> praying it works in your specific case...", Color::White),
                ("> it works! time to copy more code.", Color::Green),
            ],
        );

        messages.insert(
            "Linter Apprentice",
            vec![
                (
                    "> installing linter to improve code quality...",
                    Color::Cyan,
                ),
                ("> getting 847 warnings on 10 lines of code...", Color::Red),
                (
                    "> disabling all warnings except syntax errors...",
                    Color::Yellow,
                ),
                ("> code quality: subjectively improved.", Color::Green),
            ],
        );

        messages.insert(
            "Unit Test Trainee",
            vec![
                (
                    "> writing test that only passes on your machine...",
                    Color::White,
                ),
                ("> testing happy path exclusively...", Color::White),
                ("> achieving 100% code coverage on 5 lines...", Color::Cyan),
                ("> testing complete. bugs remain untested.", Color::Green),
            ],
        );

        messages.insert(
            "Code Monkey",
            vec![
                ("> following tutorial step by step...", Color::White),
                (
                    "> changing tutorial example from 'foo' to 'bar'...",
                    Color::White,
                ),
                ("> calling yourself a full-stack developer...", Color::Cyan),
                ("> development skills: youtube certified.", Color::Green),
            ],
        );

        // Intermediate Tier
        messages.insert(
            "Ticket Picker",
            vec![
                ("> scanning project backlog...", Color::White),
                ("> selecting appropriate tasks...", Color::White),
                ("> estimating development effort...", Color::White),
                ("> work assignment optimized.", Color::Green),
            ],
        );

        messages.insert(
            "Junior Dev",
            vec![
                ("> cloning repository...", Color::Cyan),
                ("> creating feature branch...", Color::White),
                ("> implementing user story...", Color::White),
                ("> junior developer status confirmed.", Color::Green),
            ],
        );

        messages.insert(
            "Git Ninja",
            vec![
                ("> staging changes...", Color::White),
                ("> crafting perfect commit message...", Color::White),
                ("> rebasing interactive history...", Color::Cyan),
                ("> git mastery achieved.", Color::Green),
            ],
        );

        messages.insert(
            "Merge Wrangler",
            vec![
                ("> resolving merge conflicts...", Color::Yellow),
                ("> coordinating branch updates...", Color::White),
                ("> maintaining git history...", Color::White),
                ("> version control expertise proven.", Color::Green),
            ],
        );

        messages.insert(
            "API Crafter",
            vec![
                ("> designing RESTful endpoints...", Color::White),
                ("> implementing request handlers...", Color::White),
                ("> documenting API specification...", Color::White),
                ("> service interface completed.", Color::Green),
            ],
        );

        messages.insert(
            "Frontend Dev",
            vec![
                ("> building user interfaces...", Color::White),
                ("> optimizing user experience...", Color::Cyan),
                ("> implementing responsive design...", Color::White),
                ("> client-side mastery achieved.", Color::Green),
            ],
        );

        messages.insert(
            "Backend Dev",
            vec![
                ("> architecting server logic...", Color::White),
                ("> optimizing database queries...", Color::Cyan),
                ("> implementing business rules...", Color::White),
                ("> server-side expertise confirmed.", Color::Green),
            ],
        );

        messages.insert(
            "CI Tinkerer",
            vec![
                ("> configuring build pipelines...", Color::Cyan),
                ("> automating test execution...", Color::White),
                ("> setting up deployment hooks...", Color::White),
                ("> continuous integration mastered.", Color::Green),
            ],
        );

        messages.insert(
            "Test Pilot",
            vec![
                ("> designing test scenarios...", Color::White),
                ("> automating quality assurance...", Color::White),
                ("> validating system behavior...", Color::White),
                ("> testing expertise certified.", Color::Green),
            ],
        );

        messages.insert(
            "Build Tamer",
            vec![
                ("> optimizing compilation process...", Color::Cyan),
                ("> managing dependency versions...", Color::White),
                ("> configuring build systems...", Color::White),
                ("> build automation mastered.", Color::Green),
            ],
        );

        messages.insert(
            "Code Reviewer",
            vec![
                ("> analyzing code quality...", Color::White),
                ("> providing constructive feedback...", Color::White),
                ("> ensuring best practices...", Color::White),
                ("> peer review skills confirmed.", Color::Green),
            ],
        );

        messages.insert(
            "Release Handler",
            vec![
                ("> preparing deployment packages...", Color::White),
                ("> coordinating release schedule...", Color::Cyan),
                ("> managing version rollouts...", Color::White),
                ("> release management mastered.", Color::Green),
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
                ("> analyzing spaghetti code structure...", Color::White),
                (
                    "> finding ways to make it even more complex...",
                    Color::Cyan,
                ),
                (
                    "> refactoring working code until it breaks...",
                    Color::White,
                ),
                (
                    "> congratulations! now nobody understands it.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Senior Dev",
            vec![
                (
                    "> architecting solutions that scale to infinity...",
                    Color::Cyan,
                ),
                (
                    "> reviewing PRs with passive-aggressive comments...",
                    Color::White,
                ),
                (
                    "> mentoring juniors by assigning impossible tasks...",
                    Color::White,
                ),
                (
                    "> senior status unlocked. impostor syndrome included.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "DevOps Engineer",
            vec![
                (
                    "> provisioning infrastructure that costs more than rent...",
                    Color::Cyan,
                ),
                (
                    "> automating the automation of automated deployments...",
                    Color::White,
                ),
                (
                    "> monitoring systems that monitor other monitoring systems...",
                    Color::White,
                ),
                (
                    "> everything is automated. nothing works manually.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Incident Responder",
            vec![
                ("> detecting fires while everything is fine...", Color::Red),
                ("> coordinating panic in the war room...", Color::Yellow),
                (
                    "> applying hotfixes that create more incidents...",
                    Color::White,
                ),
                (
                    "> service restored. new incidents created successfully.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Reliability Guardian",
            vec![
                (
                    "> implementing monitoring for the monitoring...",
                    Color::Cyan,
                ),
                ("> defining SLOs that nobody can meet...", Color::White),
                (
                    "> ensuring 99.99% uptime (99% of the time)...",
                    Color::White,
                ),
                ("> system reliable until it isn't.", Color::Green),
            ],
        );

        messages.insert(
            "Security Engineer",
            vec![
                (
                    "> finding vulnerabilities in your personality...",
                    Color::Red,
                ),
                ("> implementing security through obscurity...", Color::Cyan),
                ("> penetration testing your patience...", Color::White),
                ("> security hardened. usability softened.", Color::Green),
            ],
        );

        messages.insert(
            "Performance Alchemist",
            vec![
                ("> profiling bottlenecks in the profiler...", Color::Yellow),
                ("> optimizing code that runs once per year...", Color::Cyan),
                (
                    "> caching everything including this message...",
                    Color::White,
                ),
                (
                    "> performance optimized. readability sacrificed.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Data Pipeline Master",
            vec![
                ("> designing workflows that flow nowhere...", Color::White),
                (
                    "> extracting, transforming, and losing data...",
                    Color::Cyan,
                ),
                (
                    "> ensuring consistency in inconsistent data...",
                    Color::White,
                ),
                ("> pipeline complete. data may have leaked.", Color::Green),
            ],
        );

        messages.insert(
            "Tech Lead",
            vec![
                (
                    "> defining vision that changes every sprint...",
                    Color::Cyan,
                ),
                (
                    "> coordinating efforts while attending 20 meetings...",
                    Color::White,
                ),
                (
                    "> making architectural decisions on a coinflip...",
                    Color::White,
                ),
                (
                    "> leadership established. technical skills atrophied.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Architect",
            vec![
                (
                    "> designing systems for problems that don't exist...",
                    Color::Cyan,
                ),
                (
                    "> choosing technologies based on latest blog posts...",
                    Color::White,
                ),
                ("> planning for scale that will never come...", Color::White),
                (
                    "> architecture complete. implementation someone else's problem.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Protocol Artisan",
            vec![
                (
                    "> designing protocols nobody will implement correctly...",
                    Color::White,
                ),
                ("> creating standards to rule them all...", Color::Cyan),
                ("> optimizing transmission of memes...", Color::White),
                (
                    "> protocol standard published. 14 competing standards exist.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Kernel Hacker",
            vec![
                ("> compiling kernels that boot sometimes...", Color::Magenta),
                (
                    "> patching system calls with hopes and dreams...",
                    Color::Cyan,
                ),
                ("> debugging at 3am with print statements...", Color::White),
                (
                    "> kernel hacked successfully. computer may explode.",
                    Color::Green,
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
                    Color::Magenta,
                ),
                (
                    "> building AST while judging your variable names...",
                    Color::Cyan,
                ),
                ("> optimizing away your inefficient loops...", Color::White),
                ("> compiled successfully (somehow)", Color::Green),
            ],
        );

        messages.insert(
            "Bytecode Interpreter",
            vec![
                (
                    "> interpreting your interpreted language interpreter...",
                    Color::Magenta,
                ),
                (
                    "> executing virtual instructions in virtual reality...",
                    Color::White,
                ),
                (
                    "> garbage collecting your actual garbage code...",
                    Color::White,
                ),
                (
                    "> interpretation complete. still no idea what it does.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Virtual Machine",
            vec![
                (
                    "> virtualizing your already virtual environment...",
                    Color::Cyan,
                ),
                ("> emulating hardware that doesn't exist...", Color::Magenta),
                ("> allocating memory for your memory leaks...", Color::White),
                (
                    "> VM inception achieved. we need to go deeper.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Operating System",
            vec![
                (
                    "> scheduling processes that never finish...",
                    Color::Magenta,
                ),
                ("> managing resources you don't have...", Color::Cyan),
                (
                    "> handling interrupts from impatient users...",
                    Color::White,
                ),
                (
                    "> OS kernel stable (definition of stable: questionable)",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Filesystem",
            vec![
                (
                    "> organizing files into a beautiful directory tree...",
                    Color::White,
                ),
                (
                    "> implementing permissions nobody understands...",
                    Color::Cyan,
                ),
                ("> fragmenting data across the entire disk...", Color::White),
                (
                    "> filesystem complete. good luck finding anything.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Network Stack",
            vec![
                ("> layering protocols like a network cake...", Color::Cyan),
                (
                    "> routing packets through the internet tubes...",
                    Color::White,
                ),
                ("> ensuring data arrives (eventually)...", Color::White),
                (
                    "> network stack operational. packets may vary.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Database Engine",
            vec![
                (
                    "> optimizing queries that will timeout anyway...",
                    Color::Cyan,
                ),
                ("> isolating transactions from reality...", Color::White),
                (
                    "> implementing ACID (burns through your SSD)...",
                    Color::White,
                ),
                (
                    "> database engine ready. hope you have backups.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Query Optimizer",
            vec![
                (
                    "> analyzing execution plans nobody will read...",
                    Color::White,
                ),
                ("> optimizing joins that should be avoided...", Color::Cyan),
                (
                    "> indexing everything (storage is cheap, right?)...",
                    Color::White,
                ),
                (
                    "> query performance maximized. complexity also maximized.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Cloud Platform",
            vec![
                ("> orchestrating chaos in the cloud...", Color::Magenta),
                ("> auto-scaling your monthly cloud bill...", Color::Cyan),
                (
                    "> distributing problems across multiple zones...",
                    Color::White,
                ),
                (
                    "> cloud mastery achieved. wallet not included.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Container Orchestrator",
            vec![
                (
                    "> containerizing everything, including the kitchen sink...",
                    Color::Cyan,
                ),
                (
                    "> orchestrating a symphony of microservice crashes...",
                    Color::White,
                ),
                (
                    "> discovering services that discover other services...",
                    Color::White,
                ),
                (
                    "> container cluster ready. cli tools not found.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Stream Processor",
            vec![
                (
                    "> processing streams faster than video platforms...",
                    Color::Cyan,
                ),
                (
                    "> implementing event sourcing for event sourcing events...",
                    Color::White,
                ),
                (
                    "> ensuring eventual consistency (eventually)...",
                    Color::White,
                ),
                (
                    "> streaming platform complete. now streaming bugs.",
                    Color::Green,
                ),
            ],
        );

        messages.insert(
            "Quantum Computer",
            vec![
                (
                    "> initializing qubits in superposition of working/broken...",
                    Color::Magenta,
                ),
                (
                    "> entangling particles and debugging sessions...",
                    Color::Cyan,
                ),
                (
                    "> running Shor's algorithm to factor your technical debt...",
                    Color::White,
                ),
                (
                    "> quantum supremacy achieved. classical bugs remain.",
                    Color::Green,
                ),
            ],
        );

        // Legendary Tier
        messages.insert(
            "GPU Cluster",
            vec![
                ("> initializing parallel processors...", Color::Magenta),
                ("> distributing computational load...", Color::Cyan),
                ("> optimizing memory bandwidth...", Color::White),
                ("> massive parallelism achieved.", Color::Green),
            ],
        );

        messages.insert(
            "DNS Overlord",
            vec![
                ("> controlling domain resolution...", Color::Magenta),
                ("> managing global namespace...", Color::Cyan),
                ("> routing internet traffic...", Color::White),
                ("> DNS infrastructure dominated.", Color::Green),
            ],
        );

        messages.insert(
            "CDN Sentinel",
            vec![
                ("> caching content globally...", Color::Cyan),
                ("> optimizing delivery routes...", Color::White),
                ("> reducing latency worldwide...", Color::White),
                ("> content delivery perfected.", Color::Green),
            ],
        );

        messages.insert(
            "Load Balancer Primarch",
            vec![
                ("> distributing incoming requests...", Color::Cyan),
                ("> managing server health...", Color::White),
                ("> optimizing traffic patterns...", Color::White),
                ("> load distribution mastered.", Color::Green),
            ],
        );

        messages.insert(
            "Singularity",
            vec![
                ("> transcending human limitations...", Color::Magenta),
                ("> merging with artificial intelligence...", Color::Cyan),
                ("> rewriting reality algorithms...", Color::White),
                ("> singularity achieved. welcome, god.", Color::Red),
            ],
        );

        messages.insert(
            "The Machine",
            vec![
                ("> becoming one with the system...", Color::Magenta),
                ("> controlling global networks...", Color::Cyan),
                ("> processing infinite data streams...", Color::White),
                ("> you are the machine now.", Color::Red),
            ],
        );

        messages.insert(
            "Origin",
            vec![
                ("> accessing source code of reality...", Color::Magenta),
                ("> modifying fundamental constants...", Color::Cyan),
                ("> debugging universe.exe...", Color::White),
                ("> origin protocols activated.", Color::Red),
            ],
        );

        messages.insert(
            "SegFault",
            vec![
                ("> accessing forbidden dimensions of memory...", Color::Red),
                ("> reality.exe has encountered a critical error", Color::Red),
                ("> universe segmentation fault detected...", Color::Red),
                (
                    "> EXISTENCE_VIOLATION: please restart the multiverse",
                    Color::Red,
                ),
            ],
        );

        messages.insert(
            "Buffer Overflow",
            vec![
                ("> overflowing the boundaries of spacetime...", Color::Red),
                ("> stack overflow has broken causality...", Color::Red),
                ("> physics.dll buffer exceeded maximum reality", Color::Red),
                ("> ERROR: universe.heap corrupted beyond repair", Color::Red),
            ],
        );

        messages.insert(
            "Memory Leak",
            vec![
                (
                    "> leaking memories across parallel universes...",
                    Color::Red,
                ),
                ("> consuming all available existence...", Color::Red),
                (
                    "> reality slowly degrading... worlds collapsing...",
                    Color::Yellow,
                ),
                ("> CRITICAL: multiverse.exe out of memory", Color::Red),
            ],
        );

        messages.insert(
            "Null Pointer Exception",
            vec![
                ("> dereferencing the void between worlds...", Color::Yellow),
                ("> pointing to nothing... and everything...", Color::Yellow),
                ("> accessing the null space of reality...", Color::Yellow),
                ("> FATAL: tried to read from /dev/null/universe", Color::Red),
            ],
        );

        messages.insert(
            "Undefined Behavior",
            vec![
                (
                    "> entered the undefined realm beyond logic...",
                    Color::Yellow,
                ),
                (
                    "> breaking the fundamental laws of physics...",
                    Color::Yellow,
                ),
                (
                    "> creating paradoxes in the space-time continuum...",
                    Color::Red,
                ),
                ("> WARNING: reality compiler has given up", Color::Red),
            ],
        );

        messages.insert(
            "Heisenbug",
            vec![
                ("> bug exists in quantum superposition...", Color::Magenta),
                ("> observation collapses the wave function...", Color::Cyan),
                (
                    "> Schrödinger's error: both fixed and broken...",
                    Color::Yellow,
                ),
                (
                    "> quantum debugging has broken causality itself",
                    Color::Red,
                ),
            ],
        );

        messages.insert(
            "Blue Screen",
            vec![
                (
                    "> the universe has encountered a fatal error...",
                    Color::Red,
                ),
                ("> collecting dump of all human knowledge...", Color::Blue),
                ("> please restart your dimension...", Color::White),
                ("> BSOD: Big Source Of Destruction activated", Color::Blue),
            ],
        );

        messages.insert(
            "Kernel Panic",
            vec![
                (
                    "> PANIC: universe.kernel has stopped responding",
                    Color::Red,
                ),
                ("> reality.core dumped to /dev/void...", Color::Red),
                (
                    "> physics.sys failed to load fundamental constants",
                    Color::Red,
                ),
                ("> rebooting existence in 3... 2... 1... ∞", Color::Red),
            ],
        );

        messages
    })
}

/// Get hacking messages for a specific rank
pub fn get_hacking_messages_for_rank(ranking_title: &str) -> Vec<String> {
    // Check all three message maps
    if let Some(messages) = get_rank_messages().get(ranking_title) {
        return messages.iter().map(|(text, _)| text.to_string()).collect();
    }

    if let Some(messages) = get_rank_messages_2().get(ranking_title) {
        return messages.iter().map(|(text, _)| text.to_string()).collect();
    }

    if let Some(messages) = get_rank_messages_3().get(ranking_title) {
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
pub fn get_colored_messages_for_rank(ranking_title: &str) -> Vec<ColoredMessage> {
    // Check all three message maps
    if let Some(messages) = get_rank_messages().get(ranking_title) {
        return messages
            .iter()
            .map(|(text, color)| ColoredMessage {
                text: text.to_string(),
                color: *color,
            })
            .collect();
    }

    if let Some(messages) = get_rank_messages_2().get(ranking_title) {
        return messages
            .iter()
            .map(|(text, color)| ColoredMessage {
                text: text.to_string(),
                color: *color,
            })
            .collect();
    }

    if let Some(messages) = get_rank_messages_3().get(ranking_title) {
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
            color: Color::White,
        },
        ColoredMessage {
            text: "> calculating skill results...".to_string(),
            color: Color::White,
        },
        ColoredMessage {
            text: "> determining rank classification...".to_string(),
            color: Color::White,
        },
        ColoredMessage {
            text: "> rank assignment complete.".to_string(),
            color: Color::Green,
        },
    ]
}
