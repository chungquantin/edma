/* eslint-disable react-hooks/exhaustive-deps */
import { useEffect } from 'react';

import { Layout } from 'antd';

import AnimatedComponent from './components/AnimatedComponent';
import WorkspaceContainer from './components/WorkspaceContainer';
import WorkspaceHeader from './components/WorkspaceHeader';
import WorkspaceListTab from './components/WorkspaceListTab';
import { STRIPE_BOX_SHADOW } from './constants';
import { MIDDLE_STYLE } from './constants/style';
import { useBackendInvoker } from './hooks';
import './index.scss';
import { Workspace, useDatabaseWorkspaceStore } from './stores';

function App() {
  const { selectedWorkspace } = useDatabaseWorkspaceStore();
  const { setHomeDirectoryPath, setShellDirectoryPath, setWorkspaces } =
    useDatabaseWorkspaceStore();
  const {
    handleInitializeDatabase,
    handleExecuteCommand,
    handleGetShellPath,
    handleGetAllWorkspaces,
  } = useBackendInvoker();

  useEffect(() => {
    const initHistory = async () => {
      const homeDirectoryPath = await handleExecuteCommand('echo $HOME');
      setHomeDirectoryPath(homeDirectoryPath);
      const shellPath = await handleGetShellPath();
      setShellDirectoryPath(shellPath);
    };
    initHistory();
  }, []);

  useEffect(() => {
    const init = async () => {
      await handleInitializeDatabase();
      const workspaceList = await handleGetAllWorkspaces();
      const workspaces: Record<string, Workspace> = {};
      for (const workspaceListItem of workspaceList) {
        workspaces[workspaceListItem.id.toString()] = workspaceListItem;
      }
      setWorkspaces(workspaces);
    };
    init();
  }, []);

  return (
    <div style={{ overflow: 'hidden' }}>
      <div
        className="container"
        style={{ boxShadow: STRIPE_BOX_SHADOW, height: '100vh', width: '100vw' }}>
        <AnimatedComponent.OpacityFadeInDiv delay={300}>
          <Layout>
            {selectedWorkspace && (
              <Layout.Sider width={280} style={{ height: 'calc(100% - 85px)' }}>
                <div
                  style={{
                    overflow: 'auto',
                    height: '100vh',
                    padding: '20px 10px',
                  }}>
                  <WorkspaceListTab />
                </div>
              </Layout.Sider>
            )}
            <Layout.Content style={{ position: 'relative' }}>
              {selectedWorkspace && (
                <Layout.Header style={{ padding: 0, ...MIDDLE_STYLE }}>
                  <WorkspaceHeader />
                </Layout.Header>
              )}
              <div style={{ overflow: 'auto', height: '100%' }}>
                <WorkspaceContainer />
              </div>
            </Layout.Content>
          </Layout>
        </AnimatedComponent.OpacityFadeInDiv>
      </div>
    </div>
  );
}

export default App;
