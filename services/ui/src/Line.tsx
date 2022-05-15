import { createSignal, createEffect } from "solid-js";

const [metrics, setMetrics] = createSignal(null);

import * as Plot from "@observablehq/plot";
import axios from "axios";

const aapl = [
  {
    Date: "2013-05-13T00:00:00.000Z",
    Open: 64.501427,
    High: 65.414284,
    Low: 64.5,
    Close: 64.96286,
    "Adj Close": 50.961628,
    Volume: 79237200,
  },
  {
    Date: "2013-05-14T00:00:00.000Z",
    Open: 64.835716,
    High: 65.028572,
    Low: 63.164288,
    Close: 63.408573,
    "Adj Close": 49.742329,
    Volume: 111779500,
  },
  {
    Date: "2013-05-15T00:00:00.000Z",
    Open: 62.737144,
    High: 63,
    Low: 60.337143,
    Close: 61.264286,
    "Adj Close": 48.060188,
    Volume: 185403400,
  },
  {
    Date: "2013-05-16T00:00:00.000Z",
    Open: 60.462856,
    High: 62.549999,
    Low: 59.842857,
    Close: 62.082859,
    "Adj Close": 48.702328,
    Volume: 150801000,
  },
  {
    Date: "2013-05-17T00:00:00.000Z",
    Open: 62.721428,
    High: 62.869999,
    Low: 61.572857,
    Close: 61.894287,
    "Adj Close": 48.554409,
    Volume: 106976100,
  },
];

const BENCHER_API_URL: String = "http://localhost"; 

const options = {
  url: `${BENCHER_API_URL}/v0/metrics`,
  method: "get",
  headers: {
      "Content-Type": "application/json",
      // Only use with explicit CORS
      // Authorization: `Bearer ${window.localStorage.authToken}`
  }
};

async function fetchMetrics() {
  try {
    const response = await axios(options);
    console.log(response?.data);
    setMetrics(response?.data);
  } catch (error) {
    console.error(error);
  }
}

await fetchMetrics();


const intoData = () => {
  const metrics_array = metrics();
  var data_array = [];
  for (let i = 0; i < metrics_array.length; i++) {
    let metrics_data = metrics_array[i];
    let date_time = metrics_data.date_time;
    let latency = metrics_data?.metrics["tests::benchmark_a"]?.latency?.duration;
    let nanos = latency?.secs * 1_000_000_000 + latency?.nanos;
    data_array.push([date_time, nanos]);
  }
  return data_array;
}

const plot_data = intoData();

const plot = Plot.plot({
  y: {
    grid: true,
  },
  marks: [Plot.line(plot_data)],
});

export function LinePlot(prop) {
  return (
    <div>
      {plot}  
      {/* <em>{metrics}</em> */}
    </div>
  );
}