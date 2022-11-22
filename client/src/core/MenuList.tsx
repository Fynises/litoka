import * as React from 'react';
import { Link } from 'react-router-dom';
import {
  Box,
  List,
  ListItemButton,
  ListItemText,
} from '@mui/material';

const MenuList = () => {
  return (
    <Box>
      <List>
        <ListItemButton component={Link} to='/'>
          <ListItemText primary='Home' />
        </ListItemButton>
        <ListItemButton component={Link} to='/config/shoutout'>
          <ListItemText primary='Shoutout Player' />
        </ListItemButton>
      </List>
    </Box>
  );
};

export default MenuList;