import matplotlib.pyplot as plot
import numpy as np
plot.rcParams.update({
    "font.size": 18,
    "axes.facecolor": "whitesmoke",
    "font.family": "serif"
})

def print_plot(rsonpath, jsurfer, jsonski, exp_label, fileout="plot.png"):
    width = 0.6
    ratio = 1.8
    pos = np.array(range(len(exp_label)))
    fig, (ax0, ax1) = plot.subplots(1, 2, gridspec_kw={'width_ratios':[1, ratio]})
    ax0.grid(color = 'white', linestyle = '-', linewidth = 3, zorder=1)
    bar = ax0.bar(exp_label, jsurfer, width=width, label="jsurfer", color="tab:gray", zorder=3)
    ax0.legend()
    ax0.set_ylabel("GB/s")
    #ax0.bar_label(bar, [f"{e:0.2f}" for e in jsurfer])

    width = width/ratio

    bar = ax1.bar(pos+width/2+0.03, rsonpath, label="simdpath", width=width, color="tab:blue", zorder=4)
    ax1.set_xticks(pos)
    ax1.set_xticklabels(exp_label)
    ax1.bar_label(bar, [f"{e:0.0f}" for e in rsonpath/jsurfer])
    pos2, jsonski2 = zip(*filter(lambda e:e[1] > 0, zip(pos, jsonski)))
    jsonski2 = np.array(jsonski2)
    pos2 = np.array(pos2)

    bar = ax1.bar(pos2-width/2-0.03, jsonski2, label="jsonski", width=width, color="tab:red", zorder=4)
    ax1.bar_label(bar, [f"{e:0.0f}" for e in filter(bool, jsonski/jsurfer)], zorder=4)
    ax1.set_ylabel("GB/s")
    ax1.grid(color = 'white', linestyle = '-', linewidth = 3, zorder=1)
    ax1.legend()
    fig.tight_layout()
    fig.set_size_inches(20, 5)
    plot.subplots_adjust(wspace=0.2, left=0.06)
    plot.savefig("plot.png")
    
