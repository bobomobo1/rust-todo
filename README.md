A CLI todolist written in Rust.

### Quick Installation

To quickly install, clone the repo and run the installation script:

```bash
git clone https://github.com/bobomobo1/todo_cli.git
cd todo_cli
./install.sh
```
### How to use

Add your first task:

```bash
todo add "Example Task"
```

This will add a todo.js file to the directory you are in. Different directories can have different todo.js files

Then list it:

```bash
todo list
```
You should see something like

```css
1 - [ ] Example Task
```

To complete it:


```bash
todo complete 1
```

To remove it:
```bash
todo remove 1
```
