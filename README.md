# status_code

## Usage

```
extern crate status_code;
use self::status_code::statuses::*;

status(200) => 200
status(444) => panic!

code(200) => "OK"
code(418) => "I'm a teapot"

message("OK") => 200
message("I'm a teapot") => 418

retry(502) => true
retry(200) => flase

redirect(301) => true
redirect(200) => flase

retry(502) => true
retry(200) => flase

```