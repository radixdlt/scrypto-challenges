import { createContext } from "react";
import { Rdt } from "./types";

export const RdtContext = createContext<Rdt | null>(null);