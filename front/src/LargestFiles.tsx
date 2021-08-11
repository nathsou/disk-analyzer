import React, { FC } from 'react';
import { useQuery, UseQueryResult } from 'react-query';
import { Link } from 'wouter';
import { useAxios } from './AxiosProvider';
import { RepartitionPlot } from './RepartitionPlot';
import { formatSize, joinPaths } from './utils';

export interface LargestFilesProps {
  path: string
}

export type DirEndpointData = {
  path: string,
  size: number,
  files_count: number,
  duration: number,
  biggest_dirs: Array<{ path: string, size: number }>,
  biggest_files: Array<{ path: string, size: number }>,
};

export const useLargest = (path: string): UseQueryResult<DirEndpointData> => {
  const axios = useAxios();

  return useQuery(['dir', path], async () => {
    const { data } = await axios.get('dir', {
      params: {
        path
      }
    });

    return data;
  });
};

export const LargestFiles: FC<LargestFilesProps> = ({ path }) => {
  const largest = useLargest(path);

  if (largest.isError) {
    return <p>Error</p>;
  }

  if (largest.isLoading || largest.data === undefined) {
    return <p>Loading...</p>;
  }

  return (
    <div style={{ marginLeft: 4 }}>
      <p>{largest.data?.files_count} files ({formatSize(largest.data.size)})</p>
      <h4>Directories</h4>
      <ul>
        {largest.data?.biggest_dirs.map(({ size, path: dirPath }: { size: number, path: string }) =>
          <li key={dirPath}>
            <Link href={joinPaths('/dir', dirPath)}>{`${dirPath.replace(path, '')}`}</Link> {formatSize(size)}
          </li>
        )}
      </ul>

      <RepartitionPlot items={largest.data.biggest_dirs} />

      <h4>Files</h4>
      <ul>
        {largest.data?.biggest_files.map(({ size, path: filePath }: { size: number, path: string }) =>
          <li key={filePath}>{filePath.replace(`${path}/`, '')} {formatSize(size)}</li>
        )}
      </ul>
    </div>
  );
};