DISK Space Usage graph module for Waybar
----------------------------------------

(https://github.com/fseoane/waybar-modules.git)

This module will provide histogram like graph of disk space usage, that is updated on an specified interval and showing a history of usage metrics also specified in the command line

How to use
----------

1.-install binary diskspace-graph-rs wherever your user have access (I use to put them in a script folder inside .config/waybar/ folder)

2.-add to ~/.config/waybar/config.json

```
"custom/diskspace_graph": {
"format": "<span font-family='efe-graph-bold' rise='-4444'>{}</span>",
"return-type": "json",
"exec": "$HOME/.config/waybar/scripts/diskgraph/diskspace-graph-rs --interval 300 --history 5",
"tooltip": true,
"escape":false,
"on-click": "baobab /",
"tooltip-format": "<u>Processor</u>\r{}\nCPU Usage: {percentage}%\n{alt}%"
},
```

* where cli options are
  * --interval - interval to gather the usage metrics.
  * --history  - number of  metrics to show in the graph.

3.-add "custom/diskspace_graph" to one of modules-left, modules-center or modules-right

4.-install efe-graph and efe-graph-bold fonts to see the resultiung graph and restart waybar

5.- set your style in .config/waybar/style.css

```
#custom-diskspace_graph{
color: your;
margin: 0px 0px 0px 0px;
padding: 0px 0px 0px 0px;
font-size: 18px;
text-shadow: none;
}
```
