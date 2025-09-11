use crate::ui::Colors;
use ratatui::style::Color;
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
                ("> googling 'how to print hello world'...", Colors::INFO),
                ("> copying code from first search result...", Colors::TEXT),
                (
                    "> running program 47 times to make sure it works...",
                    Colors::TEXT,
                ),
                (
                    "> achievement unlocked: you are now a programmer!",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Syntax Error",
            vec![
                ("> writing code that looks right...", Colors::TEXT),
                ("> compiler disagrees with your logic...", Colors::ERROR),
                ("> googling exact error message...", Colors::WARNING),
                (
                    "> fixed by adding random semicolon somewhere.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Rubber Duck",
            vec![
                ("> explaining bug to inanimate object...", Colors::INFO),
                ("> duck stares judgmentally at your code...", Colors::TEXT),
                ("> realizing bug while talking to duck...", Colors::WARNING),
                (
                    "> duck takes full credit for the solution.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Script Kid",
            vec![
                (
                    "> downloading 'learn programming in 24 hours' course...",
                    Colors::TEXT,
                ),
                ("> copying scripts without reading them...", Colors::TEXT),
                (
                    "> changing variable names to look original...",
                    Colors::TEXT,
                ),
                (
                    "> script works! you are basically a hacker now.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Bash Newbie",
            vec![
                ("> typing 'cd ..' until something happens...", Colors::INFO),
                (
                    "> using 'ls' every 3 seconds to see where you are...",
                    Colors::TEXT,
                ),
                (
                    "> accidentally running 'rm' on important files...",
                    Colors::ERROR,
                ),
                (
                    "> terminal proficiency: accidentally achieved.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "CLI Wanderer",
            vec![
                (
                    "> exploring directories like a lost tourist...",
                    Colors::TEXT,
                ),
                ("> discovering pipes by accident...", Colors::INFO),
                (
                    "> finding .hidden files and feeling like a detective...",
                    Colors::TEXT,
                ),
                ("> navigation skills: randomly acquired.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "Tab Tamer",
            vec![
                ("> mixing tabs and spaces like a rebel...", Colors::INFO),
                (
                    "> getting into holy war about indentation...",
                    Colors::ERROR,
                ),
                (
                    "> setting up auto-formatter to fix your mess...",
                    Colors::TEXT,
                ),
                (
                    "> consistency achieved through automation.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Bracket Juggler",
            vec![
                ("> opening 47 brackets...", Colors::TEXT),
                ("> closing 23 brackets...", Colors::WARNING),
                (
                    "> spending 2 hours finding the missing bracket...",
                    Colors::ERROR,
                ),
                (
                    "> finally balanced. code still doesn't work.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Copy-Paste Engineer",
            vec![
                ("> opening 50 tabs from stack overflow...", Colors::TEXT),
                ("> copying code from highest voted answer...", Colors::TEXT),
                ("> praying it works in your specific case...", Colors::TEXT),
                ("> it works! time to copy more code.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "Linter Apprentice",
            vec![
                (
                    "> installing linter to improve code quality...",
                    Colors::INFO,
                ),
                (
                    "> getting 847 warnings on 10 lines of code...",
                    Colors::ERROR,
                ),
                (
                    "> disabling all warnings except syntax errors...",
                    Colors::WARNING,
                ),
                ("> code quality: subjectively improved.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "Unit Test Trainee",
            vec![
                (
                    "> writing test that only passes on your machine...",
                    Colors::TEXT,
                ),
                ("> testing happy path exclusively...", Colors::TEXT),
                ("> achieving 100% code coverage on 5 lines...", Colors::INFO),
                ("> testing complete. bugs remain untested.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "Code Monkey",
            vec![
                ("> following tutorial step by step...", Colors::TEXT),
                (
                    "> changing tutorial example from 'foo' to 'bar'...",
                    Colors::TEXT,
                ),
                ("> calling yourself a full-stack developer...", Colors::INFO),
                ("> development skills: youtube certified.", Colors::SUCCESS),
            ],
        );

        // Intermediate Tier
        messages.insert(
            "Ticket Picker",
            vec![
                ("> scanning project backlog...", Colors::TEXT),
                ("> selecting appropriate tasks...", Colors::TEXT),
                ("> estimating development effort...", Colors::TEXT),
                ("> work assignment optimized.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "Junior Dev",
            vec![
                ("> cloning repository...", Colors::INFO),
                ("> creating feature branch...", Colors::TEXT),
                ("> implementing user story...", Colors::TEXT),
                ("> junior developer status confirmed.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "Git Ninja",
            vec![
                ("> staging changes...", Colors::TEXT),
                ("> crafting perfect commit message...", Colors::TEXT),
                ("> rebasing interactive history...", Colors::INFO),
                ("> git mastery achieved.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "Merge Wrangler",
            vec![
                ("> resolving merge conflicts...", Colors::WARNING),
                ("> coordinating branch updates...", Colors::TEXT),
                ("> maintaining git history...", Colors::TEXT),
                ("> version control expertise proven.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "API Crafter",
            vec![
                ("> designing RESTful endpoints...", Colors::TEXT),
                ("> implementing request handlers...", Colors::TEXT),
                ("> documenting API specification...", Colors::TEXT),
                ("> service interface completed.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "Frontend Dev",
            vec![
                ("> building user interfaces...", Colors::TEXT),
                ("> optimizing user experience...", Colors::INFO),
                ("> implementing responsive design...", Colors::TEXT),
                ("> client-side mastery achieved.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "Backend Dev",
            vec![
                ("> architecting server logic...", Colors::TEXT),
                ("> optimizing database queries...", Colors::INFO),
                ("> implementing business rules...", Colors::TEXT),
                ("> server-side expertise confirmed.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "CI Tinkerer",
            vec![
                ("> configuring build pipelines...", Colors::INFO),
                ("> automating test execution...", Colors::TEXT),
                ("> setting up deployment hooks...", Colors::TEXT),
                ("> continuous integration mastered.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "Test Pilot",
            vec![
                ("> designing test scenarios...", Colors::TEXT),
                ("> automating quality assurance...", Colors::TEXT),
                ("> validating system behavior...", Colors::TEXT),
                ("> testing expertise certified.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "Build Tamer",
            vec![
                ("> optimizing compilation process...", Colors::INFO),
                ("> managing dependency versions...", Colors::TEXT),
                ("> configuring build systems...", Colors::TEXT),
                ("> build automation mastered.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "Code Reviewer",
            vec![
                ("> analyzing code quality...", Colors::TEXT),
                ("> providing constructive feedback...", Colors::TEXT),
                ("> ensuring best practices...", Colors::TEXT),
                ("> peer review skills confirmed.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "Release Handler",
            vec![
                ("> preparing deployment packages...", Colors::TEXT),
                ("> coordinating release schedule...", Colors::INFO),
                ("> managing version rollouts...", Colors::TEXT),
                ("> release management mastered.", Colors::SUCCESS),
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
                ("> analyzing spaghetti code structure...", Colors::TEXT),
                (
                    "> finding ways to make it even more complex...",
                    Colors::INFO,
                ),
                (
                    "> refactoring working code until it breaks...",
                    Colors::TEXT,
                ),
                (
                    "> congratulations! now nobody understands it.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Senior Dev",
            vec![
                (
                    "> architecting solutions that scale to infinity...",
                    Colors::INFO,
                ),
                (
                    "> reviewing PRs with passive-aggressive comments...",
                    Colors::TEXT,
                ),
                (
                    "> mentoring juniors by assigning impossible tasks...",
                    Colors::TEXT,
                ),
                (
                    "> senior status unlocked. impostor syndrome included.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "DevOps Engineer",
            vec![
                (
                    "> provisioning infrastructure that costs more than rent...",
                    Colors::INFO,
                ),
                (
                    "> automating the automation of automated deployments...",
                    Colors::TEXT,
                ),
                (
                    "> monitoring systems that monitor other monitoring systems...",
                    Colors::TEXT,
                ),
                (
                    "> everything is automated. nothing works manually.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Incident Responder",
            vec![
                (
                    "> detecting fires while everything is fine...",
                    Colors::ERROR,
                ),
                ("> coordinating panic in the war room...", Colors::WARNING),
                (
                    "> applying hotfixes that create more incidents...",
                    Colors::TEXT,
                ),
                (
                    "> service restored. new incidents created successfully.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Reliability Guardian",
            vec![
                (
                    "> implementing monitoring for the monitoring...",
                    Colors::INFO,
                ),
                ("> defining SLOs that nobody can meet...", Colors::TEXT),
                (
                    "> ensuring 99.99% uptime (99% of the time)...",
                    Colors::TEXT,
                ),
                ("> system reliable until it isn't.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "Security Engineer",
            vec![
                (
                    "> finding vulnerabilities in your personality...",
                    Colors::ERROR,
                ),
                ("> implementing security through obscurity...", Colors::INFO),
                ("> penetration testing your patience...", Colors::TEXT),
                ("> security hardened. usability softened.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "Performance Alchemist",
            vec![
                (
                    "> profiling bottlenecks in the profiler...",
                    Colors::WARNING,
                ),
                ("> optimizing code that runs once per year...", Colors::INFO),
                (
                    "> caching everything including this message...",
                    Colors::TEXT,
                ),
                (
                    "> performance optimized. readability sacrificed.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Data Pipeline Master",
            vec![
                ("> designing workflows that flow nowhere...", Colors::TEXT),
                (
                    "> extracting, transforming, and losing data...",
                    Colors::INFO,
                ),
                (
                    "> ensuring consistency in inconsistent data...",
                    Colors::TEXT,
                ),
                (
                    "> pipeline complete. data may have leaked.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Tech Lead",
            vec![
                (
                    "> defining vision that changes every sprint...",
                    Colors::INFO,
                ),
                (
                    "> coordinating efforts while attending 20 meetings...",
                    Colors::TEXT,
                ),
                (
                    "> making architectural decisions on a coinflip...",
                    Colors::TEXT,
                ),
                (
                    "> leadership established. technical skills atrophied.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Architect",
            vec![
                (
                    "> designing systems for problems that don't exist...",
                    Colors::INFO,
                ),
                (
                    "> choosing technologies based on latest blog posts...",
                    Colors::TEXT,
                ),
                ("> planning for scale that will never come...", Colors::TEXT),
                (
                    "> architecture complete. implementation someone else's problem.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Protocol Artisan",
            vec![
                (
                    "> designing protocols nobody will implement correctly...",
                    Colors::TEXT,
                ),
                ("> creating standards to rule them all...", Colors::INFO),
                ("> optimizing transmission of memes...", Colors::TEXT),
                (
                    "> protocol standard published. 14 competing standards exist.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Kernel Hacker",
            vec![
                ("> compiling kernels that boot sometimes...", Colors::SCORE),
                (
                    "> patching system calls with hopes and dreams...",
                    Colors::INFO,
                ),
                ("> debugging at 3am with print statements...", Colors::TEXT),
                (
                    "> kernel hacked successfully. computer may explode.",
                    Colors::SUCCESS,
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
                    Colors::SCORE,
                ),
                (
                    "> building AST while judging your variable names...",
                    Colors::INFO,
                ),
                ("> optimizing away your inefficient loops...", Colors::TEXT),
                ("> compiled successfully (somehow)", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "Bytecode Interpreter",
            vec![
                (
                    "> interpreting your interpreted language interpreter...",
                    Colors::SCORE,
                ),
                (
                    "> executing virtual instructions in virtual reality...",
                    Colors::TEXT,
                ),
                (
                    "> garbage collecting your actual garbage code...",
                    Colors::TEXT,
                ),
                (
                    "> interpretation complete. still no idea what it does.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Virtual Machine",
            vec![
                (
                    "> virtualizing your already virtual environment...",
                    Colors::INFO,
                ),
                ("> emulating hardware that doesn't exist...", Colors::SCORE),
                ("> allocating memory for your memory leaks...", Colors::TEXT),
                (
                    "> VM inception achieved. we need to go deeper.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Operating System",
            vec![
                ("> scheduling processes that never finish...", Colors::SCORE),
                ("> managing resources you don't have...", Colors::INFO),
                (
                    "> handling interrupts from impatient users...",
                    Colors::TEXT,
                ),
                (
                    "> OS kernel stable (definition of stable: questionable)",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Filesystem",
            vec![
                (
                    "> organizing files into a beautiful directory tree...",
                    Colors::TEXT,
                ),
                (
                    "> implementing permissions nobody understands...",
                    Colors::INFO,
                ),
                ("> fragmenting data across the entire disk...", Colors::TEXT),
                (
                    "> filesystem complete. good luck finding anything.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Network Stack",
            vec![
                ("> layering protocols like a network cake...", Colors::INFO),
                (
                    "> routing packets through the internet tubes...",
                    Colors::TEXT,
                ),
                ("> ensuring data arrives (eventually)...", Colors::TEXT),
                (
                    "> network stack operational. packets may vary.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Database Engine",
            vec![
                (
                    "> optimizing queries that will timeout anyway...",
                    Colors::INFO,
                ),
                ("> isolating transactions from reality...", Colors::TEXT),
                (
                    "> implementing ACID (burns through your SSD)...",
                    Colors::TEXT,
                ),
                (
                    "> database engine ready. hope you have backups.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Query Optimizer",
            vec![
                (
                    "> analyzing execution plans nobody will read...",
                    Colors::TEXT,
                ),
                ("> optimizing joins that should be avoided...", Colors::INFO),
                (
                    "> indexing everything (storage is cheap, right?)...",
                    Colors::TEXT,
                ),
                (
                    "> query performance maximized. complexity also maximized.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Cloud Platform",
            vec![
                ("> orchestrating chaos in the cloud...", Colors::SCORE),
                ("> auto-scaling your monthly cloud bill...", Colors::INFO),
                (
                    "> distributing problems across multiple zones...",
                    Colors::TEXT,
                ),
                (
                    "> cloud mastery achieved. wallet not included.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Container Orchestrator",
            vec![
                (
                    "> containerizing everything, including the kitchen sink...",
                    Colors::INFO,
                ),
                (
                    "> orchestrating a symphony of microservice crashes...",
                    Colors::TEXT,
                ),
                (
                    "> discovering services that discover other services...",
                    Colors::TEXT,
                ),
                (
                    "> container cluster ready. cli tools not found.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Stream Processor",
            vec![
                (
                    "> processing streams faster than video platforms...",
                    Colors::INFO,
                ),
                (
                    "> implementing event sourcing for event sourcing events...",
                    Colors::TEXT,
                ),
                (
                    "> ensuring eventual consistency (eventually)...",
                    Colors::TEXT,
                ),
                (
                    "> streaming platform complete. now streaming bugs.",
                    Colors::SUCCESS,
                ),
            ],
        );

        messages.insert(
            "Quantum Computer",
            vec![
                (
                    "> initializing qubits in superposition of working/broken...",
                    Colors::SCORE,
                ),
                (
                    "> entangling particles and debugging sessions...",
                    Colors::INFO,
                ),
                (
                    "> running Shor's algorithm to factor your technical debt...",
                    Colors::TEXT,
                ),
                (
                    "> quantum supremacy achieved. classical bugs remain.",
                    Colors::SUCCESS,
                ),
            ],
        );

        // Legendary Tier
        messages.insert(
            "GPU Cluster",
            vec![
                ("> initializing parallel processors...", Colors::SCORE),
                ("> distributing computational load...", Colors::INFO),
                ("> optimizing memory bandwidth...", Colors::TEXT),
                ("> massive parallelism achieved.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "DNS Overlord",
            vec![
                ("> controlling domain resolution...", Colors::SCORE),
                ("> managing global namespace...", Colors::INFO),
                ("> routing internet traffic...", Colors::TEXT),
                ("> DNS infrastructure dominated.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "CDN Sentinel",
            vec![
                ("> caching content globally...", Colors::INFO),
                ("> optimizing delivery routes...", Colors::TEXT),
                ("> reducing latency worldwide...", Colors::TEXT),
                ("> content delivery perfected.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "Load Balancer Primarch",
            vec![
                ("> distributing incoming requests...", Colors::INFO),
                ("> managing server health...", Colors::TEXT),
                ("> optimizing traffic patterns...", Colors::TEXT),
                ("> load distribution mastered.", Colors::SUCCESS),
            ],
        );

        messages.insert(
            "Singularity",
            vec![
                ("> transcending human limitations...", Colors::SCORE),
                ("> merging with artificial intelligence...", Colors::INFO),
                ("> rewriting reality algorithms...", Colors::TEXT),
                ("> singularity achieved. welcome, god.", Colors::ERROR),
            ],
        );

        messages.insert(
            "The Machine",
            vec![
                ("> becoming one with the system...", Colors::SCORE),
                ("> controlling global networks...", Colors::INFO),
                ("> processing infinite data streams...", Colors::TEXT),
                ("> you are the machine now.", Colors::ERROR),
            ],
        );

        messages.insert(
            "Origin",
            vec![
                ("> accessing source code of reality...", Colors::SCORE),
                ("> modifying fundamental constants...", Colors::INFO),
                ("> debugging universe.exe...", Colors::TEXT),
                ("> origin protocols activated.", Colors::ERROR),
            ],
        );

        messages.insert(
            "SegFault",
            vec![
                (
                    "> accessing forbidden dimensions of memory...",
                    Colors::ERROR,
                ),
                (
                    "> reality.exe has encountered a critical error",
                    Colors::ERROR,
                ),
                ("> universe segmentation fault detected...", Colors::ERROR),
                (
                    "> EXISTENCE_VIOLATION: please restart the multiverse",
                    Colors::ERROR,
                ),
            ],
        );

        messages.insert(
            "Buffer Overflow",
            vec![
                (
                    "> overflowing the boundaries of spacetime...",
                    Colors::ERROR,
                ),
                ("> stack overflow has broken causality...", Colors::ERROR),
                (
                    "> physics.dll buffer exceeded maximum reality",
                    Colors::ERROR,
                ),
                (
                    "> ERROR: universe.heap corrupted beyond repair",
                    Colors::ERROR,
                ),
            ],
        );

        messages.insert(
            "Memory Leak",
            vec![
                (
                    "> leaking memories across parallel universes...",
                    Colors::ERROR,
                ),
                ("> consuming all available existence...", Colors::ERROR),
                (
                    "> reality slowly degrading... worlds collapsing...",
                    Colors::WARNING,
                ),
                ("> CRITICAL: multiverse.exe out of memory", Colors::ERROR),
            ],
        );

        messages.insert(
            "Null Pointer Exception",
            vec![
                (
                    "> dereferencing the void between worlds...",
                    Colors::WARNING,
                ),
                (
                    "> pointing to nothing... and everything...",
                    Colors::WARNING,
                ),
                ("> accessing the null space of reality...", Colors::WARNING),
                (
                    "> FATAL: tried to read from /dev/null/universe",
                    Colors::ERROR,
                ),
            ],
        );

        messages.insert(
            "Undefined Behavior",
            vec![
                (
                    "> entered the undefined realm beyond logic...",
                    Colors::WARNING,
                ),
                (
                    "> breaking the fundamental laws of physics...",
                    Colors::WARNING,
                ),
                (
                    "> creating paradoxes in the space-time continuum...",
                    Colors::ERROR,
                ),
                ("> WARNING: reality compiler has given up", Colors::ERROR),
            ],
        );

        messages.insert(
            "Heisenbug",
            vec![
                ("> bug exists in quantum superposition...", Colors::SCORE),
                ("> observation collapses the wave function...", Colors::INFO),
                (
                    "> Schrödinger's error: both fixed and broken...",
                    Colors::WARNING,
                ),
                (
                    "> quantum debugging has broken causality itself",
                    Colors::ERROR,
                ),
            ],
        );

        messages.insert(
            "Blue Screen",
            vec![
                (
                    "> the universe has encountered a fatal error...",
                    Colors::ERROR,
                ),
                (
                    "> collecting dump of all human knowledge...",
                    Colors::BORDER,
                ),
                ("> please restart your dimension...", Colors::TEXT),
                (
                    "> BSOD: Big Source Of Destruction activated",
                    Colors::BORDER,
                ),
            ],
        );

        messages.insert(
            "Kernel Panic",
            vec![
                (
                    "> PANIC: universe.kernel has stopped responding",
                    Colors::ERROR,
                ),
                ("> reality.core dumped to /dev/void...", Colors::ERROR),
                (
                    "> physics.sys failed to load fundamental constants",
                    Colors::ERROR,
                ),
                ("> rebooting existence in 3... 2... 1... ∞", Colors::ERROR),
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
            color: Colors::TEXT,
        },
        ColoredMessage {
            text: "> calculating skill results...".to_string(),
            color: Colors::TEXT,
        },
        ColoredMessage {
            text: "> determining rank classification...".to_string(),
            color: Colors::TEXT,
        },
        ColoredMessage {
            text: "> rank assignment complete.".to_string(),
            color: Colors::SUCCESS,
        },
    ]
}
