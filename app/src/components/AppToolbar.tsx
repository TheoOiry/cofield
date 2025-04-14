import AppBar from "@mui/material/AppBar";
import React from "react";
import AggregationSizeInput from "./AggregationSizeInput";
import SettingsIcon from "@mui/icons-material/Settings";
import { useState } from "react";
import IconButton from "@mui/material/IconButton";
import Drawer from "@mui/material/Drawer";
import Toolbar from "@mui/material/Toolbar";
import Box from "@mui/material/Box";
import BleConnectionButton from "./BleConnectionButton";
import EnableKeyboardSimulationButton from "./EnableKeyboardSimulationButton";
import { Typography } from "@mui/material";

export interface ToolbarProps {}

const AppToolbar: React.FC<ToolbarProps> = ({}) => {
  const [isDrawerOpen, setIsDrawerOpen] = useState(false);

  return (
    <>
      <AppBar color="transparent">
        <Toolbar>
          <BleConnectionButton />

          <Box sx={{ flexGrow: 1 }} />

          <IconButton onClick={() => setIsDrawerOpen(true)}>
            <SettingsIcon />
          </IconButton>
        </Toolbar>
      </AppBar>
      <Toolbar />

      <Drawer
        anchor="right"
        open={isDrawerOpen}
        onClose={() => setIsDrawerOpen(false)}
      >
        <Box
          sx={{
            display: "flex",
            padding: 3,
            alignItems: "center",
            flexDirection: "column",
            gap: 2,
          }}
        >
          <Typography variant="body1">Settings</Typography>

          <AggregationSizeInput />
          <EnableKeyboardSimulationButton />
        </Box>
      </Drawer>
    </>
  );
};

export default AppToolbar;
