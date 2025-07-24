import { ActorSubclass } from "@dfinity/agent";
import { create } from "zustand";
import {
  _SERVICE,
  Message,
} from "../../../../../declarations/elescrow_backend/elescrow_backend.did";
import { Principal } from "@dfinity/principal";

interface ChatStoreState {
  actor: ActorSubclass<_SERVICE> | null;
  messageList: Array<Message>;
  messageInput: string;
  recipientPrincipalList: Array<Principal>;
  selectedRecipientUserKey: string | null;
  wsRef: WebSocket | null;
  reconnectTimeoutRef: number | null;
  pollingIntervalRef: number | null;
  // actions: ChatStoreActions;
}

interface ChatStoreActions {
  updateActor: (actor: ActorSubclass<_SERVICE> | null) => void;
  updateMessageList: (messageList: Array<Message>) => void;
  updateRecipientPrincipalList: (principalList: Array<Principal>) => void;
  selectRecipientUserKey: (userKey: string) => void;
  unselectRecipientUserKey: () => void;
  updateWsRef: (ws: WebSocket) => void;
  clearWsRef: () => void;
  updateReconnectTimeoutRef: (reconnectTimeout: number) => void;
  clearReconnectTimeoutRef: () => void;
  updatePollingIntervalRef: (pollingInterval: number) => void;
  clearPollingIntervalRef: () => void;
}

const useStore = create<ChatStoreState>()((set) => ({
  actor: null,
  messageList: new Array(),
  messageInput: "",
  recipientPrincipalList: new Array(),
  selectedRecipientUserKey: null,
  wsRef: null,
  reconnectTimeoutRef: null,
  pollingIntervalRef: null,
  // actions: {
  //   updateActor: (actor) => set(() => ({ actor: actor })),
  //   updateMessageList: (messageList) =>
  //     set(() => ({ messageList: messageList })),

  // },
}));

export const useChatStoreActor = () => useStore((state) => state.actor);
export const useChatStoreMessageList = () =>
  useStore((state) => state.messageList);
export const useChatStoreMessageInput = () =>
  useStore((state) => state.messageInput);
export const useChatStoreRecipientUserKeyList = () =>
  useStore((state) => state.recipientPrincipalList);
export const useChatStoreSelectedRecipientUserKey = () =>
  useStore((state) => state.selectedRecipientUserKey);
export const useChatStoreWsRef = () => useStore((state) => state.wsRef);
export const useChatStoreReconnectTimeoutRef = () =>
  useStore((state) => state.reconnectTimeoutRef);
export const useChatStorePollingIntervalRef = () =>
  useStore((state) => state.pollingIntervalRef);
