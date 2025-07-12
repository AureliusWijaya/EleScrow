import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';

import NotFoundPage from './pages/NotFoundPage';

const App: React.FC = () => {
  return (
    <Router>
        <Routes>
          <Route path="*" element={<NotFoundPage />} />
        </Routes>
    </Router>
  );
};

export default App;