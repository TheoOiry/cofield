import Button from "@mui/material/Button";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

import BluetoothDisabledIcon from "@mui/icons-material/BluetoothDisabled";
import BluetoothConnectedIcon from "@mui/icons-material/BluetoothConnected";
import { useEffect, useState } from "react";

enum GloveState {
  Disconnected,
  Connecting,
  Connected,
}

const BleConnectionButton = () => {
  const [gloveState, setGloveState] = useState<GloveState>(
    GloveState.Disconnected
  );

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

  useEffect(() => {
    const unlistenConnected = listen("glove_connected", () => {
      setGloveState(GloveState.Connected);
    });

    const unlistenDisonnected = listen("glove_disconnected", () => {
      setGloveState(GloveState.Disconnected);
    });

    return () => {
      unlistenConnected.then((unlisten) => unlisten());
      unlistenDisonnected.then((unlisten) => unlisten());
    }
  }, []);

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
    <Button
      loading={gloveState === GloveState.Connecting}
      loadingPosition="end"
      color={gloveState === GloveState.Connected ? "success" : "primary"}
      startIcon={connectButtonIcon}
      onClick={
        gloveState === GloveState.Disconnected ? connectGlove : disconnectGlove
      }
      variant="outlined"
    >
      {gloveState === GloveState.Connected
        ? "Glove connected"
        : gloveState === GloveState.Connecting
        ? "Connecting..."
        : "Connect glove"}
    </Button>
  );
};
export default BleConnectionButton;
