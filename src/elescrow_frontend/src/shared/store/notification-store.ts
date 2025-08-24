import { create } from "zustand";
import { Notification } from "../../../../declarations/elescrow_backend/elescrow_backend.did";

interface NotificationStoreState {
    notificationList: Array<Notification>;
    actions: NotificationStoreActions;
}

interface NotificationStoreActions {
    setNotificationList: (notificationList: Array<Notification>) => void;
}

export const useNotificationStore = create<NotificationStoreState>()((set) => ({
    notificationList: new Array(),
    actions: {
        setNotificationList: (notificationList) =>
            set(() => ({ notificationList: notificationList })),
    },
}));

export const useNotificationList = () =>
    useNotificationStore((state) => state.notificationList);
export const useNotificationStoreActions = () =>
    useNotificationStore((state) => state.actions);
