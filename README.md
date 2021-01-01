# wyag

This is a Rust implementation of [git](https://git-scm.com/) following the excellent guide of [Write yourself a Git](https://wyag.thb.lt).

Implemented are the following commands:

- [ ] add
- [x] cat-file
- [ ] checkout
- [ ] commit
- [ ] hash-object
- [x] init
- [ ] log
- [ ] ls-tree
- [ ] merge
- [ ] rebase
- [ ] rev-parse
- [ ] rm
- [ ] show-ref
- [ ] tag

## Usage

    wyag 0.1
    Jan-Christoph Klie
    Write your own git

    USAGE:
        wyag [SUBCOMMAND]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    SUBCOMMANDS:
        help    Prints this message or the help of the given subcommand(s)
        init    Initialize a new, empty repository.