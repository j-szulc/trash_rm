# trash_rm
Alternative for `rm` that moves files to Trash instead of deleting them. A working remake of careful_rm.

## Installation

### From source

```bash
git clone https://github.com/j-szulc/trash_rm.git && \
cargo install --path trash_rm
```

## Usage

```
trash_rm 0.1.0
Move files to Trash

USAGE:
    trash_rm [FILE]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>...    Files to process
```