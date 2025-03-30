import { listen } from "@tauri-apps/api/event";
import { createContext, ReactNode, useContext, useEffect, useRef, useState } from "react";

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
  const fingersSensibilityRef = useRef(fingersSensibility);

  const updateFingersSensibility = (fingersSensibility: Fingers<number>) => {
    setFingersSensibility(fingersSensibility);
    fingersSensibilityRef.current = fingersSensibility;
  };

  const [fingersHighlighted, setFingersHighlited] = useState<Fingers<boolean>>([
    false,
    false,
    false,
    false,
    false,
  ]);
  const fingersHighlightedRef = useRef(fingersHighlighted);

  useState(() => {
    listen<FingersNotificationPayload>("glove_notification", (event) => {
      const fingersValues = event.payload.flexValues;

      const newFingersHighlighted = fingersValues.map(
        (value, index) => value > fingersSensibilityRef.current[index]
      );

      if (
        newFingersHighlighted.some(
          (highlighted, index) =>
            highlighted !== fingersHighlightedRef.current[index]
        )
      ) {
        fingersHighlightedRef.current =
          newFingersHighlighted as Fingers<boolean>;
        setFingersHighlited(newFingersHighlighted as Fingers<boolean>);
      }
    });
  });

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