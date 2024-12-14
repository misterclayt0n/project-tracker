# Project Tracker
Very simple CLI project tracker, where you can add projects, tasks related to projects, manipulate them and see a progress bar.

### Building
1. Install the [Rust Programming Language](https://www.rust-lang.org)

2. Clone and build the project:
```zsh
git clone https://github.com/misterclayt0n/project-tracker
cd project-tracker
cargo build
```

3. Run the binary:
```zsh
./target/debug/project-tracker
```

or build + run it directly with `cargo`:

```zsh
cargo run -- <COMMAND>
```

### Note
So far I've only tested on Linux, since it keeps track of all data in a `data.json` file inside `.config/project-tracker/data.json`, I have no intention of developing this project to work on Windows or MacOS
