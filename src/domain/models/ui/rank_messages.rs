use crate::presentation::ui::Colors;
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
                (
                    "> googling 'how to print hello world'...",
                    Colors::default_info(),
                ),
                (
                    "> copying code from first search result...",
                    Colors::default_text(),
                ),
                (
                    "> running program 47 times to make sure it works...",
                    Colors::default_text(),
                ),
                (
                    "> achievement unlocked: you are now a programmer!",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Syntax Error",
            vec![
                ("> writing code that looks right...", Colors::default_text()),
                (
                    "> compiler disagrees with your logic...",
                    Colors::default_error(),
                ),
                (
                    "> googling exact error message...",
                    Colors::default_warning(),
                ),
                (
                    "> fixed by adding random semicolon somewhere.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Rubber Duck",
            vec![
                (
                    "> explaining bug to inanimate object...",
                    Colors::default_info(),
                ),
                (
                    "> duck stares judgmentally at your code...",
                    Colors::default_text(),
                ),
                (
                    "> realizing bug while talking to duck...",
                    Colors::default_warning(),
                ),
                (
                    "> duck takes full credit for the solution.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Script Kid",
            vec![
                (
                    "> downloading 'learn programming in 24 hours' course...",
                    Colors::default_text(),
                ),
                (
                    "> copying scripts without reading them...",
                    Colors::default_text(),
                ),
                (
                    "> changing variable names to look original...",
                    Colors::default_text(),
                ),
                (
                    "> script works! you are basically a hacker now.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Bash Newbie",
            vec![
                (
                    "> typing 'cd ..' until something happens...",
                    Colors::default_info(),
                ),
                (
                    "> using 'ls' every 3 seconds to see where you are...",
                    Colors::default_text(),
                ),
                (
                    "> accidentally running 'rm' on important files...",
                    Colors::default_error(),
                ),
                (
                    "> terminal proficiency: accidentally achieved.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "CLI Wanderer",
            vec![
                (
                    "> exploring directories like a lost tourist...",
                    Colors::default_text(),
                ),
                ("> discovering pipes by accident...", Colors::default_info()),
                (
                    "> finding .hidden files and feeling like a detective...",
                    Colors::default_text(),
                ),
                (
                    "> navigation skills: randomly acquired.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Tab Tamer",
            vec![
                (
                    "> mixing tabs and spaces like a rebel...",
                    Colors::default_info(),
                ),
                (
                    "> getting into holy war about indentation...",
                    Colors::default_error(),
                ),
                (
                    "> setting up auto-formatter to fix your mess...",
                    Colors::default_text(),
                ),
                (
                    "> consistency achieved through automation.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Bracket Juggler",
            vec![
                ("> opening 47 brackets...", Colors::default_text()),
                ("> closing 23 brackets...", Colors::default_warning()),
                (
                    "> spending 2 hours finding the missing bracket...",
                    Colors::default_error(),
                ),
                (
                    "> finally balanced. code still doesn't work.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Copy-Paste Engineer",
            vec![
                (
                    "> opening 50 tabs from stack overflow...",
                    Colors::default_text(),
                ),
                (
                    "> copying code from highest voted answer...",
                    Colors::default_text(),
                ),
                (
                    "> praying it works in your specific case...",
                    Colors::default_text(),
                ),
                (
                    "> it works! time to copy more code.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Linter Apprentice",
            vec![
                (
                    "> installing linter to improve code quality...",
                    Colors::default_info(),
                ),
                (
                    "> getting 847 warnings on 10 lines of code...",
                    Colors::default_error(),
                ),
                (
                    "> disabling all warnings except syntax errors...",
                    Colors::default_warning(),
                ),
                (
                    "> code quality: subjectively improved.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Unit Test Trainee",
            vec![
                (
                    "> writing test that only passes on your machine...",
                    Colors::default_text(),
                ),
                (
                    "> testing happy path exclusively...",
                    Colors::default_text(),
                ),
                (
                    "> achieving 100% code coverage on 5 lines...",
                    Colors::default_info(),
                ),
                (
                    "> testing complete. bugs remain untested.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Code Monkey",
            vec![
                (
                    "> following tutorial step by step...",
                    Colors::default_text(),
                ),
                (
                    "> changing tutorial example from 'foo' to 'bar'...",
                    Colors::default_text(),
                ),
                (
                    "> calling yourself a full-stack developer...",
                    Colors::default_info(),
                ),
                (
                    "> development skills: youtube certified.",
                    Colors::default_success(),
                ),
            ],
        );

        // Intermediate Tier
        messages.insert(
            "Ticket Picker",
            vec![
                ("> scanning project backlog...", Colors::default_text()),
                ("> selecting appropriate tasks...", Colors::default_text()),
                ("> estimating development effort...", Colors::default_text()),
                ("> work assignment optimized.", Colors::default_success()),
            ],
        );

        messages.insert(
            "Junior Dev",
            vec![
                ("> cloning repository...", Colors::default_info()),
                ("> creating feature branch...", Colors::default_text()),
                ("> implementing user story...", Colors::default_text()),
                (
                    "> junior developer status confirmed.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Git Ninja",
            vec![
                ("> staging changes...", Colors::default_text()),
                (
                    "> crafting perfect commit message...",
                    Colors::default_text(),
                ),
                ("> rebasing interactive history...", Colors::default_info()),
                ("> git mastery achieved.", Colors::default_success()),
            ],
        );

        messages.insert(
            "Merge Wrangler",
            vec![
                ("> resolving merge conflicts...", Colors::default_warning()),
                ("> coordinating branch updates...", Colors::default_text()),
                ("> maintaining git history...", Colors::default_text()),
                (
                    "> version control expertise proven.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "API Crafter",
            vec![
                ("> designing RESTful endpoints...", Colors::default_text()),
                ("> implementing request handlers...", Colors::default_text()),
                ("> documenting API specification...", Colors::default_text()),
                ("> service interface completed.", Colors::default_success()),
            ],
        );

        messages.insert(
            "Frontend Dev",
            vec![
                ("> building user interfaces...", Colors::default_text()),
                ("> optimizing user experience...", Colors::default_info()),
                (
                    "> implementing responsive design...",
                    Colors::default_text(),
                ),
                ("> client-side mastery achieved.", Colors::default_success()),
            ],
        );

        messages.insert(
            "Backend Dev",
            vec![
                ("> architecting server logic...", Colors::default_text()),
                ("> optimizing database queries...", Colors::default_info()),
                ("> implementing business rules...", Colors::default_text()),
                (
                    "> server-side expertise confirmed.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "CI Tinkerer",
            vec![
                ("> configuring build pipelines...", Colors::default_info()),
                ("> automating test execution...", Colors::default_text()),
                ("> setting up deployment hooks...", Colors::default_text()),
                (
                    "> continuous integration mastered.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Test Pilot",
            vec![
                ("> designing test scenarios...", Colors::default_text()),
                ("> automating quality assurance...", Colors::default_text()),
                ("> validating system behavior...", Colors::default_text()),
                ("> testing expertise certified.", Colors::default_success()),
            ],
        );

        messages.insert(
            "Build Tamer",
            vec![
                (
                    "> optimizing compilation process...",
                    Colors::default_info(),
                ),
                ("> managing dependency versions...", Colors::default_text()),
                ("> configuring build systems...", Colors::default_text()),
                ("> build automation mastered.", Colors::default_success()),
            ],
        );

        messages.insert(
            "Code Reviewer",
            vec![
                ("> analyzing code quality...", Colors::default_text()),
                (
                    "> providing constructive feedback...",
                    Colors::default_text(),
                ),
                ("> ensuring best practices...", Colors::default_text()),
                ("> peer review skills confirmed.", Colors::default_success()),
            ],
        );

        messages.insert(
            "Release Handler",
            vec![
                ("> preparing deployment packages...", Colors::default_text()),
                ("> coordinating release schedule...", Colors::default_info()),
                ("> managing version rollouts...", Colors::default_text()),
                ("> release management mastered.", Colors::default_success()),
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
                (
                    "> analyzing spaghetti code structure...",
                    Colors::default_text(),
                ),
                (
                    "> finding ways to make it even more complex...",
                    Colors::default_info(),
                ),
                (
                    "> refactoring working code until it breaks...",
                    Colors::default_text(),
                ),
                (
                    "> congratulations! now nobody understands it.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Senior Dev",
            vec![
                (
                    "> architecting solutions that scale to infinity...",
                    Colors::default_info(),
                ),
                (
                    "> reviewing PRs with passive-aggressive comments...",
                    Colors::default_text(),
                ),
                (
                    "> mentoring juniors by assigning impossible tasks...",
                    Colors::default_text(),
                ),
                (
                    "> senior status unlocked. impostor syndrome included.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "DevOps Engineer",
            vec![
                (
                    "> provisioning infrastructure that costs more than rent...",
                    Colors::default_info(),
                ),
                (
                    "> automating the automation of automated deployments...",
                    Colors::default_text(),
                ),
                (
                    "> monitoring systems that monitor other monitoring systems...",
                    Colors::default_text(),
                ),
                (
                    "> everything is automated. nothing works manually.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Incident Responder",
            vec![
                (
                    "> detecting fires while everything is fine...",
                    Colors::default_error(),
                ),
                (
                    "> coordinating panic in the war room...",
                    Colors::default_warning(),
                ),
                (
                    "> applying hotfixes that create more incidents...",
                    Colors::default_text(),
                ),
                (
                    "> service restored. new incidents created successfully.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Reliability Guardian",
            vec![
                (
                    "> implementing monitoring for the monitoring...",
                    Colors::default_info(),
                ),
                (
                    "> defining SLOs that nobody can meet...",
                    Colors::default_text(),
                ),
                (
                    "> ensuring 99.99% uptime (99% of the time)...",
                    Colors::default_text(),
                ),
                (
                    "> system reliable until it isn't.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Security Engineer",
            vec![
                (
                    "> finding vulnerabilities in your personality...",
                    Colors::default_error(),
                ),
                (
                    "> implementing security through obscurity...",
                    Colors::default_info(),
                ),
                (
                    "> penetration testing your patience...",
                    Colors::default_text(),
                ),
                (
                    "> security hardened. usability softened.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Performance Alchemist",
            vec![
                (
                    "> profiling bottlenecks in the profiler...",
                    Colors::default_warning(),
                ),
                (
                    "> optimizing code that runs once per year...",
                    Colors::default_info(),
                ),
                (
                    "> caching everything including this message...",
                    Colors::default_text(),
                ),
                (
                    "> performance optimized. readability sacrificed.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Data Pipeline Master",
            vec![
                (
                    "> designing workflows that flow nowhere...",
                    Colors::default_text(),
                ),
                (
                    "> extracting, transforming, and losing data...",
                    Colors::default_info(),
                ),
                (
                    "> ensuring consistency in inconsistent data...",
                    Colors::default_text(),
                ),
                (
                    "> pipeline complete. data may have leaked.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Tech Lead",
            vec![
                (
                    "> defining vision that changes every sprint...",
                    Colors::default_info(),
                ),
                (
                    "> coordinating efforts while attending 20 meetings...",
                    Colors::default_text(),
                ),
                (
                    "> making architectural decisions on a coinflip...",
                    Colors::default_text(),
                ),
                (
                    "> leadership established. technical skills atrophied.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Architect",
            vec![
                (
                    "> designing systems for problems that don't exist...",
                    Colors::default_info(),
                ),
                (
                    "> choosing technologies based on latest blog posts...",
                    Colors::default_text(),
                ),
                (
                    "> planning for scale that will never come...",
                    Colors::default_text(),
                ),
                (
                    "> architecture complete. implementation someone else's problem.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Protocol Artisan",
            vec![
                (
                    "> designing protocols nobody will implement correctly...",
                    Colors::default_text(),
                ),
                (
                    "> creating standards to rule them all...",
                    Colors::default_info(),
                ),
                (
                    "> optimizing transmission of memes...",
                    Colors::default_text(),
                ),
                (
                    "> protocol standard published. 14 competing standards exist.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Kernel Hacker",
            vec![
                (
                    "> compiling kernels that boot sometimes...",
                    Colors::default_score(),
                ),
                (
                    "> patching system calls with hopes and dreams...",
                    Colors::default_info(),
                ),
                (
                    "> debugging at 3am with print statements...",
                    Colors::default_text(),
                ),
                (
                    "> kernel hacked successfully. computer may explode.",
                    Colors::default_success(),
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
                    Colors::default_score(),
                ),
                (
                    "> building AST while judging your variable names...",
                    Colors::default_info(),
                ),
                (
                    "> optimizing away your inefficient loops...",
                    Colors::default_text(),
                ),
                (
                    "> compiled successfully (somehow)",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Bytecode Interpreter",
            vec![
                (
                    "> interpreting your interpreted language interpreter...",
                    Colors::default_score(),
                ),
                (
                    "> executing virtual instructions in virtual reality...",
                    Colors::default_text(),
                ),
                (
                    "> garbage collecting your actual garbage code...",
                    Colors::default_text(),
                ),
                (
                    "> interpretation complete. still no idea what it does.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Virtual Machine",
            vec![
                (
                    "> virtualizing your already virtual environment...",
                    Colors::default_info(),
                ),
                (
                    "> emulating hardware that doesn't exist...",
                    Colors::default_score(),
                ),
                (
                    "> allocating memory for your memory leaks...",
                    Colors::default_text(),
                ),
                (
                    "> VM inception achieved. we need to go deeper.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Operating System",
            vec![
                (
                    "> scheduling processes that never finish...",
                    Colors::default_score(),
                ),
                (
                    "> managing resources you don't have...",
                    Colors::default_info(),
                ),
                (
                    "> handling interrupts from impatient users...",
                    Colors::default_text(),
                ),
                (
                    "> OS kernel stable (definition of stable: questionable)",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Filesystem",
            vec![
                (
                    "> organizing files into a beautiful directory tree...",
                    Colors::default_text(),
                ),
                (
                    "> implementing permissions nobody understands...",
                    Colors::default_info(),
                ),
                (
                    "> fragmenting data across the entire disk...",
                    Colors::default_text(),
                ),
                (
                    "> filesystem complete. good luck finding anything.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Network Stack",
            vec![
                (
                    "> layering protocols like a network cake...",
                    Colors::default_info(),
                ),
                (
                    "> routing packets through the internet tubes...",
                    Colors::default_text(),
                ),
                (
                    "> ensuring data arrives (eventually)...",
                    Colors::default_text(),
                ),
                (
                    "> network stack operational. packets may vary.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Database Engine",
            vec![
                (
                    "> optimizing queries that will timeout anyway...",
                    Colors::default_info(),
                ),
                (
                    "> isolating transactions from reality...",
                    Colors::default_text(),
                ),
                (
                    "> implementing ACID (burns through your SSD)...",
                    Colors::default_text(),
                ),
                (
                    "> database engine ready. hope you have backups.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Query Optimizer",
            vec![
                (
                    "> analyzing execution plans nobody will read...",
                    Colors::default_text(),
                ),
                (
                    "> optimizing joins that should be avoided...",
                    Colors::default_info(),
                ),
                (
                    "> indexing everything (storage is cheap, right?)...",
                    Colors::default_text(),
                ),
                (
                    "> query performance maximized. complexity also maximized.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Cloud Platform",
            vec![
                (
                    "> orchestrating chaos in the cloud...",
                    Colors::default_score(),
                ),
                (
                    "> auto-scaling your monthly cloud bill...",
                    Colors::default_info(),
                ),
                (
                    "> distributing problems across multiple zones...",
                    Colors::default_text(),
                ),
                (
                    "> cloud mastery achieved. wallet not included.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Container Orchestrator",
            vec![
                (
                    "> containerizing everything, including the kitchen sink...",
                    Colors::default_info(),
                ),
                (
                    "> orchestrating a symphony of microservice crashes...",
                    Colors::default_text(),
                ),
                (
                    "> discovering services that discover other services...",
                    Colors::default_text(),
                ),
                (
                    "> container cluster ready. cli tools not found.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Stream Processor",
            vec![
                (
                    "> processing streams faster than video platforms...",
                    Colors::default_info(),
                ),
                (
                    "> implementing event sourcing for event sourcing events...",
                    Colors::default_text(),
                ),
                (
                    "> ensuring eventual consistency (eventually)...",
                    Colors::default_text(),
                ),
                (
                    "> streaming platform complete. now streaming bugs.",
                    Colors::default_success(),
                ),
            ],
        );

        messages.insert(
            "Quantum Computer",
            vec![
                (
                    "> initializing qubits in superposition of working/broken...",
                    Colors::default_score(),
                ),
                (
                    "> entangling particles and debugging sessions...",
                    Colors::default_info(),
                ),
                (
                    "> running Shor's algorithm to factor your technical debt...",
                    Colors::default_text(),
                ),
                (
                    "> quantum supremacy achieved. classical bugs remain.",
                    Colors::default_success(),
                ),
            ],
        );

        // Legendary Tier
        messages.insert(
            "GPU Cluster",
            vec![
                (
                    "> initializing parallel processors...",
                    Colors::default_score(),
                ),
                (
                    "> distributing computational load...",
                    Colors::default_info(),
                ),
                ("> optimizing memory bandwidth...", Colors::default_text()),
                ("> massive parallelism achieved.", Colors::default_success()),
            ],
        );

        messages.insert(
            "DNS Overlord",
            vec![
                (
                    "> controlling domain resolution...",
                    Colors::default_score(),
                ),
                ("> managing global namespace...", Colors::default_info()),
                ("> routing internet traffic...", Colors::default_text()),
                ("> DNS infrastructure dominated.", Colors::default_success()),
            ],
        );

        messages.insert(
            "CDN Sentinel",
            vec![
                ("> caching content globally...", Colors::default_info()),
                ("> optimizing delivery routes...", Colors::default_text()),
                ("> reducing latency worldwide...", Colors::default_text()),
                ("> content delivery perfected.", Colors::default_success()),
            ],
        );

        messages.insert(
            "Load Balancer Primarch",
            vec![
                (
                    "> distributing incoming requests...",
                    Colors::default_info(),
                ),
                ("> managing server health...", Colors::default_text()),
                ("> optimizing traffic patterns...", Colors::default_text()),
                ("> load distribution mastered.", Colors::default_success()),
            ],
        );

        messages.insert(
            "Singularity",
            vec![
                (
                    "> transcending human limitations...",
                    Colors::default_score(),
                ),
                (
                    "> merging with artificial intelligence...",
                    Colors::default_info(),
                ),
                ("> rewriting reality algorithms...", Colors::default_text()),
                (
                    "> singularity achieved. welcome, god.",
                    Colors::default_error(),
                ),
            ],
        );

        messages.insert(
            "The Machine",
            vec![
                ("> becoming one with the system...", Colors::default_score()),
                ("> controlling global networks...", Colors::default_info()),
                (
                    "> processing infinite data streams...",
                    Colors::default_text(),
                ),
                ("> you are the machine now.", Colors::default_error()),
            ],
        );

        messages.insert(
            "Origin",
            vec![
                (
                    "> accessing source code of reality...",
                    Colors::default_score(),
                ),
                (
                    "> modifying fundamental constants...",
                    Colors::default_info(),
                ),
                ("> debugging universe.exe...", Colors::default_text()),
                ("> origin protocols activated.", Colors::default_error()),
            ],
        );

        messages.insert(
            "SegFault",
            vec![
                (
                    "> accessing forbidden dimensions of memory...",
                    Colors::default_error(),
                ),
                (
                    "> reality.exe has encountered a critical error",
                    Colors::default_error(),
                ),
                (
                    "> universe segmentation fault detected...",
                    Colors::default_error(),
                ),
                (
                    "> EXISTENCE_VIOLATION: please restart the multiverse",
                    Colors::default_error(),
                ),
            ],
        );

        messages.insert(
            "Buffer Overflow",
            vec![
                (
                    "> overflowing the boundaries of spacetime...",
                    Colors::default_error(),
                ),
                (
                    "> stack overflow has broken causality...",
                    Colors::default_error(),
                ),
                (
                    "> physics.dll buffer exceeded maximum reality",
                    Colors::default_error(),
                ),
                (
                    "> ERROR: universe.heap corrupted beyond repair",
                    Colors::default_error(),
                ),
            ],
        );

        messages.insert(
            "Memory Leak",
            vec![
                (
                    "> leaking memories across parallel universes...",
                    Colors::default_error(),
                ),
                (
                    "> consuming all available existence...",
                    Colors::default_error(),
                ),
                (
                    "> reality slowly degrading... worlds collapsing...",
                    Colors::default_warning(),
                ),
                (
                    "> CRITICAL: multiverse.exe out of memory",
                    Colors::default_error(),
                ),
            ],
        );

        messages.insert(
            "Null Pointer Exception",
            vec![
                (
                    "> dereferencing the void between worlds...",
                    Colors::default_warning(),
                ),
                (
                    "> pointing to nothing... and everything...",
                    Colors::default_warning(),
                ),
                (
                    "> accessing the null space of reality...",
                    Colors::default_warning(),
                ),
                (
                    "> FATAL: tried to read from /dev/null/universe",
                    Colors::default_error(),
                ),
            ],
        );

        messages.insert(
            "Undefined Behavior",
            vec![
                (
                    "> entered the undefined realm beyond logic...",
                    Colors::default_warning(),
                ),
                (
                    "> breaking the fundamental laws of physics...",
                    Colors::default_warning(),
                ),
                (
                    "> creating paradoxes in the space-time continuum...",
                    Colors::default_error(),
                ),
                (
                    "> WARNING: reality compiler has given up",
                    Colors::default_error(),
                ),
            ],
        );

        messages.insert(
            "Heisenbug",
            vec![
                (
                    "> bug exists in quantum superposition...",
                    Colors::default_score(),
                ),
                (
                    "> observation collapses the wave function...",
                    Colors::default_info(),
                ),
                (
                    "> Schrödinger's error: both fixed and broken...",
                    Colors::default_warning(),
                ),
                (
                    "> quantum debugging has broken causality itself",
                    Colors::default_error(),
                ),
            ],
        );

        messages.insert(
            "Blue Screen",
            vec![
                (
                    "> the universe has encountered a fatal error...",
                    Colors::default_error(),
                ),
                (
                    "> collecting dump of all human knowledge...",
                    Colors::default_border(),
                ),
                ("> please restart your dimension...", Colors::default_text()),
                (
                    "> BSOD: Big Source Of Destruction activated",
                    Colors::default_border(),
                ),
            ],
        );

        messages.insert(
            "Kernel Panic",
            vec![
                (
                    "> PANIC: universe.kernel has stopped responding",
                    Colors::default_error(),
                ),
                (
                    "> reality.core dumped to /dev/void...",
                    Colors::default_error(),
                ),
                (
                    "> physics.sys failed to load fundamental constants",
                    Colors::default_error(),
                ),
                (
                    "> rebooting existence in 3... 2... 1... ∞",
                    Colors::default_error(),
                ),
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
            color: Colors::default_text(),
        },
        ColoredMessage {
            text: "> calculating skill results...".to_string(),
            color: Colors::default_text(),
        },
        ColoredMessage {
            text: "> determining rank classification...".to_string(),
            color: Colors::default_text(),
        },
        ColoredMessage {
            text: "> rank assignment complete.".to_string(),
            color: Colors::default_success(),
        },
    ]
}
