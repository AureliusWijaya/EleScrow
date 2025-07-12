import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';

import NotFoundPage from './pages/NotFoundPage';

const App: React.FC = () => {
  return (
    <Router>
        <Routes>
          {/* 404 - Not Found */}
          <Route path="*" element={<Layout><NotFoundPage /></Layout>} />
        </Routes>
    </Router>
  );
};

export default App;