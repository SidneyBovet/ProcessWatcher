# Process Watcher

This is a simple program that checks every N seconds if a process with a given set of arguments is running. If the program is found to have just turned on or off, it will then perform an HTTP get to a given server, with a given route.

## Building

To build the program, just run `cargo build --release` from the root of this repo.

Some fiddling like `rustup install sysinfo` may be required.

## Configuration

The config file lets you set up a number of things:

* What process should be watched for, and with what arguments
* Which server will be contacted, and at what routes (on and off)
* How often should the check be performed

```json
{
    "process": {
        "name": "name.exe",
        "required_arguments": [
            "--something",
            "--verbose"
        ]
    },
    "remote": {
        "ip": "102.168.0.1",
        "route_on": "/on",
        "route_off": "/off"
    },
    "sleep_time_sec": 1
}
```
