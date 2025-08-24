/// <reference types="vite/client" />

interface ImportMetaEnv {
    readonly VITE_CANISTER_ID: string;
    readonly VITE_HTTP_AGENT_HOST: string;
}

interface ImportMeta {
    readonly env: ImportMetaEnv;
}
