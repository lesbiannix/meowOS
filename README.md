# meowOS

This is a new OS with a shell, created by Jules.

## Architecture

meowOS is a monolithic kernel written in Rust. It is based on the Phil Oppermann tutorial series, "Writing an OS in Rust".

The kernel is composed of the following modules:

*   `vga_buffer`: Provides a safe interface to the VGA text buffer.
*   `interrupts`: Handles CPU exceptions and hardware interrupts.
*   `gdt`: Sets up the Global Descriptor Table.
*   `memory`: Provides memory management functionality.
*   `shell`: Implements the interactive shell.
    *   `parser`: Parses user input into commands and arguments.
    *   `commands`: Contains the implementation of the shell commands.

## Building and Running

To build and run the OS, you will need to have the following tools installed:

*   `rustup`
*   `cargo`
*   `bootimage`
*   `qemu`

Once you have these tools installed, you can build and run the OS with the following command:

```
cargo bootimage
```

## Adding New Shell Commands

To add a new shell command, you will need to do the following:

1.  Create a new file in `src/shell/commands` that implements the `Command` trait. The `Command` trait has two methods: `name` and `execute`. The `name` method should return the name of the command, and the `execute` method should contain the logic for the command.
2.  Add the new command to the `CommandExecutor` in `src/shell/parser.rs`. The `CommandExecutor` is responsible for parsing and executing commands.
3.  Add a `pub mod` declaration for the new command in `src/shell/commands/mod.rs`.

For an example of how to implement a new command, see `src/shell/commands/uname.rs`.