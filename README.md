<dl>
  <p align="center"><img height="120" src="https://github.com/GHaxZ/fetchy/blob/master/imgs/logo.png" alt="Fetchy logo"></p>
  <p align="center"><b>A small command line system information tool written in Rust</b></p>
  
  <div align="center">
    <img height="250" src="https://github.com/GHaxZ/fetchy/blob/master/imgs/preview.png" alt="Fetchy preview">
    <img height="250" src="https://github.com/GHaxZ/fetchy/blob/master/imgs/preview2.png" alt="Fetchy preview">
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
- [ ] Display GPU data
- [ ] Pre-compiled binary for MacOS


### Installation
---
#### There are currently 2 ways to install this tool:
- Downloading a pre-compiled binary from the [releases](https://github.com/GHaxZ/fetchy/releases) page.
- Compiling the binary yourself using cargo.

#### Downloading pre-comiled binary
1. Download the correct binary from the [releases](https://github.com/GHaxZ/fetchy/releases) page (there are no MacOS binaries available yet).
2. Extract the archive file and put the binary file into a directory that has been added to your path.
3. Done! You can now run the `fetchy` command (you may have to restart your terminal).

#### Compiling yourself
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

### Contributing
---
> To contribute to this project you can open issues to report bugs or create a pull request if you want to improve or add functionality.

