import { FolderOpenOutlined } from '@ant-design/icons';

import { MIDDLE_STYLE } from '../constants/style';
import { useBackendInvoker } from '../hooks';
import { useDatabaseWorkspaceStore } from '../stores';

const WorkspaceHeader = () => {
  const { handleOpenFolder } = useBackendInvoker();
  const { selectedWorkspace, workspaces } = useDatabaseWorkspaceStore();
  return (
    <div
      style={{
        ...MIDDLE_STYLE,
        justifyContent: 'space-between',
        width: '100%',
        padding: '0px 10px',
      }}>
      <div style={{ fontSize: 'smaller' }}>
        <span style={{ fontWeight: 'bold' }}>Path:</span>
        {workspaces[selectedWorkspace || '']?.path}
      </div>
      <div>
        <div
          style={{ cursor: 'pointer', fontSize: 12 }}
          onClick={() => handleOpenFolder(workspaces[selectedWorkspace || '']?.path)}>
          Open File <FolderOpenOutlined />
        </div>
      </div>
    </div>
  );
};

export default WorkspaceHeader;
