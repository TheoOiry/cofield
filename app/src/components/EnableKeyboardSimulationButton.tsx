import Button from "@mui/material/Button";
import PianoIcon from "@mui/icons-material/Piano";
import PianoIconOff from "@mui/icons-material/PianoOff";
import { useContext } from "react";
import { ProcessConfigContext } from "../providers/ProcessConfig";

const EnableKeyboardSimulationButton = () => {
  const { isKeyboardEmulationEnabled, toggleKeyboardEmulation } =
    useContext(ProcessConfigContext)!;

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
