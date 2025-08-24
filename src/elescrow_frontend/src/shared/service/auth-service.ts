import { AuthClient } from "@dfinity/auth-client";
import { Principal } from "@dfinity/principal";
import {
    authActions,
    useAuthStore,
    useIsAuthenticated,
    usePrincipal,
} from "../store/auth-store";
import { Actor, ActorSubclass, HttpAgent, Identity } from "@dfinity/agent";
import {
    useActor,
    useActorStore,
    useActorStoreActions,
} from "../store/actor-store";
import React from "react";
import {
    createActor,
    elescrow_backend,
    idlFactory,
} from "../../../../declarations/elescrow_backend/index";
import { _SERVICE } from "../../../../declarations/elescrow_backend/elescrow_backend.did";
import { useNotificationStore } from "../store/notification-store";
import chatIdlFactory from "../../pages/chat/utils/chat-idl-factory";
import { ApiError } from "../../../../declarations/elescrow_backend/elescrow_backend.did";
import { toast } from "react-toastify";
import { ToastUtils } from "../utils/toast/toast-utils";

export function useLoginToIcp() {
    const { setIsAuthenticated, setPrincipal } = authActions;
    const { setActor } = useActorStoreActions();
    const principal = usePrincipal();

    const loginToICP = async () => {
        try {
            const client = await AuthClient.create();

            await client.login({
                identityProvider: "https://identity.ic0.app/#authorize",
                onSuccess: async () => {
                    setPrincipal(
                        client.getIdentity().getPrincipal().toString()
                    );
                    setIsAuthenticated(true);

                    await client.isAuthenticated();

                    const actor: ActorSubclass<_SERVICE> = createActor(
                        "uxrrr-q7777-77774-qaaaq-cai",
                        {
                            agentOptions: {
                                identity: client.getIdentity(),
                            },
                        }
                    );

                    setActor(actor);
                },
                onError: (err) => {
                    console.error("Login error in callback", err);
                },
            });
        } catch (error) {
            console.error("LoginICP Error", error);
        }
    };

    return { loginToICP };
}

export function useRegister() {
    const isAuthenticated = useIsAuthenticated();
    const actor = useActor();

    const register = async (
        username: string,
        displayName: string,
        email: string
    ) => {
        if (isAuthenticated && actor) {
            await actor
                .register_user({
                    username: username,
                    display_name: [displayName],
                    email: [email],
                    referral_code: [],
                })
                .then((x) => {
                    if ("Err" in x && "AlreadyExists" in x.Err) {
                        ToastUtils.createErrorToast(
                            "You already have an account!"
                        );
                    }
                });
        }
    };

    return { register };
}

export function useLogout() {
    const { setIsAuthenticated, setPrincipal } = authActions;
    const { setActor } = useActorStoreActions();
    const isAuthenticated = useIsAuthenticated();

    const logout = async () => {
        const client = await AuthClient.create();

        if (isAuthenticated && client) {
            await client.logout().then(() => {
                setIsAuthenticated(false);
                setPrincipal(null);
                setActor(null);
            });
        }
    };

    return { logout };
}
