<p align="center">
  <img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" width="100" alt="project-logo">
</p>
<p align="center">
    <h1 align="center">I3HELPER</h1>
</p>
<p align="center">
    <em>Unlock i3s potential with seamless workspace management."</em>
</p>
<p align="center">
	<img src="https://img.shields.io/github/license/atareao/i3helper?style=default&logo=opensourceinitiative&logoColor=white&color=0080ff" alt="license">
	<img src="https://img.shields.io/github/last-commit/atareao/i3helper?style=default&logo=git&logoColor=white&color=0080ff" alt="last-commit">
	<img src="https://img.shields.io/github/languages/top/atareao/i3helper?style=default&color=0080ff" alt="repo-top-language">
	<img src="https://img.shields.io/github/languages/count/atareao/i3helper?style=default&color=0080ff" alt="repo-language-count">
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

I3helper streamlines interaction with the i3 window manager through event-driven workspace management, automatic tiling, and custom icon assignments. Leveraging Cargo.toml for external dependencies, i3helper combines async tasks with Tokio for efficient handling of window events. Featuring dynamic workspace renaming and single-instance execution, i3helper offers a robust solution for personalized i3 window manager configurations. The projects core functionalities, encapsulated in modules like config, workspace, and runner, enhance the user experience through seamless integration and simplified customization.

---

##  Features

|    |   Feature         | Description |
|----|-------------------|---------------------------------------------------------------|
| ⚙️  | **Architecture**  | The project follows a modular architecture that enables seamless interaction with the i3 window manager. It leverages event-driven programming with Tokio for handling async tasks and signal events efficiently. |
| 🔩 | **Code Quality**  | The codebase maintains high quality and style standards. It includes error handling mechanisms for robust configuration loading and structured code organization for easy maintenance. |
| 📄 | **Documentation** | The project provides comprehensive documentation with explanations of modules like Config, Parts, Workspace, etc. It clarifies dependencies using Cargo.toml and config.yml format for seamless understanding and usage. |
| 🔌 | **Integrations**  | Key integrations include async tasks, regex, serialization, single-instance handling, event-driven programming with Tokio, and tracing capabilities. It seamlessly interacts with the i3 window manager via tokio-i3ipc integration. |
| 🧩 | **Modularity**    | The codebase exhibits high modularity by exporting modules like config, parts, workspace, root, and runner. This promotes code reusability and easy access to essential components for managing i3 window manager settings. |
| 🧪 | **Testing**       | The project uses testing frameworks to ensure code quality and functionality. Additional details on specific testing tools used are not provided in the information available. |
| ⚡️  | **Performance**   | The project demonstrates efficiency in handling workspace management, autotiling, and window movements. It efficiently executes commands on the i3 window manager using async operations, ensuring speed and resource optimization. |
| 🛡️ | **Security**      | The project includes measures for data protection through error handling during configuration loading. It may benefit from additional information regarding access control mechanisms for enhanced security. |
| 📦 | **Dependencies**  | Key dependencies include 'rs', 'toml', 'lock', 'yaml', and 'rust'. The project manages external crates for various functionalities such as async tasks, regex, serialization, and event-driven programming. |
| 🚀 | **Scalability**   | The project demonstrates scalability by supporting dynamic workspace management and seamlessly interacting with the i3 window manager. It has the potential to handle increased traffic and load efficiently. |

---

##  Repository Structure

```sh
└── i3helper/
    ├── Cargo.lock
    ├── Cargo.toml
    ├── LICENSE
    ├── config.yml
    └── src
        ├── main.rs
        └── models
```

---

##  Modules

<details closed><summary>.</summary>

