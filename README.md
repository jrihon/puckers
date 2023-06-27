# Pucke.rs

A program to generate dihedral values in order to perform conformational sampling on : 
 - peptide-like molecules (or any set of two torsion angles),
 - five-membered furanose rings,
 - six-membered pyranose rings.

### Usage

```
Pucke.rs help menu :
            --peptide NUM : to generate torsion angles for peptide-like systems
            --fivering NUM : to generate torsion angles for five-membered ring systems
            --sixring NUM : to generate torsion angles for six-membered ring systems
            --rad : to convert torsion angles from degrees (default) to radians
            --help (or -h) to print this menu. 
```

#### Torsion angles
The flags `--peptide`, `--fivering` and `--sixring` are to be used with an `NUM` (integer value).</br>
 - `--peptide` `NUM` indicates the amount of evenly spaced values, with `NUM^2` being the amount of conformations to be sampled for
 - `--fivering` `NUM` indicates the amount of evenly spaced values, with `NUM^2` being the amount of conformations to be sampled for
 - `--sixring` `NUM` indicates the (near) amount of conformations to be sampled for
 - `--rad` is used to convert all torsion angles from degrees to radians. Only implemented for `--peptide` and `--sixring`


### Example
```shell 
$ puckers --peptide 37
$ puckers --fivering 21 
$ puckers --sixring 630 --rad
```



### Installation
```shell
# Download the repository anywhere on your system
$ git clone https://github.com/jrihon/puckers.git

# Go to the directory where the repository is installed on your system
$ cd path/to/puckers

# Install using the Rust's cargo tool
$ cargo install --path .
```
