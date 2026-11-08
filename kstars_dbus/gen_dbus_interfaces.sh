#!/bin/sh

set -euo pipefail

mkdir -p kstars_interfaces

dbus-send --print-reply=literal --dest=org.kde.kstars /KStars org.freedesktop.DBus.Introspectable.Introspect > kstars_interfaces/kstars.xml
dbus-send --print-reply=literal --dest=org.kde.kstars /KStars/Ekos org.freedesktop.DBus.Introspectable.Introspect > kstars_interfaces/ekos.xml

for mod in Align Capture Focus Guide Mount;do
  dbus-send --print-reply=literal --dest=org.kde.kstars /KStars/Ekos/$mod org.freedesktop.DBus.Introspectable.Introspect > kstars_interfaces/$mod.xml
done
