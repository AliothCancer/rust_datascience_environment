🦀 Rust Data Science EnvironmentA modular Rust Workspace Template designed for high-performance data analysis and machine learning experiments. This repository serves as a polished starting point, implementing a scalable architecture with pre-configured dependencies and a shared utility library.📂 Workspace StructureThe project is organized as a Cargo Workspace to share dependencies and compilation artifacts efficiently.CrateTypeDescriptionfile_utilityLibraryShared internal logic for filesystem operations and agnostic path handling.your_new_appBinary[Placeholder] Your future data analysis crates will live here.🛠️ Tech Stack & DependenciesThis environment leverages the modern Rust Data Science ecosystem. All versions are pinned in the root Cargo.toml.External CrateVersionRole & DescriptionPolars0.52.0Blazingly fast DataFrames library. Configured with Lazy API for query optimization and memory efficiency.Clap4.5.xCommand Line Argument Parser. Handles robust CLI interfaces and generates help documentation automatically.SciRs20.1.0-rcScientific computing suite inspired by SciPy. Used for linear algebra, statistics, and optimization algorithms.Plotlars0.11.1High-level plotting wrapper that integrates seamlessly with Polars for data visualization.🚀 Getting StartedSince this is a workspace, you can add multiple distinct analysis projects that share the same foundation.1. Create a New Analysis ProjectTo start a new experiment (e.g., market_analysis), run:cargo new market_analysis
Then, add it to the members list in the root Cargo.toml.2. Inherit DependenciesIn your new crate's Cargo.toml, inherit the pre-configured versions:[dependencies]
# Internal
file_utility = { workspace = true }

# External
polars = { workspace = true }
clap = { workspace = true }
3. Run Your AppSelect the specific package to run:# Syntax: cargo run -p <package_name>
cargo run -p market_analysis
⚙️ Development SetupResolver: Uses resolver = "2" for optimal feature isolation between build, dev, and runtime dependencies.Optimization: Dev profile is set to opt-level = 3 for dependencies to ensure Polars runs fast even during development.<!-- end list --># Cargo.toml configuration snippet
[profile.dev.package."*"]
opt-level = 3
