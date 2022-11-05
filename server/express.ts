import express from 'express';
import path from 'path';
import config from '../config/config';
import wsMap from './websocket-map';

const app = express();
app.use(express.json());

app.use('/dist', express.static(path.join(process.cwd(), 'dist')));

app.get(['/', '/options'], (req, res) => {
  res.sendFile(path.join(process.cwd(), 'public/index.html'));
});

app.get('/api/getwebsocket', (req, res) => {
  try {
    res.json({wsUrl: config.wsUrl});
  } catch (err) {
    return res.status(400).json({ error: err });
  }
});

//for reference later
app.post('/api/testwebsocket', (req, res) => {
  const target: string = req.body.target;
  if (wsMap.get(target) !== undefined) {
    wsMap.get(target).forEach((v,k) => {
      k.send('message from server');
    });
    return res.status(200).json({ message: 'successfully sent websocket messages'});
  } else {
    return res.status(400).json({ message: `channel ${target} is not connected to this service`});
  }
});

export default app;