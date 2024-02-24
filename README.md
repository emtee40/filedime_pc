Filedime, an open source almost feature complete file explorer written in Rust (for all filesystem interaction,backend), Tauri(for frontend, front-backend intercom).

Features implemented:

- [x] multi window, open in new window open in right click context menu
- [x] tabs, open in new tab option in context menu
- [x] hot reload/ monitor for changes: markdown, html files using the watch button that shows up on previewing the file.
- [x] search with speed and responsiveness parity with fzf
- [x] folder size compute with speed and responsiveness parity with baobab(Disk Usage Analyzer).
- [x] bookmark files or folders.
- [x] details screen for list sort by date, size.
- [x] optionally show immediate sub folder count of a folder.
- [ ] recent files list.
- [ ] show image dimension along with file type. (cannot use image crate as it cannot be used on nixos in default preference)
- [ ] move files
- [ ] undo last operation
- [ ] show ramdisk among files (for the time being can bookmark ramdisk)


Subtle features
- [x] path autocomplete as you type.
- [x] no of each file type in present location.
- [x] show name of right click file above context menu
- [x] show file location on hover
- [x] show device vendor name on hover
- [x] LOC for ts, rs, js, java, md ,css, html, toml, etc more can be implemented as required.

Will be added based on demand
- [ ] System Tray icons for opening new window and access recent files.

Thanks Tauri, Typescript, Webpack->RSPack, NextJS, ShadCN


## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Build from source
  
clone the repo  
bun i
should use node version 20
in a new terminal window "cargo tauri dev" for testing.  
in a new terminal window "cargo tauri build" for making executables (.exe for windows or bin file on linux).  

when browsing through code use the extensions listed in extensions.code-profile in vscodium/vscode
when building on nixos use the shell.nix file provided if necessary

currently the code may contains lots of comments in rust it will be cleared up in the future.