/* eslint-disable react/no-children-prop */
import * as React from 'react';
import { Box, Paper } from '@mui/material';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import markdown from './markdown';

const ShoutoutInfo = () => {
  return (
    <Box sx={{ marginTop: 2, marginBottom: 4 }}>
      <Paper sx={{
        maxWidth: 800,
        margin: 'auto'
      }}>
        <Box sx={{ padding: 2, code: {backgroundColor: '#f0f0f0'} }}>
          <ReactMarkdown children={markdown} remarkPlugins={[remarkGfm]} />
        </Box>
      </Paper>
    </Box>
  );
};

export default ShoutoutInfo;