| File                                                                     | Summary                                                                                                                                                                                                                                                                                             |
| ---                                                                      | ---                                                                                                                                                                                                                                                                                                 |
| [Cargo.toml](https://github.com/atareao/i3helper/blob/master/Cargo.toml) | Clarifies i3helpers dependencies using Cargo.toml. Manages external crates for async tasks, regex, serialization, single-instance handling, event-driven programming with Tokio, and tracing capabilities. Facilitates seamless interaction with the i3 window manager via tokio-i3ipc integration. |
| [config.yml](https://github.com/atareao/i3helper/blob/master/config.yml) | Enables automatic naming, tiling, and custom icons in i3helper. Ensures unique icons for specified applications.                                                                                                                                                                                    |

</details>

<details closed><summary>src</summary>

| File                                                                   | Summary                                                                                                                                                                                                           |
| ---                                                                    | ---                                                                                                                                                                                                               |
| [main.rs](https://github.com/atareao/i3helper/blob/master/src/main.rs) | Implements event-based workspace renaming and autotiling based on window movements. Handles signal events and manages single-instance execution. Subscribes to i3 window events for dynamic workspace management. |

</details>

<details closed><summary>src.models</summary>

| File                                                                                    | Summary                                                                                                                                                                                                                                                                   |
| ---                                                                                     | ---                                                                                                                                                                                                                                                                       |
| [config.rs](https://github.com/atareao/i3helper/blob/master/src/models/config.rs)       | Defines a Config struct with icon mappings, features, and read_configuration method to load settings from config.yml. Ensures robust error handling during configuration loading for the i3helper repository.                                                             |
| [mod.rs](https://github.com/atareao/i3helper/blob/master/src/models/mod.rs)             | Exports configurations, parts, workspace, root, and runner modules. Facilitates modularity and organization within the codebase. Supports easy access to essential components for managing i3 window manager settings.                                                    |
| [parts.rs](https://github.com/atareao/i3helper/blob/master/src/models/parts.rs)         | Converts capture groups from a regular expression into structured parts for easy manipulation and back. Handles number, name, and icons fields. Enables seamless conversion between Parts struct and string representation.                                               |
| [root.rs](https://github.com/atareao/i3helper/blob/master/src/models/root.rs)           | Retrieves and organizes workspaces from the i3 window managers tree structure. Includes functions to fetch default workspaces, find specific workspaces, and recursively navigate the tree. Supports the i3helper repositorys architecture.                               |
| [runner.rs](https://github.com/atareao/i3helper/blob/master/src/models/runner.rs)       | Executes commands on the i3 window manager using async operations. Logs results and errors for each command.                                                                                                                                                              |
| [workspace.rs](https://github.com/atareao/i3helper/blob/master/src/models/workspace.rs) | Defines a Workspace struct for managing i3 workspace details.-Features methods to retrieve workspace properties like name, number, ID, focused status, contained nodes, and application names.-Implements recursive functions for focused node and application retrieval. |

</details>

---

##  Getting Started

**System Requirements:**

* **Rust**: `version x.y.z`

###  Installation

<h4>From <code>source</code></h4>

> 1. Clone the i3helper repository:
>
> ```console
> $ git clone https://github.com/atareao/i3helper
> ```
>
> 2. Change to the project directory:
> ```console
> $ cd i3helper
> ```
>
> 3. Install the dependencies:
> ```console
> $ cargo build
> ```

###  Usage

<h4>From <code>source</code></h4>

> Run i3helper using the command below:
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

- **[Report Issues](https://github.com/atareao/i3helper/issues)**: Submit bugs found or log feature requests for the `i3helper` project.
- **[Submit Pull Requests](https://github.com/atareao/i3helper/blob/main/CONTRIBUTING.md)**: Review open PRs, and submit your own PRs.
- **[Join the Discussions](https://github.com/atareao/i3helper/discussions)**: Share your insights, provide feedback, or ask questions.

<details closed>
<summary>Contributing Guidelines</summary>

1. **Fork the Repository**: Start by forking the project repository to your github account.
2. **Clone Locally**: Clone the forked repository to your local machine using a git client.
   ```sh
   git clone https://github.com/atareao/i3helper
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
   <a href="https://github.com{/atareao/i3helper/}graphs/contributors">
      <img src="https://contrib.rocks/image?repo=atareao/i3helper">
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
