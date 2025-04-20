```
     _             __ 
    | |           / _|
  __| | __ _  ___| |_ 
 / _` |/ _` |/ _ \  _|
| (_| | (_| |  __/ |  
 \__,_|\__, |\___|_|  
        __/ |         
       |___/          
```

Damn good encoding fixer
or
Dead-simple goofy encoding fixer

<div>
  <img alt="Repo Size" src="https://img.shields.io/github/repo-size/michalszmidt/dgef" />
  <img alt="Lines of code" src="https://sloc.xyz/github/michalszmidt/dgef?category=code" />
  <img alt="Last Commit" src="https://img.shields.io/github/last-commit/michalszmidt/dgef" />
  <img alt="Assets Downloads" src="https://img.shields.io/github/downloads/michalszmidt/dgef/total" />
</div>

# Downloads
From release page

# Why
because enca/vim/recode/iconv/... all failed.

There are plenty of legacy encodings like windows-1250 which are commonly used for some polish subtitiles in old movies. All ligatures are broken, ending is crlf instead lf and some programs fail to read them...

# Usage

```bash
$ dgef -E --help
Convert between encodings

Usage: dgef {enoding|--encoding|-E} [OPTIONS] --input <input>

Options:
  -o, --out <out>            [path] Path to the out file
  -i, --input <input>        [path] Path to the in file
  -e, --encoding <encoding>  [utf-8/...] Desired encoding
  -l, --lf <lf>              [yes/no] Fix windows-style ending clrf to lf (unix-style line ending)
  -h, --help                 Print help
```

# Magic under the hood
- chardetect algorithm is: https://github.com/nickspring/charset-normalizer-rs which has no language detection but works as far
- parallel processing with thread_count=cpus()-1

# License
BSD-3-clause