import * as Plot from "@observablehq/plot";
import * as d3 from "d3";
import { PerKind } from "../../../config/types";

// TODO query the backend based off of metric kind
const getLabel = (kind) => {
  switch (kind) {
    case PerKind.LATENCY:
      return "↑ Nanoseconds";
    case PerKind.THROUGHPUT:
      return "↑ Events per Nanoseconds";
    case PerKind.COMPUTE:
    case PerKind.MEMORY:
    case PerKind.STORAGE:
      return "↑ Average Performance";
    default:
      return "↑ UNITS";
  }
};

const LinePlot = (props) => {
  const plotted = () => {
    const json_perf = props.perf_data();
    if (
      typeof json_perf !== "object" ||
      json_perf === null ||
      !Array.isArray(json_perf.results)
    ) {
      return;
    }

    const plot_arrays = [];
    const colors = d3.schemeTableau10;
    json_perf.results.forEach((result, index) => {
      const perf_metrics = result.metrics;
      if (!(Array.isArray(perf_metrics) && props.perf_active[index])) {
        return;
      }

      const line_data = [];
      perf_metrics.forEach((perf_metric) => {
        const x_value = new Date(perf_metric.start_time);
        x_value.setSeconds(x_value.getSeconds() + perf_metric.iteration);
        const y_value = perf_metric.metric?.value;
        const xy = [x_value, y_value];
        line_data.push(xy);
      });

      const color = colors[index % 10];
      plot_arrays.push(Plot.line(line_data, { stroke: color }));
    });

    return Plot.plot({
      y: {
        grid: true,
        label: getLabel(json_perf.kind),
      },
      marks: plot_arrays,
      width: props.width(),
    });
  };

  return <>{plotted()}</>;
};

export default LinePlot;
