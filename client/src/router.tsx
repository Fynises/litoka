import * as React from 'react';
import { createBrowserRouter } from 'react-router-dom';
import Home from './pages/main/Home';
import Todo from './pages/misc/Todo';
import Shoutout from './pages/tools/shoutout/Shoutout';

const router = createBrowserRouter([
  {
    path: '/',
    element: <Home />
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