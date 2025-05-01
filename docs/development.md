# Precommit

## Install
Install pre-commit as a hook to run checks on `git commit`.

```shell
pip install pre-commit
```

Requires `rustfmt` to run the hook.
```
rustup component add rustfmt
```


## Usage

Add the following file to the `root` of the project :

```yaml
# .pre-commit-config.yaml
repos:
-   repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v2.3.0
    hooks:
    -   id: check-yaml
    -   id: end-of-file-fixer
    -   id: trailing-whitespace
-   repo: https://github.com/doublify/pre-commit-rust
    rev: master
    hooks:
    -   id: cargo-check
        args:
```


Install `hooks` :
```shell
pre-commit install
```
