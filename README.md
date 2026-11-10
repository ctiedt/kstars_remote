# KStars Remote

Almost any part of KStars (and Ekos in particular) can be controlled via DBus.
This project implements Rust bindings to make the DBus interface more easily
accesible.

`kstars_dbus` contains those bindings (using the `dbus` crate).

`kstars_api_server` offers a gRPC-based interface to remotely control Ekos.
