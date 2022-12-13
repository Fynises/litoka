import * as React from 'react';
import { useEffect } from 'react';
import { Box, Card, CardHeader } from '@mui/material';
import { ScaleLoader } from 'react-spinners';
import AuthJson from './AuthJson';
import axios from 'axios';
import { useCookies } from 'react-cookie';
import { useNavigate } from 'react-router-dom';

interface TokenResponse {
  token: string
}

const AuthPage = () => {

  const navigate = useNavigate();

  const searchParams: URLSearchParams = new URLSearchParams(document.location.search);

  // TODO: find a way to declare setCookie without needing to declare cookies
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const [cookies, setCookie] = useCookies(['login_token']);

  useEffect(() => {
    if (searchParams.has('code')) {
      const successJson = new AuthJson(searchParams).getSuccessJson();
      axios.post<TokenResponse>('/api/auth/login', successJson).then(res => {
        setCookie('login_token', res.data.token, { path: '/' });
        navigate(0);
      }).catch(error => {
        // TODO: handle errors in a more sophisticated way
        console.log(JSON.stringify(error));
      });
    } else {
      // TODO: handle this error in a more sophisticated way
      alert('there was an error in authenticating,');
      navigate(0);
    }
  }, []);

  return (
    <Box sx={{ paddingTop: 2 }}>
      <Card sx={{ maxWidth: 200, margin: 'auto' }}>
        <CardHeader
          title='Authenticating'
        />
        <Box sx={{ display: 'flex', alignItems: 'center', padding: 2 }}>
          <ScaleLoader
            cssOverride={{ display: 'block', margin: 'auto' }}
          />
        </Box>
      </Card>
    </Box>
  );
};

export default AuthPage;