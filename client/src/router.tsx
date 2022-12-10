import * as React from 'react';
import { createBrowserRouter } from 'react-router-dom';
import { ThemeProvider } from '@mui/system';
import { CssBaseline } from '@mui/material';
import { createTheme } from '@mui/material/styles';
import Home from './pages/main/home/Home';
import Menu from './core/Menu';
import Todo from './pages/misc/Todo';
import Shoutout from './pages/tools/shoutout/Shoutout';
import ShoutoutConfig from './pages/main/shoutout/ShoutoutConfig';
import AuthPage from './pages/main/auth/AuthPage';

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
      <ThemeProvider theme={makeTheme}>
        <CssBaseline />
        <Menu />
      </ThemeProvider>
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