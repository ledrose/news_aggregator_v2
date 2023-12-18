import logo from './logo.svg';
import './App.css';
import MainLayout from './Layouts/MainLayout';
import MainPage from './pages/MainPage';
import {Route, Routes} from 'react-router-dom';
import {QueryClientProvider, QueryClient } from 'react-query'

const queryClient = new QueryClient();

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <Routes>
        <Route exact path='/' element={<MainLayout/>}>
          <Route index element={<MainPage/>}/>
        </Route>
      </Routes>
    </QueryClientProvider>
  );
}

export default App;
