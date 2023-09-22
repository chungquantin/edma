import { create } from 'zustand';
import { devtools } from 'zustand/middleware';

export enum SnackbarType {
  Info,
  Error,
  Success,
}

export type SnackbarItem = {
  name: string;
  description: string;
  type: keyof typeof SnackbarType;
};

export interface SnackbarStoreState {
  queue: SnackbarItem[];
  front: SnackbarItem | null;
  lastModifiedDate: number;
  enqueueNotification: (item: SnackbarItem) => void;
  dequeueNotification: () => void;
}

export const useSnackbarStore = create<SnackbarStoreState>()(
  devtools(set => ({
    queue: [],
    front: null,
    lastModifiedDate: +new Date(),

    enqueueNotification(item) {
      set(state => ({ queue: state.queue.concat([item]), lastModifiedDate: +new Date() }));
    },
    dequeueNotification() {
      set(state => ({
        front: state.queue.shift(),
      }));
    },
  }))
);
