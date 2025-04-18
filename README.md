# Pucke.rs

A program to generate dihedral values in order to perform conformational sampling on : 
 - peptide-like molecules (or any set of two torsion angles),
 - five-membered rings,
 - six-membered rings.


## Documentation

```
Pucke.rs help menu :
            --peptide  NUM : to generate torsion angles for peptide-like systems
            --fivering NUM : to generate torsion angles for five-membered ring systems
            --sixring  NUM : to generate torsion angles for six-membered ring systems
            -h or --help   : to print this menu.

# --peptide  NUM (samples `NUM^2` conformations).
# --fivering NUM (samples `NUM^2` conformations).
# --sixring  NUM (samples approx. `NUM` conformations).



# Example usage :
$ puckers --peptide 37
$ puckers --fivering 21 
$ puckers --sixring 630
```



### Installation
1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install pucke.rs
```shell
# Download the repository anywhere on your system
$ git clone https://github.com/jrihon/puckers.git

# Go to the directory where the repository is installed on your system
$ cd puckers

# Install using the Rust's package manager
$ cargo install --path . --release
```

## Citation 
If you've used pucke.rs or any of its parts, please cite the following published article : 

[1] Rihon, J., Reynders, S., Bernardes Pinheiro, V. et al. The pucke.rs toolkit to facilitate sampling the conformational space of biomolecular monomers. J Cheminform 17, 53 (2025). https://doi.org/10.1186/s13321-025-00977-7

```bibtex
@article{Rihon_2025,
    title={The pucke.rs toolkit to facilitate sampling the conformational space of biomolecular monomers},
    volume={17},
    ISSN={1758-2946},
    url={http://dx.doi.org/10.1186/s13321-025-00977-7},
    DOI={10.1186/s13321-025-00977-7},
    number={1},
    journal={Journal of Cheminformatics},
    publisher={Springer Science and Business Media LLC},
    author={Rihon, Jérôme and Reynders, Sten and Bernardes Pinheiro, Vitor and Lescrinier, Eveline},
    year={2025},
    month=apr
}
```

## Author
Written and designed by [Jérôme Rihon](https://github.com/jrihon/jrihon)
