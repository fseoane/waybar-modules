https://github.com/fseoane/waybar-modules.git
----------------------------------------------------------------
Why not just exec checkupdates in custom waybar module?

    This module will provide relevant local information constantly and periodically update data from the network in backgroud. Direct "checkupdates" will only give you one of two things: updating the information with a long delay or having the module constantly active on the network.
    This module has 2 states which gives you the ability to display different icons depending on status.
    Waybar expects JSON in an infinite loop from modules. So we have this.
    See updates list in tooltip.

This small program will give you fast updates with less network usage. After you have installed all the updates, the module will immediately go into the Updated state. You don't need to send signals to waybar to update this module state.


How to use

1.-install binary archupdates-rs wherever your user have access (I use to put them in a script folder inside .config/waybar/ folder)
2.-add to ~/.config/waybar/config

        "custom/updatepackages": {
            "exec": "$HOME/.config/waybar/scripts/updates/archupdates-rs --interval 900",
            "return-type": "json",
            "hide-empty-text": true,
            "format": "<span font='12'>󰏖</span>  {}",
            "tooltip": true,
            "tooltip-format": "<span font='10'>{alt}</span>",
            "escape": true,
            "exec-on-event": true,
            "on-click": "pacman -Syu",                      // install pacman updates only
            "on-click-middle": "pacman -Syu; yay -Syu",     // install pacman and aur updates
            "on-click-right": "yay -Syu",                   // install aur updates only
        },


    where cli options are
    --interval - interval to gather the updates (defaults to 900 seconds or 15 minutes).

3.-add "custom/updatepackages" to one of modules-left, modules-center or modules-right
4.-set your style in .config/waybar/style.css

        #custom-updatepackages{
            color: <your foreground color>;
            background-color: <your background color>;
            margin: 0px 0px 0px 0px;
            padding: 0px 0px 0px 0px;
            font-size: 13px;
            text-shadow: none;
        }