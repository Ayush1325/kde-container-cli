# Introduction
This is a simple cli tool for creating and managing container environments for KDE development.

# Goals
1. Expand [kdepim docker scripts](https://community.kde.org/KDE_PIM/Docker) and provide a better cli.
2. Add support for podman along with docker.
3. Use nvidia-container-toolkit for nvidia systems.
4. Be minimal and easy to extend.

# Usage
1. Download the linux tar file from releases and extract its contents.
2. Navigate to the directory.
3. Use the kde_container_cli binary.
``` sh
kde_container_cli 1.0
Ayush Singh <ayush1325@gmail.com>
This is a cli tool to manage containers created for kde development

USAGE:
    kde_container_cli <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build     Build the container image
    config    Extra configuration that might be needed
    help      Prints this message or the help of the given subcommand(s)
    launch    Launch GUI application inside container
    run       Run the kdepim container. Creates a new container if it does not already exist
```

# Features
1. Build the image:
``` sh
./kde_container_cli build --nvidia podman
```

2. Run the conntainer:
``` sh
./kde_container_cli run [kdesrc path] --nvidia podman
```

3. Launch GUI apps like kdevelop and qtcreator.

``` sh
./kde_container_cli launch kdevelop podman
```

4. Some additional config
``` sh
./kde_container_cli config --xhost
```

# FAQ
1. Cannot start gui appications in podman container.
You might need add to xhost. This can be done either manually or using the cli by:
``` sh
./kde_container_cli config --xhost
```

# Why Rust
Rust is a fast and great language, but more importantly, it produces binaries which are easier to distribute. 
Golang might have been a good choice too but I am just more familiar with Rust.

# Special Thanks
- Daniel Vrátil: For kdepim docker scripts.
