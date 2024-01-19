# Course Manager CLI
This is a CLI for managing course folders and their resources such as laboratories, projects, notes and references.

## Installation
```shell
cargo install course_manager
```

## Features
- Create a course folder with the following structure:
```text
<course_name>
├── Labs
├── Notes
├── Projects
└── References
```
- Manage Course in separate semester folders
- CRUD operations for semesters and courses
- CRUD operations for course resources
- Open course resources in default editor

## Usage
### Common commands
```shell
Usage: course_manager <COMMAND>

Commands:
  create   
  remove   
  list     
  go       Open the resource folder in a new terminal window
  summary  Print a summary of the specified resource
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```




