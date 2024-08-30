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

Have an authetication machanism with qr codes. (like Discord)

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
    - change some options eg user, password, ip-addr, ssh-key, ...
    - hit install and select drive to install onto
    - wait
    - done
    - if it's the running disk, this will make some space at the end of the disk (or somewhere else on it), install a tiny linux distro onto it, copy the selected config to it and tell the firmware to boot that partition, this distro will copy itself completely to ram on boot and then install the selected config onto the drive.
- options for building webfiles
    - one option
    - so that i can download c2vi.dev/ml (ml for my-linux)
    - eg to include certain items as an overlay....
- make a thing, to quickly build a env to crosscompile a kernel module for a lineageos kernel
    - compiling this: https://github.com/oandrew/ipod-gadget
    - for the kernel: https://github.com/LineageOS/android_kernel_samsung_exynos9810
- cloud-init: https://github.com/canonical/cloud-init/blob/main/doc/examples/cloud-config.txt
- https://github.com/nix-community/dream2nix
- https://github.com/DeterminateSystems/nix-installer
- instruction items, for less techy people
    - eg instructions to flash vic based lineageos (generated from lineage docs of course)
    - a adb based script (with instruction item), that generates robotnix config from a running android phone + ectract app data and secrets the vic way
- app-data items: we as vic define program x stores its command history at this path..... and you see appdata items for victor items and can decide which to delete (eg delete caches), they have types like cache, history, user_data,...
- vic run http - runs darkhttp and serves pwd, have a index html that allows uploads + scriptable dir listings
- a way to only download what is needed for a nix build and cache that in a nix store.
    - a way to submit that downloaded stuff to the webarchive
    - and a way to tell nix to fetch from the webarchive if the url no longer exists
- victor spaces
    - a place to try out building and packaging stuff
    - you specify what you need (eg: sources, envs, ...)
        - also define artifacts, like original samsung firmware from some web location
    - then document steps (and other things) (with markdown)
        - see: victor steps
        - records bash_history, ....
        - have a way, to add important command to a history file, with markdown docs for each one...
        - and a way to go through your bash_history and mark certain commands as important
    - you can then keep the data if you are stil working on it, or delete it, when the project is sone
    - should also be usable to record steps taken to setup a rpi
- victor steps
    - a way to document and execute certain steps ... usually cmds
- vic init
    - inits example projects form all sources (rust, nix-flake-templates, npm packages, ...)
    - has config options obviosly, where you can change the project name eg
    - make one for my htl dipl markdown flake
    - be able to change certain options afterwards
        - for this an option implements imperative code, that tries to change some file in the project
- submit victor to debian, ubuntu, arch, t2, ..... package repos
- https://medium.com/nttlabs/container2wasm-2dd90a18cc9a

# Programs to package portably
- incus
- podman
- docker
- wine apps and windows apps (for the windows versio of it)
    - altium
    - fusion (make the export dialog work)
    - https://github.com/GloriousEggroll/wine-ge-custom
- ross (robotic communication)
- multisim
- raspi images as a vm
- python
    - a python pip venv
    - or a pyton nix env ... where you can also add pip and anaconda packages
- linix distros runnin in container with windowed Xserver
- gnome, kde, xfce that start as a windowed Xserver
- package programms with their config bundeled in
    - includes shell completions
    - package some cool neovim, bash, tmux, wm (cool looking rices) and other cool dotfiles repos as runnable with one command....
        - eg: https://www.youtube.com/watch?v=VljhZ0e9zGE
- chris-titus's windows toolbox
- atlasOS
- a quick run openvpn, wireguard, ....
- nix-ld and run-appimage (a way to run normal linux binaries)
- binfmt_misc registrations (with qemu but also winvm and macos vms)
- reverse wifi acces point
    - image on raspberry
- xserver
    - run as a window of another x-server
    - run and render directly to gpu device
    - run and render as a framebuffer in a file
    - run as a http server, that views the contents
    - run as a vnc server
    - run on another tty
- alpine build image
    - runs an alpine image, where you can instantly start building an alpine package (and aports checked out)
    - maybe other distros
