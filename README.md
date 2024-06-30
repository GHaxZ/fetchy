<dl>
  <p align="center"><img height="120" src="https://github.com/GHaxZ/fetchy/blob/master/imgs/logo.png" alt="Fetchy logo"></p>
  <p align="center"><b>A small command line system information tool written in Rust</b></p>
  
  <div align="center">
    <img height="300" src="https://github.com/GHaxZ/fetchy/blob/master/imgs/preview.png" alt="Fetchy preview">
    <img height="300" src="https://github.com/GHaxZ/fetchy/blob/master/imgs/preview2.png" alt="Fetchy preview">
  </div>
</dl>

Fetchy is a small command line tool that displays basic system information in a neat way.
It is inspired by <a href="https://github.com/dylanaraps/neofetch">neofetch</a>, but also wants to show more general information about the system. 
Fetchy also tries to be as cross-platform compatible as possible.
It is also written in Rust to make information gathering as fast and efficient as possible.


> [!NOTE]
> The version you are seeing right now is still an early version. There are still some features and system information missing.

### Roadmap
---
#### These are some features I'm currently planning on implementing

- [x] Display storage data
- [x] Pre-compiled binaries
- [x] Make colors customizable
- [x] Display total network usage (usage since last established connection)
- [x] Display battery data
- [x] Display GPU data
- [ ] Display current network usage (usage per second) + improve current network adapter detection
- [ ] Design overhaul
- [x] Pre-compiled binary for MacOS

#### And these are some internal code changes that are planned

- [ ] Changing configuration file type from .json to .toml
- [ ] Storing the configuration file in conventional places instead of next to binary
- [ ] Improve argument parsing
- [ ] Improve error handling

### Installation
---
#### There are currently 3 ways to install this tool:
- (Recommended) Installing the tool using the `cargo install` command. (cargo required)
- Downloading a pre-compiled binary from the [releases](https://github.com/GHaxZ/fetchy/releases) page. (cargo not required)
- Compiling the binary yourself using cargo. (cargo required)

#### Which one should you choose
- The first and also recommended way is using the `cargo install` command. This ensures the program gets compiled correctly for your specific system and automatically adds the binary to your path. Use this if you have cargo installed.
- If you don't have cargo installed on your system, you need to download a pre-compiled binary from the [releases](https://github.com/GHaxZ/fetchy/releases) page (there are no MacOS binaries available yet). You need to add the binary to your path manually and may encounter issues when running the tool, since it was not compiled for your specific system.
- You can also compile the program yourself from your local source code using `cargo build`. You still need to add the binary to your path manually. You can do it this way if you like, but using the first recommended way is easier.

#### (Recommended) Installing using cargo (cargo required)
1. Open up your terminal and enter the following command: `cargo install --git https://github.com/GHaxZ/fetchy/`
2. Then run the command and wait for cargo to finish compiling the program.
3. Done! Cargo has automatically added the binary to your path. You can now run the `fetchy` command (you may have to restart your terminal).

#### Downloading pre-comiled binary (cargo not required)
1. Download the correct binary from the [releases](https://github.com/GHaxZ/fetchy/releases) page (there are no MacOS binaries available yet).
2. Extract the archive file and put the binary file into a directory that has been added to your path.
3. Done! You can now run the `fetchy` command (you may have to restart your terminal).

#### Compiling yourself (cargo required)
1. Open a terminal in any directory where you want the source code to go.
2. Run `git clone https://github.com/GHaxZ/fetchy/`.
3. Then run `cargo build`.
4. After you have done this there will be a compiled binary in the `/target/debug` folder.
5. Put this binary into a directory that has been added to your path.
6. Done! You can now run the `fetchy` command (you may have to restart your terminal).


### How to use
---
#### Running the tool
To run the tool you can simply run the `fetchy` command in your terminal no matter what directory you are currently in.

_If it's not working, make sure you have added the directory the fetchy binary is located in to you path variable._



#### Changing the accent color
To change the accent color run the `fetchy` command with the additional `--color` argument. After the `--color` argument add a RGB color value, where the single color values are sepperated by a `,`.

Here's an example how the command should look: `fetchy --color 155,0,255`

It should tell you that the accent color has been changed, and after running the `fetchy` command again, you should see your custom color being used.

_You can also reset the accent color by running the `fetchy --color default` command._

> [!WARNING]
> Custom colors should be supported on most modern os versions. If you are using an older os however, the colors may not work as expected.

#### Opening the save directory
To open the directory fetchy is saved in use the `--dir` argument: `fetchy --dir`



### Contributing
---
> To contribute to this project you can open issues to report bugs or create a pull request if you want to improve or add functionality.

