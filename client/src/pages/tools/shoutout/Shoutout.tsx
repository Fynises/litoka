import * as React from 'react';
import Player from './Player';

const Shoutout = () => {
  let message: string | undefined = undefined;
  let hasError = false;

  const searchParams: URLSearchParams = new URLSearchParams(document.location.search);
  if (!searchParams.has('channel')) {
    message = 'there is an error with the url parameters, refer to: https://github.com/Fynises/twitch-open-so for further documentation';
    console.log(message);
    hasError = true;
  }

  return (
    <>
      <p>{message}</p>
      {
        !hasError && (<span>
          <Player />
        </span>)
      }
    </>
  );
};

export default Shoutout;