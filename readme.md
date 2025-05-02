# 📝 TaskApp (Rust CLI Task Manager)

[link to task](https://roadmap.sh/projects/task-tracker)

**TaskApp** is a simple, file-based command-line application written in Rust that helps users manage tasks. It supports creating, listing, updating, deleting, and filtering tasks by status (`pending`, `done`, `in-progress`). Tasks are stored as JSON objects in a local file (`tasks.json`), making it lightweight and easy to use.

---

## 🚀 Features

- ✅ Create new tasks (default to `Pending`)
- 📋 List all tasks, or filter by `status`
- ✏️ Update task descriptions
- 🔄 Mark tasks as `Done` or `In Progress`
- ❌ Delete tasks by ID
- 🔍 View internal structure and logic using well-commented Rust code

---

## 🧱 Project Structure

```bash
task-app/
├── src/
│   ├── main.rs                # CLI entry point (not shown here, but assumed)
│   ├── tasks_module/
│   │   ├── tasks.rs           # Task logic: CRUD, serialization, filtering
│   │   └── tasks_definitions.rs # Structs and trait definitions
│   ├── utils.rs               # Utility functions (JSON helpers, file IO)
├── tasks.json                 # File-based task store
├── Cargo.toml                 # Dependencies and project metadata
└── README.md                  # You're reading it!
```

---

## 🛠️ Requirements

- [Rust (1.70+)](https://www.rust-lang.org/tools/install)
- No external dependencies required (Serde is used internally for JSON handling)

---

## ⚙️ How to Use

### 🔧 Build and Run

1. Clone the repo:

   ```bash
   git clone https://github.com/your-username/task-app.git
   cd task-app
   ```

2. Run the app (assuming `main.rs` provides CLI interface):

```bash
   cargo run -- [command] [options]
```

---

### 🧑‍💻 Available Commands (Assumed from Code)

| Command                      | Description                   |
| ---------------------------- | ----------------------------- |
| `create`                     | Create a new task             |
| `list`                       | List all tasks                |
| `list --status [status]`     | List tasks filtered by status |
| `update --id 1 --desc "..."` | Update a task's description   |
| `done --id 2`                | Mark task as done             |
| `progress --id 3`            | Mark task as in-progress      |
| `delete --id 4`              | Delete a task                 |

> ⚠️ These commands must be implemented in `main.rs`. If not, refer to the `TaskTrait` implementation for how to use the Rust methods programmatically.

---

### 🗃️ Data Format

Tasks are saved in `tasks.json` like so:

```json
{
  "tasks": [
    {
      "id": 1,
      "description": "Take out trash",
      "status": "pending"
    },
    {
      "id": 2,
      "description": "Read Rust docs",
      "status": "done"
    }
  ]
}
```

---

## 👨‍💻 Developer Guide

### Key Types

- `Task`: Represents a task with `id`, `description`, and `status`
- `TaskStatus`: Enum for `Pending`, `Done`, and `InPROGRESS`
- `TaskTrait`: Trait implementation for all task actions (create, update, etc.)

### Notable Functions

- `Task::new()` — creates a new task with a unique ID
- `Task::get(id)` — retrieves a task by ID
- `Task::create()` — serializes and saves a new task
- `Task::update(...)` — updates the description
- `Task::update_task_as_done(...)` — sets status to `Done`
- `replace_task_in_file(...)` — helper to update JSON content
- `sort_by_status(...)` — filters tasks by status

---

## ❓Troubleshooting

- **No tasks are listed**: Ensure `tasks.json` exists and contains a valid `"tasks"` array.
- **Invalid status error**: Status strings in JSON must match the `serde` renames (e.g. `"in-progress"` not `"InPROGRESS"`).
- **App panics**: Add logging or run with `RUST_BACKTRACE=1` for better error output.

---

## 📌 To Do

- [ ] Add CLI argument parser (e.g. `clap`)
- [ ] Add unit tests for all task methods
- [ ] Add file locking for concurrent safety
- [ ] Optional: Convert to persistent DB (e.g., SQLite)

---

## 📄 License

MIT License. Feel free to use, contribute, or fork this project.

---

## 🙋‍♀️ Contributing

Contributions are welcome! If you'd like to submit improvements, refactors, or new features, just fork the repo and open a pull request.

---

## ✉️ Contact

Questions, feedback, or bugs? Feel free to reach out via GitHub Issues.

---
