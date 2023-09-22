import { MenuProps } from 'antd';

export type MenuItem = Required<MenuProps>['items'][number];

export function getItem(args: {
  label: React.ReactNode;
  key: React.Key;
  disabled?: boolean;
  icon?: React.ReactNode;
  children?: MenuItem[];
  type?: 'group';
}): MenuItem {
  return {
    ...args,
  } as MenuItem;
}

export function getNavigationItem<T extends string>(
  label: React.ReactNode,
  key: T,
  disabled?: boolean,
  icon?: React.ReactNode,
  children?: MenuItem[],
  type?: 'group'
) {
  return getItem({ label, key, icon, children, type, disabled });
}
