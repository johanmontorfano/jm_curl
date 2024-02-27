# jm_curl

This Rust app is a reproduction of the cUrl utility, it may not feature all the functionalities of the original cUrl but
it will try to.

This is a side-project, so don't mind slow updates.

## Usage
```
jm_curl [URI] [-f [file_name]] [--no-headers] [-P] [-H] [-D]
```

The `-f` parameter is used to output the received content into a file. Warning: headers will be written onto the file
too, if you want to only load the raw body of the URI, please use this parameter with `--no-headers`.

The `-P`, `-H` and `-D` parameters are used to override the default request method (GET) and replace it respectively by
`Post`, `Head` or  `Delete`.

The `--no-headers` parameter is used to remove headers from the command's output.

## Roadmap
- [X] Basic data retrieving
- [X] Support for every request method
  - [X] GET
  - [X] POST
  - [ ] OPTIONS
  - [X] DELETE
  - [X] HEAD
  - [ ] CONNECT
  - [ ] PUT
  - [ ] TRACE
  - [ ] PATCH
- [X] Support for file outputting.
- [X] Support for raw file downloading.
- [ ] Support for forms filling from CLI args.
- [ ] Support for headers modification from CLI args.