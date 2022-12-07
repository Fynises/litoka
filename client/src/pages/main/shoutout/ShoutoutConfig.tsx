import * as React from 'react';
import { useState, useEffect } from 'react';
import {
  Alert,
  Box,
  Card,
  Checkbox,
  Divider,
  FormControl,
  FormControlLabel,
  IconButton,
  Paper,
  Radio,
  RadioGroup,
  Snackbar,
  TextField,
  Tooltip,
  Typography
} from '@mui/material';
import { ContentCopyOutlined } from '@mui/icons-material';
import ShoutoutInfo from './ShoutoutInfo';

interface URLParameters {
  base_url: string,
  channel: string,
  allow_mods: boolean,
  allow_vip: boolean,
  allow_subs: boolean,
  disable_overrides: boolean,
  enable_filtering: boolean,
  filter_type: string,
  filter_params: string
}

const ShoutoutConfig = () => {

  const [resultUrl, setResultUrl] = useState<string>();

  const [urlParams, setUrlParams] = useState({
    base_url: 'https://litoka.net/tools/shoutout',
    channel: '',
    allow_mods: true,
    allow_vip: false,
    allow_subs: false,
    disable_overrides: false,
    filter_type: '',
    filter_params: '1'
  } as URLParameters);

  const [snackBar, setSnackBar] = useState<boolean>(false);

  const [filterValue, setFilterValue] = useState<string>('topViewed');
  const [filterPrompt, setFilterPrompt] = useState<string>();

  const buildUrl = () => {
    const url = new URL(urlParams.base_url);
    url.searchParams.append('channel', urlParams.channel.toLowerCase());
    if (urlParams.allow_mods) url.searchParams.append('allowMods', 'true');
    if (urlParams.allow_vip) url.searchParams.append('allowVip', 'true');
    if (urlParams.allow_subs) url.searchParams.append('allowSubs', 'true');
    if (urlParams.disable_overrides) url.searchParams.append('disableOverrides', 'true');
    if (urlParams.enable_filtering) {
      url.searchParams.append('filterType', urlParams.filter_type);
      url.searchParams.append('filterParams', urlParams.filter_params.toString());
    }
    setResultUrl(url.href);
  };

  useEffect(() => {
    buildUrl();
    createFilterPrompt();
    setFilterUrlParams();
  });

  const setFilterUrlParams = () => {
    setUrlParams({ ...urlParams, filter_type: filterValue });
  };

  const urlParamHasError = (): boolean => {
    if (urlParams.filter_params.includes('e')) {
      return true;
    }
    try {
      if (parseInt(urlParams.filter_params) < 1) {
        return true;
      }
    } catch (error) {
      return true;
    }
    return false;
  };

  const copyToClipboard = () => {
    navigator.clipboard.writeText(resultUrl as string);
    handleAlertOpen();
  };

  const handleAlertOpen = () => {
    setSnackBar(true);
  };

  const handleAlertClose = () => {
    setSnackBar(false);
  };

  const handleRadioChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setFilterValue((event.target as HTMLInputElement).value);
  };

  const createFilterPrompt = () => {
    switch (filterValue) {
      case 'topViewed':
        setFilterPrompt('Number of top viewed clips');
        break;
      case 'number':
        setFilterPrompt('Number of most recent clips');
        break;
      case 'time':
        setFilterPrompt('Number of days to filter from');
        break;
    }
  };

  return (
    <Box sx={{ marginTop: 2 }}>
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
            <FormControlLabel
              label='Disable Overrides'
              control={
                <Checkbox
                  checked={urlParams.disable_overrides}
                  onChange={() => setUrlParams({ ...urlParams, disable_overrides: !urlParams.disable_overrides })}
                />
              }
            />
            <FormControlLabel
              label='Enable filtering'
              control={
                <Checkbox
                  checked={urlParams.enable_filtering}
                  onChange={() => setUrlParams({ ...urlParams, enable_filtering: !urlParams.enable_filtering })}
                />
              }
            />
          </FormControl>
          {
            urlParams.enable_filtering &&
            <Box sx={{ paddingLeft: 2, paddingBottom: 1 }}>
              <Card variant='outlined' sx={{ paddingLeft: 2, paddingBottom: 1 }}>
                <FormControl>
                  <RadioGroup
                    defaultValue='topViewed'
                    defaultChecked={true}
                    value={filterValue}
                    onChange={handleRadioChange}
                  >
                    <FormControlLabel value='topViewed' control={<Radio />} label='Top Viewed' />
                    <FormControlLabel value='number' control={<Radio />} label='Number Recent' />
                    <FormControlLabel value='time' control={<Radio />} label='Time' />
                  </RadioGroup>
                  <TextField
                    sx={{ marginTop: 1 }}
                    required
                    type='number'
                    id='outlined-basic'
                    variant='outlined'
                    InputProps={{ inputProps: { min: 1 } }}
                    label={filterPrompt}
                    value={urlParams.filter_params}
                    error={urlParamHasError()}
                    onChange={e => setUrlParams({ ...urlParams, filter_params: e.target.value })}
                  />
                </FormControl>
              </Card>
            </Box>
          }
          <Divider />
          <Card variant='outlined' sx={{ display: 'flex', marginTop: 1 }}>
            <Tooltip title='Copy to clipboard'>
              <IconButton aria-label='copy-clipboard' onClick={() => copyToClipboard()}>
                <ContentCopyOutlined />
              </IconButton>
            </Tooltip>
            <Snackbar open={snackBar} autoHideDuration={6000} onClose={handleAlertClose}>
              <Alert onClose={handleAlertClose} severity='success' variant='filled'>
                Copied link to clipboard
              </Alert>
            </Snackbar>
            <Typography component='div' sx={{ padding: 1, whiteSpace: 'nowrap', overflow: 'auto' }}>
              {resultUrl}
            </Typography>
          </Card>
        </Box>
      </Paper>
      <ShoutoutInfo />
    </Box>
  );
};

export default ShoutoutConfig;