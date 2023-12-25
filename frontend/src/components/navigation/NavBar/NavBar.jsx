import Container from 'react-bootstrap/Container';
import Navbar from 'react-bootstrap/Navbar';
import { Button, Col, Row, Nav, NavDropdown } from 'react-bootstrap';
import { logout_api } from '../../backend_api/login';
import usePersistentState from '../../../_helpers/UsePersistent';
import useCustomFetch from '../../../_helpers/CustomFetchHook';
import { useDispatch, useSelector } from 'react-redux';
import { reset, setUser } from '../../../_store/userSlice';
import { Link } from 'react-router-dom';
import { useEffect, useRef, useState } from 'react';
export default function NavBar({passHeaderHeight}) {
    const headerRef = useRef(null);
    useEffect(()=>passHeaderHeight(headerRef.current.offsetHeight),[]);
    return (
        <Navbar ref={headerRef} expand="lg" bg='primary' className='bg-body-tetiary'>
            <Container fluid>
                    <Col md="2">
                        <Navbar.Brand href='/'>NewsRss</Navbar.Brand>
                    </Col>
                    <Col md="1">  
                        <Navbar.Collapse id='navbar-colapse-login'>
                            <SelectInfo></SelectInfo>
                        </Navbar.Collapse>
                    </Col>
            </Container>
        </Navbar>
    );
}

function SelectInfo() {
    const userInfo = useSelector((state) => state.user);
    const dispatch = useDispatch();
    const [isLoading,data,err,logout] = useCustomFetch(logout_api,(data)=>{dispatch(reset())});
    if (userInfo.email!==null) {
        return (
            <NavDropdown title={userInfo.email} id="auth-nav-dropdown">
                {userInfo.role=="admin" &&
                <>
                    <NavDropdown.Item className='' href='/sources'>Источники</NavDropdown.Item>
                    <NavDropdown.Item href='/themes'>Темы</NavDropdown.Item>
                    <NavDropdown.Item href='/users'>Пользователи</NavDropdown.Item>
                </>
                }
                <NavDropdown.Item onClick={logout}>Выйти</NavDropdown.Item>
            </NavDropdown>
        )
    }
    return <>
        <Nav.Link href='/login'>Войти</Nav.Link>
    </>;
}
