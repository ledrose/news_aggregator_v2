import logo from './logo.svg';
import './App.css';
import MainLayout from './Layouts/MainLayout';
import MainPage from './pages/MainPage';
import {Route, Routes} from 'react-router-dom';
import {QueryClientProvider, QueryClient } from 'react-query'
import LoginPage from './pages/LoginPage';
import 'bootstrap/dist/css/bootstrap.min.css';
import usePersistentState from './_helpers/UsePersistent';
const queryClient = new QueryClient();

function App() {
  const userState = usePersistentState("username","");
  return (
    <QueryClientProvider client={queryClient}>
      <Routes>
        <Route exact path='/' element={<MainLayout userState={userState}/>}>
          <Route index element={<MainPage/>}/>
*         <Route path='/login' element={<LoginPage userState={userState}/>}></Route>
        </Route>
      </Routes>
    </QueryClientProvider>
  );
}

export default App;
