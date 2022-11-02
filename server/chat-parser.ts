import { ShoutOutCommand } from './server-types';

const parseIRC = (message: string): ShoutOutCommand => {

  /**
   * i know that this code looks awful, it'll be cleaned up in the future... hopefully
   */

  const splittedmessage = message.split(';');
  console.log(splittedmessage);

  //check that the message contains a valid command
  const lastelement: string = splittedmessage[splittedmessage.length-1];
  const splitlast: string = lastelement.split(':')[2];
  if (!splitlast.includes('!so')) {
    return null;
  }

  let userName: string;
  let userId: string;
  let isMod: boolean;

  splittedmessage.forEach((element) => {
    if (element.includes('display-name')) {
      userName = element.split('=')[1];
    } else if (element.includes('mod')) {
      isMod = !element.split('=')[1].includes('0');
    } else if (element.includes('user-id')) {
      userId = element.split('=')[1];
    }
  });

  const fromChannel = lastelement.split('#')[1].split(' ')[0];
  const targetChannel = lastelement.split('#')[1].split('@')[1].replace('\r\n', '');
  const overrideRandom = lastelement.includes('-r');

  const command: ShoutOutCommand = {
    fromChannel: fromChannel,
    userName: userName,
    userId: userId,
    isMod: isMod,
    overrideRandom: overrideRandom,
    targetChannel: targetChannel
  };

  return command;

};

export {
  parseIRC
};