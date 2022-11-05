import * as React from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import ErrorPage from './ErrorPage';
import Shoutout from './Shoutout';


const App = () => {
  return (
    <>
      <Router>
        <Routes>
          <Route path='/'
            element={
              <React.Suspense fallback={<>...</>}>
                <ErrorPage />
              </React.Suspense>
            }
          />
          <Route path='/options'
            element={
              <React.Suspense fallback={<>...</>}>
                <Shoutout />
              </React.Suspense>
            }
          />
        </Routes>
      </Router>
    </>
  );
};

export default App;