import { Box, Card, CardHeader } from '@mui/material';
import * as React from 'react';
import { ScaleLoader } from 'react-spinners';

const AuthPage = () => {
  return (
    <Box sx={{ paddingTop: 2 }}>
      <Card sx={{ maxWidth: 200, margin: 'auto' }}>
        <CardHeader
          title='Authenticating'
        />
        <Box sx={{ display: 'flex', alignItems: 'center', padding: 2 }}>
          <ScaleLoader
            cssOverride={{ display: 'block', margin: 'auto' }}
          />
        </Box>
      </Card>
    </Box>
  );
};

export default AuthPage;