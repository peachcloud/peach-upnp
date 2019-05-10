# peach-upnp

Use [Universal Plug and Play (UPnP)](https://en.wikipedia.org/wiki/Universal_Plug_and_Play) to expose your Peach device to the internet via your router.

`peach-upnp` is really a stupid simple binary that calls [`upnpc`](http://miniupnp.free.fr/). The only interesting part will be how we package this using [`debcargo`](https://salsa.debian.org/rust-team/debcargo/) into a Debian package for the PeachCloud ecosystem.

## Demo

On Debian Linux

```shell
git clone git@github.com:peachcloud/peach-upnp
cd peach-upnp
sudo apt install miniupnpc
cargo run
```

## Building Debian package

Dependencies: [`debcargo`](https://salsa.debian.org/rust-team/debcargo/)

```
apt install libssl-dev libcurl4-gnutls-dev quilt
cargo build debcargo
```
