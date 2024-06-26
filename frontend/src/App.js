import logo from './logo.svg';
import './App.css';
import MainLayout from './Layouts/MainLayout/MainLayout';
import MainPage from './pages/MainPage';
import {Route, Routes} from 'react-router-dom';
import LoginPage from './pages/LoginPage/LoginPage';
import 'bootstrap/dist/css/bootstrap.min.css';
import ListSourcesPage from './pages/admin/ListSourcesPage';
import ListThemesPage from './pages/admin/ListThemesPage';
import ListUsersPage from './pages/admin/ListUsersPage';
import SettingsPage from './pages/User/SettingsPage';
import "./colors.css";
import RegisterPage from './pages/RegisterPage/RegisterPage';

function App() {
  return (
      <Routes>
        <Route exact path='/' element={<MainLayout/>}>
          <Route index element={<MainPage/>}/>
          <Route path='/login' element={<LoginPage/>}></Route>
          <Route path='/register' element={<RegisterPage/>}></Route>
          <Route path='/sources' element={<ListSourcesPage/>}></Route>
          <Route path='/themes' element={<ListThemesPage/>}></Route>
          <Route path='/users' element={<ListUsersPage/>}></Route>
          <Route path='/settings' element={<SettingsPage/>}></Route>
        </Route>
      </Routes>
  );
}

export default App;
