from model import KlineEntry
from pyecharts import options as opts
from pyecharts.charts import Grid, Kline


class Drawer:
    def draw_kline(klines: list[KlineEntry], symbol: str, interval: str) -> Grid:
        time = []
        data = []
        for i in klines:
            item = []
            item.append(i.open_price)
            item.append(i.close_price)
            item.append(i.low_price)
            item.append(i.high_price)

            data.append(item)
            time.append(i.open_time)

        kline = (
            Kline()
            .add_xaxis(time)
            .add_yaxis(
                "kline",
                data,
                itemstyle_opts=opts.ItemStyleOpts(
                    color="#46a95e",
                    border_color="#46a95e",
                    color0="#d95050",
                    border_color0="#d95050",
                ),
            )
            .set_global_opts(
                xaxis_opts=opts.AxisOpts(is_scale=True),
                yaxis_opts=opts.AxisOpts(
                    is_scale=True,
                    splitarea_opts=opts.SplitAreaOpts(
                        is_show=True, areastyle_opts=opts.AreaStyleOpts(opacity=1)
                    ),
                ),
                datazoom_opts=[opts.DataZoomOpts(range_start=70, range_end=100)],
                tooltip_opts=opts.TooltipOpts(trigger="axis", axis_pointer_type="cross"),
                toolbox_opts=opts.ToolboxOpts(),
                title_opts=opts.TitleOpts(title=f"{symbol} - {interval}"),
            )
        )
        grid_chart = Grid(
            init_opts=opts.InitOpts(
                width="1800px",
                height="800px",
                animation_opts=opts.AnimationOpts(animation=False),
            )
        )
        grid_chart.add(
            kline,
            grid_opts=opts.GridOpts(is_contain_label=True),
            is_control_axis_index=True,
        )
        return grid_chart
