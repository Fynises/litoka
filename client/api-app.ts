const fetchWsUrl = async (signal: AbortSignal) => {
  try {
    const response = await fetch('/api/getwebsocket', { method: 'GET', signal: signal });
    return await response.json();
  } catch (err) {
    console.log(err);
  }
};

export {
  fetchWsUrl
};