import React, { FC } from 'react';
import { useQuery } from 'react-query';
import { useAxios } from './AxiosProvider';
import { Link } from 'wouter';

export interface DirContentsProps {
    path: string
}

export const DirContents: FC<DirContentsProps> = ({ path }) => {
    const axios = useAxios();
    const ls = useQuery(['ls', path], async () => {
        const { data } = await axios.get('ls', {
            params: {
                path
            }
        });

        return data;
    });

    if (ls.isError) {
        return <p>Error</p>;
    }

    if (ls.isLoading) {
        return <p>Loading...</p>;
    }

    return (
        <div style={{ marginLeft: 4 }}>
            <h3>{ }</h3>
            <h4>Directories</h4>
            <ul>
                {ls.data?.directories.map((dirName: string) =>
                    <li key={dirName}>
                        <Link href={`/ls${dirName}`}>{dirName.replace(path, '')}</Link>
                    </li>
                )}
            </ul>

            <h4>Files</h4>
            <ul>
                {ls.data?.files.map((fileName: string) =>
                    <li key={fileName}>{fileName.replace(`${path}/`, '')}</li>
                )}
            </ul>
        </div>
    );
};