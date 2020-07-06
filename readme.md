hh.ru parser. Is uses api and concurrently parses all vacancies based on a query.

----------
Supported export formats :
* direct output
* json file
* txt file
----------

hhparser --help
```
USAGE:
    hhparser [FLAGS] [OPTIONS]

FLAGS:
    -e, --export-only    export_only - omit search
    -h, --help           Prints help information
    -V, --version        Prints version information

OPTIONS:
    -f, --filename <filename>    filename - Export filename, used in txt/json format [default: export.txt]
        --fmt <fmt>              fmt - Export format [print|txt|json] [default: print]
    -k, --keyword <keyword>      keyword - Search keyword [default: C++]

```



[![Build Status](https://travis-ci.org/ThatEmbeddedGuy/hhparser.svg?branch=master)](https://travis-ci.org/ThatEmbeddedGuy/hhparser)

[![license](https://img.shields.io/github/license/DAVFoundation/captain-n3m0.svg?style=flat-square)](https://github.com/DAVFoundation/captain-n3m0/blob/master/LICENSE)
