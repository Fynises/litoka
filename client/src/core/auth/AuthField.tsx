import * as React from 'react';
import { Box, Button, Link } from '@mui/material';
import { useState, useEffect } from 'react';
import { fetchAuthParams } from './api-auth';
import { v4 as uuidv4 } from 'uuid';
import { URLSearchParams } from 'url';

const twitchAuthBaseUrl = 'https://id.twitch.tv/oauth2/authorize?';

const AuthField = () => {

  const [authUrl, setAuthUrl] = useState<string>();

  useEffect(() => {
    const abortController = new AbortController();
    const signal = abortController.signal;
    fetchAuthParams(signal).then((data) => {
      console.log(`fetched auth params from server: ${data}`);
      const params = new URLSearchParams({
        client_id: data.client_id,
        redirect_uri: data.redirect_uri,
        response_type: 'code',
        scope: data.scope,
        state: uuidv4()
      }).toString();
      setAuthUrl(`${twitchAuthBaseUrl}${params}`);
    }).catch(() => {
      console.log('error fetching auth params');
    });
  },[]);

  return (
    <Box>
      <Link href={authUrl}>
        <Button variant='contained' disableElevation={true} sx={{ backgroundColor: '#9146ff' }}>
          Log in with Twitch
        </Button>
      </Link>
    </Box>
  );
};

export default AuthField;