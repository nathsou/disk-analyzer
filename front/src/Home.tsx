import React, { FC } from 'react';

type DirContentsInfo = {
  size: number,
  filesCount: number
};

const ignore = new Set(['.git', 'node_modules']);

const ls = async (dir: FileSystemDirectoryHandle): Promise<DirContentsInfo> => {
  let totalSize = 0; // bytes
  let filesCount = 0;

  const stack = [dir];

  while (stack.length > 0) {
    const dir = stack.pop();
    if (!dir || ignore.has(dir.name)) continue;

    for await (const entry of dir.values()) {
      if (entry.kind === 'directory') {
        stack.push(entry);
      } else {
        totalSize += (await entry.getFile()).size;
        filesCount++;
      }
    }
  }

  return {
    size: totalSize,
    filesCount
  };
};

export const Home: FC<{}> = () => {
  const onClick = async () => {
    const dirHandle = await window.showDirectoryPicker();
    const start = Date.now();
    console.log(await ls(dirHandle));
    console.log(`Took ${Date.now() - start}ms`);
  };

  return (
    <button onClick={onClick}>Open directory</button>
  );
};