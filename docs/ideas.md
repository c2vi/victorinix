# My Ideas around this repo

## Fully portable and fastest way to get running
I have my NixOS vastly customized (ref, my NixOS config [here](https://github.com/c2vi/nixos)) to the point, where if I have to use someone elses system, where the CTRL and ALT keys are not switched like on my system, it annoyes tremendously.

So the main goal here is to load personal config, Tools, Scripts and so on **as quickly as possible** and fully contained to one folder onto any system, and if no longer needed, just delete this folder.

## The "you can move the folder" Hack
I want the one static executable, that you download to know where the folder it is using (to store all things it needs) is located. So in order to be able to move the folder the executable should be able to patch itself to a folder location. (like done here: [https://blog.dend.ro/self-modifying-rust/](https://blog.dend.ro/self-modifying-rust/))

## "Registering" things with the system.
Being fully portable is a main goal, but sometimes you just need to install/register something to/with your system. eg: a url handler, to log into any of those modern apps, that redirect you to the browser in order to log in. Se there will be a feature, to register things with your system, eg: install a program to /bin/. But each of those registrations will be fully tracked (you'll know exactly, what Victorinix changed about your system) in order to be completely reversible.

## The nix privision part
I am a keen NixOS user, but there is one thing that bothers me about it. In order to install my system, I first need to format the disks the way they are specified in the config. I want a "system config" to be a thing, where I can run one command which i give a disk and then after booting this disk, i have my exact system.

### Secrets management
I don't know how exactly this is going to work, but i want an ability to write the secrets needed for a system install also to the specified disk.

### The Victorinix Search
The end goal is that anything you would want to do on your system, you could just search here and find it.

#### Search categories
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

#### Search categories from external sources
- google results
- ask llm
- CVEs
- [grepper answers](https://www.grepper.com/)
- NixOS options
- programs (from nixpkgs and debian)
- debian packages

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





