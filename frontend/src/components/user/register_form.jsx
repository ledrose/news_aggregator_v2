import {check_login, login_api, register_api} from "../backend_api/login"
import { Link, redirect, useNavigate } from "react-router-dom";
import {Button, Form, Row,Col, Spinner} from "react-bootstrap";
import useCustomFetch from "../../_helpers/CustomFetchHook";
import { reset, setUser } from "../../_store/userSlice";
import { useDispatch } from "react-redux";
import { useState } from "react";
import { useForm } from "react-hook-form";
export default function RegisterForm() {
    const navigate = useNavigate();
    const dispatch = useDispatch();
    const {register, handleSubmit, formState: {errors}} = useForm();
    const [isLoading,data,err,sendRequest] = useCustomFetch(register_api,(data)=>{
        console.log({email:data.email,role:data.role});
        // reset();
        dispatch(setUser({email:data.email,role:data.role}));
        navigate("/");
    });
    const validatePasword = () => {

    }
    const onSubmit = (data) => {
        console.log(data);
        sendRequest(data.email,data.password);
        // e.preventDefault()
        // const target = e.target;
        // const email = target.email.value;
        // const password = target.password.value;
        // const password_rep = target.password_repeat.value;
        // if (target.checkValidity() === false) {
        //     e.stopPropagation();
        // }  else {
        //     sendRequest(email,password);
        // }
        // setValidated(true);
    }
    return <>
        <Form onSubmit={handleSubmit(onSubmit)}>
            <Form.Group controlId="formEmail">
                <Form.Label>Почтовый адрес</Form.Label>
                <Form.Control required name="email" type="email" placeholder="example@mail.ru" {...register("email",{required: true, pattern: /^[\w\-\.]+@([\w-]+\.)+[\w-]{2,}$/})}></Form.Control>
                {errors.email && <small>Почта должна быть корректной</small>}
                <div className="mb-3"></div>
            </Form.Group>
            <Form.Group controlId="formPassword">
                <Form.Label>Пароль</Form.Label>
                <Form.Control name="password" type="password" placeholder="Пароль" {...register("password",{required: true})} ></Form.Control>
                {errors.password && <small>Пароль не должен быть пустым</small>}
                <div className="mb-3"></div>
                <Form.Control name="password_repeat" type="password" placeholder="Повторите пароль" {...register("password_repeat",{required: true,
                    validate: (value,formValues) => formValues.password==value
                })}></Form.Control>
                {errors.password_repeat && <small>Пароли должны совпадать</small>}
                <div className="mb-3"></div>
            </Form.Group>
            <Row className="justify-content-start">
                <Col sm="3">
                    <Button variant="primary" type="submit">
                        Зарегистрироваться
                    </Button>
                </Col>
            </Row>
        </Form>
    </>
}
