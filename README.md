# lights project

Using this as a means of getting familiar with rust lang and cross compiling for aarch64 on x86_64.

cargo clean - remove all target files ~ 6000 files and 4GB for x64 and arm targets
cargo build - from scratch for both targets 1m 45s
cargo run  --target=x86_64-unknown-linux-gnu for running on x86 dev laptop

To copy executable to raspi:
jonb@ryzen7040:~/rs/lights/target/aarch64-unknown-linux-gnu/debug$ scp lights jonb@raspi:/home/jonb/rsx/

