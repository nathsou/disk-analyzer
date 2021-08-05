import React, { FC } from 'react';
import { useQuery } from 'react-query';
import { Link } from 'wouter';
import { useAxios } from './AxiosProvider';
import { formatSize } from './utils';

export interface LargestFilesProps {
  path: string
}

export const LargestFiles: FC<LargestFilesProps> = ({ path }) => {
  const axios = useAxios();
  const largest = useQuery(['dir', path], async () => {
    const { data } = await axios.get('dir', {
      params: {
        path
      }
    });

    return data;
  });

  if (largest.isError) {
    return <p>Error</p>;
  }

  if (largest.isLoading) {
    return <p>Loading...</p>;
  }

  return (
    <div style={{ marginLeft: 4 }}>
      <p>{largest.data?.files_count} files ({formatSize(largest.data?.size)})</p>
      <h4>Directories</h4>
      <ul>
        {largest.data?.biggest_dirs.map(({ size, path: dirPath }: { size: number, path: string }) =>
          <li key={dirPath}>
            <Link href={`/dir${dirPath}`}>{`${dirPath.replace(path, '')}`}</Link> {formatSize(size)}
          </li>
        )}
      </ul>

      <h4>Files</h4>
      <ul>
        {largest.data?.biggest_files.map(({ size, path: filePath }: { size: number, path: string }) =>
          <li key={filePath}>{filePath.replace(`${path}/`, '')} {formatSize(size)}</li>
        )}
      </ul>
    </div>
  );
};