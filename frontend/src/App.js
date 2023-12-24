import logo from './logo.svg';
import './App.css';
import MainLayout from './Layouts/MainLayout/MainLayout';
import MainPage from './pages/MainPage';
import {Route, Routes} from 'react-router-dom';
import LoginPage from './pages/LoginPage';
import 'bootstrap/dist/css/bootstrap.min.css';
import ListSourcesPage from './pages/admin/ListSourcesPage';
import ListThemesPage from './pages/admin/ListThemesPage';
import ListUsersPage from './pages/admin/ListUsersPage';
import "./colors.css";

function App() {
  return (
      <Routes>
        <Route exact path='/' element={<MainLayout/>}>
          <Route index element={<MainPage/>}/>
          <Route path='/login' element={<LoginPage/>}></Route>
          <Route path='/sources' element={<ListSourcesPage/>}></Route>
          <Route path='/themes' element={<ListThemesPage/>}></Route>
          <Route path='/users' element={<ListUsersPage/>}></Route>
        </Route>
      </Routes>
  );
}

export default App;
