import Paper from "@mui/material/Paper";
import { useTheme } from "@mui/material/styles";
import Typography from "@mui/material/Typography";
import React, { useContext } from "react";
import { TextWriterContext } from "../providers/TextWriter";
import Box from "@mui/material/Box";

export interface TextRetributionProps {}

const TextRetribution: React.FC<TextRetributionProps> = ({}) => {
  const { palette } = useTheme();
  const { text } = useContext(TextWriterContext)!;

  return (
    <Box sx={{ width: "100%", padding: "10px" }}>

      <Paper
        sx={{
          backgroundColor: palette.info.main,
          width: "100%",
          height: "50px",
          display: "flex",
          justifyContent: "start",
          alignItems: "center",
        }}
        elevation={5}
      >
        <Typography variant="h5" color={palette.info.contrastText}>{text}</Typography>
      </Paper>
    </Box>
  );
};

export default TextRetribution;
