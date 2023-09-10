<div align="center">
    <h1>ToolInst</h1>

    CLI written in Rust to handle installers and programs.
</div>

With **ToolInst** you can organize compressed files and installers into one single folder. You can manage this folder with just a few simple commands.

# How to Use
> Make sure that you've already installed ToolInst, [installation here](#installation).

First, you have to create a config file for **ToolInst**. This file should be in your home directory and should be named `.toolinstrc`.

In this file, you have to set `INSTALL` with the path to the folder you want to use to organize all your files.

Something like this:
```
INSTALL=path
```
> Replace `path` with an absolute path to the folder.

In this folder, there should be two folders for compressed files and installers. These are named `.compressed` and `.installer` respectively by default.

If you want to set different names or paths to these folders, you can use these configs:
```
COMPRESSED_DIR_NAME=name_or_path
INSTALLER_DIR_NAME=name_or_path
```
> Replace `name_or_path` with an absolute path to the folder or a name.

It's recommended that you create these folders before using ToolInst, commands may fail if these folders don't exist.

Finally, you can run `toolinst --help` in your terminal to see all commands you can use. Hope it'll useful!

# Installation
You can install ToolInst by downloading a release from the releases page.

You simply have to unzip the file and add the resulting folder to your path.

If you want, you can also build from source by cloning this repository and running this command in the cloned repository:
```sh
cargo build --release
```
This will build ToolInst in release mode. You can add it to your `~/.cargo/bin` by running:
```sh
cargo install --path .
```

That should work. You can try running `toolinst --version` in your terminal, if you get output then everything should be working fine.

# License
<a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/"><img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by-sa/4.0/88x31.png" /></a><br /><span xmlns:dct="http://purl.org/dc/terms/" property="dct:title">ToolInst</span> by <a xmlns:cc="http://creativecommons.org/ns#" href="https://youtube.com/@yisuscoding" property="cc:attributionName" rel="cc:attributionURL">YisusGaming</a> is licensed under a <a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/">Creative Commons Attribution-ShareAlike 4.0 International License</a>.

More details in [here](LICENSE).
