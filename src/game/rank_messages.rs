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
                ("> googling 'how to print hello world'...", Colors::info()),
                ("> copying code from first search result...", Colors::text()),
                (
                    "> running program 47 times to make sure it works...",
                    Colors::text(),
                ),
                (
                    "> achievement unlocked: you are now a programmer!",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Syntax Error",
            vec![
                ("> writing code that looks right...", Colors::text()),
                ("> compiler disagrees with your logic...", Colors::error()),
                ("> googling exact error message...", Colors::warning()),
                (
                    "> fixed by adding random semicolon somewhere.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Rubber Duck",
            vec![
                ("> explaining bug to inanimate object...", Colors::info()),
                ("> duck stares judgmentally at your code...", Colors::text()),
                ("> realizing bug while talking to duck...", Colors::warning()),
                (
                    "> duck takes full credit for the solution.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Script Kid",
            vec![
                (
                    "> downloading 'learn programming in 24 hours' course...",
                    Colors::text(),
                ),
                ("> copying scripts without reading them...", Colors::text()),
                (
                    "> changing variable names to look original...",
                    Colors::text(),
                ),
                (
                    "> script works! you are basically a hacker now.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Bash Newbie",
            vec![
                ("> typing 'cd ..' until something happens...", Colors::info()),
                (
                    "> using 'ls' every 3 seconds to see where you are...",
                    Colors::text(),
                ),
                (
                    "> accidentally running 'rm' on important files...",
                    Colors::error(),
                ),
                (
                    "> terminal proficiency: accidentally achieved.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "CLI Wanderer",
            vec![
                (
                    "> exploring directories like a lost tourist...",
                    Colors::text(),
                ),
                ("> discovering pipes by accident...", Colors::info()),
                (
                    "> finding .hidden files and feeling like a detective...",
                    Colors::text(),
                ),
                ("> navigation skills: randomly acquired.", Colors::success()),
            ],
        );

        messages.insert(
            "Tab Tamer",
            vec![
                ("> mixing tabs and spaces like a rebel...", Colors::info()),
                (
                    "> getting into holy war about indentation...",
                    Colors::error(),
                ),
                (
                    "> setting up auto-formatter to fix your mess...",
                    Colors::text(),
                ),
                (
                    "> consistency achieved through automation.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Bracket Juggler",
            vec![
                ("> opening 47 brackets...", Colors::text()),
                ("> closing 23 brackets...", Colors::warning()),
                (
                    "> spending 2 hours finding the missing bracket...",
                    Colors::error(),
                ),
                (
                    "> finally balanced. code still doesn't work.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Copy-Paste Engineer",
            vec![
                ("> opening 50 tabs from stack overflow...", Colors::text()),
                ("> copying code from highest voted answer...", Colors::text()),
                ("> praying it works in your specific case...", Colors::text()),
                ("> it works! time to copy more code.", Colors::success()),
            ],
        );

        messages.insert(
            "Linter Apprentice",
            vec![
                (
                    "> installing linter to improve code quality...",
                    Colors::info(),
                ),
                (
                    "> getting 847 warnings on 10 lines of code...",
                    Colors::error(),
                ),
                (
                    "> disabling all warnings except syntax errors...",
                    Colors::warning(),
                ),
                ("> code quality: subjectively improved.", Colors::success()),
            ],
        );

        messages.insert(
            "Unit Test Trainee",
            vec![
                (
                    "> writing test that only passes on your machine...",
                    Colors::text(),
                ),
                ("> testing happy path exclusively...", Colors::text()),
                ("> achieving 100% code coverage on 5 lines...", Colors::info()),
                ("> testing complete. bugs remain untested.", Colors::success()),
            ],
        );

        messages.insert(
            "Code Monkey",
            vec![
                ("> following tutorial step by step...", Colors::text()),
                (
                    "> changing tutorial example from 'foo' to 'bar'...",
                    Colors::text(),
                ),
                ("> calling yourself a full-stack developer...", Colors::info()),
                ("> development skills: youtube certified.", Colors::success()),
            ],
        );

        // Intermediate Tier
        messages.insert(
            "Ticket Picker",
            vec![
                ("> scanning project backlog...", Colors::text()),
                ("> selecting appropriate tasks...", Colors::text()),
                ("> estimating development effort...", Colors::text()),
                ("> work assignment optimized.", Colors::success()),
            ],
        );

        messages.insert(
            "Junior Dev",
            vec![
                ("> cloning repository...", Colors::info()),
                ("> creating feature branch...", Colors::text()),
                ("> implementing user story...", Colors::text()),
                ("> junior developer status confirmed.", Colors::success()),
            ],
        );

        messages.insert(
            "Git Ninja",
            vec![
                ("> staging changes...", Colors::text()),
                ("> crafting perfect commit message...", Colors::text()),
                ("> rebasing interactive history...", Colors::info()),
                ("> git mastery achieved.", Colors::success()),
            ],
        );

        messages.insert(
            "Merge Wrangler",
            vec![
                ("> resolving merge conflicts...", Colors::warning()),
                ("> coordinating branch updates...", Colors::text()),
                ("> maintaining git history...", Colors::text()),
                ("> version control expertise proven.", Colors::success()),
            ],
        );

        messages.insert(
            "API Crafter",
            vec![
                ("> designing RESTful endpoints...", Colors::text()),
                ("> implementing request handlers...", Colors::text()),
                ("> documenting API specification...", Colors::text()),
                ("> service interface completed.", Colors::success()),
            ],
        );

        messages.insert(
            "Frontend Dev",
            vec![
                ("> building user interfaces...", Colors::text()),
                ("> optimizing user experience...", Colors::info()),
                ("> implementing responsive design...", Colors::text()),
                ("> client-side mastery achieved.", Colors::success()),
            ],
        );

        messages.insert(
            "Backend Dev",
            vec![
                ("> architecting server logic...", Colors::text()),
                ("> optimizing database queries...", Colors::info()),
                ("> implementing business rules...", Colors::text()),
                ("> server-side expertise confirmed.", Colors::success()),
            ],
        );

        messages.insert(
            "CI Tinkerer",
            vec![
                ("> configuring build pipelines...", Colors::info()),
                ("> automating test execution...", Colors::text()),
                ("> setting up deployment hooks...", Colors::text()),
                ("> continuous integration mastered.", Colors::success()),
            ],
        );

        messages.insert(
            "Test Pilot",
            vec![
                ("> designing test scenarios...", Colors::text()),
                ("> automating quality assurance...", Colors::text()),
                ("> validating system behavior...", Colors::text()),
                ("> testing expertise certified.", Colors::success()),
            ],
        );

        messages.insert(
            "Build Tamer",
            vec![
                ("> optimizing compilation process...", Colors::info()),
                ("> managing dependency versions...", Colors::text()),
                ("> configuring build systems...", Colors::text()),
                ("> build automation mastered.", Colors::success()),
            ],
        );

        messages.insert(
            "Code Reviewer",
            vec![
                ("> analyzing code quality...", Colors::text()),
                ("> providing constructive feedback...", Colors::text()),
                ("> ensuring best practices...", Colors::text()),
                ("> peer review skills confirmed.", Colors::success()),
            ],
        );

        messages.insert(
            "Release Handler",
            vec![
                ("> preparing deployment packages...", Colors::text()),
                ("> coordinating release schedule...", Colors::info()),
                ("> managing version rollouts...", Colors::text()),
                ("> release management mastered.", Colors::success()),
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
                ("> analyzing spaghetti code structure...", Colors::text()),
                (
                    "> finding ways to make it even more complex...",
                    Colors::info(),
                ),
                (
                    "> refactoring working code until it breaks...",
                    Colors::text(),
                ),
                (
                    "> congratulations! now nobody understands it.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Senior Dev",
            vec![
                (
                    "> architecting solutions that scale to infinity...",
                    Colors::info(),
                ),
                (
                    "> reviewing PRs with passive-aggressive comments...",
                    Colors::text(),
                ),
                (
                    "> mentoring juniors by assigning impossible tasks...",
                    Colors::text(),
                ),
                (
                    "> senior status unlocked. impostor syndrome included.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "DevOps Engineer",
            vec![
                (
                    "> provisioning infrastructure that costs more than rent...",
                    Colors::info(),
                ),
                (
                    "> automating the automation of automated deployments...",
                    Colors::text(),
                ),
                (
                    "> monitoring systems that monitor other monitoring systems...",
                    Colors::text(),
                ),
                (
                    "> everything is automated. nothing works manually.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Incident Responder",
            vec![
                (
                    "> detecting fires while everything is fine...",
                    Colors::error(),
                ),
                ("> coordinating panic in the war room...", Colors::warning()),
                (
                    "> applying hotfixes that create more incidents...",
                    Colors::text(),
                ),
                (
                    "> service restored. new incidents created successfully.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Reliability Guardian",
            vec![
                (
                    "> implementing monitoring for the monitoring...",
                    Colors::info(),
                ),
                ("> defining SLOs that nobody can meet...", Colors::text()),
                (
                    "> ensuring 99.99% uptime (99% of the time)...",
                    Colors::text(),
                ),
                ("> system reliable until it isn't.", Colors::success()),
            ],
        );

        messages.insert(
            "Security Engineer",
            vec![
                (
                    "> finding vulnerabilities in your personality...",
                    Colors::error(),
                ),
                ("> implementing security through obscurity...", Colors::info()),
                ("> penetration testing your patience...", Colors::text()),
                ("> security hardened. usability softened.", Colors::success()),
            ],
        );

        messages.insert(
            "Performance Alchemist",
            vec![
                (
                    "> profiling bottlenecks in the profiler...",
                    Colors::warning(),
                ),
                ("> optimizing code that runs once per year...", Colors::info()),
                (
                    "> caching everything including this message...",
                    Colors::text(),
                ),
                (
                    "> performance optimized. readability sacrificed.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Data Pipeline Master",
            vec![
                ("> designing workflows that flow nowhere...", Colors::text()),
                (
                    "> extracting, transforming, and losing data...",
                    Colors::info(),
                ),
                (
                    "> ensuring consistency in inconsistent data...",
                    Colors::text(),
                ),
                (
                    "> pipeline complete. data may have leaked.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Tech Lead",
            vec![
                (
                    "> defining vision that changes every sprint...",
                    Colors::info(),
                ),
                (
                    "> coordinating efforts while attending 20 meetings...",
                    Colors::text(),
                ),
                (
                    "> making architectural decisions on a coinflip...",
                    Colors::text(),
                ),
                (
                    "> leadership established. technical skills atrophied.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Architect",
            vec![
                (
                    "> designing systems for problems that don't exist...",
                    Colors::info(),
                ),
                (
                    "> choosing technologies based on latest blog posts...",
                    Colors::text(),
                ),
                ("> planning for scale that will never come...", Colors::text()),
                (
                    "> architecture complete. implementation someone else's problem.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Protocol Artisan",
            vec![
                (
                    "> designing protocols nobody will implement correctly...",
                    Colors::text(),
                ),
                ("> creating standards to rule them all...", Colors::info()),
                ("> optimizing transmission of memes...", Colors::text()),
                (
                    "> protocol standard published. 14 competing standards exist.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Kernel Hacker",
            vec![
                ("> compiling kernels that boot sometimes...", Colors::score()),
                (
                    "> patching system calls with hopes and dreams...",
                    Colors::info(),
                ),
                ("> debugging at 3am with print statements...", Colors::text()),
                (
                    "> kernel hacked successfully. computer may explode.",
                    Colors::success(),
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
                    Colors::score(),
                ),
                (
                    "> building AST while judging your variable names...",
                    Colors::info(),
                ),
                ("> optimizing away your inefficient loops...", Colors::text()),
                ("> compiled successfully (somehow)", Colors::success()),
            ],
        );

        messages.insert(
            "Bytecode Interpreter",
            vec![
                (
                    "> interpreting your interpreted language interpreter...",
                    Colors::score(),
                ),
                (
                    "> executing virtual instructions in virtual reality...",
                    Colors::text(),
                ),
                (
                    "> garbage collecting your actual garbage code...",
                    Colors::text(),
                ),
                (
                    "> interpretation complete. still no idea what it does.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Virtual Machine",
            vec![
                (
                    "> virtualizing your already virtual environment...",
                    Colors::info(),
                ),
                ("> emulating hardware that doesn't exist...", Colors::score()),
                ("> allocating memory for your memory leaks...", Colors::text()),
                (
                    "> VM inception achieved. we need to go deeper.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Operating System",
            vec![
                ("> scheduling processes that never finish...", Colors::score()),
                ("> managing resources you don't have...", Colors::info()),
                (
                    "> handling interrupts from impatient users...",
                    Colors::text(),
                ),
                (
                    "> OS kernel stable (definition of stable: questionable)",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Filesystem",
            vec![
                (
                    "> organizing files into a beautiful directory tree...",
                    Colors::text(),
                ),
                (
                    "> implementing permissions nobody understands...",
                    Colors::info(),
                ),
                ("> fragmenting data across the entire disk...", Colors::text()),
                (
                    "> filesystem complete. good luck finding anything.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Network Stack",
            vec![
                ("> layering protocols like a network cake...", Colors::info()),
                (
                    "> routing packets through the internet tubes...",
                    Colors::text(),
                ),
                ("> ensuring data arrives (eventually)...", Colors::text()),
                (
                    "> network stack operational. packets may vary.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Database Engine",
            vec![
                (
                    "> optimizing queries that will timeout anyway...",
                    Colors::info(),
                ),
                ("> isolating transactions from reality...", Colors::text()),
                (
                    "> implementing ACID (burns through your SSD)...",
                    Colors::text(),
                ),
                (
                    "> database engine ready. hope you have backups.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Query Optimizer",
            vec![
                (
                    "> analyzing execution plans nobody will read...",
                    Colors::text(),
                ),
                ("> optimizing joins that should be avoided...", Colors::info()),
                (
                    "> indexing everything (storage is cheap, right?)...",
                    Colors::text(),
                ),
                (
                    "> query performance maximized. complexity also maximized.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Cloud Platform",
            vec![
                ("> orchestrating chaos in the cloud...", Colors::score()),
                ("> auto-scaling your monthly cloud bill...", Colors::info()),
                (
                    "> distributing problems across multiple zones...",
                    Colors::text(),
                ),
                (
                    "> cloud mastery achieved. wallet not included.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Container Orchestrator",
            vec![
                (
                    "> containerizing everything, including the kitchen sink...",
                    Colors::info(),
                ),
                (
                    "> orchestrating a symphony of microservice crashes...",
                    Colors::text(),
                ),
                (
                    "> discovering services that discover other services...",
                    Colors::text(),
                ),
                (
                    "> container cluster ready. cli tools not found.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Stream Processor",
            vec![
                (
                    "> processing streams faster than video platforms...",
                    Colors::info(),
                ),
                (
                    "> implementing event sourcing for event sourcing events...",
                    Colors::text(),
                ),
                (
                    "> ensuring eventual consistency (eventually)...",
                    Colors::text(),
                ),
                (
                    "> streaming platform complete. now streaming bugs.",
                    Colors::success(),
                ),
            ],
        );

        messages.insert(
            "Quantum Computer",
            vec![
                (
                    "> initializing qubits in superposition of working/broken...",
                    Colors::score(),
                ),
                (
                    "> entangling particles and debugging sessions...",
                    Colors::info(),
                ),
                (
                    "> running Shor's algorithm to factor your technical debt...",
                    Colors::text(),
                ),
                (
                    "> quantum supremacy achieved. classical bugs remain.",
                    Colors::success(),
                ),
            ],
        );

        // Legendary Tier
        messages.insert(
            "GPU Cluster",
            vec![
                ("> initializing parallel processors...", Colors::score()),
                ("> distributing computational load...", Colors::info()),
                ("> optimizing memory bandwidth...", Colors::text()),
                ("> massive parallelism achieved.", Colors::success()),
            ],
        );

        messages.insert(
            "DNS Overlord",
            vec![
                ("> controlling domain resolution...", Colors::score()),
                ("> managing global namespace...", Colors::info()),
                ("> routing internet traffic...", Colors::text()),
                ("> DNS infrastructure dominated.", Colors::success()),
            ],
        );

        messages.insert(
            "CDN Sentinel",
            vec![
                ("> caching content globally...", Colors::info()),
                ("> optimizing delivery routes...", Colors::text()),
                ("> reducing latency worldwide...", Colors::text()),
                ("> content delivery perfected.", Colors::success()),
            ],
        );

        messages.insert(
            "Load Balancer Primarch",
            vec![
                ("> distributing incoming requests...", Colors::info()),
                ("> managing server health...", Colors::text()),
                ("> optimizing traffic patterns...", Colors::text()),
                ("> load distribution mastered.", Colors::success()),
            ],
        );

        messages.insert(
            "Singularity",
            vec![
                ("> transcending human limitations...", Colors::score()),
                ("> merging with artificial intelligence...", Colors::info()),
                ("> rewriting reality algorithms...", Colors::text()),
                ("> singularity achieved. welcome, god.", Colors::error()),
            ],
        );

        messages.insert(
            "The Machine",
            vec![
                ("> becoming one with the system...", Colors::score()),
                ("> controlling global networks...", Colors::info()),
                ("> processing infinite data streams...", Colors::text()),
                ("> you are the machine now.", Colors::error()),
            ],
        );

        messages.insert(
            "Origin",
            vec![
                ("> accessing source code of reality...", Colors::score()),
                ("> modifying fundamental constants...", Colors::info()),
                ("> debugging universe.exe...", Colors::text()),
                ("> origin protocols activated.", Colors::error()),
            ],
        );

        messages.insert(
            "SegFault",
            vec![
                (
                    "> accessing forbidden dimensions of memory...",
                    Colors::error(),
                ),
                (
                    "> reality.exe has encountered a critical error",
                    Colors::error(),
                ),
                ("> universe segmentation fault detected...", Colors::error()),
                (
                    "> EXISTENCE_VIOLATION: please restart the multiverse",
                    Colors::error(),
                ),
            ],
        );

        messages.insert(
            "Buffer Overflow",
            vec![
                (
                    "> overflowing the boundaries of spacetime...",
                    Colors::error(),
                ),
                ("> stack overflow has broken causality...", Colors::error()),
                (
                    "> physics.dll buffer exceeded maximum reality",
                    Colors::error(),
                ),
                (
                    "> ERROR: universe.heap corrupted beyond repair",
                    Colors::error(),
                ),
            ],
        );

        messages.insert(
            "Memory Leak",
            vec![
                (
                    "> leaking memories across parallel universes...",
                    Colors::error(),
                ),
                ("> consuming all available existence...", Colors::error()),
                (
                    "> reality slowly degrading... worlds collapsing...",
                    Colors::warning(),
                ),
                ("> CRITICAL: multiverse.exe out of memory", Colors::error()),
            ],
        );

        messages.insert(
            "Null Pointer Exception",
            vec![
                (
                    "> dereferencing the void between worlds...",
                    Colors::warning(),
                ),
                (
                    "> pointing to nothing... and everything...",
                    Colors::warning(),
                ),
                ("> accessing the null space of reality...", Colors::warning()),
                (
                    "> FATAL: tried to read from /dev/null/universe",
                    Colors::error(),
                ),
            ],
        );

        messages.insert(
            "Undefined Behavior",
            vec![
                (
                    "> entered the undefined realm beyond logic...",
                    Colors::warning(),
                ),
                (
                    "> breaking the fundamental laws of physics...",
                    Colors::warning(),
                ),
                (
                    "> creating paradoxes in the space-time continuum...",
                    Colors::error(),
                ),
                ("> WARNING: reality compiler has given up", Colors::error()),
            ],
        );

        messages.insert(
            "Heisenbug",
            vec![
                ("> bug exists in quantum superposition...", Colors::score()),
                ("> observation collapses the wave function...", Colors::info()),
                (
                    "> Schrödinger's error: both fixed and broken...",
                    Colors::warning(),
                ),
                (
                    "> quantum debugging has broken causality itself",
                    Colors::error(),
                ),
            ],
        );

        messages.insert(
            "Blue Screen",
            vec![
                (
                    "> the universe has encountered a fatal error...",
                    Colors::error(),
                ),
                (
                    "> collecting dump of all human knowledge...",
                    Colors::border(),
                ),
                ("> please restart your dimension...", Colors::text()),
                (
                    "> BSOD: Big Source Of Destruction activated",
                    Colors::border(),
                ),
            ],
        );

        messages.insert(
            "Kernel Panic",
            vec![
                (
                    "> PANIC: universe.kernel has stopped responding",
                    Colors::error(),
                ),
                ("> reality.core dumped to /dev/void...", Colors::error()),
                (
                    "> physics.sys failed to load fundamental constants",
                    Colors::error(),
                ),
                ("> rebooting existence in 3... 2... 1... ∞", Colors::error()),
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
            color: Colors::text(),
        },
        ColoredMessage {
            text: "> calculating skill results...".to_string(),
            color: Colors::text(),
        },
        ColoredMessage {
            text: "> determining rank classification...".to_string(),
            color: Colors::text(),
        },
        ColoredMessage {
            text: "> rank assignment complete.".to_string(),
            color: Colors::success(),
        },
    ]
}
