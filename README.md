## Ressources

[Documentation for skyline-rs](https://ultimate-research.github.io/skyline-rs-template/doc/skyline/index.html)

[Documentation from skyline-rs-template](https://github.com/ultimate-research/skyline-rs-template)

[SSBU Dumped Scripts](https://github.com/WuBoytH/SSBU-Dumped-Scripts/tree/ef9fcebd7e66dc53fa6a6f034e38e8ceb469342a)

## Setup

#### Prerequisites

* [Rust](https://www.rust-lang.org/install.html) - make sure rustup, cargo, and rustc (preferrably nightly) are installed.
* [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

Install [cargo skyline](https://github.com/jam1garner/cargo-skyline).
```bash
# inside a folder where you will dev all of your plugins going forward
git clone <project>
cd <project>
cargo install cargo-skyline
```

* [Visual Studio Code](https://code.visualstudio.com)

Install extension named "rust-analyzer", click "Switch to pre-release version", click the down arrow next to "Uninstall", click "Install Another Version", search for version 0.4.1082, and select it.

## Creating and Building a Plugin

First, make sure skyline and arcropolis are installed on you switch, follow this [tutorial](https://gamebanana.com/tuts/12827#H2_6)

Then, you need to install these following extra plugins before building and installing your own plugin :
* [libnro_hook.nro](https://github.com/ultimate-research/nro-hook-plugin/releases)
* [libacmd_hook.nro](https://github.com/ultimate-research/acmd_hook/releases)
* [libsmashline_hook.nro](https://github.com/blu-dev/smashline_hook/releases)

Put those plugin files on your SD card at :
```
sd:/atmosphere/contents/01006A800016E000/romfs/skyline/plugins
```

To compile your plugin use the following command in the root of the project (beside the `Cargo.toml` file):

### Installing via FTP connection

The suggested workflow is installing and reading logs via FTP, using a sysmodule like [sys-ftpd-light](https://github.com/cathery/sys-ftpd-light/releases).

Find your Switch's IP and run `cargo skyline set-ip [Your Switch's IP]`.

When you'd like to test a plugin, you can try `cargo skyline run`, which installs the plugin automatically to the correct folder on your Switch, as well as Skyline and dependency plugins. The command will then hang while waiting for logs from the Switch upon booting Smash. You should be able to see any logs you printed in Rust code using `println!`!

`cargo skyline run` is shorthand for `cargo skyline install`, which just installs the plugin without waiting for logs + `cargo skyline listen`, which only waits for logs from your system.

To view installed plugins, you can run `cargo skyline list`.

### Installing manually
 ```sh
 cargo skyline build --release
  ```
 Your resulting plugin will be the `.nro` found in the folder
```
/target/aarch64-skyline-switch/release
```
To install (you must already have skyline installed on your switch), put the plugin on your SD at:
```
sd:/atmosphere/contents/01006A800016E000/romfs/skyline/plugins
```

`cargo skyline` can also automate some of this process via FTP. If you have an FTP client on your Switch, you can run:
```sh
cargo skyline set-ip [Switch IP]
# install to the correct plugin folder on the Switch and listen for logs
cargo skyline run 
```
