import React, { FC, useCallback } from 'react';
import { useQuery, UseQueryResult } from 'react-query';
import { useAxios } from './AxiosProvider';
import { Link, useLocation } from 'wouter';
import { RepartitionPlot } from './RepartitionPlot';
import { Path } from './Path';

export interface DirContentsProps {
  path: string
}

type DirContentsData = {
  files: Array<{ path: string, size: number }>,
  directories: Array<{ path: string, size: number | null }>,
};

const useDirContents = (path: string, showDirSize = false): UseQueryResult<DirContentsData> => {
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

  const { files, directories } = ls.data;

  return (
    <>
      <div style={{ display: 'flex', justifyContent: 'center', margin: '10px' }}>
        <Path path={path} />
      </div>
      <div style={{ display: 'flex', justifyContent: 'center' }}>
        <div>
          <h3 style={{ textAlign: 'center' }}>Directories</h3>
          <RepartitionPlot items={directories as any} onClick={redirect} />
          <ul>
            {directories.map(({ path: dirName }) =>
              <li key={dirName}>
                <Link href={`/ls${dirName}`}>{dirName.replace(path, '')}</Link>
              </li>
            )}
          </ul>
        </div>

        <div>
          <h3 style={{ textAlign: 'center' }}>Files</h3>
          <RepartitionPlot items={files} />
          <ul>
            {files.map(({ path: fileName }) =>
              <li key={fileName}>{fileName.replace(`${path}/`, '')}</li>
            )}
          </ul>
        </div>
      </div>
    </>
  );
};