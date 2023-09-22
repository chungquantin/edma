/* eslint-disable react-hooks/exhaustive-deps */
import React, { useEffect } from 'react';

import { message } from 'antd';

import { useSnackbarStore } from '../stores';
import { delay } from '../utils';

type Props = {
  children: React.ReactElement;
};

const GLOBAL_SNACKBAR_DELAY = 300;

const SnackbarWrapper = ({ children }: Props) => {
  const [messageApi, contextHolder] = message.useMessage();
  const { front, dequeueNotification, lastModifiedDate } = useSnackbarStore();

  useEffect(() => {
    const handleNotification = async () => {
      dequeueNotification();
      await delay(GLOBAL_SNACKBAR_DELAY);
    };
    handleNotification();
  }, [lastModifiedDate]);

  useEffect(() => {
    if (front) {
      let method = messageApi.open;
      switch (front.type) {
        case 'Error':
          method = messageApi.error;
          break;
        case 'Info':
          method = messageApi.info;
          break;
        case 'Success':
          method = messageApi.success;
          break;
      }
      method({
        content: front.name,
      });
    }
  }, [front]);

  return (
    <React.Fragment>
      {contextHolder}
      {children}
    </React.Fragment>
  );
};

export default SnackbarWrapper;
