import { ClientConnectOptions } from '../types/server-types';
import WebSocket from 'ws';

const wsMap = new Map<string, Map<WebSocket, ClientConnectOptions>>();
export default wsMap;