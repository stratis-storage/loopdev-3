# loopdev

Setup and control loop devices.

Provides rust interface with similar functionality to the Linux utility `losetup`.

## [Documentation](https://docs.rs/loopdev)

## Examples

```rust
use loopdev::LoopControl;
let lc = LoopControl::open().unwrap();
let ld = lc.next_free().unwrap();

println!("{}", ld.path().unwrap().display());

ld.attach_file("disk.img").unwrap();
// ...
ld.detach().unwrap();
```

## Development

### Running The Tests Locally

Unfortunately the tests require root only syscalls and thus must be run as root.
There is little point in mocking out these syscalls as I want to test they
actually function as expected and if they were to be mocked out then the tests
would not really be testing anything useful.
