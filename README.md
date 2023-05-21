endpoint
========

`endpoint` is command line program to make HTTP requests to API backend.
In a nutshell, instead of this:

    curl http://localhost:8000/api/foos -d @payload.json

you run this:

    endpoint post https://localhost:8000/api/foos @payload.json

or, with a little bit of configuration, you run this:

    endpoint post foos @payload.json
    

Configuration
-------------

The default configuration file is `.endpoint.toml`. The program tries to
read it in current directory. If the config file is not found, it tries
to read it from the parent directories, all the way up to `$HOME`. If no
config file is found, the program uses the default values.

The most common (and useful) configuration option is `base_url`.

    base_url = "http://localhost:8000/api/"

It allows to cut on the boilerplate and turns this:

    endpoint get http://localhost:8000/api/some-endpoint

into this:

    endpoint get some-endpoint


Syntax
------

    endpoint <options> HTTP_VERB API_ENDPOINT <PAYLOAD>

`endpoint` understands basic HTTP verbs: GET, HEAD, POST, PUT, DELETE, and PATCH.
They do not need to be capitalized on the command line.

`API_ENDPOINT` is full URL or contacenated with `base_url` configuration
option.

`PAYLOAD` is optional and can be literal or a filename if prefixed with
@.

Examples
--------

Make a GET request at `version` endpoint.

    endpoint get http://localhost:8000/api/version

Make a POST request at `items` endpoint, sending literal JSON string as
body.

    endpoint post http://localhost:8000/api/items '{"name": "New item"}'

Make a DELETE request at specific `items` endpoint.

    endpoint delete http://localhost:8000/api/items/1

Make a PUT request at specific `items` ednpoint, with body read from a
file.

    endpoint put http://localhost:8000/api/items/1 @item.json


Install
-------

The binaries are not provided, use cargo to build and install.

    cargo build

`cargo` can be installed by following [Install
Rust](https://www.rust-lang.org/tools/install) documentation.


License
-------

endpoint is MIT licensed.
