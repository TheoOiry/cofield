import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { GloveDataProvider } from "./providers/GloveData";
import { TextWriterProvider } from "./providers/TextWriter";
import { ProcessConfigProvider } from "./providers/ProcessConfig";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ProcessConfigProvider>
      <GloveDataProvider>
        <TextWriterProvider>
          <App />
        </TextWriterProvider>
      </GloveDataProvider>
    </ProcessConfigProvider>
  </React.StrictMode>
);
