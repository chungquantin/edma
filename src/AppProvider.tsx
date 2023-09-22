import React from 'react';

import SnackbarWrapper from './components/SnackbarWrapper';

type Props = {
  children: React.ReactNode;
};

const AppProvider = ({ children }: Props) => {
  return (
    <SnackbarWrapper>
      <React.Fragment>{children}</React.Fragment>
    </SnackbarWrapper>
  );
};

export default AppProvider;
