# Git Implementation in Rust

This project is a simplified implementation of Git, written in Rust. It's designed as a learning tool for understanding the inner workings of Git and the Rust programming language.

## Features

Currently, the project implements the following Git commands:

- `init`: Initializes a new Git repository.
- `add`: Stages changes for the next commit.
- `cat-file`: Print the file content (only work for blobs until now).

Each command is implemented as a separate struct that implements a `Command` trait, following the strategy pattern. This design makes it easy to add new commands.

## How to Run

1. **Compile the Program**

    First, you need to compile the program. Navigate to the project directory and run the following command:

    ```bash
    cargo build --release
    ```

    This will create an executable in the ./target/release/ directory.

2. **Run the Program**

    You can run the program using the following command:

    ./target/release/{your-program-name} {command} {arguments}

    Replace {your-program-name} with the name of your program, {command} with the git command you want to execute (like init or add), and {arguments} with any arguments the command needs.

    For example, to initialize a new repository, you would run:

    ./target/release/{your-program-name} init

    And to stage a file, you would run:

    ./target/release/{your-program-name} add {file-path}

    Replace {file-path} with the path to the file you want to stage.

Please note that you need to have Rust and Cargo installed on your machine to compile and run the program. If you haven’t installed them yet, you can do so from the official Rust website.

## Disclaimer

Please note that this is a simplified version and doesn't handle all the cases that a real Git implementation would need to handle. It's meant for educational purposes only.

## Future Work

The next steps for this project include implementing more Git commands and improving error handling.

📚
