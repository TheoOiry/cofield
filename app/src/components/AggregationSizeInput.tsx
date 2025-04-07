import Box from "@mui/material/Box";
import Grid from "@mui/material/Grid";
import Input from "@mui/material/Input";
import Slider from "@mui/material/Slider";
import Typography from "@mui/material/Typography";
import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

const AggregationSizeInput: React.FC = () => {
  const [value, setValue] = useState(10);

  const handleSliderChange = (_: Event, newValue: number) => {
    setValue(newValue);
  };

  const handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setValue(event.target.value === "" ? 0 : Number(event.target.value));
  };

  const handleBlur = () => {
		const adaptedValue = Math.min(Math.max(1, value), 500);
    if (adaptedValue !== value) {
      setValue(adaptedValue);
    }

		invoke("set_aggregation_size", {
			aggregation_size: adaptedValue,
		});
  };

  return (
    <Box sx={{ width: 250 }}>
      <Typography
        sx={{ textAlign: "left" }}
        variant="subtitle2"
        id="input-slider"
      >
        Aggregation Size
      </Typography>
      <Grid container spacing={2} sx={{ alignItems: "center" }}>
        <Grid size="grow">
          <Slider
            value={typeof value === "number" ? value : 0}
            onChange={handleSliderChange}
            aria-labelledby="input-slider"
						min={1}
            max={500}
          />
        </Grid>
        <Grid>
          <Input
            value={value}
            size="small"
            onChange={handleInputChange}
            onBlur={handleBlur}
            inputProps={{
              step: 10,
              min: 0,
              max: 500,
              type: "number",
              "aria-labelledby": "input-slider",
            }}
          />
        </Grid>
      </Grid>
    </Box>
  );
};

export default AggregationSizeInput;
