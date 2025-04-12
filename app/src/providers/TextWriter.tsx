import { createContext, ReactNode, useState } from "react";
import { listen } from "@tauri-apps/api/event";

const BACKSPACE_CHARACTER = "\u0008";

export interface TextWriterContextProps {
  text: string;
}

const TextWriterContext = createContext<TextWriterContextProps | null>(null);

const TextWriterProvider: React.FC<{ children: ReactNode }> = ({
  children,
}) => {
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
    }}>
      {children}
    </TextWriterContext.Provider>
  );
};

export { TextWriterContext, TextWriterProvider };
