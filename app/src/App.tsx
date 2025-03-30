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
import { Toolbar } from "@mui/material";

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

  useState(() => {
    listen("glove_connected", () => {
      setGloveState(GloveState.Connected);
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
          <Toolbar>
            <Button
                loading={gloveState === GloveState.Connecting}
                loadingPosition="end"
                color={gloveState === GloveState.Connected ? "success" : "primary"}
                startIcon={connectButtonIcon}
                onClick={connectGlove}
                variant="outlined"
              >
                {gloveState === GloveState.Connected
                  ? "Glove connected"
                  : gloveState === GloveState.Connecting
                  ? "Connecting..."
                  : "Connect glove"}
              </Button>
          </Toolbar>
        {/* <Box
          sx={{
            display: "flex",
            flexDirection: "column",
            justifyContent: "space-between",
            alignItems: "center",
            padding: "10px",
          }}
        >
          <Button
            loading={gloveState === GloveState.Connecting}
            loadingPosition="end"
            color={gloveState === GloveState.Connected ? "success" : "primary"}
            startIcon={connectButtonIcon}
            onClick={connectGlove}
            variant="outlined"
          >
            {gloveState === GloveState.Connected
              ? "Glove connected"
              : gloveState === GloveState.Connecting
              ? "Connecting..."
              : "Connect glove"}
          </Button>
        </Box> */}
        <Hand fingerColor={palette.success.light} isRightHand />
        <TextRetribution />
      </Box>
    </main>
  );
}

export default App;
