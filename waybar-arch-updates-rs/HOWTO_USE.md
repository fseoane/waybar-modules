https://github.com/coffebar/waybar-module-pacman-updates
----------------------------------------------------------------
Why not just exec checkupdates in custom waybar module?

    This module will provide relevant local information constantly and periodically update data from the network in backgroud. Direct "checkupdates" will only give you one of two things: updating the information with a long delay or having the module constantly active on the network.
    This module has 2 states which gives you the ability to display different icons depending on status.
    Waybar expects JSON in an infinite loop from modules. So we have this.
    See updates list in tooltip.

This small program will give you fast updates with less network usage. After you have installed all the updates, the module will immediately go into the Updated state. You don't need to send signals to waybar to update this module state.


How to use

    install binary waybar-arch-updates-rs to your PATH
    add to ~/.config/waybar/config

"custom/updates": {
    "format": "{} {icon}",
    "return-type": "json",
    "format-icons": {
        "has-updates": "󱍷",
        "updated": "󰂪"
    },
    "exec-if": "which waybar-arc-updates-rs",
    "exec": "waybar-arch-updates-rs --interval-seconds 5 --network-interval-seconds 300"
}

    add "custom/updates" to one of modules-left, modules-center or modules-right
    install nerd font to see icons or change icons as you like and restart waybar

Options

--no-zero-output - don't print "0" if there are no updates available.

--interval-seconds - interval to run checkupdates without network usage.

--network-interval-seconds - interval to run checkupdates with network usage.

--tooltip-align-columns - format tooltip as a table using given monospaced font.


How to hide the module when there are no updates available
waybar config

"custom/updates": {
    "format": "{} {icon}",
    "return-type": "json",
    "format-icons": {
        "has-updates": "󱍷",
        "updated": ""
    },
    "exec-if": "which waybar-arch-updates-rs",
    "exec": "waybar-arc-updates-rs --no-zero-output"
}

style.css

#custom-updates {
	background-color: transparent;
}
