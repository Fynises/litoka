import { ShoutOutCommand, TwitchMessage } from '../types/server-types';

const parseShoutOutCommand = (messageObject: TwitchMessage): ShoutOutCommand => {

  const message: string = messageObject.message;
  if (message.slice(0,3) !== '!so') {
    return null;
  }

  const output: ShoutOutCommand ={
    fromChannel: messageObject.channel,
    requesterName: messageObject.chatterName,
    requesterId: messageObject.chatterId,
    isStreamer: messageObject.isBroadcaster,
    isMod: messageObject.isMod,
    overrideRandom: false,
    targetChannel: ''
  };

  if (message.includes(' -r ')) output.overrideRandom = true;

  output.targetChannel = message.split('@')[1].trim();
  return output;

};

export {
  parseShoutOutCommand
};