TEMPERATURE Usage graph module for Waybar
-----------------------------------------

(https://github.com/fseoane/waybar-modules.git)

Why not just get temperature usage statistics in custom waybar module?

This module will provide histogram like graph of temperature usage, that is updated on an specified interval and showing a history of usage metrics also specified in the command line

How to use
----------

1.-install binary tempgraph-rs wherever your user have access (I use to put them in a script folder inside .config/waybar/ folder)

2.-add to ~/.config/waybar/config.json

```
"custom/temp_graph": {
   "format": "{}",
   "return-type": "json",
   "exec": "$HOME/.config/waybar/scripts/tempgraph/tempgraph-rs --interval 5 --history 7 --item max",
   "tooltip": true,
   "escape":false,
   "tooltip-format": "<u>Temperature (max)</u>\r{}\n{alt}",
},
```

* where cli options are
  * --interval - interval to gather the usage metrics.
  * --history  - number of usage metrics to show in the graph.
  * --item      - name of the temperature item/sensor to show in the graph. If not specified, it shows the average between all sensors found (if any)

3.-add "custom/temp_graph" to one of modules-left, modules-center or modules-right

4.-install efe-graph and efe-graph-bold fonts to see the resultiung graph and restart waybar

5.- set your style in .config/waybar/style.css

#custom-temp_graph{
color: <your background color>;
margin: 0px 0px 0px 0px;
padding: 0px 0px 0px 0px;
font-size: 18px;
text-shadow: none;
}
