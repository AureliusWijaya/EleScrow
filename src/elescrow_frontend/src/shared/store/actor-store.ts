import { ActorSubclass, Identity } from "@dfinity/agent";
import { _SERVICE } from "../../../../declarations/elescrow_backend/elescrow_backend.did";
import { create } from "zustand";

interface ActorStoreState {
    actor: ActorSubclass<_SERVICE> | null;
    actions: ActorActions;
}

interface ActorActions {
    setActor: (actor: ActorStoreState["actor"]) => void;
}

export const useActorStore = create<ActorStoreState>()((set) => ({
    actor: null,
    actions: {
        setActor: (actor) => set(() => ({ actor: actor })),
    },
}));

export const useActor = () => useActorStore((state) => state.actor);
export const useActorStoreActions = () =>
    useActorStore((state) => state.actions);
