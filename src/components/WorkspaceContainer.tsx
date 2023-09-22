/* eslint-disable no-useless-escape */
import React, { useEffect, useState } from 'react';

import { DeleteOutlined, EditOutlined, SaveOutlined } from '@ant-design/icons';
import { Input, Space } from 'antd';

import { STRIPE_BOX_SHADOW } from '../constants';
import { MIDDLE_STYLE } from '../constants/style';
import { useBackendInvoker } from '../hooks';
import { useWorkspaceAction } from '../hooks/useWorkspaceAction';
import { useDatabaseWorkspaceStore } from '../stores';
import AddNewDatabaseContainer from './AddNewDatabaseContainer';

const WorkspaceContainer = () => {
  const [editMode, setEditMode] = useState<boolean>(false);
  const [workspaceCurrentName, setWorkspaceCurrentName] = useState<string>();
  const { handleFetchWorkspaces } = useWorkspaceAction();
  const { handleDeleteWorkspaceById, handleUpdateNameofWorkspace } = useBackendInvoker();
  const { workspaces, selectedWorkspace } = useDatabaseWorkspaceStore();

  const handleToggleEditMode = async () => {
    setEditMode(!editMode);
  };

  const handleDeleteWorkspace = async () => {
    if (!selectedWorkspace) return;
    await handleDeleteWorkspaceById(selectedWorkspace);
    await handleFetchWorkspaces();
  };

  const handleSave = async () => {
    if (!selectedWorkspace) return;
    await handleUpdateNameofWorkspace(
      selectedWorkspace,
      workspaceCurrentName || workspaces[selectedWorkspace].name
    );
    await handleFetchWorkspaces();
    handleToggleEditMode();
  };

  useEffect(() => {
    if (!selectedWorkspace) return;
    setWorkspaceCurrentName(workspaces[selectedWorkspace]?.name);
  }, [selectedWorkspace, workspaces]);

  return (
    <div className="grid-style-background" style={{ width: '100%', height: '100%' }}>
      {selectedWorkspace && workspaces[selectedWorkspace || ''] ? (
        <React.Fragment>
          <div
            className="border-bottom search-section"
            style={{
              width: 'calc(100% - 280px)',
              padding: '5px 0px',
              borderRadius: 5,
              position: 'fixed',
              boxShadow: STRIPE_BOX_SHADOW,
              zIndex: 100,
            }}>
            <div style={{ padding: '7px 20px', ...MIDDLE_STYLE, justifyContent: 'space-between' }}>
              {editMode ? (
                <div style={{ ...MIDDLE_STYLE }}>
                  <Input
                    value={workspaceCurrentName}
                    onChange={e => setWorkspaceCurrentName(e.target.value)}
                  />
                  <SaveOutlined onClick={handleSave} />
                </div>
              ) : (
                <div style={{ ...MIDDLE_STYLE }}>
                  <h4>{workspaces[selectedWorkspace || '']?.name}</h4>{' '}
                  <EditOutlined onClick={handleToggleEditMode} style={{ marginLeft: 10 }} />
                </div>
              )}
              <Space>
                <DeleteOutlined onClick={handleDeleteWorkspace} />
              </Space>
            </div>
          </div>
          <div style={{ ...MIDDLE_STYLE, height: '100%', width: '100%' }}>
            <AddNewDatabaseContainer />
          </div>
        </React.Fragment>
      ) : (
        <React.Fragment>
          <div style={{ ...MIDDLE_STYLE, height: '100%', width: '100%' }}>
            <AddNewDatabaseContainer />
          </div>
        </React.Fragment>
      )}
    </div>
  );
};

export default WorkspaceContainer;
