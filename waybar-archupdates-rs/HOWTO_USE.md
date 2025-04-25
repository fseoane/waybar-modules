Arch Linux updates (pacman and AUR)  module for Waybar
------------------------------------------------------

(https://github.com/fseoane/waybar-modules.git)

This small program will give you fast Arch Linux (BTW) updates .

How to use
----------

1.-install binary archupdates-rs wherever your user have access (I use to put them in a script folder inside .config/waybar/ folder)

2.-add to ~/.config/waybar/config.json

```
  "custom/updatepackages": {
    "exec": "$HOME/.config/waybar/scripts/updates/archupdates-rs",
    "return-type": "json",
    "hide-empty-text": true,
    "format": "<span font='12'>󰏖</span>  {}",
    "tooltip": true,
    "tooltip-format": "<span font='10'>{alt}</span>",
    "escape": true,
    "on-click": "pacman -Syu",                      // install pacman updates only
    "on-click-middle": "pacman -Syu; yay -Syu",     // install pacman and aur updates
    "on-click-right": "yay -Syu",                   // install aur updates only
    "interval": 300,
    "signal": 11
},
```

3.-add "custom/updatepackages" to one of modules-left, modules-center or modules-right

4.-set your style in .config/waybar/style.css

```
#custom-updatepackages{
color: your;
background-color: your;
margin: 0px 0px 0px 0px;
padding: 0px 0px 0px 0px;
font-size: 13px;
text-shadow: none;
}
```
