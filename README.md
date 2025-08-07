## 🛠 Command Overview

This CLI app supports the following commands:

---

### ➕ `add`

Add a new task with a description.

```bash
cargo run add "Buy groceries"
```

---

### 📋 `list`

List all current tasks.

```bash
cargo run list
```

---

### 📝 `update`

Update the description of a task by its ID.

```bash
cargo run update 1 "Buy groceries and cook dinner"
```

---

### 🔄 `status`

Change the status of a task by its ID.  
Valid statuses: `Todo`, `InProgress`, `Done`

```bash
cargo run status 1 Done
```

---

### ❌ `delete`

Delete a task by its ID.

```bash
cargo run delete 1
```

---
