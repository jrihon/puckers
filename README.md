# Under Active Development
# Torsions

A program to generate dihedral values in order to perform conformational sampling on : 
 - peptide-like molecules (or any set of two torsion angles),
 - five-membered furanose rings,
 - six-membered pyranose rings.

### Usage

```
Torsions help menu :
            --peptide NUM : to generate torsion angles for peptide-like systems
            --fivering NUM : to generate torsion angles for five-membered ring systems
            --sixring NUM : to generate torsion angles for six-membered ring systems
            

            --twopi : to convert torsion angles from [-180,180] (default) to [0,360]
            --rad : to convert torsion angles from degrees (default) to radians
            

            -h or --help to print this menu. 
```

#### Torsion angles
The flags `--peptide`, `--fivering` and `--sixring` are to be used with an `NUM` (integer value), indicated a range with `NUM` amount of evenly spaced values.</br>
Exactly how the `linspace()` function is used in all math libraries.
</br>
</br>
The flag `--twopi` is used to change the range of possible torsion angles from [-π, π] to [0, 2π].</br>
The flag `--rad` is used to convert all torsion angles from degrees to radians.


### Example
``` 
$ torsions --fivering 21 --rad
$ torsions --peptide 37 --twopi 
$ torsions --sixring 630 
```



### Installation
```shell
$ cd path/to/torsions
$ cargo build --release

# There are two ways to do it.

# Method 1 : To put it directly in the path (recommended)
$ sudo cp path/to/torsions/target/release/torsions /usr/bin/torsions 

# Method 2 : Add PATH to ~/.bashrc
$ export PATH="$PATH:path/to/torsions/target/release/torsions"
```
