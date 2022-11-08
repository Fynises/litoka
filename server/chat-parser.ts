import { TwitchMessage } from '../types/server-types';

const parseIRC = (message: string): TwitchMessage => {
  try {
    const messageSplit: string[] = message.split(';');
    console.log(messageSplit);

    const parsedMessage: TwitchMessage = {
      chatterName: '',
      chatterId: '',
      channel: '',
      isBroadcaster: false,
      isMod: false,
      isSubscriber: false,
      message: ''
    };

    //parse metadata
    for (const element of messageSplit) {
      if (element.includes('user-type=')) {
        break;
      } else if (element.includes('badges=')) {
        if (element.includes('broadcaster')) {
          parsedMessage.isBroadcaster = true;
        }
      } else if (element.includes('display-name=')) {
        parsedMessage.chatterName = element.split('=')[1];
      } else if (element.includes('mod=1')) {
        parsedMessage.isMod = true;
      } else if (element.includes('subscriber=') && !element.includes('subscriber=0')) {
        parsedMessage.isSubscriber = true;
      } else if (element.includes('user-id=')) {
        parsedMessage.chatterId = element.split('=')[1];
      }
    }

    parseFinalMessage(message, parsedMessage);

    return parsedMessage;
  } catch (err) {
    console.log(err);
    return null;
  }
};


const parseFinalMessage = (message: string, parsedMessage: TwitchMessage) => {
  const splitMessage: string[] = message.split(' PRIVMSG #');
  splitMessage.shift();
  const messageProper: string = splitMessage.join('');
  //get channel room and streamer channel name
  parsedMessage.channel = messageProper.split(' :')[0];
  const messageArray: string[] = messageProper.split(' :');
  messageArray.shift();
  let finalMessage = messageArray.join('');
  finalMessage = finalMessage.slice(0, finalMessage.length - 2);
  parsedMessage.message = finalMessage;
};

export {
  parseIRC
};