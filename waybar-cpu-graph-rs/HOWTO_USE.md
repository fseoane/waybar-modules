https://github.com/fseoane/waybar-modules
----------------------------------------------------------------
Why not just get cpu usage statistics in custom waybar module?

    This module will provide histogram like graph of cpu usage, that is updated on an specified interval and showing a history of usage metrics also specified in the command line


How to use

1.-install binary cpugraph-rs wherever your user have access (I use to put them in a script folder inside .config/waybar/ folder)
2.-add to ~/.config/waybar/config

    "custom/cpu_graph": {
        "format": "{}|",
        "return-type": "json",
        "exec": "$HOME/.config/waybar/scripts/cpugraph/cpugraph-rs --interval 2 --history 10",
        "tooltip": true,
        "escape":false,
        "on-click": "flatpak run net.nokyan.Resources",
        "tooltip-format": "<u>Processor</u>\r<span font='30' font-family='efe-graph'>{}</span>\nCPU Usage: {percentage}%\n{alt}%"
    },
    where cli options are
    --interval - interval to gather the usage metrics.
    --history  - number of usage metrics to show in the graph.

3.-add "custom/cpu_graph" to one of modules-left, modules-center or modules-right
4.-install efe-graph and efe-graph-bold fonts to see the resultiung graph and restart waybar
5.- set your style in .config/waybar/style.css

    #custom-cpu_graph{
        color: <your background color>;
        margin: 0px 0px 0px 0px;
        padding: 0px 0px 0px 0px;
        font-size: 18px;
        text-shadow: none;
    }