import * as React from 'react';
import { createBrowserRouter } from 'react-router-dom';
import Home from './pages/main/Home';
import Menu from './core/Menu';
import Todo from './pages/misc/Todo';
import Shoutout from './pages/tools/shoutout/Shoutout';
import ShoutoutConfig from './pages/main/ShoutoutConfig';

const router = createBrowserRouter([
  {
    path: '/',
    element: <Menu />,
    children: [
      {
        path: '',
        element: <Home />
      },
      {
        path: 'config/shoutout',
        element: <ShoutoutConfig />
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