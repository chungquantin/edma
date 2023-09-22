import { FolderOpenOutlined } from '@ant-design/icons';
import { Col, Row } from 'antd';

import { MIDDLE_STYLE } from '../constants/style';
import { useBackendInvoker } from '../hooks';
import { useDatabaseWorkspaceStore } from '../stores';

const WorkspaceStatusBar = () => {
  const { homeDirectoryPath, shellDirectoryPath } = useDatabaseWorkspaceStore();
  const { handleOpenFolder } = useBackendInvoker();
  return (
    <Row
      className="status-container"
      gutter={25}
      style={{
        ...MIDDLE_STYLE,
      }}>
      <Col
        className="status-item"
        span={12}
        style={{ ...MIDDLE_STYLE, justifyContent: 'space-between' }}>
        <h4>Home Path</h4>
        <p style={{ ...MIDDLE_STYLE }}>
          {homeDirectoryPath}{' '}
          <div
            style={{ cursor: 'pointer', fontSize: 12, marginLeft: 15 }}
            onClick={() => handleOpenFolder(`${homeDirectoryPath}`)}>
            <FolderOpenOutlined />
          </div>
        </p>
      </Col>
      <Col
        className="status-item"
        span={12}
        style={{ ...MIDDLE_STYLE, justifyContent: 'space-between' }}>
        <h4>Shell Path</h4>
        <p style={{ ...MIDDLE_STYLE }}>
          {shellDirectoryPath}{' '}
          <div
            style={{ cursor: 'pointer', fontSize: 12, marginLeft: 15 }}
            onClick={() => handleOpenFolder(`${shellDirectoryPath}`)}>
            <FolderOpenOutlined />
          </div>
        </p>
      </Col>
    </Row>
  );
};

export default WorkspaceStatusBar;
