import Container from 'react-bootstrap/Container';
import Navbar from 'react-bootstrap/Navbar';
import { Button, Col, Row, Nav, NavDropdown, Form } from 'react-bootstrap';
import { logout_api } from '../../backend_api/login';
import usePersistentState from '../../../_helpers/UsePersistent';
import useCustomFetch from '../../../_helpers/CustomFetchHook';
import { useDispatch, useSelector } from 'react-redux';
import { reset, resetAllowedSources, setAllowedSources, setUser } from '../../../_store/userSlice';
import { useNavigate } from 'react-router-dom';
import { Link } from 'react-router-dom';
import { useEffect, useRef, useState } from 'react';
import { QueryBlock } from '../../main_page/QuerySettings/QuerySettings';
import "./NavBar.css";
export default function NavBar({passHeaderHeight,reset}) {
    const headerRef = useRef(null);
    useEffect(()=>passHeaderHeight(headerRef.current.offsetHeight),[]);
    return (
        <Navbar ref={headerRef} expand="lg" bg='primary' className='bg-body-tetiary'>
            <Container fluid>
                {/* <Row className='justify-conntent-between'> */}
                    <Col md="1">
                        <Navbar.Brand href='/'>NewsRss</Navbar.Brand>
                    </Col>
                    <Col md="1" className='offset-md-4'>
                        <NavDropdown className='col-query' title="Поиск" drop='down-centered'>
                            <div className='border-search'>
                            {/* <NavDropdown.Item> */}
                                <QueryBlock reset={reset} dispatchQuery={()=>{}}/>
                            {/* </NavDropdown.Item> */}
                            </div>
                        </NavDropdown>
                    </Col>
                    <Col md="3">  
                        <Navbar.Collapse id='navbar-colapse-login'>
                            <SelectInfo reset={reset}></SelectInfo>
                        </Navbar.Collapse>
                    </Col>
                {/* </Row> */}
            </Container>
        </Navbar>
    );
}

function SelectInfo({reset}) {
    const userInfo = useSelector((state) => state.user);
    const navigate = useNavigate();
    const channel = useSelector((state) => state.user.current_channel);
    const dispatch = useDispatch();
    const channelTitle = channel!=null?"Channel: "+channel:"Channel: default"; 
    const [isLoading,data,err,logout] = useCustomFetch(logout_api,(data)=>{dispatch(reset());});
    if (userInfo.email!==null) {
        return (<>
            {userInfo.channels.length>0 &&
                <NavDropdown title={channelTitle} id="channels-nav-dropdown" className='me-4'>
                    <NavDropdown.Item onClick={()=>{dispatch(resetAllowedSources()); reset();}}>Default</NavDropdown.Item>
                    {userInfo.channels.map((el) => 
                        <NavDropdown.Item key={el.name} onClick={()=>{dispatch(setAllowedSources(el)); reset()}}> {el.name}</NavDropdown.Item>
                    )}
                </NavDropdown>
            }
            <NavDropdown title={userInfo.email} id="auth-nav-dropdown">
                <NavDropdown.Item href='/settings'>Настройки</NavDropdown.Item>
                {userInfo.role=="admin" &&
                <>
                    <NavDropdown.Item href='/sources'>Источники</NavDropdown.Item>
                    <NavDropdown.Item href='/themes'>Темы</NavDropdown.Item>
                    <NavDropdown.Item href='/users'>Пользователи</NavDropdown.Item>
                </>
                }
                <NavDropdown.Item onClick={logout}>Выйти</NavDropdown.Item>
            </NavDropdown>
            {/* </Col> */}
        </>)
    }
    return <>
        <Nav.Link href='/login'>Войти</Nav.Link>
    </>;
}
