https://github.com/fseoane/waybar-modules.git
----------------------------------------------------------------
Why not just get network usage statistics in custom waybar module?

    This module will provide histogram like graph of network usage (differencing between up and down traffic) that is updated on an specified interval and showing a history of usage metrics also specified in the command line


How to use

1.-install binary netgraph-rs wherever your user have access (I use to put them in a script folder inside .config/waybar/ folder)
2.-add to ~/.config/waybar/config

        "custom/net_graph": {
            "format": "{}|",
            "return-type": "json",
            "exec": "$HOME/.config/waybar/scripts/netgraph/netgraph-rs --interval 2 --history 10 --interface eth0",
            "tooltip": true,
            "escape":false,
            "on-click": "/usr/bin/sniffnet",
            "tooltip-format": "<u>Network</u>\r<span font='40' font-family='efe-graph'>{}</span>\n{alt}"
        },

    where cli options are
    --interval - interval to gather the usage metrics.
    --history  - number of usage metrics to show in the graph.
    --interface - the interface name to get usage from.
                  This option is optional anf if not provided the program will gather the aggregate usage of all network interfaces present.

3.-add "custom/net_graph" to one of modules-left, modules-center or modules-right
4.-install efe-graph and efe-graph-bold fonts to see the resultiung graph and restart waybar
5.- set your style in .config/waybar/style.css

        #custom-net_graph{
            color: <your background color>;
            margin: 0px 0px 0px 0px;
            padding: 0px 0px 0px 0px;
            font-size: 18px;
            text-shadow: none;
        }