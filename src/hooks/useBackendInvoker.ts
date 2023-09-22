/* eslint-disable react-hooks/exhaustive-deps */
import { useCallback } from 'react';

import { invoke } from '@tauri-apps/api';
import Database from 'tauri-plugin-sql-api';

import { Workspace } from '../stores';
import { formatCommandOutput } from '../utils';

const databasePath = 'sqlite:scripion.db';

const getDatabase = async () => {
  const db = await Database.load(databasePath);
  return db;
};

export const useBackendInvoker = () => {
  const handleInvoke = async <T>(command: string, args?: any) => {
    try {
      const output = await invoke<T>(command, args);
      return output;
    } catch (error: any) {
      throw new Error(error);
    }
  };

  const handleGetShellPath = async () => {
    return handleInvoke<string>('get_shell_path');
  };

  const handleExecuteCommand = async (command: string): Promise<string> => {
    const base64Output = await handleInvoke<string>('execute_command', { command });
    return atob(formatCommandOutput(base64Output)).trim();
  };

  const handleOpenFolder = async (folderPath: string) => {
    await handleExecuteCommand(`open ${folderPath}`);
  };

  const handleOpenTerminalAndExecuteCommand = useCallback((command: string) => {
    const script = `osascript -e 'tell application "Terminal" to do script "${command}"' > /dev/null`;
    return handleExecuteCommand(script);
  }, []);

  const handleAddWorkspace = async (name: string, path: string) => {
    const db = await getDatabase();
    return db.execute(`INSERT INTO Workspaces (name, path) VALUES (?1, ?2);`, [name, path]);
  };

  const handleDeleteWorkspaceById = async (id: string) => {
    const db = await getDatabase();
    await db.execute(`DELETE FROM Workspaces WHERE id=${id}`);
  };

  const handleGetAllWorkspaces = async () => {
    const workspaces: Workspace[] = [];
    return workspaces;
  };

  const handleUpdateNameofWorkspace = async (id: string, name: string) => {
    const db = await getDatabase();
    console.log(id, name);
    await db.execute(`UPDATE Workspaces SET name='${name}' WHERE id=${id}`);
  };

  const handleInitializeDatabase = async () => {
    const db = await Database.load('sqlite:scripion.db');
    db.execute(`
      CREATE TABLE IF NOT EXISTS Workspaces (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        path TEXT NOT NULL
      );
    `);
  };

  return {
    handleInvoke,
    handleGetShellPath,
    handleExecuteCommand,
    handleOpenFolder,
    handleOpenTerminalAndExecuteCommand,
    handleAddWorkspace,
    handleGetAllWorkspaces,
    handleInitializeDatabase,
    handleDeleteWorkspaceById,
    handleUpdateNameofWorkspace,
  };
};
