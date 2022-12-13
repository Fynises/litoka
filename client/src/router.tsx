import * as React from 'react';
import { createBrowserRouter } from 'react-router-dom';
import { ThemeProvider } from '@mui/system';
import { CssBaseline } from '@mui/material';
import { createTheme } from '@mui/material/styles';
import Home from './main/home/Home';
import Menu from './core/Menu';
import Todo from './misc/Todo';
import Shoutout from './tools/shoutout/Shoutout';
import ShoutoutConfig from './main/shoutout/ShoutoutConfig';
import AuthPage from './core/auth/AuthPage';
import { CookiesProvider } from 'react-cookie';

const makeTheme = createTheme({
  palette: {
    background: {
      default: '#cccccc'
    }
  },
});

const router = createBrowserRouter([
  {
    path: '/',
    element: (
      <CookiesProvider>
        <ThemeProvider theme={makeTheme}>
          <CssBaseline />
          <Menu />
        </ThemeProvider>
      </CookiesProvider>
    ),
    children: [
      {
        path: '',
        element: <Home />
      },
      {
        path: 'config/shoutout',
        element: <ShoutoutConfig />
      },
      {
        path: 'twitchcallback',
        element: <AuthPage />
      }
    ]
  },
  {
    path: '/tools',
    children: [
      {
        path: '',
        element: <Todo />
      },
      {
        path: 'shoutout',
        element: <Shoutout />
      }
    ]
  }
]);

export default router;