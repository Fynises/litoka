import WebSocket from 'ws';

export interface ClientConnectOptions {
  channel: string,
  allowMods: string,
  filterType: string,
  filterParams: string
}

export interface ClientConnectMessage {
  options: ClientConnectOptions
  hash: string
}

export interface ServerWSObject {
  wsObj: WebSocket,
  clientInfo: ClientConnectMessage
}