import React, { FC } from 'react';
import { DirContents } from './DirContents';

export const Home: FC<{}> = () => {
  return <DirContents path="/" />;
};