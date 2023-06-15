from model import KlineEntry
from pyecharts import options as opts
from pyecharts.charts import Bar, Grid, Kline
from pyecharts.commons.utils import JsCode
from service import Converter
from stockstats import wrap


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

        macd_data = [1723, -1650, 2319, -1568]

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
                datazoom_opts=[opts.DataZoomOpts(range_start=0, range_end=100)],
                tooltip_opts=opts.TooltipOpts(trigger="axis", axis_pointer_type="cross"),
                toolbox_opts=opts.ToolboxOpts(),
                title_opts=opts.TitleOpts(title=f"{symbol} - {interval}"),
            )
        )
        macd = (
            Bar()
            .add_xaxis(xaxis_data=time)
            .add_yaxis(
                series_name="MACD",
                y_axis=macd_data,
                xaxis_index=2,
                yaxis_index=2,
                label_opts=opts.LabelOpts(is_show=False),
                itemstyle_opts=opts.ItemStyleOpts(
                    color=JsCode(
                        """
                            function(params) {
                                var colorList;
                                if (params.data >= 0) {
                                  colorList = '#46a95e';
                                } else {
                                  colorList = '#d95050';
                                }
                                return colorList;
                            }
                            """
                    )
                ),
            )
            .set_global_opts(
                xaxis_opts=opts.AxisOpts(
                    type_="category",
                    grid_index=2,
                    axislabel_opts=opts.LabelOpts(is_show=False),
                ),
                yaxis_opts=opts.AxisOpts(
                    grid_index=2,
                    split_number=4,
                    axisline_opts=opts.AxisLineOpts(is_on_zero=False),
                    axistick_opts=opts.AxisTickOpts(is_show=False),
                    splitline_opts=opts.SplitLineOpts(is_show=False),
                    axislabel_opts=opts.LabelOpts(is_show=True),
                ),
                legend_opts=opts.LegendOpts(is_show=False),
                datazoom_opts=[opts.DataZoomOpts(range_start=0, range_end=100)],
            )
        )

        grid_chart = Grid(
            init_opts=opts.InitOpts(
                width="2800px",
                height="1400px",
                animation_opts=opts.AnimationOpts(animation=False),
            )
        )
        grid_chart.add(
            kline,
            grid_opts=opts.GridOpts(height="70%", is_contain_label=True),
            is_control_axis_index=True,
        )
        grid_chart.add(
            macd,
            grid_opts=opts.GridOpts(pos_top="82%", height="14%", is_contain_label=True),
        )
        return grid_chart
