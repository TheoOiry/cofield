import { createContext, ReactNode, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";

const BACKSPACE_CHARACTER = "\u0008";

export interface TextWriterContextProps {
  text: string;
  setLetterApplyDelay: (delay: number) => void;
}

const TextWriterContext = createContext<TextWriterContextProps | null>(null);

const TextWriterProvider: React.FC<{ children: ReactNode }> = ({
  children,
}) => {
  const [letterApplyDelay, setLetterApplyDelay] = useState<number>(1000);

  const fingerTemp = useRef<number | null>(null);
  const writeTimeout = useRef<number | null>(null);
  
  const [text, setText] = useState<string>("");
  
  useState(() => {
    listen<string>("new_character", ({ payload }) => {
      if (payload === BACKSPACE_CHARACTER) {
        setText((prevText) => prevText.slice(0, -1));
        return;
      }
      
      setText((prevText) => prevText + payload);
    })
  })

  return (
    <TextWriterContext.Provider value={{
      text,
      setLetterApplyDelay,
    }}>
      {children}
    </TextWriterContext.Provider>
  );
};

export { TextWriterContext, TextWriterProvider };
