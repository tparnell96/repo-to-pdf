# repo-to-pdf

-----

## Install

-----

This assumes you have cargo installed on the host system and git.

```

git clone https://github.com/tparnell96/repo-to-pdf.git

```

```	

cargo build

```

Proceed to place the binary in /target/debug/repo-to-pdf in a place of your choosing.

-----

## Usage

-----

```

repo-to-pdf <directory of repo> <extension type of source files> <output name for pdf file> <exclusion directories>

```

The default directory if none is given is .

extension type defaults to .py

output name defaults to output.pdf

exclusion directories are mainly so you can pass virtual env directories and cache directories to the program, to prevent those from being captured as well.

This program is very basic, and contains little to no error handling or nuance, use with caution if you're going to.
