import React, { useContext } from "react";
import { ProcessConfigContext } from "../providers/ProcessConfig";
import { Box, Button, TextField } from "@mui/material";
import FolderOpenIcon from "@mui/icons-material/FolderOpen";
import { open } from "@tauri-apps/plugin-dialog";

const EditFolderPath: React.FC = () => {
  const { rawOutputFolder, setRawOutputFolder } =
    useContext(ProcessConfigContext)!;

  const chooseFolder = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Select a folder for recording",
      defaultPath: rawOutputFolder,
    });

		if (selected) setRawOutputFolder(selected);
  };

  return (
    <Box sx={{ display: "flex", width: "100%" }}>
      <TextField
        label="Record folder"
				fullWidth
        value={rawOutputFolder}
        onChange={(event: React.ChangeEvent<HTMLInputElement>) => {
          setRawOutputFolder(event.target.value);
        }}
      />

      <Button onClick={chooseFolder} variant="outlined"><FolderOpenIcon /></Button>
    </Box>
  );
};

export default EditFolderPath;
