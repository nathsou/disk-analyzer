
export const formatSize = (bytes: number, isMac = true, precision = 2): string => {
  const kb = isMac ? 1000 : 1024;
  const mb = kb * kb;
  const gb = mb * kb;

  if (bytes < kb) return `${bytes} bytes`;
  if (bytes < mb) return `${(bytes / kb).toFixed(precision)} kb`;
  if (bytes < gb) return `${(bytes / mb).toFixed(precision)} mb`;
  return `${(bytes / gb).toFixed(precision)} gb`;
};