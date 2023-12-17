<dl>
  <p align="center"><img height="120" src="https://github.com/GHaxZ/fetchy/blob/master/imgs/logo.png" alt="Fetchy logo"></p>
  <p align="center"><b>A small command line system information tool written in Rust</b></p>
  
  <div>
    <img align="left" width="40%" src="https://github.com/GHaxZ/fetchy/blob/master/imgs/preview.png" alt="Fetchy preview">
    <p>
    Fetchy is a small command line tool that displays basic system information in a neat way.
    It is inspired by <a href="https://github.com/dylanaraps/neofetch">neofetch</a>, but also wants to show more general information about the system. 
    Fetchy also tries to be as cross-platform compatible as possible.
    It is also written in Rust to make information gathering as fast and efficient as possible.
    </p>
  </div>
  
  <br clear="both">
</dl>

> [!NOTE]
> The version you are seeing right now is still a very early version. There are still a bunch of features and system information missing.

### Roadmap
---
##### These are some features I'm currently planning on implementing

- [ ] Make colors customizable
- [ ] Display GPU data
- [ ] Display storage data
- [ ] Make binary available using `cargo install`
- [ ] Pre-compile binaries for different platforms

### Installation
---
##### Currently there are no pre-compiled binaries available. To use this tool you have to compile it yourself using cargo.
Here are the steps needed to install this tool:

1. Create a new directory for the source code.
2. Open a terminal in this directory.
3. Run `git clone https://github.com/GHaxZ/fetchy/`.
4. Then run `cargo build`.
5. After you have done this there will be a compiled binary in the `/target/debug` folder.
6. Put this binary into a directory that has been added to your path.
7. Done! You can now run the `fetchy` command (you may have to restart your terminal).

### Contributing
---
To contribute to this project you can open issues to report bugs or make a pull request if you want to improve or add functionality.

