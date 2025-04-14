import { invoke } from "@tauri-apps/api/core";
import { createContext, ReactNode, useState } from "react";

export interface ProcessConfigContextProps {
  isKeyboardEmulationEnabled: boolean;
  toggleKeyboardEmulation: () => Promise<void>;

  aggregationSize: number;
  updateAggregationSize: (newSize: number) => Promise<void>;
}

const ProcessConfigContext = createContext<ProcessConfigContextProps | null>(
  null
);

const ProcessConfigProvider: React.FC<{ children: ReactNode }> = ({
  children,
}) => {
  const [isKeyboardEmulationEnabled, setIsKeyboardEmulationEnabled] =
    useState<boolean>(true);
  const [aggregationSize, setAggregationSize] = useState<number>(10);

  const toggleKeyboardEmulation = async () => {
    setIsKeyboardEmulationEnabled(!isKeyboardEmulationEnabled);
    await invoke("set_keyboard_emulation_config", {
      isEnabled: !isKeyboardEmulationEnabled,
    });
  };

  const updateAggregationSize = async (newSize: number) => {
    setAggregationSize(newSize);
    await invoke("set_aggregation_size", {
      aggregationSize: newSize,
    });
  };

  return (
    <ProcessConfigContext.Provider
      value={{
        isKeyboardEmulationEnabled,
        toggleKeyboardEmulation,
        aggregationSize,
        updateAggregationSize,
      }}
    >
      {children}
    </ProcessConfigContext.Provider>
  );
};

export { ProcessConfigContext, ProcessConfigProvider };
