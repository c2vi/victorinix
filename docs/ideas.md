# My Ideas around this repo

## Fully portable and fastest way to get running
I have my NixOS vastly customized (ref, my NixOS config [here](https://github.com/c2vi/nixos)) to the point, where if I have to use someone elses system, where the CTRL and ALT keys are not switched like on my system, it annoyes tremendously.

So the main goal here is to load personal config, Tools, Scripts and so on **as quickly as possible** and fully contained to one folder onto any system, and if no longer needed, just delete this folder.

## The "you can move the folder" Hack
I want the one static executable, that you download to know where the folder it is using (to store all things it needs) is located. So in order to be able to move the folder the executable should be able to patch itself to a folder location. (like done here: [https://blog.dend.ro/self-modifying-rust/](https://blog.dend.ro/self-modifying-rust/))

## "Registering" things with the system.
Being fully portable is a main goal, but sometimes you just need to install/register something to/with your system. eg: a url handler, to log into any of those modern apps, that redirect you to the browser in order to log in. Se there will be a feature, to register things with your system, eg: install a program to /bin/. But each of those registrations will be fully tracked (you'll know exactly, what Victorinix changed about your system) in order to be completely reversible.

### Inspiration from snaps and flatpacks
A snap package has "plugs and slots" ... that snaps can have... this is the thing, i want to do as well: run systems more isolated and define interfaces between them ... eg a sock file, dbus paths, ....

## The nix privision part
I am a keen NixOS user, but there is one thing that bothers me about it. In order to install my system, I first need to format the disks the way they are specified in the config. I want a "system config" to be a thing, where I can run one command which i give a disk and then after booting this disk, i have my exact system.

### Secrets management
I don't know how exactly this is going to work, but i want an ability to write the secrets needed for a system install also to the specified disk.

### The Victorinix Search
The end goal is that anything you would want to do on your system, you could just search here and find it.

#### Search categories (should also just be victorinix types)
- random useful peaces of information
- bash aliases and functions
- quick actions
- NixOS system config modules (to add some options, or combine with other modules to it and use right away)
- programs packaged here
    - wine applications
    - windows applications
- systems
    - chroot systems
    - vm systems (goal: open Victorinix gui, search for MacOS, hit enter twice, wait and you should have a fully setup MacOS VM)
        - a way to search all packages available on macos including mac-ports, homebrew, app-store, nixpkgs, ...
- all the victorinix types
- build-info
    - interesting things you discover when building and packaging software
    - aka my t/build tag in obsidian
- build-err and bug
    - have a flag: fixed

#### Search categories from external sources
- google results
- ask llm
- CVEs
- [grepper answers](https://www.grepper.com/)
- NixOS options
- programs (from nixpkgs, debian, and many other distros)
- debian packages
- mac and ios app store
- google play-store
- windows store
- winget
- this flakehub

## Improved NixOS Module System

## Other ideas
- run a nixos system inside of an Xephyr xserver
- make installing a new OS onto a pc FAST
    - doanload Victorinix binary
    - open gui
    - search os of choice
    - hit install and select drive to install onto
    - wait
    - done
    - if it's the running disk, this will make some space at the end of the disk (or somewhere else on it), install a tiny linux distro onto it, copy the selected config to it and tell the firmware to boot that partition, this distro will copy itself completely to ram on boot and then install the selected config onto the drive.


# Programs to package portably
- incus
- podman
- docker
- wine apps and windows apps (for the windows versio of it)
    - altium
    - fusion (make the export dialog work)
- ross (robotic communication)
- multisim
- raspi images as a vm
- python
    - a python pip venv
    - or a pyton nix env ... where you can also add pip and anaconda packages
- linix distros runnin in container with windowed Xserver
- gnome, kde, xfce that start as a windowed Xserver
- package some cool neovim, bash, tmux, wm (cool looking rices) and other cool dotfiles repos as runnable with one command....
    - eg: https://www.youtube.com/watch?v=VljhZ0e9zGE
- chris-titus's windows toolbox
- atlasOS

## package programs protably with their config
Programs packaged as an victorinix item have options, where you can configure them. 

### home-manager programs standalone
Programs from just nixpkgs do not have that. The established way to configure programs with nix is home-manager. So make a nix func, that takes a home.nix and turns it into a attrset with programs, that you can run. This will work, with turning the home.nix into one, with only that program configured and some mount namespcae magic, to write config files for this program into an overlayfs with the activation script from home-manager.

With such a mount namespcae hack also a one-command home-manager env should be able to run.

- or take inspiration from this: https://gist.github.com/siph/288b7c6b5f68a1902d28aebc95fde4c5#bonus-lazy-method

# WIP: brainstorming
## command line interface
- eg: `vic run firefox`
    - should run the newest firefox from nixpkgs, basically: `nix run nixpkgs#firefox`
- eg: `vic run github:c2vi/nixos#cbm`
    - should run the cbm program i packaged
- eg: `vic run ubuntu -o vm,gui -n "{...}: {gui=true;}"`
    - should run a vm of the ubuntu distribution
    - with -n you can define a nix func, that returns options for this item
    - with -o you can set those options more quickly. "-o vm" would be the same as "-n '{...}:{vm=true;}'"
    - -o sshkey should empty the list of ssh keys
    - -o sshkey=$key should set the ssh keys to this one
    - to add a key: -n "{config, ...}: {sshkey=config.sshkey ++ ['$key']}"
- eg: vic run debian
    ⁻ should drop you into the shell of a debian container
- eg: vic run github:c2vi/nxios
    - should run the default nixosConfiguration of this flake, as a docker container, with systemd and gui (if the config has services.xserver set) as a window.....
- vic run -e
    - edit the default options in editor set in the editor option of vic itself ... this option should default to $EDITOR
    - recommended options, mandatory options, options from env should be at the top
- vic get
    - same as run, but downloads it, so that running would be faster
    - for things that get config, you can configure them here
- vic stat
    - show gotten items, their closure size, what would be freed by `vic nix store gc`
    - size of app-data folders
- vic list app-data
    - list app-data folders
- vic list installed
- vic list registrations
- vic list daemons
- vic gui
    - should run the gui
- shortcuts with only few letters!!!!

## option types
Victorinix items like ubuntu, debian, nixos,  .... aka linux distros all will have some common options like vm, gui, username, pwd, rootpwd, .... The list of those options forms an victorinix type, in this case linuxDistro.

## types
- linuxDistro
- nixpkgsProgram
- aurProgram
- nixDeriviation
- provision
- vm
- runnable
- daemonizable
- buildable
- oci-container
- instruction
- script

## the victorinix env options
In order to not add common options like sshkey all the time to your vms, you can set an option in a so called victorinix environment, and those options will automatically be applied to items that support those. (as a layer over default options, but underneath options set by the user, so you can overwrite those with -o sshkey)


# inspiration
- kasmweb: stream docker containers with desktop env to the browser
- vagrant: vm management software from HashiCorp


# victorinix concepts
## items
The things that you find when searching, can be run, instantiated, built, ... Always have options to configure them.

## options
For any item, you can configure the way it is run/built/... by setting certain options.

## types
A set of options. I an item has this type, it has all the options of this type. An item can have multiple types.

## instances
A thing that represents an output of an item. Can be runnable as a daemon, can have app-data stored with it, ...

## connections or plugs
Instances are mostly isolated. But some communication or shared resources are needed between them. So you eg connect your dbus into this instance, so that it can use it for bluetooth.

## env
A set of parameters, that describe your current system. eg: `env.os = "linux"`. Items can then require certain parameters of a system, eg programs that only run on windows require env.os to be "windows".

## transformations
If you want to run an item, that requires `env.os = "windows"`, but your current system is linux. A transformation describes ways to get from `env.os = windows` to `env.os = "linux"`. One way for that would be to start a vm (which would be done by a different item, that optionally requires `env.linux.kvm = true`), another to use wine.

## registrations
Victorinix should be fully portable, but sometimes you need/want to install things into your os. (eg url handlers, add a binary to the PATH, ...). Registrations are the only way victorinix changes your host os, and they are always fully reversible and you can view them to know exactly, what victorinix has changed about your system.


# config vs secrets vs build-inputs vs app-data
## config
Everything that says how an item should be instantiated.

## secrets
Strings, that are supposed to be secret. Like passwords, keys, ... They should not appear in any config, so that you can share your configs.

## build-inputs
Program files, ....

## app-data
Things that are created, while running an item and associated with this instance. Like minecraft Worlds, or if you instantiate a windows vm, all the things you install into that, are part of app-data.


