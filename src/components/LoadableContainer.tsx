import React from 'react';

type Props = {
  isLoading: boolean;
  loadComponent: React.ReactElement | React.ReactNode | React.ReactNode[];
  children: React.ReactElement | React.ReactNode | React.ReactNode[];
  style?: React.CSSProperties;
};

const LoadableContainer = ({ loadComponent, isLoading, children, style }: Props) => {
  return (
    <div style={style}>
      {isLoading ? loadComponent : <React.Fragment>{children}</React.Fragment>}
    </div>
  );
};

export default LoadableContainer;
