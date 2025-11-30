import { invoke } from "@tauri-apps/api/core";
import { documentDir } from "@tauri-apps/api/path";
import { openPath } from "@tauri-apps/plugin-opener";
import { LazyStore } from "@tauri-apps/plugin-store";
import {
  createContext,
  Dispatch,
  ReactNode,
  SetStateAction,
  useEffect,
  useState,
} from "react";
import FolderOpenIcon from "@mui/icons-material/FolderOpen";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";
import Box from "@mui/material/Box";
import { toast } from "react-toastify";

const AGGREGATION_SIZE_STORE_KEY = "aggregation_size";
const RAW_OUTPUT_FOLDER_STORE_KEY = "raw_output_folder";

export interface ProcessConfigContextProps {
  isKeyboardEmulationEnabled: boolean;
  toggleKeyboardEmulation: () => Promise<void>;

  aggregationSize: number;
  updateAggregationSize: (newSize: number) => Promise<void>;

  toggleRecording: () => Promise<void>;
  isRecording: boolean;

  setRawOutputFolder: Dispatch<SetStateAction<string>>;
  rawOutputFolder: string;
}

const ProcessConfigContext = createContext<ProcessConfigContextProps | null>(
  null
);

const store = new LazyStore("proccess_settings.json");

const OpenFilePathToast: React.FC<{ folderPath: string }> = ({
  folderPath,
}) => {
  const handleOpenFolder = async () => {
    await openPath(folderPath);
  };

  return (
    <Box
      sx={{
        display: "flex",
        flexDirection: "row",
        gap: 1,
        marginRight: 2,
        width: "100%",
      }}
    >
      <Typography variant="subtitle1" color="white" fontWeight={600}>
        File registered
      </Typography>
      <Button
        variant="outlined"
        sx={{
          color: "white",
          borderColor: "white",
          ml: "auto",
        }}
        size="small"
        startIcon={<FolderOpenIcon />}
        onClick={handleOpenFolder}
      >
        Open
      </Button>
    </Box>
  );
};

const ProcessConfigProvider: React.FC<{ children: ReactNode }> = ({
  children,
}) => {
  const [isKeyboardEmulationEnabled, setIsKeyboardEmulationEnabled] =
    useState<boolean>(true);
  const [aggregationSize, setAggregationSize] = useState<number>(10);
  const [rawOutputFolder, setRawOutputFolder] = useState<string>("");
  const [isRecording, setIsRecording] = useState<boolean>(false);

  const toggleKeyboardEmulation = async () => {
    setIsKeyboardEmulationEnabled(!isKeyboardEmulationEnabled);
    await invoke("set_keyboard_emulation_config", {
      isEnabled: !isKeyboardEmulationEnabled,
    });
  };

  const toggleRecording = async () => {
    if (!rawOutputFolder) return;

    try {
      const res = await invoke<string | null>("set_output_raw_data", {
        folderPath: isRecording ? null : rawOutputFolder,
      });

      if (!res && isRecording) {
        toast.info(<OpenFilePathToast folderPath={rawOutputFolder} />, {
          position: "bottom-right",
          theme: "colored",
        });
      }

      if (!res && !isRecording) {
        toast.error(
          "You need to be connected to the device to start recording",
          {
            position: "bottom-right",
            theme: "colored",
          }
        );
      }

      setIsRecording(res !== null);
    } catch (err) {
      toast.error(`Failed to start recording: ${err}`);
    }
  };

  const updateAggregationSize = async (newSize: number) => {
    setAggregationSize(newSize);

    store.set(AGGREGATION_SIZE_STORE_KEY, newSize);
    store.save();

    await invoke("set_aggregation_size", {
      aggregationSize: newSize,
    });
  };

  useEffect(() => {
    const fillStates = async () => {
      const aggregationSize = await store.get<number>(
        AGGREGATION_SIZE_STORE_KEY
      );
      const rawOutputFolder =
        (await store.get<string>(RAW_OUTPUT_FOLDER_STORE_KEY)) ||
        (await documentDir());

      if (aggregationSize) setAggregationSize(aggregationSize);
      setRawOutputFolder(rawOutputFolder);
    };

    fillStates();
  }, []);

  return (
    <ProcessConfigContext.Provider
      value={{
        isKeyboardEmulationEnabled,
        toggleKeyboardEmulation,
        aggregationSize,
        updateAggregationSize,
        toggleRecording,
        isRecording,

        setRawOutputFolder,
        rawOutputFolder,
      }}
    >
      {children}
    </ProcessConfigContext.Provider>
  );
};

export { ProcessConfigContext, ProcessConfigProvider };
