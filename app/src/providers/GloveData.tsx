import { listen } from "@tauri-apps/api/event";
import { createContext, ReactNode, useContext, useEffect, useState } from "react";

export type Fingers<T> = [T, T, T, T, T];

export interface FingersNotificationPayload {
  flexValues: Fingers<number>;
}

export interface GloveDataContextProps {
  fingersHighlighted: Fingers<boolean>;

  updateFingersSensibility: (fingersSensibility: Fingers<number>) => void;
  fingersSensibility: Fingers<number>;
}

const GloveDataContext = createContext<GloveDataContextProps | null>(null);

const GloveDataProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [fingersSensibility, setFingersSensibility] = useState<Fingers<number>>(
    [15, 15, 15, 15, 15]
  );

  const updateFingersSensibility = (fingersSensibility: Fingers<number>) => {
    setFingersSensibility(fingersSensibility);
  };

  const [fingersHighlighted, setFingersHighlited] = useState<Fingers<boolean>>([
    false,
    false,
    false,
    false,
    false,
  ]);

  useEffect(() => {
    const unlisten = listen<Fingers<boolean>>("moved_fingers", ({ payload }) => {
      setFingersHighlited(payload);
    });

    return () => {
      unlisten.then((unlisten) => unlisten());
    }
  }, []);

  return (
    <GloveDataContext.Provider
      value={{
        fingersSensibility,
        fingersHighlighted,
        updateFingersSensibility,
      }}
    >
      {children}
    </GloveDataContext.Provider>
  );
};

export { GloveDataContext, GloveDataProvider };

export const useGloveData = () => {
  const gloveData = useContext(GloveDataContext);

  if (!gloveData) {
    throw new Error("useGloveData must be used within a GloveDataProvider");
  }

  return gloveData;
};

export const useOnFingersChange = (
  onFingersChange: (fingers: Fingers<boolean>) => void
) => {
  const { fingersHighlighted } = useGloveData();

  useEffect(() => {
    onFingersChange(fingersHighlighted);
  }, [fingersHighlighted]);  
}