# **🦀 Rust Data Science Environment**

A modular **Rust Workspace Template** designed for high-performance data analysis and machine learning experiments. This repository serves as a polished starting point, implementing a scalable architecture with pre-configured dependencies and a shared utility library.

## **📂 Workspace Structure**

The project is organized as a Cargo Workspace to share dependencies and compilation artifacts efficiently.

| Crate | Type | Description |
| :---- | :---- | :---- |
| **file\_utility** | Library | Shared internal logic for filesystem operations and agnostic path handling. |
| **your\_new\_app** | Binary | *\[Placeholder\]* Your future data analysis crates will live here. |

## **🛠️ Tech Stack & Dependencies**

This environment leverages the modern Rust Data Science ecosystem. All versions are pinned in the root Cargo.toml.

| External Crate | Version | Role & Description |
| :---- | :---- | :---- |
| **[Polars](https://pola.rs/)** | 0.52.0 | Blazingly fast DataFrames library. Configured with **Lazy API** for query optimization and memory efficiency. |
| [**Clap**](https://github.com/clap-rs/clap) | 4.5.x | Command Line Argument Parser. Handles robust CLI interfaces and generates help documentation automatically. |
| [**SciRs2**](https://www.google.com/search?q=https://github.com/scirs/scirs2) | 0.1.0-rc | Scientific computing suite inspired by SciPy. Used for linear algebra, statistics, and optimization algorithms. |
| [**Plotlars**](https://www.google.com/search?q=https://github.com/plotlars-rs/plotlars) | 0.11.1 | High-level plotting wrapper that integrates seamlessly with Polars for data visualization. |

## **🚀 Getting Started**

Since this is a workspace, you can add multiple distinct analysis projects that share the same foundation.

### **1\. Create a New Analysis Project**

To start a new experiment (e.g., market\_analysis), run:

```bash
cargo new market\_analysis
```

Then, add it to the members list in the root Cargo.toml.

### **2\. Inherit Dependencies**

In your new crate's Cargo.toml, inherit the pre-configured versions:

```toml
[dependencies]  
# Internal  
file_utility = { workspace = true }

# External  
polars = { workspace = true }  
clap = { workspace = true }  
```
### **3. Run Your App**

Select the specific package to run:

```
# Syntax: cargo run -p <package_name>  
cargo run -p market_analysis

```
## **⚙️ Development Setup**

In the root cargo project:

* **Resolver:** Uses resolver = "2" for optimal feature isolation between build, dev, and runtime dependencies.  
* **Optimization:** Dev profile is set to opt-level = 3 for dependencies.

