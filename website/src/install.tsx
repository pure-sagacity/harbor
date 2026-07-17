/**
 * Install page entry point.
 */

import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { InstallApp } from "./InstallApp";

const elem = document.getElementById("root")!;
const app = (
  <StrictMode>
    <InstallApp />
  </StrictMode>
);

(import.meta.hot.data.root ??= createRoot(elem)).render(app);
