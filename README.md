## Changelog

[Doc](https://docs.google.com/document/d/1Whdhejekuf2kosSxJcyl7eSTQ6tFqyiRZuqPIGk3i-4/edit?usp=sharing)

## Development Ressources

### Plugin

[Documentation for skyline-rs](https://ultimate-research.github.io/skyline-rs-template/doc/skyline/index.html)

[Documentation from skyline-rs-template](https://github.com/ultimate-research/skyline-rs-template)

[SSBU Dumped Scripts](https://github.com/WuBoytH/SSBU-Dumped-Scripts/tree/ef9fcebd7e66dc53fa6a6f034e38e8ceb469342a)

### Parameters, animation, and more...

[SSBU Dumped Motion List](https://github.com/WuBoytH/SSBU-Dumped-Motion-Lists/tree/main)

[Motion List Convertor](https://github.com/ultimate-research/motion_lib/releases)

## Development Setup

### Plugin 

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

#### Creating and Building a Plugin

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

##### Installing via FTP connection

The suggested workflow is installing and reading logs via FTP, using a sysmodule like [sys-ftpd-light](https://github.com/cathery/sys-ftpd-light/releases).

Find your Switch's IP and run `cargo skyline set-ip [Your Switch's IP]`.

When you'd like to test a plugin, you can try `cargo skyline run`, which installs the plugin automatically to the correct folder on your Switch, as well as Skyline and dependency plugins. The command will then hang while waiting for logs from the Switch upon booting Smash. You should be able to see any logs you printed in Rust code using `println!`!

`cargo skyline run` is shorthand for `cargo skyline install`, which just installs the plugin without waiting for logs + `cargo skyline listen`, which only waits for logs from your system.

To view installed plugins, you can run `cargo skyline list`.

##### Installing manually
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

### Edit Parameters, animations, and more...

#### Prerequisites

* [ArcExplorer](https://github.com/ScanMountGoat/ArcExplorer/releases) - This application is a file browser and extractor for Smash Ultimate
* [libarc_network.nro](https://github.com/jam1garner/arc-network/releases/tag/master) - This plugin is required for connecting ArcExplorer to your Switch. Install it like any other skyline plugin
* [prcEditor](https://github.com/benhall-7/paracobNET/releases/download/1.9/Release.zip) - This application is used to edit parameter files (.prc)
* [ParamLabels.csv](https://github.com/ultimate-research/param-labels/blob/master/ParamLabels.csv) - Put this file in the same directory as prcEditor

#### Extract files

In ArcExplorer, "Settings > Preferences" and change "Extract Location" to any folder you want.

Run Smash and keep it open. In ArcExplorer, click "File > Connect to ARC". Enter your Switch IP and click connect.

Right click on any file you want to extract, and select "Extract File", the file will be in your "Extract Location" folder.

Use the left arrow on your keyboard to navigate in parent folders

#### Some useful folders

* `fighter/common/param` - Every general fighters parameters in files such as `common.prc` and `fighter_param.prc`

If you want to edit `fighter_param.prc`, you can directly use [Smash Ultimate Tools](https://smashultimatetools.com/)

* `fighter/<fighter_name>/motion/body/<any>` - Contains fighter's attack animation, as well as important parameter files such as `motion_list.bin` for moves data and `update.prc` for animations

If you want to edit `motion_list.bin`, you can directly use the dumped motion list in Ressources

#### Edit a prc file

Download `ParamLabels.csv` and put it inside prcEditor folder

Run prcEditor

Open your .prc file, press the "Enter" key to view the content of a struct / array and modify values

Save button to export