- a thing to try out:
    - if you enter robotnix debugEnterEnv with nix installed on debian, you can't run vim, file, ... anymore
    - even running a vim, that is linked with things from the nix store does not work
    ```
./seb/file/bin/file: /usr/lib/libc.so.6: version `GLIBC_2.38' not found (required by ./seb/file/bin/file)
./seb/file/bin/file: /usr/lib/libc.so.6: version `GLIBC_2.34' not found (required by ./seb/file/bin/file)
./seb/file/bin/file: /usr/lib/libc.so.6: version `GLIBC_2.38' not found (required by /nix/store/4if234lnkhfkindsr9m62s4b4lh3iynf-file-5.45/lib/libmagic.so.1)
    ```
    - building a pkgsStatic.vim works
    - i also want `vic build vim; ./vim` to work in that case
- have a /a as an output, which is an apk, that can be installed onto android, basically a termux with vic installed
- run a nixos chroot, that mounts the /nix/store
- EasyBCD
- Stirling PDF
- https://github.com/CorentinTh/it-tools
- https://github.com/tteck/Proxmox
- windows powertoys.takeAll()
- https://joeyh.name/code/moreutils/
    - and have good ways to find the small tool, you need right now
- hand tracking, Gesichtserkennung, gegenstands erkennung
    - so that you can run that with: vic run oci-demo ... xD

## The reverse wifi acces point problem
You should be able to: `vic flash reverse-wifi-acces-point -s wifi-ssid=my-wifi,wifi-psk=supersecret /dev/mmcblk0`, stick the sdcard into a pi and it works.

Therefore you need the name and mac-addr for the ethernet card from env.hw.nic.0.hwaddr. But you can't have that without running vic-env dump on the target pi.

So either you flash something, which boots, runns vic-env fully from ram, finished the build and then reflashesa. (This is a pfusch)

It'd be better to inject this value at rumtime, which is actually similar to how secrets should work.

## The service dependencie problem
A windows exe requires a winvm to be running, a incus vm requires a incus daemon running. Nix does not really have a way to configure such things.

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
- vic list (short: ls)
    - by default lists all  things in sections
    - vic list data
    - vic list built
    - vic list registrations (short: reg)
    - vic list running
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

For a "system" we need to differentiate between usespace, kernel stuff and maybe boot stuff..... NixOS imo has a problem there, because it has userspace + kernelspace + bootstuff and seperates out hardware stuff (which does not work properly all the time)
One of the Biggest problems: change of interface names. (in my rpi python script wlp2s0 was hardcoded).

So when you ran a env-scan on my main, you should get, that it has a wifi card named wlp2s0, through which can connect to a network with ssid pw, which is made by the phone system. I want to be able to take this, and tell  qemu to emulate a wifi card, where i can then make an ssh connection over to the phone system also running as a vm.

There are some env props or config props can be changed at runtime. (eg pluging a usb drive into a system)

## There are different types of systems
- user-space system
    - basically a docker container
    - defined by env (docker cmdline, what network interfaces, port forwards, ...) and root fs and entrypoint
- kernel system
    - kernel, initrd and kernel args
- bios system or bootloader system??
    - series of bytes (usially a drive), where the first 512 bytes are read and executed, which shall load the rest of the system in a bios manner
    - need to check, how this works with windows (the chainloading)
- efi system
    - an efi exe and a drive
    - efi exe needs to laod rest of the system in an efi way

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


# nix projects to keep in mind, while building this
- https://github.com/serokell/deploy-rs
- https://github.com/nix-community/disko
- https://github.com/nix-community/awesome-nix
- https://github.com/NixOS/nixos-hardware
- https://github.com/paisano-nix/core
- https://github.com/divnix/hive
- https://github.com/chenxiaolong/aosproot
- https://github.com/divnix
- https://github.com/antifob/incus-windows
- https://github.com/thiagokokada/nix-alien
- https://www.youtube.com/watch?v=4VhJSxMKIqM
- found someone, who basically wants to do the same thing: https://discourse.nixos.org/t/nix-on-windows/1113/98
    - https://github.com/airtonix

