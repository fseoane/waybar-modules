This repo contains some modules for waybar I built with Rust.

These modules are NOT based in CFFI but based on returning a json in the format:

`{"text": "$text", "alt": "$alt", "tooltip": "$tooltip", "class": "$class", "percentage": $percentage }`

Those modules are:

* cpu usage graph

  ![1744548771610](images/README/1744548771610.png)
* memory usage graph

  ![1744551508486](images/README/1744551508486.png)
* network usage graph

  ![1744548996563](images/README/1744548996563.png)![1744549237965](images/README/1744549237965.png)
* updates (for Arch linux with both pacman and aur updates)

  ![1744548548826](images/README/1744548548826.png)

You may find a HOWTO_USE.md file on each module folder with more detailed instructions on how to use and integraate them on waybar.

***NOTE**: these modules are tested in waybar 0.12.0 and hyprland 0.48.1*
