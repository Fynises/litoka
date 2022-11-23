import * as React from 'react';
import { useState, useEffect } from 'react';
import {
  Box,
  Card,
  Checkbox,
  Divider,
  FormControl,
  FormControlLabel,
  Paper,
  TextField,
  Typography
} from '@mui/material';

interface URLParameters {
  base_url: string,
  channel: string,
  allow_mods: boolean,
  allow_vip: boolean,
  allow_subs: boolean
}

const ShoutoutConfig = () => {

  const [resultUrl, setResultUrl] = useState<string>();

  const [urlParams, setUrlParams] = useState({
    base_url: 'https://litoka.net/tools/shoutout',
    channel: '',
    allow_mods: true,
    allow_vip: false,
    allow_subs: false
  } as URLParameters);

  const buildUrl = () => {
    const url = new URL(urlParams.base_url);
    url.searchParams.append('channel', urlParams.channel.toLowerCase());
    if (urlParams.allow_mods) url.searchParams.append('allowMods', 'true');
    if (urlParams.allow_vip) url.searchParams.append('allowVip', 'true');
    if (urlParams.allow_subs) url.searchParams.append('allowSubs', 'true');
    setResultUrl(url.href);
  };

  useEffect(() => {
    buildUrl();
  });

  return (
    <Box>
      <Paper sx={{
        paddingTop: 2,
        maxWidth: 800,
        margin: 'auto'
      }}>
        <Box sx={{ paddingLeft: 2, paddingBottom: 2 }}>
          <Typography align='center' variant='h6'>
            Shoutout Player Configurator
          </Typography>
        </Box>
        <Divider />
        <Box sx={{ padding: 2 }}>
          <FormControl fullWidth>
            <TextField
              required
              id='outlined-basic'
              label='twitch username'
              variant='outlined'
              value={urlParams.channel}
              onChange={e => setUrlParams({ ...urlParams, channel: e.target.value })}
            />
            <FormControlLabel
              label='Allow Mods'
              control={
                <Checkbox
                  checked={urlParams.allow_mods}
                  onChange={() => setUrlParams({ ...urlParams, allow_mods: !urlParams.allow_mods })}
                />
              }
            />
            <FormControlLabel
              label='Allow VIP'
              control={
                <Checkbox
                  checked={urlParams.allow_vip}
                  onChange={() => setUrlParams({ ...urlParams, allow_vip: !urlParams.allow_vip })}
                />
              }
            />
            <FormControlLabel
              label='Allow Subscribers'
              control={
                <Checkbox
                  checked={urlParams.allow_subs}
                  onChange={() => setUrlParams({ ...urlParams, allow_subs: !urlParams.allow_subs })}
                />
              }
            />
          </FormControl>
          <Card variant='outlined'>
            <Typography sx={{ padding: 1 }}>
              {resultUrl}
            </Typography>
          </Card>
        </Box>
      </Paper>
    </Box>
  );
};

export default ShoutoutConfig;