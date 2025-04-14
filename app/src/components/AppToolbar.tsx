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

export interface ToolbarProps {}

const AppToolbar: React.FC<ToolbarProps> = ({}) => {
  const [isDrawerOpen, setIsDrawerOpen] = useState(false);

  return (
    <>
      <AppBar color="transparent">
        <Toolbar>
          <BleConnectionButton />
          <IconButton onClick={() => setIsDrawerOpen(true)}>
            <SettingsIcon />
          </IconButton>
        </Toolbar>
      </AppBar>
      <Drawer
        anchor="right"
        open={isDrawerOpen}
        onClose={() => setIsDrawerOpen(false)}
      >
        <Box sx={{ padding: 3 }}>
          <AggregationSizeInput />
          <EnableKeyboardSimulationButton />
        </Box>
      </Drawer>
      <Toolbar />
    </>
  );
};

export default AppToolbar;
