import React, { createContext, FC, useContext } from 'react';
import { AxiosInstance } from 'axios';

const axiosContext = createContext<AxiosInstance | undefined>(undefined);

export interface AxiosProviderProps {
    instance: AxiosInstance
}

export const useAxios = (): AxiosInstance => {
    const val = useContext(axiosContext);

    if (val === undefined) {
        throw new Error(`called useAxios() outside of an AxiosProvider`);
    }

    return val;
};

export const AxiosProvider: FC<AxiosProviderProps> = ({ instance, children }) => {
    return (
        <axiosContext.Provider value={instance}>
            {children}
        </axiosContext.Provider>
    );
};