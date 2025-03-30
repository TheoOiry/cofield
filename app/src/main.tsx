import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { GloveDataProvider } from "./providers/GloveData";
import { TextWriterProvider } from "./providers/TextWriter";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <GloveDataProvider>
      <TextWriterProvider>
        <App />
      </TextWriterProvider>
    </GloveDataProvider>
  </React.StrictMode>,
);
