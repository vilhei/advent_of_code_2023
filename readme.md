# Advent of code rust template

## Installation
- Create repository using this as template
- Open terminal inside repository root
- Run the generate_advent_of_code executable inside [init](./init/) folder.
```bash
cd path_to_this_repo
./init/generate_advent_of_code # Linux
./init/generate_advent_of_code.exe # Windows
```
[init](./init/) has both Windows and Linux executable. Executable [source](https://github.com/vilhei/generate_advent_of_code).

- The initialization executable creates 24 .rs files inside [advent](./advent) folder one for each day.
- It also creates 24 text folders inside [inputs](./inputs/) folder

**Important** do not run the init binaries after you have made changes to the day*.rs files,  the binary will overwrite the files.

## Usage

- Implement `Task` trait which has 2 functions `task_part_one` and `task_part_two`.
- Build and run the cargo project with following command giving the day as argument to the built binary.

```bash
cargo run -- <day>
```

If `Task` for given day has been implemented the results are printed out on the console. 