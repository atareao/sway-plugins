<p align="center">
  <img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" width="100" alt="project-logo">
</p>
<p align="center">
    <h1 align="center">SWAY-PLUGINS</h1>
</p>
<p align="center">
    <em>Empower Sway, Elevate Workspaces, Embrace Efficiency</em>
</p>
<p align="center">
	<img src="https://img.shields.io/github/license/atareao/sway-plugins?style=default&logo=opensourceinitiative&logoColor=white&color=0080ff" alt="license">
	<img src="https://img.shields.io/github/last-commit/atareao/sway-plugins?style=default&logo=git&logoColor=white&color=0080ff" alt="last-commit">
	<img src="https://img.shields.io/github/languages/top/atareao/sway-plugins?style=default&color=0080ff" alt="repo-top-language">
	<img src="https://img.shields.io/github/languages/count/atareao/sway-plugins?style=default&color=0080ff" alt="repo-language-count">
<p>
<p align="center">
	<!-- default option, no dependency badges. -->
</p>

<br><!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary><br>

- [ Overview](#-overview)
- [ Features](#-features)
- [ Repository Structure](#-repository-structure)
- [ Modules](#-modules)
- [ Getting Started](#-getting-started)
  - [ Installation](#-installation)
  - [ Usage](#-usage)
  - [ Tests](#-tests)
- [ Project Roadmap](#-project-roadmap)
- [ Contributing](#-contributing)
- [ License](#-license)
- [ Acknowledgments](#-acknowledgments)
</details>
<hr>

##  Overview

Sway-plugins is a versatile project that enhances the functionality of the Sway window manager through autonaming, autotiling, and autotransparency features. With a focus on transparency and seamless integration with command-line utilities, Sway-plugins creates a robust plugin ecosystem to improve user experience. These plugins enable dynamic workspace renaming, efficient window tiling, and dynamic window opacity adjustments, providing users with enhanced productivity and customization options within the Sway environment.

---

##  Features

|    |   Feature         | Description |
|----|-------------------|---------------------------------------------------------------|
| ⚙️  | **Architecture**  | *The project follows a modular architecture emphasizing dynamic configuration handling and event-driven scripts. It focuses on creating a robust plugin ecosystem for enhanced user experience within the Sway window manager.* |
| 🔩 | **Code Quality**  | *The codebase exhibits well-structured and maintainable code with a focus on modularity. It follows Rust best practices and emphasizes readability and extensibility for plugin development.* |
| 📄 | **Documentation** | *The project includes detailed documentation embedded within the codebase, explaining the purpose and functionality of various components. However, external documentation could be enhanced to onboard new contributors and users efficiently.* |
| 🔌 | **Integrations**  | *Key integrations include Cargo for dependency management and various Rust libraries like toml-rs, yaml-rust for configuration handling. These integrations enhance the project's functionality and interoperability.* |
| 🧩 | **Modularity**    | *The codebase demonstrates high modularity with separate modules for configurations, plugins, and models. This design promotes code reusability and makes it easier to extend and customize plugin functionalities.* |
| 🧪 | **Testing**       | *The project uses Rust's built-in testing framework for unit tests to ensure the reliability and correctness of the codebase. However, additional integration and end-to-end testing could further improve overall code quality.* |
| ⚡️  | **Performance**   | *The codebase emphasizes efficiency in event processing and plugin lifecycle management. It aims to provide a smooth user experience within the Sway environment, focusing on speed and resource optimization.* |
| 🛡️ | **Security**      | *Security measures include graceful error handling, configuration validation, and dynamic command execution to prevent vulnerabilities. However, further security audits and validation checks are recommended.* |
| 📦 | **Dependencies**  | *Key dependencies include Cargo for package management, with dependencies like toml-rs and yaml-rust for configuration handling. These libraries enhance the project's capabilities and contribute to its functionality.* |

---

##  Repository Structure

```sh
└── sway-plugins/
    ├── Cargo.lock
    ├── Cargo.toml
    ├── LICENSE
    ├── README.md
    ├── config.yml
    └── src
        ├── main.rs
        ├── models
        └── plugins
```

---

##  Modules

<details closed><summary>.</summary>

| File                                                                         | Summary                                                                                                                                                                                                                                                                               |
| ---                                                                          | ---                                                                                                                                                                                                                                                                                   |
| [Cargo.toml](https://github.com/atareao/sway-plugins/blob/master/Cargo.toml) | Defines plugin dependencies for Sway window manager to enhance functionality. Emphasizes transparency, autonaming, and autotiling features. Facilitates seamless integration with command-line utilities. Focuses on creating a robust plugin ecosystem for improved user experience. |
| [config.yml](https://github.com/atareao/sway-plugins/blob/master/config.yml) | Enables autonaming, autotiling, and autotransparency configurations in Sway window manager plugins. Custom icons for various apps are defined in the config.yml file. These settings enhance user experience and productivity within the Sway environment.                            |

</details>

<details closed><summary>src</summary>

| File                                                                       | Summary                                                                                                                                                                                                                                                                                           |
| ---                                                                        | ---                                                                                                                                                                                                                                                                                               |
| [main.rs](https://github.com/atareao/sway-plugins/blob/master/src/main.rs) | Executes event-driven scripts based on a single-instance model. Manages plugin lifecycle with dynamic configuration handling. Signals termination/interruption to gracefully end processes. Emphasizes modular plugin architecture and efficient event processing for the SwayPlugins repository. |

</details>

<details closed><summary>src.models</summary>

| File                                                                                        | Summary                                                                                                                                                                                                                                      |
| ---                                                                                         | ---                                                                                                                                                                                                                                          |
| [config.rs](https://github.com/atareao/sway-plugins/blob/master/src/models/config.rs)       | Reads and parses YAML configuration file to struct. Contains autonaming, autotiling, and autotransparency settings. Handles errors gracefully. Influential in defining plugin behaviors within sway-plugins repository.                      |
| [mod.rs](https://github.com/atareao/sway-plugins/blob/master/src/models/mod.rs)             | Exposes configuration, parts, workspace, root, and runner modules for the sway-plugins repository. Facilitates access to critical components defining the architecture. Essential for configuring and managing the sway plugins effectively. |
| [parts.rs](https://github.com/atareao/sway-plugins/blob/master/src/models/parts.rs)         | Transforms string into structured parts using regex captures; enables conversion between Parts struct and string representation for easy manipulation and display.                                                                           |
| [root.rs](https://github.com/atareao/sway-plugins/blob/master/src/models/root.rs)           | Retrieves workspaces from a tree node recursively.-Instantiates a root node with connection details.-Finds a workspace by node reference.                                                                                                    |
| [runner.rs](https://github.com/atareao/sway-plugins/blob/master/src/models/runner.rs)       | Connection`. Handles multiple commands or a single command, logging results and errors. Contributes to the parent repositorys plugin architecture to interact with the Sway window manager.                                                  |
| [workspace.rs](https://github.com/atareao/sway-plugins/blob/master/src/models/workspace.rs) | Defines a Workspace struct with methods to retrieve workspace details, like name, number, ID, and applications. It also includes functions to check for focused nodes and determine if a specific node exists within the workspace.          |

</details>

<details closed><summary>src.plugins</summary>

| File                                                                                                       | Summary                                                                                                                                                                                                                                                                         |
| ---                                                                                                        | ---                                                                                                                                                                                                                                                                             |
| [autonaming.rs](https://github.com/atareao/sway-plugins/blob/master/src/plugins/autonaming.rs)             | Implements Autonaming plugin to dynamically rename workspaces based on app icons. Initiates, updates, and reverts workspace names. Accepts configuration for custom icon mappings and duplicate handling. Controlled by workspace events, ensuring accurate workspace labeling. |
| [autotiling.rs](https://github.com/atareao/sway-plugins/blob/master/src/plugins/autotiling.rs)             | Processes window events based on dimensions, executing split commands. Enables dynamic tiling in the Sway window manager.                                                                                                                                                       |
| [autotransparency.rs](https://github.com/atareao/sway-plugins/blob/master/src/plugins/autotransparency.rs) | Implements Autotransparency plugin for dynamic window opacity adjustments based on focus events. Utilizes configuration settings for enabling transparency and defining the opacity level. Executes specific commands for marking, unmarking, and adjusting window opacity.     |
| [mod.rs](https://github.com/atareao/sway-plugins/blob/master/src/plugins/mod.rs)                           | Enables integration of autonaming, autotiling, and autotransparency plugins. Exposes PluginTrait and Plugin for extensibility. Contributes to sway-plugins architectural flexibility and modularity.                                                                            |
| [plugin.rs](https://github.com/atareao/sway-plugins/blob/master/src/plugins/plugin.rs)                     | Defines plugins handling events in Sway window manager config. Orchestrates plugin lifecycle methods and interactions based on configuration, enabling modularity and extensibility in the repositorys architecture.                                                            |

</details>

---

##  Getting Started

**System Requirements:**

* **Rust**: `version x.y.z`

###  Installation

<h4>From <code>source</code></h4>

> 1. Clone the sway-plugins repository:
>
> ```console
> $ git clone https://github.com/atareao/sway-plugins
> ```
>
> 2. Change to the project directory:
> ```console
> $ cd sway-plugins
> ```
>
> 3. Install the dependencies:
> ```console
> $ cargo build
> ```

###  Usage

<h4>From <code>source</code></h4>

> Run sway-plugins using the command below:
> ```console
> $ cargo run
> ```

###  Tests

> Run the test suite using the command below:
> ```console
> $ cargo test
> ```

---

##  Project Roadmap

- [X] `► INSERT-TASK-1`
- [ ] `► INSERT-TASK-2`
- [ ] `► ...`

---

##  Contributing

Contributions are welcome! Here are several ways you can contribute:

- **[Report Issues](https://github.com/atareao/sway-plugins/issues)**: Submit bugs found or log feature requests for the `sway-plugins` project.
- **[Submit Pull Requests](https://github.com/atareao/sway-plugins/blob/main/CONTRIBUTING.md)**: Review open PRs, and submit your own PRs.
- **[Join the Discussions](https://github.com/atareao/sway-plugins/discussions)**: Share your insights, provide feedback, or ask questions.

<details closed>
<summary>Contributing Guidelines</summary>

1. **Fork the Repository**: Start by forking the project repository to your github account.
2. **Clone Locally**: Clone the forked repository to your local machine using a git client.
   ```sh
   git clone https://github.com/atareao/sway-plugins
   ```
3. **Create a New Branch**: Always work on a new branch, giving it a descriptive name.
   ```sh
   git checkout -b new-feature-x
   ```
4. **Make Your Changes**: Develop and test your changes locally.
5. **Commit Your Changes**: Commit with a clear message describing your updates.
   ```sh
   git commit -m 'Implemented new feature x.'
   ```
6. **Push to github**: Push the changes to your forked repository.
   ```sh
   git push origin new-feature-x
   ```
7. **Submit a Pull Request**: Create a PR against the original project repository. Clearly describe the changes and their motivations.
8. **Review**: Once your PR is reviewed and approved, it will be merged into the main branch. Congratulations on your contribution!
</details>

<details closed>
<summary>Contributor Graph</summary>
<br>
<p align="center">
   <a href="https://github.com{/atareao/sway-plugins/}graphs/contributors">
      <img src="https://contrib.rocks/image?repo=atareao/sway-plugins">
   </a>
</p>
</details>

---

##  License

This project is protected under the [SELECT-A-LICENSE](https://choosealicense.com/licenses) License. For more details, refer to the [LICENSE](https://choosealicense.com/licenses/) file.

---

##  Acknowledgments

- List any resources, contributors, inspiration, etc. here.

[**Return**](#-overview)

---
