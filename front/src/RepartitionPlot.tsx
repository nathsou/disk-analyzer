import { Pie } from '@nivo/pie';
import React, { FC, useMemo } from "react";
import { formatSize } from "./utils";

interface RepartitionPlotProps {
  items: Array<{ path: string, size: number }>,
  onClick?: (path: string) => void,
}

const colors = [
  "eae4e9", "fff1e6", "fde2e4", "fad2e1", "e2ece9",
  "bee1e6", "f0efeb", "dfe7fd", "cddafd"
].map(c => `#${c}`);

// const colors = ["006ba6", "0496ff", "ffbc42", "d81159", "8f2d56"].map(c => `#${c}`);

function last<T>(values: T[]): T {
  return values[values.length - 1];
}

export const RepartitionPlot: FC<RepartitionPlotProps> = ({ items, onClick }) => {
  const data = useMemo(() => items.map(({ path, size }) => ({
    id: path,
    label: last(path.split('/')),
    value: size,
  })), [items]);

  return (
    <Pie
      width={450}
      height={450}
      data={data}
      margin={{ top: 20, right: 20, bottom: 20, left: 20 }}
      cornerRadius={3}
      activeOuterRadiusOffset={8}
      borderWidth={1}
      colors={colors}
      arcLabel={'label'}
      arcLabelsSkipAngle={15}
      arcLabelsTextColor={{ from: 'color', modifiers: [['darker', 2]] }}
      valueFormat={formatSize}
      enableArcLinkLabels={false}
      onClick={({ id: path }) => {
        if (onClick) {
          onClick(path.toString());
        }
      }}
    />
  );
};