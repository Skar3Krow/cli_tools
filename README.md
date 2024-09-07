# ClapBack
## Project Overview
This project is a cross-platform re-implementation of several essential GNU utility tools, offering a consistent user experience across various operating systems. The tools included in this package are designed to perform basic file and text manipulation tasks, mimicking the behavior of traditional Unix commands.

## Installation

### Pre-requisites:
1. Ensure that Rust is installed on your system. You can install Rust using the following command:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Cloning the Repository:
```
git clone https://github.com/your-repo/gnu-tools-reimplementation.git
cd gnu-tools-reimplementation
```

4. Building the Project:
```
cargo build --release
```

5. Running the Tools:
Once built, you can run the tools using the following command:
```
awu tool-name [options] [arguments]
```
Replace tool-name with any of the implemented functions, such as echo, list, cat, find, grep, or create.

## Features

### 1. Echo (echo)
- Description: Outputs the provided text to the console.
- Usage:
    ```
  echo [options] [text]
    ```
- Options:
   - n: Do not output the trailing newline.
     
### 2. List (ls)
- Description: Lists files and directories within a specified directory.
- Usage:
```
list [options] [directory]
```
- Options:
  - -a: List all files including hidden files.
  - -l: Use a long listing format.
  - -h: Print sizes in human-readable format.
    
### 3. Cat (cat)
- Description: Concatenates and displays the content of one or more files.
- Usage:
```
cat [options] [files...n]
```
- Options:
  - n: Number of input lines.
  
### 4. Find (find)
-Description: Searches for files in a directory hierarchy.
-Usage:
```
find [path...] [options] [directory]
```
- Options:
  - -name [pattern]: Search for files matching the given name pattern.
  - -type [type]: Search in a specific directory.

### 5. Grep (grep)
- Description: Searches for patterns within files.
- Usage:
```
grep [options] [pattern] [file...]
```
- Options:
  - -i: Ignore case distinctions.
  - -r: Recursively search subdirectories.
  - -v: Invert the sense of matching, to select non-matching lines.
  
### 6. Create (create)
- Description: Creates a new file with specified content or touch an existing one to update its timestamp.
- Usage:
```
create [options] [file] [content]
```
- Options:
  - -f: Force creation, even if the file exists.
  - -c: Create an empty file.


## Code Structure

src/
- args.rs - Contains the options and arguments of all the functions
- main.rs - Contains the implementation of the functions 
- Cargo.toml - Contains metadata about the project including dependencies. 

## Contributing

Contributions are welcome! Please follow the steps below to contribute to the project:

- Fork the repository.
- Create a new branch for your feature or bugfix.
- Make your changes and commit them with clear messages.
- Push your branch to your forked repository.
- Submit a pull request.
