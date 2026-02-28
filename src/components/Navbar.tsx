import { useEffect, useRef } from "react";
import { Link } from "react-router-dom";
import { FiUploadCloud, FiDownloadCloud } from "react-icons/fi";
import { IconButton, Text } from "@chakra-ui/react";
import { Tooltip } from "./ui/tooltip";
import useSyncStore from "../stores/syncStore";
import useCharacterStore from "../stores/characterStore";

export default function Navbar() {
  const {
    isSyncAvailable,
    isSyncing,
    syncError,
    lastSyncMessage,
    checkSyncStatus,
    pushToCloud,
    pullFromCloud,
    clearSyncMessage,
  } = useSyncStore();

  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    checkSyncStatus();
  }, [checkSyncStatus]);

  useEffect(() => {
    if (syncError || lastSyncMessage) {
      if (timerRef.current) clearTimeout(timerRef.current);
      timerRef.current = setTimeout(() => clearSyncMessage(), 5000);
    }
    return () => {
      if (timerRef.current) clearTimeout(timerRef.current);
    };
  }, [syncError, lastSyncMessage, clearSyncMessage]);

  const handlePush = async () => {
    if (!confirm("This will replace ALL cloud data with your local data. Continue?")) return;
    await pushToCloud();
  };

  const handlePull = async () => {
    if (!confirm("This will replace ALL local data with cloud data. Continue?")) return;
    await pullFromCloud();
    useCharacterStore.getState().getCharacters();
  };

  return (
    <nav className="grid grid-cols-[1fr_auto_1fr] items-center">
      <div />
      <div className="flex items-center gap-8">
        <Link to="/">Dashboard</Link>
        <Link to="characters-list">Characters</Link>
      </div>

      {isSyncAvailable ? (
        <div className="flex items-center justify-end gap-3">
          {(syncError || lastSyncMessage) && (
            <Text fontSize="sm" color={syncError ? "red.400" : "green.400"}>
              {syncError || lastSyncMessage}
            </Text>
          )}
          <Tooltip content="Push to Cloud">
            <IconButton
              aria-label="Push to Cloud"
              onClick={handlePush}
              disabled={isSyncing}
              variant="outline"
              size="sm"
            >
              <FiUploadCloud />
            </IconButton>
          </Tooltip>
          <Tooltip content="Pull from Cloud">
            <IconButton
              aria-label="Pull from Cloud"
              onClick={handlePull}
              disabled={isSyncing}
              variant="outline"
              size="sm"
            >
              <FiDownloadCloud />
            </IconButton>
          </Tooltip>
        </div>
      ) : (
        <div />
      )}
    </nav>
  );
}
