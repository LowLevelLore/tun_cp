<p align="center">
  <a href="" rel="noopener">
 <img width=200px height=200px src="https://i.imgur.com/6wj0hh6.jpg" alt="Project logo"></a>
</p>

<h3 align="center">TUN-CP - A TCP stack using tun/tap on linux</h3>

<div align="center">

[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![GitHub Issues](https://img.shields.io/github/issues/kylelobo/The-Documentation-Compendium.svg)]()
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/kylelobo/The-Documentation-Compendium.svg)]()
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](/LICENSE)

</div>



## 🧐 About <a name = "about"></a>

This is a func project that mimics TCP stack on the tuntap0 interface which provides communicaton between program space instead of the physical media.

## 🏁 Getting Started <a name = "getting_started"></a>

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

Adding a tuntap interface on any linux system (this will act as our NIC).
```
sudo ip tuntap add mode tap tap0
```

Enabling the tap0 interface.
```
sudo ifconfig tap0 up
```

### Installing

Cloning this repo on your local machine

```
git clone <this_repo>
```

Moving into the cloned directory

```
cd tun_cp
```


## 🎈 Usage <a name="usage"></a>

To run this program make sure to execute the following command in the <b> tun_cp
</b> folder
```
./run.sh
```


## ⛏️ Built Using <a name = "built_using"></a>

- [Rust](https://www.rust-lang.org/)
- [tun-tap](https://docs.rs/tun-tap/latest/tun_tap/)
- [etherparse](https://docs.rs/etherparse/latest/etherparse/)
- [bitflags](https://docs.rs/bitflags/latest/bitflags/)
- [nix](https://crates.io/crates/nix/dependencies)

## ✍️ Authors <a name = "authors"></a>

- [@LowLevelLore](https://github.com/LowLevelLore) - All Idea & No Work

## 🎉 Acknowledgements <a name = "acknowledgement"></a>

- [IP](https://datatracker.ietf.org/doc/html/rfc791/)
- [Jon Gjengset](https://www.youtube.com/watch?v=bzja9fQWzdA)
