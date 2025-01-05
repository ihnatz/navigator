# Navigator

A lightning-fast CLI utility that helps you navigate and execute your frequently used commands through an interactive hierarchical menu.

![demo](https://github.com/user-attachments/assets/21bd709d-7a6f-4c6a-925e-a831360d0209)

## Features

- Hierarchical command organization through a simple JSON configuration
- Fast keyboard-driven navigation
- Automatic command insertion into your terminal prompt
- No need to remember complex commands or their syntax
- Minimal and straightforward interface

## Usage

- Press Ctrl+G to open the navigator menu
- Use arrow keys (or h,j,k,l) to navigate through categories and commands
- Press Enter to select a category or command
- When a command is selected, it will be automatically inserted into your terminal prompt
- Press Enter again to execute the command

## How It Works

Navigator provides a simple TUI (Text User Interface) that:

- Reads your command hierarchy from the JSON configuration
- Displays an interactive menu for navigation
- When a command is selected, it automatically inserts it into your terminal prompt
- Returns control to your shell for command execution
