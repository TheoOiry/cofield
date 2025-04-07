import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { listen } from "@tauri-apps/api/event";
import Button from "@mui/material/Button";
import BluetoothDisabledIcon from "@mui/icons-material/BluetoothDisabled";
import BluetoothConnectedIcon from "@mui/icons-material/BluetoothConnected";
import { useState } from "react";
import Box from "@mui/material/Box";
import Hand from "./components/Hand";
import TextRetribution from "./components/TextRetribution";
import { useTheme } from "@mui/material/styles";
import { AppBar, Toolbar } from "@mui/material";
import AggregationSizeInput from "./components/AggregationSizeInput";

enum GloveState {
  Disconnected,
  Connecting,
  Connected,
}

function App() {
  const [gloveState, setGloveState] = useState<GloveState>(
    GloveState.Disconnected
  );

  const { palette } = useTheme();

  const connectGlove = async () => {
    if (gloveState !== GloveState.Disconnected) {
      return;
    }

    setGloveState(GloveState.Connecting);
    await invoke("start_listening_glove");
  };

  const disconnectGlove = async () => {
    if (gloveState !== GloveState.Connected) {
      return;
    }

    await invoke("stop_listening_glove");
  };

  useState(() => {
    listen("glove_connected", () => {
      setGloveState(GloveState.Connected);
    });

    listen("glove_disconnected", () => {
      setGloveState(GloveState.Disconnected);
    });
  });

  let connectButtonIcon = null;
  switch (gloveState) {
    case GloveState.Disconnected:
    case GloveState.Connecting:
      connectButtonIcon = <BluetoothDisabledIcon />;
      break;
    case GloveState.Connected:
      connectButtonIcon = <BluetoothConnectedIcon />;
      break;
  }

  return (
    <main className="container">
      <Box
        sx={{
          height: "100%",
          width: "auto",
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          flexDirection: "column",
          padding: "10px",
        }}
      >
        <AppBar color="transparent">
          <Toolbar>
              <AggregationSizeInput />
              <Button
                loading={gloveState === GloveState.Connecting}
                loadingPosition="end"
                color={gloveState === GloveState.Connected ? "success" : "primary"}
                startIcon={connectButtonIcon}
                onClick={gloveState === GloveState.Disconnected ? connectGlove : disconnectGlove}
                variant="outlined"
              >
                {gloveState === GloveState.Connected
                  ? "Glove connected"
                  : gloveState === GloveState.Connecting
                  ? "Connecting..."
                  : "Connect glove"}
              </Button>
          </Toolbar>
        </AppBar>
        <Toolbar />
        <Hand fingerColor={palette.success.light} isRightHand />
        <TextRetribution />
      </Box>
    </main>
  );
}

export default App;
