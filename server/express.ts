import express from 'express';
import path from 'path';

const app = express();

app.use('/dist', express.static(path.join(process.cwd(), 'dist')));

app.get('*', (req, res) => {
  res.sendFile(path.join(process.cwd(), 'public/index.html'));
});

export default app;