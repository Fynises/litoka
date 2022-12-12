/* eslint-disable react/no-children-prop */
import { Box, Button, Card, Divider, Paper, Typography, Link } from '@mui/material';
import * as React from 'react';
import { ReactMarkdown } from 'react-markdown/lib/react-markdown';
import { Link as LinkIcon } from '@mui/icons-material';
import remarkGfm from 'remark-gfm';
import markdown from './markdown';

const Home = () => {

  console.log('not yet implemented');

  return (
    <Box sx={{ paddingTop: 2 }}>
      <Paper sx={{
        paddingTop: 2,
        maxWidth: 1000,
        margin: 'auto'
      }}>
        <Box sx={{ paddingLeft: 2, paddingBottom: 2 }}>
          <Typography align='center' variant='h6'>
            Litoka Open Source Streaming Tools
          </Typography>
        </Box>
        <Divider />
        <Box sx={{ paddingLeft: 2, paddingBottom: 2 }}>
          <ReactMarkdown children={markdown} remarkPlugins={[remarkGfm]} />
        </Box>
        <Divider />
        <Box sx={{ padding: 2 }}>
          <Typography variant='h6'>
            Available Tools
          </Typography>
        </Box>
        <Divider />
        <Box sx={{ padding: 2 }}>
          <Card variant='outlined'>
            <Box sx={{ padding: 1, display: 'flex', alignItems: 'center' }}>
              <Link
                target='_blank'
                rel='noopener'
                href='/config/shoutout'
              >
                <Button
                  variant='contained'
                  startIcon={<LinkIcon />}
                  disableElevation={true}
                >
                  Shoutout Player
                </Button>
              </Link>
              <Typography sx={{ paddingLeft: 2 }}>
                Shout out other streamers with a random clip!
              </Typography>
            </Box>
          </Card>
        </Box>
      </Paper>
    </Box>
  );
};

export default Home;