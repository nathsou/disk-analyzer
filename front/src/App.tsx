import React from 'react';
import { QueryClient, QueryClientProvider } from 'react-query';
import { AxiosProvider } from './AxiosProvider';
import axios from 'axios';
import { DirContents } from './DirContents';
import { Route } from 'wouter';
import { Home } from './Home';
import { LargestFiles } from './LargestFiles';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      refetchOnMount: false,
      refetchIntervalInBackground: false,
      staleTime: 1000 * 60 * 60 * 24,
      refetchInterval: false
    }
  }
});

const axiosInstance = axios.create({
  baseURL: 'http://localhost:3030/api/'
});

function App() {
  return (
    <AxiosProvider instance={axiosInstance}>
      <QueryClientProvider client={queryClient}>
        <Route path='/ls/:path*'>
          {({ path }) => <DirContents path={`/${path ?? ''}`} />}
        </Route>

        <Route path='/dir/:path*'>
          {({ path }) => <LargestFiles path={`/${path ?? ''}`} />}
        </Route>

        <Route path='/'>
          <Home />
        </Route>
      </QueryClientProvider>
    </AxiosProvider>
  );
}

export default App;
