import { useState } from 'react';

import { PlusOutlined, UploadOutlined } from '@ant-design/icons';
import { Button, Input, Select, Space } from 'antd';

import { MIDDLE_STYLE } from '../constants/style';
import { useWorkspaceAction } from '../hooks/useWorkspaceAction';
import { DatabaseType } from '../stores';
import { getFileNameFromPath } from '../utils';

const AddNewDatabaseContainer = () => {
  const { handleSelectDatabaseFile, handleAddDatabase } = useWorkspaceAction();
  const [databaseName, setDatabaseName] = useState<string>('');
  const [databaseType, setDatabaseType] = useState<DatabaseType | undefined>(undefined);
  const [databasePath, setDatabasePath] = useState<string | undefined>(undefined);

  const handleSelectDatabasePath = async () => {
    const _databasePath: string = (await handleSelectDatabaseFile()) as any;
    setDatabasePath(_databasePath);
  };

  const handleAddNewDatabase = async () => {
    if (databaseName.length === 0 || !databaseType || !databasePath) return;
    handleAddDatabase(databaseName, databasePath, databaseType);
  };

  return (
    <div style={{ ...MIDDLE_STYLE, justifyContent: 'flex-start' }}>
      <div
        className="container"
        style={{
          borderRadius: 10,
          padding: '20px 20px',
          width: '500px',
          position: 'relative',
        }}>
        <div style={{ ...MIDDLE_STYLE, justifyContent: 'space-between' }}>
          <h3 style={{ whiteSpace: 'nowrap' }}>New database</h3>
          <Input
            value={databaseName}
            onChange={e => setDatabaseName(e.target.value)}
            style={{ textAlign: 'right' }}
            placeholder="Enter database name"
          />
        </div>
        <Space direction="vertical" style={{ width: '100%', marginTop: 20 }}>
          <div style={{ ...MIDDLE_STYLE, justifyContent: 'space-between' }}>
            <div>Database Type</div>
            <Select
              onChange={setDatabaseType}
              value={databaseType}
              placeholder="Select a database type"
              options={[
                {
                  value: DatabaseType.RocksDB,
                  label: 'RocksDB',
                },
                {
                  value: DatabaseType.SQLite,
                  label: 'SQLite',
                },
              ]}
            />
          </div>
          <div style={{ ...MIDDLE_STYLE, justifyContent: 'space-between' }}>
            <div>File Path</div>
            <div className="sider-button" onClick={handleSelectDatabasePath}>
              {databasePath ? (
                getFileNameFromPath(databasePath)
              ) : (
                <div>
                  <UploadOutlined /> Select path to database
                </div>
              )}
            </div>
          </div>
          <div style={{ marginTop: 20, ...MIDDLE_STYLE, justifyContent: 'flex-end' }}>
            <Button
              disabled={databaseName.length === 0 || !databaseType || !databasePath}
              onClick={handleAddNewDatabase}
              type="primary">
              <PlusOutlined /> Add database
            </Button>
          </div>
        </Space>
      </div>
    </div>
  );
};

export default AddNewDatabaseContainer;
