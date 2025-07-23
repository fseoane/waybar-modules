Disk IO Usage graph module for Waybar
-------------------------------------

(https://github.com/fseoane/waybar-modules.git)

This module will provide histogram like graph of disk IO usage (differencing between reads and writes ) that is updated on an specified interval and showing a history of usage metrics also specified in the command line

How to use
----------

1.-install binary diskusage-graph-rs wherever your user have access (I use to put them in a script folder inside .config/waybar/ folder)

2.-add to ~/.config/waybar/config.json

```
  "custom/diskusage_graph": {
    "format": "<span font-family='efe-graph-bold' rise='-4444'>{}</span>",
    "return-type": "json",
    "exec": "$HOME/.config/waybar/scripts/diskgraph/diskusage-graph-rs --interval 2 --history 5",
    "tooltip": true,
    "escape":false,
    "on-click": "/usr/bin/baobab /",
    "tooltip-format": "<u>Disk Usage</u>\r<span font='30' font-family='efe-graph'>{}</span>\n{alt}"
},
```

* where cli options are
  * --interval - interval to gather the usage metrics.
  * --history  - number of usage metrics to show in the graph.

3.-add "custom/diskusage_graph" to one of modules-left, modules-center or modules-right

4.-install efe-graph and efe-graph-bold fonts to see the resultiung graph and restart waybar

5.- set your style in .config/waybar/style.css

```
#custom-diskusage_graph{
color: your;
margin: 0px 0px 0px 0px;
padding: 0px 0px 0px 0px;
font-size: 12px;
text-shadow: none;
}
```
