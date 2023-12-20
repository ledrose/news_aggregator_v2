import {useForm, SubmitHandler} from "react-hook-form"
import {check_login, login_api} from "../backend_api/login"
import usePersistentState from "../../_helpers/UsePersistent";
import { redirect, useNavigate } from "react-router-dom";
import { useEffect, useState } from "react";
export default function LoginForm({userState}) {
    const {register, handleSubmit} = useForm();
    const navigate = useNavigate();
    const [logged,setLogged] = useState();
    const [username,setUsername] = userState;
    const onSubmit = (data) => login_api(data.email,data.password).then((j) => {
        setUsername(j.email);
        navigate("/");
    });
    return <>
        <form onSubmit={handleSubmit(onSubmit)}>
            <input type="email" {...register("email", {required: true})}/>
            <input htmlFor="password" type="password" {...register("password",{required:true})}/>
            <input type="submit"></input>
        </form>
        {/* <button onClick={onRolePress}>GetRole</button> */}
    </>
}