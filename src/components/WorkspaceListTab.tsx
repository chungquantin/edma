/* eslint-disable react-hooks/exhaustive-deps */
import { useEffect, useMemo } from 'react';

import { PlusOutlined } from '@ant-design/icons';
import { Space, Tooltip } from 'antd';

import { MIDDLE_STYLE } from '../constants/style';
import { useWorkspaceAction } from '../hooks/useWorkspaceAction';
import { useDatabaseWorkspaceStore } from '../stores';
import { generateHSLColor, shortenString } from '../utils';

const WorkspaceListTab = () => {
  const { handleImportWorkspace } = useWorkspaceAction();
  const { selectWorkspace, selectedWorkspace, workspaces } = useDatabaseWorkspaceStore();

  const workspaceNames = useMemo(
    () =>
      Object.keys(workspaces)
        .sort((nameA, nameB) => (nameA > nameB ? 1 : -1))
        .map(item => item),
    [workspaces]
  );

  useEffect(() => {
    selectWorkspace(workspaceNames.length > 0 ? workspaceNames?.[0] : undefined);
  }, [workspaceNames]);

  return (
    <Space direction="vertical" style={{ width: '100%' }}>
      <div style={{ fontSize: 'smaller', marginBottom: 10, ...MIDDLE_STYLE }}>
        <div className="sider-button" onClick={handleImportWorkspace}>
          <PlusOutlined />
          Add workspace
        </div>
      </div>
      {workspaceNames.map(item => (
        <div
          onClick={() => selectWorkspace(item)}
          className={
            selectedWorkspace === item
              ? 'history-directory-item-selected'
              : 'history-directory-item'
          }>
          <div style={{ ...MIDDLE_STYLE, justifyContent: 'space-between' }}>
            <div>
              <div style={{ ...MIDDLE_STYLE, justifyContent: 'flex-start' }}>
                <div
                  style={{
                    width: 10,
                    height: 10,
                    backgroundColor: generateHSLColor(workspaces[item].name),
                    borderRadius: 20,
                    marginRight: 10,
                  }}></div>
                {workspaces[item].name}
              </div>
              <Tooltip title={workspaces[item].path}>
                <div className="history-directory-sub">
                  {shortenString(workspaces[item].path, 40)}
                </div>
              </Tooltip>
            </div>
          </div>
        </div>
      ))}
    </Space>
  );
};

export default WorkspaceListTab;
