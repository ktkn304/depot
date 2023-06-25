# depot
depot is a command-line utility designed to simplify directory management.
It allows users to generate destination paths based on specified URLs and create directories or clone git repositories into those paths.
The behavior of depot is controlled by a configuration file, making it compatible with various version control systems (VCS).

## Features

- Generate destination paths based on user-provided URLs.
- Create directories at the generated paths.
- Clone git repositories into the generated paths.
- List the created directories.

## Installation

1. Clone this repository.
2. Execute the following command to install depot using Cargo:
```shell
cargo install --path <path_to_cloned_repository>
```
3. Copy configuration file.
```shell
cp <path_to_cloned_repository>/files/default.depotconfig.toml ~/.depotconfig.toml
```

## Usage
TODO

## Demo
TODO

## License
This project is licensed under the MIT License. See the LICENSE file for details.
