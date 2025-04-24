MEMORY Usage graph module for Waybar
------------------------------------

(https://github.com/fseoane/waybar-modules.git)

Why not just get memory usage statistics in custom waybar module?

This module will provide histogram like graph of memory usage, that is updated on an specified interval and showing a history of usage metrics also specified in the command line

How to use
----------

1.-install binary memgraph-rs wherever your user have access (I use to put them in a script folder inside .config/waybar/ folder)

2.-add to ~/.config/waybar/config

```
  "custom/memory_graph": {
    "format": "<span font-family='efe-graph' rise='-4444'>{}</span>",
    "return-type": "json",
    "exec": "$HOME/.config/waybar/scripts/memgraph/memgraph-rs --interval 5 --history 8",
    "tooltip": true,
    "escape":false,
    "on-click": "flatpak run net.nokyan.Resources",
    "tooltip-format": "<u>Memory</u>\r<span font='30' font-family='efe-graph'>{}</span>\nMEM Usage: {percentage}%\n{alt}",
  },
```

* where cli options are
  * --interval - interval to gather the usage metrics.
  * --history  - number of usage metrics to show in the graph.

3.-add "custom/mem_graph" to one of modules-left, modules-center or modules-right

4.-install efe-graph and efe-graph-bold fonts to see the resultiung graph and restart waybar

5.- set your style in .config/waybar/style.css

```
#custom-mem_graph{
color: your;
margin: 0px 0px 0px 0px;
padding: 0px 0px 0px 0px;
font-size: 18px;
text-shadow: none;
}
```
