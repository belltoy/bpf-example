BPF Example in Rust
===================

An example repo to show a eBPF development environment for Linux Kernel on Mac, with **Rust** programming language.

## Requirement

- Install [VirtualBox](http://virtualbox.org) and its Extension Pack.
- Install [vagrant](http://vagrantup.com), and plugins.

```sh
brew install vagrant
vagrant plguin install vagrant-reload
vagrant plugin install vagrant-vbguest
```

## Initialize Vagrant VM

Note: the default memory for the VM has set to 4GB in [Vagrantfile](Vagrantfile).
If you have less memory of your host mac machine, it can be reduced.
But be sure no less than 2GB.

Now let's bring the environment up with ONE command:

```sh
vagrant up
```

## Almost There

If everything goes fine, we can log into the vagrant VM and build and run bpf programs.

```sh
# SSH login into the Vagrant VM env
vagrant ssh

# Project workspace path
cd /workspace/bpf-example

# Build bpf programs
cargo bpf build foo

# Use `ip` command to show your net interfaces
ip addr

# Loading eBPF programs requires admin priviledges, so you'll have to run load as root or with sudo:
# Assume your iface name is eth0
sudo cargo bpf load -i eth0 target/bpf/programs/foo/foo.elf
```
Happy coding.

## About Tools and Libs

This environment dependents on [redbpf](https://github.com/ingraind/redbpf) and [cargo-bfp](https://ingraind.org/api/cargo_bpf/).

```sh
$ cargo bpf -h
cargo-bpf 1.3.0
Alessandro Decina <alessandro.d@gmail.com>
Peter Parkanyi <peter@redsift.io>
A cargo subcommand for developing eBPF programs

USAGE:
    cargo bpf <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add        Adds a new eBPF program at src/<NAME>
    bindgen    Generates rust bindings from C headers
    build      Compiles the eBPF programs in the package
    help       Prints this message or the help of the given subcommand(s)
    load       Loads the specified eBPF program
    new        Creates a new eBPF package at <PATH>
```

## License

Under [MIT license](LICENSE).
