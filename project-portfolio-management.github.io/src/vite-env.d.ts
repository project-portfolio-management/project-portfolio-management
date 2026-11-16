/// <reference types="vite/client" />

interface ImportMetaEnv {
  /** Where the reader's choices are kept: `cookie`, or anything else for localStorage (the default). */
  readonly VITE_CHOICE_STORAGE?: string;
}
