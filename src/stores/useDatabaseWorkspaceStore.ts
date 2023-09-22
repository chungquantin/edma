import { create } from 'zustand';

export type Workspace = { id: string; path: string; name: string };

export enum DatabaseType {
  RocksDB = 'RocksDB',
  SQLite = 'SQLite',
  ReDB = 'ReDB',
}

export interface DatabaseWorkspaceStore {
  selectedWorkspace: string | undefined;
  homeDirectoryPath: string;
  shellDirectoryPath: string;
  workspaces: Record<string, Workspace>;
  setWorkspaces: (workspaces: Record<string, Workspace>) => void;
  addWorkspace: (
    workspaceId: string,
    workspaceName: string,
    path: string,
    type: DatabaseType
  ) => void;
  removeWorkspace: (workspaceId: string) => void;
  selectWorkspace: (workspaceName: string | undefined) => void;
  setHomeDirectoryPath: (path: string) => void;
  setShellDirectoryPath: (path: string) => void;
}

export const useDatabaseWorkspaceStore = create<DatabaseWorkspaceStore>()(set => ({
  selectedWorkspace: undefined,
  homeDirectoryPath: '',
  shellDirectoryPath: '',
  workspaces: {},
  setWorkspaces(workspaces: Record<string, Workspace>) {
    set(state => ({
      ...state,
      workspaces,
    }));
  },
  removeWorkspace(workspaceId: string) {
    set(state => {
      delete state.workspaces[workspaceId];
      return state;
    });
  },
  selectWorkspace(workspaceName: string | undefined) {
    set(state => ({
      ...state,
      selectedWorkspace: workspaceName,
    }));
  },
  addWorkspace(workspaceId: string, workspaceName: string, path: string) {
    set(state => ({
      ...state,
      workspaces: {
        ...state.workspaces,
        [workspaceId]: {
          id: workspaceId,
          name: workspaceName,
          path,
        },
      },
    }));
  },
  setHomeDirectoryPath(path) {
    set(state => ({
      ...state,
      homeDirectoryPath: path,
    }));
  },
  setShellDirectoryPath(path) {
    set(state => ({
      ...state,
      shellDirectoryPath: path,
    }));
  },
}));
