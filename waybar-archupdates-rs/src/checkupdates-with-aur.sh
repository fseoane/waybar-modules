#!/bin/sh
checkupdates $1 $2 &
pacman -Qm | aur vercmp &
wait
