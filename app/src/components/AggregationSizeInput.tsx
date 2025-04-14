import React, { useContext, useState } from "react";
import { ProcessConfigContext } from "../providers/ProcessConfig";
import { TextField } from "@mui/material";

const AggregationSizeInput: React.FC = () => {
  const { aggregationSize, updateAggregationSize } = useContext(ProcessConfigContext)!;
  const [value, setValue] = useState<number>(aggregationSize);

  const handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setValue(event.target.value === "" ? 0 : Number(event.target.value));
  };

  const handleBlur = () => {
		const adaptedValue = Math.min(Math.max(1, value), 50000);
    if (adaptedValue !== value) {
      setValue(adaptedValue);
    }

    updateAggregationSize(adaptedValue);
  };

  return (
    <TextField
      value={value}
      size="small"
      onChange={handleInputChange}
      onBlur={handleBlur}
      variant="outlined"
      label="Aggregation size"
      type="number"
      fullWidth
      slotProps={{
        htmlInput: {
          step: 10,
          min: 1,
          max: 50000,
          type: "number",
          "aria-labelledby": "input-slider",
        }
      }}
    />
  );
};

export default AggregationSizeInput;
