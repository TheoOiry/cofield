import Box from "@mui/material/Box";
import Hand from "./components/Hand";
import TextRetribution from "./components/TextRetribution";
import { useTheme } from "@mui/material/styles";
import AppToolbar from "./components/AppToolbar";
import FingersValueChart from "./components/FingersValueChart";
import { ToastContainer } from "react-toastify";

function App() {
  
  const { palette } = useTheme();

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
        <ToastContainer />
        <AppToolbar />
        <Hand fingerColor={palette.success.light} isRightHand />
        <FingersValueChart />
        <TextRetribution />

      </Box>
    </main>
  );
}

export default App;
