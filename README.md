# Hina - File and Process Management Utility

Hina is a versatile command-line utility for managing files and processes. It provides several modules to perform various tasks, from file manipulation to process monitoring.



## Table of Contents

- [Installation](#installation-and-uninstallation)
- [Usage](#usage)
- [Modules](#modules)
  - [rm - Remove Files or Directories](#rm---remove-files-or-directories-to-recycle-bin)
  - [rb - Recycle Bin Management](#rb---recycle-bin-management)
  - [mkndir - Create Nested Directories](#mkndir---create-nested-directories)
  - [rn - Batch Rename Files or Symbolic Links](#rn---batch-rename-files-or-symbolic-links)
  - [lc - Link Conversion](#lc---link-conversion)
  - [ps - Advanced Process Listing](#ps---advanced-process-listing)
- [DEBUG Mode](#debug-mode)
- [Examples](#examples)
- [Reporting Bugs](#reporting-bugs)



## Installation and Uninstallation

To install Hina, follow these steps:

```bash
# Clone the repository
git clone https://github.com/B1ACK917/Hina.git

# Navigate to the Hina directory
cd Hina

# configure to generate Makefile
# this will set the executable install path to /usr/local/bin
# and the manual to /usr/local/share/man/man1
./configure
# if you want to customize the installation, use --prefix
# e.g ./configure --prefix=$HOME/.local

# Compile the Rust code and install the executable & manual for man usage
make && sudo make install
```

To uninstall Hina, follow these stops:
```bash
# This will remove the executable and manual pages
sudo make uninstall

# Clean the generated sth. to reduce disk usage
make clean
```



## Usage

``` bash
# Run Hina with a specific command and options
hina <COMMAND> [OPTIONS]

# Read the help of Hina
hina --help

# Read the manual of Hina
man hina

# Read the manual of Hina module
hina <COMMAND> --help
# or
man hina-<COMMAND>
# e.g man hina-rm
```



## Modules

### rm - Remove Files or Directories to Recycle Bin

The `rm` module allows you to remove files or directories to recycle bin and later decide to truly remove them or restore them.

#### Usage

```bash
hina rm [path]
```



### rb - Recycle Bin Management

The `rb` module is used for managing the recycle bin. It provides options to list bin contents, restore files, and empty the bin.

#### Usage

```bash
hina rb [options]
```

#### Options

- **-ls, --list**: List the contents of the recycle bin.
- **-rs, --restore**: Restore a file from the recycle bin.
- **-ept, --empty**: Empty the recycle bin.



### mkndir - Create Nested Directories

The `mkndir` module creates nested directories for each file in the specified path. It supports recursive execution.

#### Usage

```bash
hina mkndir [path] [options]
```

#### Options

- **-r, --recursive**: Create nested directories recursively for files in subdirectories.



### rn - Batch Rename Files or Symbolic Links

The `rn` module allows batch renaming of files or symbolic links. It supports various options for flexible renaming.

#### Usage

```bash
hina rn [path] [options]
```

#### Options

- **-i=INPUT_PATTERN, --input=INPUT_PATTERN**: Specify the input pattern for renaming.
- **-o=OUTPUT_PATTERN, --output=OUTPUT_PATTERN**: Specify the output pattern for renaming.
- **-a=APPEND_STRING, --append=APPEND_STRING**: Specify the string to append during renaming.
- **-n=NUM_POSITION, --num=NUM_POSITION**: Specify the position for appending the string. 0 for prefix, 1 for suffix.
- **-r, --recursive**: Batch rename files recursively in subdirectories.
- **-s, --symlink**: Batch rename symbolic links.



### lc - Link Conversion

The `lc` module converts symbolic links to hard links or vice versa. It provides options for in-depth memory usage information.

#### Usage

```bash
hina lc [path] [options]
```

#### Options

- **--s2l**: Convert all symbolic links to hard links.
- **--l2s -i=INPUT_PATH**: Convert all hard links to symbolic links. Requires specifying the search path for finding source paths.
- **-r, --recursive**: Convert links recursively in subdirectories.



### ps - Advanced Process Listing

The `ps` module is an advanced process listing utility. It provides options for filtering, tracking, and detailed memory usage information.

#### Usage

```bash
hina ps [path] [options]
```

#### Options

- **-i=INPUT_PATTERN, --input=INPUT_PATTERN**: Filter processes by command containing the specified input pattern.
- **-t=PID, --track=PID**: Display the hierarchy of processes for the specified process ID.
- **-d[=PATH], --dump[=PATH]**: Dump detailed memory usage information for all processes. If `PATH` is provided, dump to the specified folder, otherwise dump to the 'proc' folder in the current directory.
- **-x, --x-ray**: Display detailed memory usage information including Swap, USS, PSS, and Size.
- **-s=SORT_FIELD, --sort-by=SORT_FIELD**: Sort detailed memory usage information by the specified field. Options: [swap, uss, pss, size, pid].
- **-h, --human-readable**: Display memory usage information in human-readable units.



## DEBUG Mode

Hina supports a DEBUG mode, which can be enabled by setting the `DEBUG` environment variable. When DEBUG is enabled, the utility will print additional debug information during execution. To activate DEBUG mode, you can use the following command:

```bash
DEBUG=1 hina <COMMAND> <OPTIONS>
```

Replace `<COMMAND>` and `<OPTIONS>` with the actual command and options you are using. When DEBUG mode is active, Hina will provide more detailed information about its internal processes, aiding in troubleshooting and debugging.

Note that if the `DEBUG` environment variable is not set, Hina will operate in the regular mode without printing debug information.

This feature is particularly useful when diagnosing issues or understanding the inner workings of Hina during command execution.



## Examples

- **Remove a File:**

  ```
  hina rm /path/to/file.txt
  ```
  
- **List Recycle Bin Contents:**

  ```
  hina rb --list
  ```
  
- **Create Nested Directories:**

  ```
  hina mkndir /path/to/files
  ```
  
- **Batch Rename Files:**

  ```
  hina rn /path/to/files -i=input_pattern -o=output_pattern -r
  ```
  
- **Convert Symbolic Links to Hard Links:**

  ```
  hina lc --s2l /path/to/links
  ```
  
- **Advanced Process Listing:**

  ```
  hina ps -x -s=uss -h
  ```



## Reporting Bugs

If you encounter any issues or have suggestions, please report them on the [GitHub issues page](https://github.com/B1ACK917/Hina/issues).
