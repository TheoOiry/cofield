import Button from "@mui/material/Button";
import { invoke } from "@tauri-apps/api/core";
import PianoIcon from '@mui/icons-material/Piano';
import PianoIconOff from '@mui/icons-material/PianoOff';
import { useState } from "react";

const EnableKeyboardSimulationButton = () => {
  const [isKeyboardEmulationEnabled, setIsKeyboardEmulationEnabled] = useState<boolean>(true);

  const toggleKeyboardEmulation = async () => {
    setIsKeyboardEmulationEnabled(!isKeyboardEmulationEnabled);
    await invoke("set_keyboard_emulation_config", {
        isEnabled: !isKeyboardEmulationEnabled,
    });
  };

  return (
    <Button
      color={isKeyboardEmulationEnabled ? "success" : "primary"}
      startIcon={isKeyboardEmulationEnabled ? <PianoIcon /> : <PianoIconOff />}
      onClick={toggleKeyboardEmulation}
      variant="outlined"
    >
      {isKeyboardEmulationEnabled ? "Disable emulation" : "Enable emulation"}
    </Button>
  );
};
export default EnableKeyboardSimulationButton;
