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
  const userState = usePersistentState("username",null);
  const errorState = usePersistentState("error",null);
  return (
    <QueryClientProvider client={queryClient}>
      <Routes>
        <Route exact path='/' element={<MainLayout userState={userState} errorState={errorState}/>}>
          <Route index element={<MainPage errorState={errorState}/>}/>
*         <Route path='/login' element={<LoginPage userState={userState} errorState={errorState}/>}></Route>
        </Route>
      </Routes>
    </QueryClientProvider>
  );
}

export default App;
