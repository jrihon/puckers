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
            

            --twopi : to convert torsion angles from [-180,180] (default) to [0,360]
            --rad : to convert torsion angles from degrees (default) to radians
            

            -h or --help to print this menu. 
```

#### Torsion angles
The flags `--peptide`, `--fivering` and `--sixring` are to be used with an `NUM` (integer value).</br>
 - `--peptide` `NUM` indicates the amount of evenly spaced values, with `NUM^2` being the amount of conformations to be sampled for
 - `--fivering` `NUM` indicates the amount of evenly spaced values, with `NUM^2` being the amount of conformations to be sampled for
 - `--sixring` `NUM` indicates the (near) amount of conformations to be sampled for
</br>
</br>
The flag `--twopi` is used to change the range of possible torsion angles from [-π, π] to [0, 2π].</br>
The flag `--rad` is used to convert all torsion angles from degrees to radians.


### Example
``` 
$ puckers --fivering 21 --rad
$ puckers --peptide 37 --twopi 
$ puckers --sixring 630 
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
