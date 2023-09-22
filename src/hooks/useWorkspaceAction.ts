import { open } from '@tauri-apps/api/dialog';
import { appConfigDir } from '@tauri-apps/api/path';

import { useBackendInvoker } from '.';
import { DatabaseType, Workspace, useDatabaseWorkspaceStore } from '../stores';
import { makeid } from '../utils';

export const useWorkspaceAction = () => {
  const { handleAddWorkspace, handleGetAllWorkspaces } = useBackendInvoker();
  const { addWorkspace, selectWorkspace, setWorkspaces } = useDatabaseWorkspaceStore();

  const handleFetchWorkspaces = async () => {
    const workspaceList = await handleGetAllWorkspaces();
    const workspaces: Record<string, Workspace> = {};
    for (const workspaceListItem of workspaceList) {
      workspaces[workspaceListItem.id.toString()] = workspaceListItem;
    }
    setWorkspaces(workspaces);
  };

  const handleSelectDatabaseFile = async () => {
    const selected = await open({
      directory: false,
      multiple: false,
      title: 'Add embedded databse file',
      defaultPath: await appConfigDir(),
    });
    return selected;
  };

  const handleAddDatabase = async (
    databaseName: string,
    databasePath: string,
    databaseType: DatabaseType
  ) => {
    selectWorkspace(databaseName);
    handleAddWorkspace(databaseName, databasePath);
    addWorkspace(makeid(5), databaseName, databasePath, databaseType);
  };

  return {
    handleFetchWorkspaces,
    handleSelectDatabaseFile,
    handleAddDatabase,
  };
};
