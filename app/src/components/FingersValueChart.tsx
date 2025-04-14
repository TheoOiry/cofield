import { useTheme } from "@mui/material/styles";
import { ChartsReferenceLine } from "@mui/x-charts/ChartsReferenceLine";
import { LineChart } from "@mui/x-charts/LineChart";
import { listen } from "@tauri-apps/api/event";
import React, { useEffect, useState } from "react";
import { Fingers } from "../providers/GloveData";
import { LineSeriesType } from "@mui/x-charts/models";
import { DatasetType } from "@mui/x-charts/internals";

interface GloveNotificationPayload {
  dt: string;
  flexValues: Fingers<number>;
}

interface GloveSeriesData {
  dt: Date;

  finger_1: number;
  finger_2: number;
  finger_3: number;
  finger_4: number;
  finger_5: number;
}

const NOTIFICATION_BUFFER_SIZE = 10;
const MAX_NOTIFICATION_SAVED = 200;

const LINE_SERIES_CONFIG = {
  type: "line",
  curve: "linear",
  showMark: false,
} as const;

const SERIES: LineSeriesType[] = [
  { dataKey: "finger_1", label: "Finger 1", ...LINE_SERIES_CONFIG },
  { dataKey: "finger_2", label: "Finger 2", ...LINE_SERIES_CONFIG },
  { dataKey: "finger_3", label: "Finger 3", ...LINE_SERIES_CONFIG },
  { dataKey: "finger_4", label: "Finger 4", ...LINE_SERIES_CONFIG },
  { dataKey: "finger_5", label: "Finger 5", ...LINE_SERIES_CONFIG },
];

export interface FingersValueChartProps {}

const FingersValueChart: React.FC<FingersValueChartProps> = ({}) => {
  const { palette } = useTheme();

  const [dataset, setDataset] = useState<DatasetType>([]);

  useEffect(() => {
    let notificationsBuffer: GloveSeriesData[] = [];
    const unlisten = listen<GloveNotificationPayload>(
      "glove_notification",
      ({ payload }) => {
        const newData: GloveSeriesData = {
          dt: new Date(payload.dt),

          finger_1: payload.flexValues[0],
          finger_2: payload.flexValues[1],
          finger_3: payload.flexValues[2],
          finger_4: payload.flexValues[3],
          finger_5: payload.flexValues[4],
        };

        notificationsBuffer.push(newData);

        if (notificationsBuffer.length < NOTIFICATION_BUFFER_SIZE) {
          return;
        }

        // because `setDataset` is called asynchronously at render time, we need to clean the buffer
        // at this point, in case of a new notification appears before the render
        const newRows = notificationsBuffer;
        notificationsBuffer = [];

        setDataset((prevDataset) => {
          const newDataset = [...prevDataset, ...newRows];

          newDataset.splice(0, newDataset.length - MAX_NOTIFICATION_SAVED);

          return newDataset as DatasetType;
        });
      }
    );

    return () => {
      unlisten.then((unlisten) => unlisten());
    };
  }, []);

  return (
    <LineChart
      width={1000}
      skipAnimation
      height={500}
      dataset={dataset}
      series={SERIES}
      xAxis={[{ scaleType: "time", dataKey: "dt" }]}
      yAxis={[{ min: 0, max: 1000 }]}
    >
      <ChartsReferenceLine
        y={200}
        label="Moved sensibility"
        lineStyle={{ stroke: palette.success.main }}
      />
    </LineChart>
  );
};

export default FingersValueChart;
