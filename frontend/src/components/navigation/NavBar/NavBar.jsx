import Container from 'react-bootstrap/Container';
import Navbar from 'react-bootstrap/Navbar';
import { Button, Nav, NavDropdown } from 'react-bootstrap';
import { logout_api } from '../../backend_api/login';
import usePersistentState from '../../../_helpers/UsePersistent';
import useCustomFetch from '../../../_helpers/CustomFetchHook';
import { useDispatch, useSelector } from 'react-redux';
import { reset, setUser } from '../../../_store/userSlice';
import { Link } from 'react-router-dom';
export default function NavBar() {
    return (
        <Navbar expand="lg" bg='primary' className='bg-body-tetiary'>
            <Container fluid>
                <Navbar.Brand href='/'>NewsRss</Navbar.Brand>
                <Navbar.Text>
                    {/* <InfoPopover></InfoPopover> */}
                </Navbar.Text>
                <Navbar.Collapse id='navbar-colapse-login' className='justify-content-end'>
                    <SelectInfo></SelectInfo>
                </Navbar.Collapse>
                {/* <Navbar.Brand href='/login'>I am home</Navbar.Brand> */}
            </Container>
        </Navbar>
    );
}

function SelectInfo() {
    const userInfo = useSelector((state) => state.user);
    const dispatch = useDispatch();
    const [isLoading,data,err,logout] = useCustomFetch(logout_api,(data)=>{dispatch(reset())});
    if (userInfo.email!==null) {
        return <>
            {userInfo.role=="admin" &&
            <>
                <Button variant="secondary">
                    <Link to={"/sources"}>Sources</Link>
                </Button>
                <Button variant="secondary">
                    <Link to={"/themes"}>Themes</Link>
                </Button>
            </>
            }
            <Navbar.Text className='m-1'>
                Logged as: {userInfo.email}
            </Navbar.Text>
            <Button variant='secondary' onClick={logout}>
                Logout
            </Button>
        </>
    }
    return <>
        <Nav.Link href='/login'>Login</Nav.Link>
    </>;
}
