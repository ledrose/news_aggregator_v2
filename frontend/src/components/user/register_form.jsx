import {check_login, login_api, register_api} from "../backend_api/login"
import { Link, redirect, useNavigate } from "react-router-dom";
import {Button, Form, Row,Col, Spinner} from "react-bootstrap";
import useCustomFetch from "../../_helpers/CustomFetchHook";
import { reset, setUser } from "../../_store/userSlice";
import { useDispatch } from "react-redux";
import { useState } from "react";
export default function RegisterForm() {
    const navigate = useNavigate();
    const dispatch = useDispatch();
    const [validated,setValidated] = useState(false);
    const [isLoading,data,err,sendRequest] = useCustomFetch(register_api,(data)=>{
        console.log({email:data.email,role:data.role});
        // reset();
        dispatch(setUser({email:data.email,role:data.role}));
        navigate("/");
    });
    const onSubmit = (e) => {
        e.preventDefault()
        const target = e.target;
        const email = target.email.value;
        const password = target.password.value;
        const password_rep = target.password_repeat.value;
        if (password!=password_rep) {
            target.password_repeat.setCustomValidity("1");
        }
        if (target.checkValidity() === false) {
            e.stopPropagation();
        }  else {
            sendRequest(email,password);
        }
        setValidated(true);
    }
    return <>
        <Form onSubmit={onSubmit} validated={validated}>
            <Form.Group controlId="formEmail">
                <Form.Label>Почтовый адрес</Form.Label>
                <Form.Control required name="email" type="email" placeholder="example@mail.ru"></Form.Control>
                <Form.Text className="text-muted">
                    Введите почту
                </Form.Text>
                <Form.Control.Feedback type="invalid">Введите правильную почту</Form.Control.Feedback>
                <div className="mb-3"></div>
            </Form.Group>
            <Form.Group controlId="formPassword">
                <Form.Label>Пароль</Form.Label>
                <Form.Control required className="mb-3" name="password" type="password" placeholder="Пароль"></Form.Control>
                <Form.Control.Feedback type="invalid">Введите пароль</Form.Control.Feedback>
                <div className="mb-3"></div>
                <Form.Control required name="password_repeat" type="password" placeholder="Повторите пароль"></Form.Control>
                <Form.Control.Feedback type="invalid">Пароли должны быть одинаковы</Form.Control.Feedback>
                <div className="mb-3"></div>
            </Form.Group>
            <Form.Group className="mb-3" controlId="formPassword">
            </Form.Group>
            <Row className="justify-content-start">
                <Col sm="3">
                    <Button variant="primary" type="submit">
                        Зарегистрироваться
                    </Button>
                </Col>
            </Row>
        </Form>
        { isLoading===true &&
            <div>Загрузка</div>
        }
    </>
}
