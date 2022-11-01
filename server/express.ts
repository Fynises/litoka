import express from 'express';
import path from 'path';
import config from '../config/config';

const app = express();

app.use('/dist', express.static(path.join(process.cwd(), 'dist')));

app.get('*', (req, res) => {
  res.sendFile(path.join(process.cwd(), 'public/index.html'));
});

app.get('/api/getwebsocket', (req, res) => {
  try {
    res.json({wsUrl: config.wsUrl});
  } catch (err) {
    return res.status(400).json({ error: err });
  }
});

export default app;