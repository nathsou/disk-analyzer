import React, { FC } from 'react';
import { useQuery, UseQueryResult } from 'react-query';
import { useLocation } from 'wouter';
import { useAxios } from './AxiosProvider';
import { joinPaths } from './utils';

export type OSInfo = {
  home: string,
  root: string,
  os: 'linux' | 'macos' | 'ios' | 'freebsd' | 'dragonfly' | 'netbsd' | 'openbsd' | 'solaris' | 'android' | 'windows',
};

export const useOSInfo = (): UseQueryResult<OSInfo> => {
  const axios = useAxios();

  return useQuery('home', async () => {
    const { data } = await axios.get('os_info');
    return data;
  }, {
    staleTime: 24 * 3600 * 1000,
  });
};

export const Home: FC<{}> = () => {
  const info = useOSInfo();
  const [, setLocation] = useLocation();

  if (info.isError) {
    return <p>Error</p>;
  }

  if (info.isLoading || info.data === undefined) {
    return <p>Loading...</p>;
  }

  setLocation(joinPaths('/ls/', info.data.home));

  return null;
};