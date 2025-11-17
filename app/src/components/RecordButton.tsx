import React, { useContext } from "react";
import { ProcessConfigContext } from "../providers/ProcessConfig";
import { Button } from "@mui/material";
import StopIcon from "@mui/icons-material/Stop";
import RadioButtonCheckedIcon from "@mui/icons-material/RadioButtonChecked";

const RecordButton: React.FC = () => {
  const { isRecording, toggleRecording } = useContext(ProcessConfigContext)!;

  return (
    <Button
      color={isRecording ? "error" : "primary"}
      startIcon={isRecording ? <StopIcon /> : <RadioButtonCheckedIcon />}
      onClick={toggleRecording}
      variant="outlined"
    >
      {isRecording ? "Stop recording" : "Record"}
    </Button>
  );
};

export default RecordButton;
