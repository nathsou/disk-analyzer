import React, { FC, useCallback } from 'react';
import { useQuery, UseQueryResult } from 'react-query';
import { useAxios } from './AxiosProvider';
import { Link, useLocation } from 'wouter';
import { RepartitionPlot } from './RepartitionPlot';
import { Path } from './Path';
import { formatSize, joinPaths } from './utils';

export interface DirContentsProps {
  path: string
}

type DirContentsData<ShowDirSize extends boolean> = {
  files: Array<{ path: string, size: number }>,
  directories: Array<{ path: string, size: ShowDirSize extends true ? number : null }>,
  size: number,
};

const useDirContents = <ShowDirSize extends boolean>(
  path: string,
  showDirSize: ShowDirSize
): UseQueryResult<DirContentsData<ShowDirSize>> => {
  const axios = useAxios();

  return useQuery(['ls', path], async () => {
    const { data } = await axios.get('ls', {
      params: {
        path,
        show_dir_size: showDirSize,
      }
    });

    return data;
  });
};

export const DirContents: FC<DirContentsProps> = ({ path }) => {
  const [, setLocation] = useLocation();
  const ls = useDirContents(path, true);

  const redirect = useCallback((path: string) => {
    setLocation(`/ls${path}`);
  }, []);

  if (ls.isError) {
    return <p>Error</p>;
  }

  if (ls.isLoading || ls.data === undefined) {
    return <p>Loading...</p>;
  }

  const { files, directories, size } = ls.data;

  return (
    <>
      <div style={{ display: 'flex', justifyContent: 'center', margin: '10px' }}>
        <Path path={path} />
      </div>
      <div style={{ display: 'flex', justifyContent: 'center' }}>
        <div>
          <h3 style={{ textAlign: 'center' }}>Directories</h3>
          <RepartitionPlot items={directories} onClick={redirect} />
          <ul>
            <li>Total: <b>{formatSize(size)}</b></li>
            {directories.map(({ path: dirName, size }) =>
              <li key={dirName}>
                <Link href={joinPaths('/ls', dirName)}>{dirName.replace(path, '')}</Link> - {formatSize(size)}
              </li>
            )}
          </ul>
        </div>

        <div>
          <h3 style={{ textAlign: 'center' }}>Files</h3>
          <RepartitionPlot items={files} />
          <ul>
            {files.map(({ path: fileName, size }) =>
              <li key={fileName}>{fileName.replace(`${path}/`, '')} - {formatSize(size)}</li>
            )}
          </ul>
        </div>
      </div>
    </>
  );
};