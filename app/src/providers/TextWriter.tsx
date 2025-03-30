import { createContext, ReactNode, useRef, useState } from "react";
import { useOnFingersChange } from "./GloveData";

const NUMBER_MODE_IDENTIFIER = "<number_mode>";
const BACKSPACE_IDENTIFIER = "<backspace>";

const FINGERS_LETTERS = [
  "A",
  "B",
  "C",
  "D",
  "E",
  "F",
  "G",
  "H",
  "I",
  "J",
  "K",
  "L",
  "M",
  "N",
  "O",
  "P",
  "Q",
  "R",
  "S",
  "T",
  "U",
  "V",
  "W",
  "X",
  "Y",
  "Z",
  NUMBER_MODE_IDENTIFIER,
  BACKSPACE_IDENTIFIER,
  ".",
  " ",
];

enum WritingModes {
  Letters,
  Numbers,
}

export interface TextWriterContextProps {
  text: string;
  writingMode: WritingModes;
  setLetterApplyDelay: (delay: number) => void;
}

const TextWriterContext = createContext<TextWriterContextProps | null>(null);

const TextWriterProvider: React.FC<{ children: ReactNode }> = ({
  children,
}) => {
  const [letterApplyDelay, setLetterApplyDelay] = useState<number>(1000);
  const [writingMode, setWritingMode] = useState<WritingModes>(WritingModes.Letters);

  const fingerTemp = useRef<number | null>(null);
  const writeTimeout = useRef<number | null>(null);
  
  const [text, setText] = useState<string>("");
  
  const writeValue = (value: number) => {
    fingerTemp.current = null;
    if (writeTimeout.current) {
      clearTimeout(writeTimeout.current!);
      writeTimeout.current = null;
    }

    if (writingMode === WritingModes.Numbers) {
      setWritingMode(WritingModes.Letters);
      
      if (value === 30) {
        setText((text) => text + "0");
      } else {
        setText((text) => text + value.toString());
      }

      return;
    }

    const letter = FINGERS_LETTERS[value - 1];

    if (letter === NUMBER_MODE_IDENTIFIER) {
      setWritingMode(WritingModes.Numbers); 
      return;
    }
    
    if (letter === BACKSPACE_IDENTIFIER) {
      setText((text) => text.slice(0, -1));
      return;
    }
    
    setText((text) => text + letter);      
  }

  useOnFingersChange((fingers) => {
    const currentFinger = fingers.findIndex((finger) => finger) + 1;
    if (currentFinger === 0) {
      return;
    }

    if (fingerTemp.current !== null) {
      const value = fingerTemp.current * 5 + currentFinger;
      writeValue(value);
      return;
    }

    fingerTemp.current = currentFinger;
    
    writeTimeout.current = setTimeout(() => {
      writeValue(currentFinger);
    }, letterApplyDelay);
  });

  return (
    <TextWriterContext.Provider value={{
      text,
      writingMode,
      setLetterApplyDelay,
    }}>
      {children}
    </TextWriterContext.Provider>
  );
};

export { TextWriterContext, TextWriterProvider };
