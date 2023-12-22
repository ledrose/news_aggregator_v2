import logo from './logo.svg';
import './App.css';
import MainLayout from './Layouts/MainLayout';
import MainPage from './pages/MainPage';
import {Route, Routes} from 'react-router-dom';
import LoginPage from './pages/LoginPage';
import 'bootstrap/dist/css/bootstrap.min.css';
import usePersistentState from './_helpers/UsePersistent';
import ThemesPage from './pages/ThemesPage';

function App() {
  return (
      <Routes>
        <Route exact path='/' element={<MainLayout/>}>
          <Route index element={<MainPage/>}/>
*         <Route path='/login' element={<LoginPage/>}></Route>
          <Route path='/themes' element={<ThemesPage/>}></Route>
        </Route>
      </Routes>
  );
}

export default App;
