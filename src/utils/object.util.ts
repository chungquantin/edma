export function iterateObject<T, R>(
  obj: Record<string, T>,
  callback: (key: string, item: T, index: number) => R
) {
  return Object.keys(obj).map((key, index) => callback(key, obj[key], index));
}

export function countObjectItems<T>(obj: Record<string, T>) {
  return Object.keys(obj);
}
