import * as React from 'react';
import { useState } from 'react';
import {
  AppBar,
  Toolbar,
  Typography,
  Box,
  IconButton,
  Drawer,
} from '@mui/material';
import MenuIcon from '@mui/icons-material/Menu';
import MenuList from './MenuList';
import { Outlet } from 'react-router-dom';
import AuthField from './auth/AuthField';

const Menu = () => {

  const [state, setState] = useState(false);

  const toggleDrawer = (open: boolean) => (event: React.KeyboardEvent | React.MouseEvent) => {
    if (event.type === 'keydown' && ((event as React.KeyboardEvent).key === 'Tab' || (event as React.KeyboardEvent).key === 'Shift')) {
      return;
    }
    setState(open);
  };

  const list = () => (
    <Box
      sx={{ width: 250 }}
      role='presentation'
      onKeyDown={toggleDrawer(false)}
    >
      <MenuList />
    </Box>
  );

  return (
    <Box sx={{ flexGrow: 1 }}>
      <AppBar position='static' elevation={0} enableColorOnDark sx={{ backgroundColor: '#40826d' }}>
        <Toolbar>
          <IconButton
            size='large'
            edge='start'
            color='inherit'
            aira-label='menu'
            sx={{ mr: 2 }}
            onClick={toggleDrawer(true)}
          >
            <MenuIcon />
          </IconButton>
          <Drawer
            anchor='left'
            open={state}
            onClose={toggleDrawer(false)}
          >
            {list()}
          </Drawer>
          <Typography variant='h6' component='div' sx={{ flexGrow: 1 }}>
            Litoka open source stream tools
          </Typography>
          <AuthField />
        </Toolbar>
      </AppBar>
      <Outlet />
    </Box>
  );
};

export default Menu;