import { WebSocketURLApiReturn } from '../../types/api-types';

const fetchWsUrl = async (signal: AbortSignal) => {
  try {
    const response = await fetch('/api/getwebsocket', { method: 'GET', signal: signal });
    return await response.json() as WebSocketURLApiReturn;
  } catch (err) {
    console.log(err);
  }
};

export {
  fetchWsUrl
};