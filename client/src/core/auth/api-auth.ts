import { ApiAuthParams } from '../../types/api-types';

const fetchAuthParams = async (signal: AbortSignal) => {
  try {
    const response = await fetch('/api/get-auth-params', {
      method: 'GET',
      signal: signal
    });
    return await response.json() as ApiAuthParams;
  } catch (err) {
    console.log(err);
    throw err;
  }
};

export { fetchAuthParams };