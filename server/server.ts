import app from './express';
import config from '../config/config';

app.listen(config.port, () => {
  console.info('Server started on point %s.', config.port);
});