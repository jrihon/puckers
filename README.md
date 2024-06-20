# Pucke.rs

A program to generate dihedral values in order to perform conformational sampling on : 
 - peptide-like molecules (or any set of two torsion angles),
 - five-membered furanose rings,
 - six-membered pyranose rings.


### Usage

```
Pucke.rs help menu :
            --peptide  NUM : to generate torsion angles for peptide-like systems
            --fivering NUM : to generate torsion angles for five-membered ring systems
            --sixring  NUM : to generate torsion angles for six-membered ring systems
            -h or --help   : to print this menu. "
```

### Torsion angles
The flags `--peptide`, `--fivering` and `--sixring` are to be used with an `NUM` (integer value).</br>
 - `--peptide` `NUM` indicates the amount of evenly spaced values, with `NUM^2` being the amount of conformations to be sampled for
 - `--fivering` `NUM` indicates the amount of evenly spaced values, with `NUM^2` being the amount of conformations to be sampled for
 - `--sixring` `NUM` indicates the (near) amount of conformations to be sampled for


### Example
```shell 
$ puckers --peptide 37
$ puckers --fivering 21 
$ puckers --sixring 630
```



### Installation
Install Rustlang on Linux [Download Rust](https://www.rust-lang.org/tools/install)
```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Update Rustlang
```shell
$ rustup update
```
Install pucke.rs
```shell
# Download the repository anywhere on your system
$ git clone https://github.com/jrihon/puckers.git

# Go to the directory where the repository is installed on your system
$ cd puckers

# Install using the Rust's package manager
$ cargo install --path . --release
```
## Author
Written and designed by [Jérôme Rihon](https://github.com/jrihon/jrihon)
