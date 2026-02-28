import { create } from "zustand";
import syncService from "../services/sync-service";
import { getErrorMessage } from "../utils";

interface SyncState {
	isSyncAvailable: boolean;
	isSyncing: boolean;
	syncError: string | null;
	lastSyncMessage: string | null;

	checkSyncStatus(): Promise<void>;
	pushToCloud(): Promise<void>;
	pullFromCloud(): Promise<void>;
	clearSyncMessage(): void;
}

const useSyncStore = create<SyncState>((set) => ({
	isSyncAvailable: false,
	isSyncing: false,
	syncError: null,
	lastSyncMessage: null,

	checkSyncStatus: async () => {
		const available = await syncService.checkSyncStatus();
		set({ isSyncAvailable: available });
	},

	pushToCloud: async () => {
		set({ isSyncing: true, syncError: null, lastSyncMessage: null });
		try {
			const count = await syncService.pushToCloud();
			set({
				isSyncing: false,
				lastSyncMessage: `Pushed ${count} character${count !== 1 ? "s" : ""} to cloud`,
			});
		} catch (error) {
			set({ syncError: getErrorMessage(error), isSyncing: false });
		}
	},

	pullFromCloud: async () => {
		set({ isSyncing: true, syncError: null, lastSyncMessage: null });
		try {
			const characters = await syncService.pullFromCloud();
			set({
				isSyncing: false,
				lastSyncMessage: `Pulled ${characters.length} character${characters.length !== 1 ? "s" : ""} from cloud`,
			});
		} catch (error) {
			set({ syncError: getErrorMessage(error), isSyncing: false });
		}
	},

	clearSyncMessage: () => {
		set({ syncError: null, lastSyncMessage: null });
	},
}));

export default useSyncStore;
